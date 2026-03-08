# Arboretum FFI Module

**Rust FFI Bindings for C++ Integration with PostgreSQL**

## Purpose

The `arboretum-ffi` module provides:
1. **C-compatible FFI exports**: Functions that C++ code can call
2. **Direct PostgreSQL Access**: Writes graph data directly to PostgreSQL tables
3. **Simplified Architecture**: No TCP communication, no arboretumd daemon

## Architecture

### Components

| Component | Description |
|-----------|-------------|
| `lib.rs` | Main module with FFI exports and direct PostgreSQL writes |

## Key Components

### TableBuilders

```rust
pub struct TableBuilders {
    db_url: String,
}
```

Manages the PostgreSQL connection pool:

1. **Connection Pooling**: Uses `deadpool_postgres` for efficient connections
2. **No TCP Server**: Eliminated arboretumd TCP server connection
3. **Direct Writes**: FFI calls immediately insert into PostgreSQL

### FFI Functions

Exported with `#[no_mangle]` for C++ compatibility:

```rust
#[no_mangle]
pub extern "C" fn arboretum_connect(url: *const c_char) -> bool;

#[no_mangle]
pub extern "C" fn arboretum_subgraph_id() -> u64;

#[no_mangle]
pub extern "C" fn arboretum_finalize() -> bool;

#[no_mangle]
pub extern "C" fn arboretum_set_db_url(url: *const c_char) -> bool;
```

## PostgreSQL Integration

### Connection Setup

```rust
let config = deadpool_postgres::Config::from_string(&self.db_url)?;
let pool = config.create_pool(None)?;
```

### Direct INSERT

Records are inserted immediately without buffering:

```rust
let mut conn = pool.get().await?;
conn.execute(
    "INSERT INTO FunctionDecl (id, name, is_virtual) VALUES ($1, $2, $3)",
    &[&record.c0, &record.c1, &record.c2],
).await?;
```

## Connection Flow

### Client Initialization

```rust
pub fn new(db_url: String) -> Self {
    // Create table builders with PostgreSQL connection
    Self { db_url }
}
```

### Usage from C++

```cpp
// Connect to PostgreSQL (URL format: postgresql://user@host/db)
if (!arboretum_connect("postgresql://user@localhost/arboretum")) {
    llvm::errs() << "Failed to connect!\n";
}

// Get subgraph ID (placeholder for PostgreSQL)
uint64_t id = arboretum_subgraph_id();

// Emit graph data via FFI (calls into Rust - direct PostgreSQL INSERT)
arboretum_emit_FunctionDecl(id, name, is_defined, is_virtual);

// Finalize (PostgreSQL writes are immediate - no explicit flush needed)
arboretum_finalize();
```

## Message Protocol

### No RECORD_SINK Pattern

| Old Pattern | New Pattern |
|-------------|-------------|
| `RECORD_SINK` closure | Direct `TABLE_BUILDERS` access |
| TCP channel | Direct PostgreSQL INSERT |
| Buffered writes | Immediate INSERT statements |

### Simplified Flow

```
C++ code calls:
  arboretum_emit_FunctionDecl(id, name, is_virtual)

↓
Rust FFI function (no RECORD_SINK!)

↓
Direct PostgreSQL INSERT

↓
PostgreSQL database
```

## FFI Message Flow (Simplified)

```
C++ code
  ↓ arboretum_emit_FunctionDecl()
Rust extern "C" function
  ↓ TABLE_BUILDERS.get_pool()
PostgreSQL connection pool
  ↓ execute("INSERT ...")
PostgreSQL table row inserted
```

## Building

### Dependencies (Cargo.toml)

```toml
[package]
name = "arboretum-ffi"
version = "0.1.0"

[lib]
crate-type = ["staticlib", "rlib"]

[dependencies]
once_cell = "1.19"
deadpool_postgres = "0.12"
tokio = { version = "1.38", features = ["rt"] }
tracing = "0.1"
anyhow = "1.0"
```

### Build Command

```bash
cargo build --release
# Creates target/release/libarboretum_ffi.a
```

## C++ Header

Generated header `arboretum_ffi.h` contains:

```cpp
extern "C" {
bool arboretum_connect(const char* addr);
uint64_t arboretum_subgraph_id();
bool arboretum_finalize();
bool arboretum_set_db_url(const char* url);

void arboretum_emit_FunctionDecl(uint64_t id, const char* name, bool is_virtual);
// ... one function per graph property
}
```

## Integration Points

| Dependency | Purpose |
|------------|---------|
| `reify-rs/` | Data model and PostgreSQL I/O operations |
| `reificator/` | Schema configuration |

### Link Order

When building the full `libarboretum.so`:
1. `libarboretum_ffi.a` (Rust FFI with PostgreSQL)
2. `libreify-cpp.a` (C++ visitor)
3. `plugin.o` (Clang plugin)

## Notes

- Uses `#[no_mangle]` for C++ compatibility
- No TCP server - eliminated arboretumd daemon
- Direct PostgreSQL INSERT statements for simplicity
- Connection pooling via deadpool_postgres for efficiency
- FFI functions must be thread-safe since Clang may call from any context
