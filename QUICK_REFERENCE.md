# Quick Reference Guide

## Project Structure

```
/workspace/
├── AGENTS.md                      # Main documentation entry point
├── BUILD_SYSTEM.md                # Build system documentation
├── README.md                      # High-level overview
├── Cargo.toml                     # Rust workspace manifest
├── Makefile                       # Build orchestration
│
├── llvm-project/                  # LLVM/Clang submodule (external)
│   └── AGENTS.md
│
├── reificator/                    # Clang plugin for schema generation
│   ├── AGENTS.md
│   ├── include/                   # Headers
│   ├── src/                       # Implementation
│   └── properties.csv             # Schema definition (CSV format)
│
├── reify-cpp/                     # C++ AST visitor library
│   ├── AGENTS.md
│   ├── include/                   # Headers
│   └── src/                       # Implementation
│
├── reify-rs/                      # Rust AST reification with PostgreSQL
│   ├── AGENTS.md
│   └── src/                       # Rust implementation
│       ├── ffi.rs                 # FFI record types
│       └── io.rs                  # PostgreSQL table builders
│
├── arboretum-ffi/                 # Rust FFI bindings for PostgreSQL
│   ├── AGENTS.md
│   └── src/                       # FFI implementation
│
├── arboretum-plugin/              # Clang plugin integration
│   ├── AGENTS.md
│   └── src/                       # Plugin source
│
└── tests/                         # Test suite
    ├── AGENTS.md
    └── 0/                         # Integration tests
        ├── AGENTS.md
        ├── a.cc, b.cc, c.cc, d.cc
        └── run.sh
```

## Quick Start Commands

```bash
# Install dependencies (first time)
sudo apt install cmake postgresql postgresql-contrib

# Build LLVM first (required)
make llvm-project/build/llvm-stamp

# Build the complete project
make arboretum

# Run tests
./tests/run.sh
```

## Key Files by Component

| Component | Main Source | Configuration |
|-----------|-------------|---------------|
| reificator | `reificator/src/*.cc` | `properties.csv` |
| reify-cpp | `reify-cpp/src/*.cc` | Header-only API |
| reify-rs | `reify-rs/src/*.rs` | Cargo.toml |
| arboretum-ffi | `arboretum-ffi/src/lib.rs` | Cargo.toml |
| arboretum-plugin | `arboretum-plugin/src/plugin.cc` | None |

## FFI Functions (C++ → Rust → PostgreSQL)

```cpp
bool arboretum_connect(const char* db_url);
uint64_t arboretum_subgraph_id();
void arboretum_finalize();
void arboretum_set_db_url(const char* db_url);

void arboretum_emit_FunctionDecl(uint64_t id, const char* name, bool is_virtual);
void arboretum_emit_CXXRecordDecl(uint64_t id, const char* name);
// ... one function per graph property from properties.csv
```

## PostgreSQL Integration

### Connection URL Format

```
postgresql://user@host/database
```

### Example Usage

```bash
# Compile with Arboretum plugin - writes directly to PostgreSQL
clang++ -fplugin=./build/libarboretum.so \
    -c your_code.cpp

# Query results
psql -d arboretum -c "SELECT * FROM FunctionDecl;"
```

## Important Notes

1. **PostgreSQL Backend**: Replaces Parquet files - data goes directly to database
2. **No Daemon**: Eliminated `arboretumd` - no TCP communication
3. **Generated Code**: Headers contain `BEGIN ARBORETUM GENERATED CODE` markers - don't edit manually
4. **LLVM**: Must be built before using Clang plugins (`make llvm-project/build/llvm-stamp`)
5. **Test Files**: Located in `tests/0/`

## Common Tasks

| Task | Command |
|------|---------|
| Build all | `make arboretum` |
| Clean build | `make clean && make arboretum` |
| Run tests | `./tests/run.sh` |
| Start PostgreSQL | `pg_ctl -D /usr/local/var/postgres start` |

## Documentation Files

- **AGENTS.md**: Agent-readable project documentation
- **BUILD_SYSTEM.md**: Makefile-based build instructions
- **README.md**: User-facing overview

## Architecture Flow

```
C++ Code → Clang Plugin (reificator) → FFI → PostgreSQL (direct INSERT)
                              ↓
                         PostgreSQL tables
```

## Key Differences from Original

| Original | PostgreSQL Version |
|----------|-------------------|
| `arboretumd` daemon | Eliminated |
| Parquet files on disk | PostgreSQL tables |
| TCP client/server | Direct FFI calls |
| Buffered I/O | Immediate INSERT statements |
