# Build System Guide

**Build the Arboretum Project**

---

## Table of Contents

| Section | Description |
|---------|-------------|
| [Overview](#overview) | Build overview |
| [Quick Start](#quick-start) | Build commands |
| [Build Dependencies](#build-dependencies) | Required tools |
| [Project Structure](#project-structure) | Directory layout |
| [Troubleshooting](#troubleshooting) | Common issues |

---

## Overview

This project uses a Makefile-based build system with both C++ (Clang plugin) and Rust components. The build process involves:

1. **LLVM/Clang** - Required for building the Clang plugin
2. **Rust** - For FFI bindings and PostgreSQL I/O
3. **PostgreSQL** - Database backend (no longer uses arboretumd daemon)

### Architecture Changes: PostgreSQL Migration

| Before | After |
|--------|-------|
| `arboretumd` daemon | Direct PostgreSQL writes |
| Parquet files on disk | PostgreSQL tables |
| TCP client/server | FFI direct calls |
| Buffer + flush | Immediate INSERT |

### Key Code Changes

#### Removed Files/Components

- `arboretumd/` - Daemon process (eliminated)
- Parquet-based I/O in `reify-rs/src/io.rs` - replaced with PostgreSQL

#### New Patterns

```rust
// Direct PostgreSQL connection
let config = deadpool_postgres::Config::from_string(&db_url)?;
let pool = config.create_pool(Some(10))?;

// No more TCP client for arboretumd
// let client = CollectorClient::new("localhost:3232");
```

---

## Quick Start

### Prerequisites

Install the required dependencies:

```bash
# Install Rust/Cargo
curl --proto '=https' --tlsonly -sSf https://sh.rustup.rs | sh

# Install CMake (for LLVM build)
sudo apt install cmake  # Ubuntu/Debian
brew install cmake      # macOS

# Install PostgreSQL
sudo apt install postgresql postgresql-contrib  # Ubuntu/Debian
brew install postgresql  # macOS
```

### Build Commands

```bash
# Build LLVM first (required for Clang plugin)
make llvm-project/build/llvm-stamp

# Build the full project
make arboretum
```

This will build:
- `libreificator.so` - Schema generation plugin
- `libarboretum.so` - Main Clang plugin with PostgreSQL I/O
- `libarboretum_ffi.a` - Rust FFI library
- `reify-cpp.a` - C++ AST visitor library

---

## Build Dependencies

| Dependency | Purpose |
|------------|---------|
| **LLVM/Clang** | AST analysis and parsing (via reificator plugin) |
| **Rust 1.70+** | FFI bindings, PostgreSQL I/O |
| **PostgreSQL 14+** | Database backend for graph storage |
| **CMake** | LLVM build system |

### Version Requirements

- Rust: 1.70+ (required for `asm!` macro)
- LLVM/Clang: 15+ (for plugin support)
- PostgreSQL: 14+ (with pgvector extension)

---

## Project Structure

```
/workspace/
├── Makefile                 # Build system and compilation rules
├── Cargo.toml               # Rust workspace configuration
│
├── llvm-project/           # LLVM/Clang submodule (external dependency)
│   └── build/             # Compiled LLVM (generated)
│
├── reificator/             # Clang plugin for schema generation
│   └── src/*.cc           # Reificator implementation
│
├── arboretum-plugin/       # Integration plugin for Clang
│   └── src/plugin.cc      # Main plugin entry point
│
├── reify-cpp/              # C++ AST reification library
│   ├── include/           # Header files
│   └── src/
│       ├── arboretum_ast_visitor.cc
│       ├── arboretum_context.cc
│       └── arboretum_data_model.cc
│
├── reify-rs/               # Rust AST reification with PostgreSQL I/O
│   └── src/
│       ├── ffi.rs         # FFI bindings and record types
│       └── io.rs          # PostgreSQL table builders
│
└── arboretum-ffi/          # Rust FFI bindings for C++ integration
    └── src/lib.rs          # Main FFI module with PostgreSQL I/O
```

### Build Output

```
/workspace/build/
├── libreificator.so     # Schema generation plugin
├── libarboretum.so      # Main Clang plugin
├── reify-cpp/           # C++ object files
├── reify-cpp.a          # Static library
└── arboretum-plugin/    # Plugin object files
```

---

## Troubleshooting

### Common Build Issues

#### 1. Rust Compilation Errors

```bash
# Clean build artifacts
cargo clean

# Rebuild
cargo build --release
```

#### 2. LLVM Linking Issues

```bash
# Ensure LLVM is built first
make llvm-project/build/llvm-stamp

# Check LLVM location
ls -la llvm-project/build/lib/
```

#### 3. PostgreSQL Connection Errors

```sql
-- Ensure database exists
CREATE DATABASE arboretum;

-- Ensure pgvector extension is installed
CREATE EXTENSION IF NOT EXISTS pgvector;
```

### Debugging

Enable debug logging:

```bash
RUST_LOG=trace clang++ -fplugin=./build/libarboretum.so your_code.cpp
```

---

## References

- [Main Documentation](AGENTS.md)
- [Quick Reference](QUICK_REFERENCE.md)
- [Reificator Guide](reificator/AGENTS.md)
- [Reify-RS Guide](reify-rs/AGENTS.md)
