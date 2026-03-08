# Arboretum

**Analysis of C/C++ Abstract Syntax Trees at Scale**

---

Arboretum is a framework designed for analyzing C/C++ syntax trees at scale. It extracts code properties from C/C++ source files using Clang's AST and stores them directly in PostgreSQL for efficient querying.

## Overview

### What Changed?

Arboretum has been migrated from Parquet file storage with a daemon-based architecture to direct PostgreSQL table storage:

| Before | After |
|--------|-------|
| `arboretumd` daemon | Direct PostgreSQL writes |
| Parquet files on disk | PostgreSQL tables |
| TCP communication (arboretum-ffi → arboretumd) | FFI calls directly to PostgreSQL |
| Buffered I/O | Immediate INSERT statements |

### Key Components

| Component | Location | Description |
|-----------|----------|-------------|
| **reificator** | `reificator/` | Clang plugin for schema generation and AST extraction |
| **arboretum-plugin** | `arboretum-plugin/` | Clang plugin integration |
| **reify-cpp** | `reify-cpp/` | C++ AST visitor library |
| **reify-rs** | `reify-rs/` | Rust AST reification with PostgreSQL I/O |
| **arboretum-ffi** | `arboretum-ffi/` | Rust FFI bindings for C++ integration |

## Quick Start

### Prerequisites

1. **LLVM/Clang**: Build LLVM (~15-20 minutes)
   ```bash
   make llvm-project/build/llvm-stamp
   ```

2. **PostgreSQL**: Install and configure PostgreSQL
   ```bash
   # Ubuntu/Debian
   sudo apt install postgresql postgresql-contrib
   
   # Create database
   sudo -u postgres createdb arboretum
   sudo -u postgres createuser -s $(whoami)
   ```

### Build

```bash
make arboretum
```

### Extract Code Properties

```bash
clang++ -fplugin=./build/libarboretum.so \
    -c your_code.cpp
```

The plugin automatically creates PostgreSQL tables and inserts records for each AST node.

## Architecture

```
┌─────────────────┐     FFI      ┌──────────────────┐
│  Clang Plugin   │ ──────────►  │   Rust FFI       │
│ (reificator)    │              │ (arboretum-ffi)  │
└─────────────────┘              └────────┬─────────┘
                                         │
                                         │ Direct INSERT
                                         ▼
                                 ┌──────────────────┐
                                 │   PostgreSQL     │
                                 │  (arboretum db)  │
                                 └──────────────────┘
```

### Key Differences from Original Architecture

1. **No Daemon**: Eliminated `arboretumd` - writes go directly to PostgreSQL
2. **No TCP**: Removed network communication layer
3. **No Parquet**: Replaced file-based storage with database tables
4. **Simplified**: Direct FFI → PostgreSQL pipeline

## Storage Schema

Tables are dynamically generated from `reificator/properties.csv`. Each C++ class becomes a table, and each method becomes a column:

```sql
-- Example generated table
CREATE TABLE FunctionDecl (
    id BIGINT PRIMARY KEY,
    name TEXT,
    is_defined BOOL,
    is_virtual BOOL,
    -- ... more columns based on properties.csv
);

-- Insert example
INSERT INTO FunctionDecl (id, name, is_defined, is_virtual)
VALUES (1, 'main', true, false);
```

## Querying Data

```sql
-- List all tables
\dt

-- Query function declarations
SELECT id, name FROM FunctionDecl WHERE is_virtual = true;

-- Query type relationships
SELECT t.id, t.name, c.name 
FROM Type t 
JOIN Class c ON t.parent_id = c.id;
```

## Project Structure

```
/workspace/
├── reificator/             # Schema generation and AST extraction plugin
├── arboretum-plugin/       # Clang plugin integration
├── reify-cpp/              # C++ AST visitor library
├── reify-rs/               # Rust with PostgreSQL I/O (replaces Parquet)
├── arboretum-ffi/          # FFI bindings for C++ → Rust communication
├── llvm-project/           # LLVM/Clang dependency
└── tests/                  # Integration tests
```

## Migration Notes

### For Existing Users

If you were using the old architecture:

| Old | New |
|-----|-----|
| `./build/arboretumd --db-path ./graphs &` | Not needed! |
| Parquet files in `./graphs/*` | PostgreSQL tables |
| `tcp_client.rs` | `lib.rs` with direct PostgreSQL writes |

### Code Changes

```rust
// Old (with arboretumd):
let client = CollectorClient::new("localhost:3232");

// New (direct PostgreSQL):
let builders = TableBuilders::new("postgresql://user@localhost/arboretum");
```

## Dependencies

| Project | Purpose |
|---------|---------|
| clang/llvm-project | AST parsing and analysis |
| deadpool_postgres | Connection pooling for PostgreSQL |
| arrow | Record batch processing (optional) |

## License

See LICENSE file for details.
