# Arboretum Project

**C/C++ Abstract Syntax Tree Analysis via PostgreSQL**

---

## 📑 Table of Contents

| Section | Description |
|---------|-------------|
| [Overview](#overview) | High-level project description |
| [Architecture](#architecture) | Component structure and flow |
| [Quick Start](#quick-start) | Getting started guide |
| [Project Structure](#project-structure) | Directory layout |
| [Build Instructions](#build-instructions) | Compiling the project |
| [Working with Code](#working-with-code) | Development workflows |
| [Dependencies](#dependencies) | Required tools and libraries |

---

## Overview

### Core Components

| Component | Location | Description |
|-----------|----------|-------------|
| `reificator` | `/workspace/reificator/` | Clang plugin for schema generation and AST extraction |
| `arboretum-plugin` | `/workspace/arboretum-plugin/` | Clang plugin integration |
| `reify-cpp` | `/workspace/reify-cpp/` | C++ AST visitor library |
| `reify-rs` | `/workspace/reify-rs/` | Rust AST reification with PostgreSQL I/O |
| `arboretum-ffi` | `/workspace/arboretum-ffi/` | Rust FFI bindings for C++ integration |

### Architecture

Arboretum extracts code properties from C/C++ source using Clang's AST. The extracted data forms a graph structure that can be queried. Key changes:

1. **Direct PostgreSQL Storage**: No more Parquet files or `arboretumd` daemon.
2. **Immediate Writes**: Records are inserted directly into PostgreSQL tables via FFI.
3. **Simplified Pipeline**: C++ → FFI → PostgreSQL (no buffering, no TCP).

### Build Dependencies

| Dependency | Purpose |
|------------|---------|
| LLVM/Clang | AST analysis and parsing |
| Rust | Backend services and FFI |
| PostgreSQL | Database backend for graph storage |

## Quick Start

### Prerequisites

Install required tools:

```bash
# Install CMake (for LLVM build)
sudo apt install cmake  # Ubuntu/Debian
brew install cmake      # macOS

# Install PostgreSQL (for database backend)
sudo apt install postgresql postgresql-contrib  # Ubuntu/Debian
brew install postgresql  # macOS

# Install Rust and cargo
curl --proto '=https' --tlsonly -sSf https://sh.rustup.rs | sh
```

### Build LLVM First

```bash
make llvm-project/build/llvm-stamp
```

This builds LLVM/Clang (~15-20 minutes).

### Build the Project

```bash
# Ensure PostgreSQL is running
pg_ctl -D /usr/local/var/postgres start

# Create database and tables
psql -d postgres -c "CREATE DATABASE arboretum;"
psql -d arboretum -c "CREATE EXTENSION IF NOT EXISTS pgvector;"

# Build the project
make arboretum
```

### Extract Code Properties

```bash
# Compile with Arboretum plugin - writes directly to PostgreSQL
clang++ -fplugin=./build/libarboretum.so \
    -std=c++20 \
    your_code.cpp
```

Note: The FFI layer now writes directly to PostgreSQL without requiring a TCP connection.

---

## Documentation

| File | Description |
|------|-------------|
| [LONG_TERM_PLAN_DRAFT.md](LONG_TERM_PLAN_DRAFT.md) | Long-term strategic plan and architecture |
| [ROADMAP.md](ROADMAP.md) | Detailed roadmap with milestones and timelines |
| [TASKS.md](TASKS.md) | Task board with prioritized work items |

### Roadmap Overview

| Version | Status | Timeline | Focus |
|---------|--------|----------|-------|
| V1 | In Progress | Q2-Q3 2026 | C/C++ extraction, unification, analyses |
| V2 | Planned | Q4 2026 - Q1 2027 | Rust, additional languages, distro integration |
| V3 | Planned | Q2-Q3 2027 | Global catalog, enterprise features |
| V4+ | Future | Q4 2027+ | Advanced analyses, AI/ML integrations |

### Current Status

- ✅ C++ Clang plugin extraction (in progress)
- ⏳ Cross-TU unification (not started - see ROADMAP.md)
- ⏳ All V1 analyses (not started - see ROADMAP.md)
- ⏳ Docker distribution (not started)

### How to Contribute

1. Review [TASKS.md](TASKS.md) for available tasks
2. Review [ROADMAP.md](ROADMAP.md) for context and milestones
3. Comment on a task to claim it
4. Create a branch: `git checkout -b task/<task-number>-<description>`
5. Submit a PR with the task number in the title

---

## 🛠️ Git Workflow for Agents

This is a **critical workflow requirement** for all agents working on this project.

### Before Starting Work

```bash
# Ensure you're on main and up to date
git checkout main
git pull origin main

# Create a branch for your task
git checkout -b task/<task-number>-<short-description>
# Example: git checkout -b task/1-fix-ffi-connection
```

### While Working

```bash
# Commit frequently with descriptive messages
git add -A
git commit -m "feat: implement structural hash computation"
git commit -m "test: add unit tests for unification"

# Keep commits focused and atomic
# One logical change per commit
```

### When Finished

```bash
# Run tests to verify changes
make test

# Check code quality
make analyze-cpp
make analyze-rust

# Reformat code if needed
cargo fmt
clang-format -i path/to/file.cc

# Commit all changes
git add -A
git commit -m "feat: implement unification with structural hash

- Compute bottom-up structural hash via Postgres CTE
- Verify candidate matches with AST comparison
- Create unified graph tables
- Handle declaration/definition linking

Closes: #<task-number>"
```

### Submitting for Review

```bash
# Push your branch
git push origin task/<task-number>-<short-description>

# Create a Pull Request with:
# - Clear title: "feat: implement structural hash computation (#<task-number>)"
# - Description explaining what changed and why
# - References to any relevant issues or tasks
```

### After Merge

```bash
# Pull latest changes to main
git checkout main
git pull origin main

# Delete your branch (optional but recommended)
git branch -d task/<task-number>-<description>
```

### Common Commands Reference

```bash
# View your changes before commit
git status
git diff

# View commit history
git log --oneline
git log --graph --oneline --all

# Reset to clean state (be careful!)
git reset --hard HEAD
git clean -fd

# Rebase onto latest main (if needed)
git checkout main
git pull origin main
git checkout task/<task-number>-<description>
git rebase main
```

## Project Structure

```
/workspace/
├── Makefile                 # Build system and compilation rules
├── Cargo.toml               # Rust workspace configuration
├── llvm-project/           # LLVM/Clang submodule (external dependency)
│
├── reificator/             # Clang plugin for schema generation and AST extraction
│   └── src/
│       └── *.cc            # Reificator implementation
│
├── arboretum-plugin/       # Integration plugin for Clang
│   └── src/plugin.cc       # Main plugin entry point
│
├── reify-cpp/              # C++ AST reification library
│   ├── include/            # Header files
│   └── src/
│       ├── arboretum_ast_visitor.cc
│       ├── arboretum_context.cc
│       ├── arboretum_data_model.cc
│       └── arboretum_source_model.cc
│
├── reify-rs/               # Rust AST reification library with PostgreSQL I/O
│   └── src/
│       ├── ffi.rs          # FFI bindings and generated code
│       └── io.rs           # PostgreSQL table builders
│
├── arboretum-ffi/          # Rust FFI bindings for C++ integration
│   └── src/
│       ├── lib.rs          # Main FFI module - direct PostgreSQL writes
│       └── tcp_client.rs   # (Legacy) TCP client - now optional
│
└── tests/                  # Test suite
    └── 0/                  # Test cases
        ├── a.cc, b.cc, c.cc, d.cc
        └── run.sh
```

## Building from Source

### Full Build

```bash
make arboretum
```

This builds:
1. LLVM (first time only) - ~15-20 minutes
2. `libreificator.so` - Reificator plugin
3. `libarboretum.so` - Main Clang plugin with all dependencies

### Cargo Build (Alternative)

For Rust components only:

```bash
# Build FFI library
cd arboretum-ffi && cargo build --release

# Build reify-rs
cd reify-rs && cargo build --release
```

### Component Builds

| Target | Description |
|--------|-------------|
| `make $(BUILD_DIR)/libreificator.so` | Build reificator plugin |
| `make $(BUILD_DIR)/reify-cpp.a` | Build reify-cpp library |
| `make target/release/libarboretum_ffi.a` | Build FFI library |

## Key Concepts

### Code Property Graphs

Arboretum extracts code properties from C/C++ source using Clang's AST. The extracted data forms a graph structure stored in PostgreSQL.

### Schema Generation

The `reificator` plugin generates table definitions from `properties.csv`. Each C++ class method becomes a column in the corresponding PostgreSQL table.

### Code Generation Architecture

**How the reificator works:**

1. **AST Parsing**: The reificator plugin runs as a Clang plugin and parses the AST of the input file (`reificator/input.cc`). It extracts all Clang classes and their methods from `properties.csv`.

2. **Schema Extraction**: From `properties.csv`, the reificator builds a `Model` containing:
   - Meta tables: `file`, `source_loc`, `source_range`, `enum`, `enum_value`, `QualType`, etc.
   - Clang tables: Generated dynamically based on enabled properties in `properties.csv`

3. **Code Generation**: The reificator generates code via `emit_ffi` and `emit_reify_cpp`:
   
   - **Reificator's Code Generator** (`emit_ffi.cc`, `emit_reify_cpp.cc`):
     - Reads partially generated files that contain `//// BEGIN ARBORETUM GENERATED CODE ////` and `////   END ARBORETUM GENERATED CODE ////` markers
     - Replaces content between markers with new generated code
   
   - **Generated Rust Code** (`reify-rs/src/ffi.rs`, `reify-rs/src/io.rs`):
     - **FFI Functions** (`arboretum_emit_*`): These are `extern "C"` functions callable from C++. They:
       - Receive individual parameters for each column
       - Convert u64 → i64 (tokio-postgres doesn't support u64 directly)
       - Store records in thread-local Vec queues (not async operations - FFI functions are sync!)
     - **Queue System**: Records are queued in memory per table type using `thread_local!`
     - **Flush Function** (`flush_records`): A sync FFI function that:
       - Takes all queued records from thread-local storage
       - Uses `tokio::runtime::Runtime::block_on` to run async PostgreSQL operations
       - Inserts records into PostgreSQL tables
   
   - **Generated C++ Code** (`reify-cpp/src/arboretum_ast_visitor.cc`, etc.):
     - Contains `Visit*` methods for each AST node type
     - Resolves references (types, locations) to IDs
     - Calls the FFI functions to emit records

4. **PostgreSQL I/O** (`reify-rs/src/io.rs`):
   - **TableBuilders**: Delegates to individual TableBuilderN structs
   - **TableBuilderN**: For each table type, stores records in memory and flushes them via async PostgreSQL operations
   - Uses `deadpool_postgres` connection pool with `tokio-postgres`

### FFI Architecture

```
C++ Code (Clang AST Visitor)
    ↓ FFI calls (extern "C")
Rust FFI layer (arboretum-ffi)
    ↓ Queue records in thread-local storage
    ↓ Flush on finalize
PostgreSQL tables (via deadpool_postgres + tokio-postgres)
```

### Database Schema

Tables are created dynamically based on `properties.csv`. Each table has:
- An `id` column (u64 → i64 in Rust)
- Additional columns for each enabled property
- STRING types mapped to PostgreSQL TEXT
- BOOL types mapped to PostgreSQL BOOLEAN
- Numeric types mapped appropriately

## 🛠️ Working with the Code

| Task | Guide |
|------|-------|
| Add new AST node types | [Reificator Guide](reificator/AGENTS.md#adding-new-ast-node-types) |
| Debug compilation | [Build System](BUILD_SYSTEM.md) |
| Query graphs via SQL | See PostgreSQL documentation |
| Write tests | [tests/0](tests/0/AGENTS.md) |

### Querying Data

```sql
-- List all tables
\dt

-- Query files
SELECT * FROM file WHERE filename LIKE '%main%';

-- Query function declarations
SELECT * FROM FunctionDecl WHERE name = 'main';
```

### Debugging

Enable debug logging for Rust:

```bash
RUST_LOG=trace clang++ -fplugin=./build/libarboretum.so ...
```

## 📖 Documentation Files

| File | Purpose |
|------|---------|
| `AGENTS.md` | This file - main project overview |
| [BUILD_SYSTEM.md](BUILD_SYSTEM.md) | Makefile-based build instructions |
| [QUICK_REFERENCE.md](QUICK_REFERENCE.md) | Quick command reference |
| [README.md](README.md) | User-facing overview |
| [CODE_QUALITY.md](CODE_QUALITY.md) | Code quality tools, linting, testing, and maintenance |

## 📖 Component Documentation

| Component | Documentation |
|-----------|---------------|
| [reificator](reificator/AGENTS.md) | Clang plugin for schema generation and AST extraction |
| [reify-cpp](reify-cpp/AGISTS.md) | C++ visitor implementation |
| [reify-rs](reify-rs/AGENTS.md) | Rust FFI, PostgreSQL I/O |
| [arboretum-ffi](arboretum-ffi/AGENTS.md) | C++ ↔ Rust bindings |
| [arboretum-plugin](arboretum-plugin/AGENTS.md) | Clang plugin integration |
| [llvm-project](llvm-project/AGENTS.md) | Submodule setup |
| [tests](tests/AGENTS.md) | Integration tests |

## 🛠️ Development Guidelines

| Task | Guide |
|------|-------|
| Code quality & linting | [CODE_QUALITY.md](CODE_QUALITY.md) |
| Running static analysis | `make analyze-cpp`, `make analyze-rust` |
| Code formatting | `clang-format`, `cargo fmt` |
| Testing | `make test`, `tests/integration/run.sh` |

## Notes

- The `llvm-project` directory is a git submodule containing LLVM/Clang source code
- PostgreSQL tables are created dynamically based on properties.csv
- No `arboretumd` daemon - writes go directly to PostgreSQL
- FFI layer uses direct INSERT statements instead of TCP communication
- **Important**: FFI functions (`extern "C"`) cannot be async. Records are queued in memory and flushed via `block_on` in sync context.
