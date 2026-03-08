# Reify Rust Module

**Rust AST Reification Library with PostgreSQL Storage**

## Purpose

The `reify-rs` module provides:
1. **FFI Bindings**: Converts between Rust and C++ types for FFI communication
2. **PostgreSQL I/O**: Writes graph data directly to PostgreSQL tables
3. **Data Structures**: Manages record structures and table builders

## Architecture

### Components

| Component | Description |
|-----------|-------------|
| `ffi.rs` | FFI record types and TABLE_BUILDERS global |
| `io.rs` | TableBuilders for PostgreSQL table management |

## Key Modules

### ffi.rs - FFI Record Types

The FFI layer generates record types based on `properties.csv`:

```rust
#[derive(Debug, Serialize, Deserialize)]
pub enum Record {
    Record0(FunctionDeclRecord),
    Record1(CXXRecordDeclRecord),
    // ... one per table
}

// Global TABLE_BUILDERS - no RECORD_SINK anymore
pub static mut TABLE_BUILDERS: Option<TableBuilders> = None;
```

#### Key Changes from Parquet Architecture

| Old | New |
|-----|-----|
| `RECORD_SINK` global closure | `TABLE_BUILDERS` struct with pool |
| TCP communication to arboretumd | Direct PostgreSQL INSERT |
| Buffering and flushing | Immediate writes with optional buffering |

### io.rs - Table Builders

```rust
pub struct TableBuilders {
    db_url: String,
}

impl TableBuilders {
    pub fn new(db_url: impl AsRef<str>) -> Self {
        Self { db_url }
    }

    pub async fn push(&mut self, record: Record) -> Result<()> {
        // Delegates to dynamically generated table builders
    }
}
```

#### Generated TableBuilderN

Each table gets its own builder with direct PostgreSQL access:

```rust
pub struct TableBuilder0 {
    db_url: String,
}

impl TableBuilder0 {
    pub fn new(db_url: impl AsRef<str>, _partition_size: usize) -> Self {
        Self { db_url }
    }

    pub async fn push(&self, record: Record0) -> Result<()> {
        // Build pool
        let pool = self.get_pool()?;
        
        // Direct INSERT - no buffering in this implementation
        let mut conn = pool.get().await?;
        conn.execute(
            "INSERT INTO file (id, filename, content) VALUES ($1, $2, $3)",
            &[&record.c0, &record.c1, &record.c2],
        ).await?;
        
        Ok(())
    }
}
```

## PostgreSQL Integration

### Connection Pooling

Uses `deadpool_postgres` for connection pooling:

```rust
let config = deadpool_postgres::Config::from_string(&self.db_url)?;
let pool = config.create_pool(None)?;
```

### INSERT Statements

Records are inserted directly using prepared statements:

```sql
INSERT INTO FunctionDecl (id, name, is_defined, is_virtual)
VALUES ($1, $2, $3, $4)
```

### Table Naming

Tables are named after the corresponding C++ class:
- `FunctionDecl` → `FunctionDecl` table
- `CXXRecordDecl` → `CXXRecordDecl` table
- `QualType` → `QualType` table

## Data Flow

```
C++ (Clang Plugin)
    ↓ FFI call
Rust FFI function (arboretum_emit_*)
    ↓ Direct INSERT
PostgreSQL connection pool
    ↓ COMMIT
PostgreSQL table
```

### Message Flow (Simplified)

1. **C++ emits record**:
   ```cpp
   arboretum_emit_FunctionDecl(id, name, is_defined, is_virtual);
   ```

2. **Rust FFI receives**:
   ```rust
   #[no_mangle]
   pub extern "C" fn arboretum_emit_FunctionDecl(c0, c1, c2, c3) {
       // Direct PostgreSQL INSERT - no RECORD_SINK!
   }
   ```

3. **PostgreSQL insert**:
   ```rust
   let pool = table_builders.get_pool()?;
   conn.execute("INSERT INTO FunctionDecl (...) VALUES ($1, $2, ...)", ...).await?;
   ```

## Schema Generation

The schema is auto-generated from `properties.csv` via the reificator:

```rust
// Each property becomes a column
pub struct Record0 {
    pub c0: u64,   // id (BIGINT)
    pub c1: String, // name (TEXT)
    pub c2: bool,  // is_defined (BOOL)
    pub c3: bool,  // is_virtual (BOOL)
}
```

### Rust Type Mapping

| DataType | Rust Type | PostgreSQL Type |
|----------|-----------|-----------------|
| BOOL | `bool` | `BOOLEAN` |
| I8 | `i8` | `SMALLINT` |
| I16 | `i16` | `SMALLINT` |
| I32 | `i32` | `INTEGER` |
| I64 | `i64` | `BIGINT` |
| U8 | `u8` | `SMALLINT` |
| U16 | `u16` | `INTEGER` |
| U32 | `u32` | `INTEGER` |
| U64 | `u64` | `BIGINT` |
| F32 | `f32` | `REAL` |
| F64 | `f64` | `DOUBLE PRECISION` |
| STRING | `String` | `TEXT` |

## Building

### Dependencies (Cargo.toml)

```toml
[dependencies]
anyhow = "1.0"
deadpool_postgres = "0.12"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.43", features = ["fs"] }
tracing = "0.1"
```

### Build Output

```bash
cargo build --release
# Creates target/release/libreify_rs.rlib
```

## Integration Points

| Dependency | Purpose |
|------------|---------|
| `arboretum-ffi/` | C++ FFI layer connecting to PostgreSQL |
| `reificator/` | Schema configuration via properties.csv |

### FFI Connection Flow

```
C++ libarboretum.so (via FFI)
    ↓ arboretum_emit_* calls
Rust functions (direct PostgreSQL)
    ↓ deadpool_postgres
PostgreSQL database
```

## Async Runtime

Uses Tokio runtime with features:
- `fs`: File system operations (not used in PostgreSQL version)
- `net`: TCP networking (not used - no daemon)
- `rt`: Runtime support
- `sync`: Synchronization primitives

```rust
// TableBuilders uses async methods for PostgreSQL I/O
pub async fn push(&mut self, record: Record) -> Result<()>
```

## Testing

The module is tested as part of the full build:

```bash
make target/release/libarboretum_ffi.a
```

### Integration Testing

```bash
# Build everything
make arboretum

# Compile with plugin
clang++ -fplugin=./build/libarboretum.so \
    -c test.cpp

# Check PostgreSQL tables
psql -d arboretum -c "SELECT * FROM file;"
```

## 📖 Additional Documentation

| Component | Documentation |
|-----------|---------------|
| [Reificator](../reificator/AGENTS.md) | Schema generation and code generation |
| [Arboretum FFI](../arboretum-ffi/AGENTS.md) | C++ FFI layer |

## Notes

- Generated code sections marked with `BEGIN ARBORETUM GENERATED CODE` (don't edit manually)
- Uses direct PostgreSQL INSERT instead of buffering to files
- Connection pooling via deadpool_postgres for efficiency
- FFI uses `#[no_mangle]` exports for C++ interop
