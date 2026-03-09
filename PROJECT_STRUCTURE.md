# Arboretum Project Structure

**Last Updated**: March 9, 2026

---

## Directory Layout

```
/workspace/
├── AGENTS.md                      # Main documentation entry point for agents
├── PROJECT_OVERVIEW.md            # This file - executive summary
├── LONG_TERM_PLAN_DRAFT.md        # Strategic plan and architecture
├── ROADMAP.md                     # Detailed milestones and timeline
├── TASKS.md                       # Prioritized task board
├── PROJECT_STRUCTURE.md           # This file - directory layout
├── README.md                      # User-facing overview
├── QUICK_REFERENCE.md             # Quick command reference
├── BUILD_SYSTEM.md                # Build system documentation
├── CODE_QUALITY.md                # Code quality standards
├── Cargo.toml                     # Rust workspace manifest
├── Makefile                       # Build orchestration
│
├── reificator/                    # Clang plugin for schema generation
│   ├── AGENTS.md                  # Component documentation
│   ├── include/                   # Header files
│   ├── src/                       # Implementation
│   └── properties.csv             # Schema definition (CSV format)
│
├── reify-cpp/                     # C++ AST visitor library
│   ├── AGENTS.md                  # Component documentation
│   ├── include/                   # Header files
│   └── src/                       # Implementation
│
├── reify-rs/                      # Rust AST reification with PostgreSQL
│   ├── AGENTS.md                  # Component documentation
│   └── src/                       # Rust implementation
│       ├── ffi.rs                 # FFI record types
│       └── io.rs                  # PostgreSQL table builders
│
├── arboretum-ffi/                 # Rust FFI bindings for C++ integration
│   ├── AGENTS.md                  # Component documentation
│   └── src/                       # FFI implementation
│
├── arboretum-plugin/              # Clang plugin integration
│   ├── AGENTS.md                  # Component documentation
│   └── src/                       # Plugin source
│
├── llvm-project/                  # LLVM/Clang submodule (external)
│
├── tests/                         # Test suite
│   ├── AGENTS.md                  # Test documentation
│   └── 0/                         # Integration tests
│
├── docker/                        # Docker images (V1)
│   ├── RHEL/
│   ├── Debian/
│   └── Ubuntu/
│
├── scripts/                       # Utility scripts
│   └── validate_properties.py     # Property CSV validation
│
├── .clang-tidy                    # C++ linting config (TBD)
├── .clang-format                  # C++ formatting config (TBD)
├── rustfmt.toml                   # Rust formatting config (TBD)
└── clippy.toml                    # Rust linter config (TBD)
```

---

## Component Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                        C++ Source Code                           │
└─────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Clang Plugin (reificator)                     │
│  - RecursiveASTVisitor                                           │
│  - LLVM IR generation (PreOpt, PostInline, PostOpt)            │
└─────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────┐
│                  FFI Layer (arboretum-ffi)                       │
│  - extern "C" functions                                          │
│  - Record queuing (thread-local)                                 │
└─────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────┐
│                 PostgreSQL Store (reify-rs)                      │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  Normalized Tables                                       │   │
│  │  - cpg_node                                              │   │
│  │  - cpg_edge                                              │   │
│  └──────────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  Language-Specific Tables                                │   │
│  │  - cpp_function_decl                                     │   │
│  │  - cpp_var_decl                                          │   │
│  │  - ir_function, ir_block, ir_instruction                │   │
│  └──────────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  Build Artifact Tables                                   │   │
│  │  - build_artifact                                        │   │
│  │  - symbol_provider                                       │   │
│  │  - runtime_resolution                                    │   │
│  └──────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Unification Layer                             │
│  - Structural hash via Postgres CTE                             │
│  - Candidate verification                                        │
│  - Unified graph creation                                        │
└─────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────┐
│                  Program Analysis                                │
│  - Def-use chains (CTEs)                                         │
│  - Alias analysis (CTEs)                                         │
│  - CDG (CTEs)                                                    │
│  - Dominator tree (CTEs)                                         │
│  - Lifetime extents (CTEs)                                       │
│  - Call graph (CTEs)                                             │
│  - Ownership inference (CTEs)                                    │
└─────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────┐
│                  Query Interface (PostgreSQL)                    │
│  - Direct SQL access for consumers                              │
│  - pgvector extension support                                   │
│  - Recursive CTEs for analysis results                          │
└─────────────────────────────────────────────────────────────────┘
```

---

## Data Model Overview

### Core Tables

| Table | Purpose | Key Columns |
|-------|---------|-------------|
| `subgraph` | Compilation unit | subgraph_id, content_hash, build_id, kind |
| `cpg_node` | Normalized graph node | node_id, subgraph_id, node_kind, visit_order |
| `cpg_edge` | Normalized graph edge | parent_id, child_id, edge_kind |
| `unification_map` | Unified node mapping | unified_*, source_* |

### Language-Specific Tables

| Table | Purpose | Key Columns |
|-------|---------|-------------|
| `cpp_function_decl` | C++ function | mangled_name, return_type, is_virtual |
| `cpp_var_decl` | C++ variable | type, storage_class |
| `cpp_call_expr` | C++ call | callee_id |
| `ir_function` | LLVM IR function | llvm_name, opt_stage |
| `ir_instruction` | LLVM IR instruction | opcode, opt_stage |

### Build Artifact Tables

| Table | Purpose | Key Columns |
|-------|---------|-------------|
| `build_artifact` | Build result | artifact_id, kind, capture_status |
| `symbol_provider` | Symbol provider | mangled_name, provider_id |
| `runtime_resolution` | Runtime symbol resolution | consumer_id, provider_id |

---

## Build Flow

```
1. LLVM Build (first time only)
   make llvm-project/build/llvm-stamp
   └─> llvm-project/build/bin/clang++

2. Reificator Plugin Build
   make $(BUILD_DIR)/libreificator.so
   └─> Generates schema from properties.csv
       Generates FFI functions
       Generates C++ AST visitor

3. FFI Library Build
   make target/release/libarboretum_ffi.a
   └─> Builds Rust FFI bindings

4. Complete Build
   make arboretum
   └─> Links all components
       └─> build/libarboretum.so

5. Usage
   clang++ -fplugin=./build/libarboretum.so \
       -std=c++20 \
       your_code.cpp
   └─> Extracts AST
       └─> Writes to PostgreSQL via FFI
           └─> Unification runs
               └─> Analyses run
```

---

## Key Paths

| Purpose | Path |
|---------|------|
| Main entry point | `reificator/src/*.cc` |
| AST visitor | `reify-cpp/src/arboretum_ast_visitor.cc` |
| FFI layer | `arboretum-ffi/src/lib.rs` |
| PostgreSQL I/O | `reify-rs/src/io.rs` |
| Plugin integration | `arboretum-plugin/src/plugin.cc` |
| Schema definition | `reificator/properties.csv` |
| Test suite | `tests/0/` |

---

## Versioning Strategy

| Type | Format | Source |
|------|--------|--------|
| Package versions | Standard versioning | Package metadata |
| Symbol versions | GNU symbol versioning | ELF files |
| Schema versions | Upstream version | llvm/clang |
| Software versions | SemVer | Project releases |

---

*This project structure is maintained as part of the Arboretum project documentation.*
