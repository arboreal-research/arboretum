# Arboretum Project Overview

**Last Updated**: March 9, 2026  
**Status**: Active Development (V1 in progress)

---

## Executive Summary

Arboretum is a general-purpose static analysis system for multi-language codebases. It extracts a semantic graph from source code during compilation, unifies shared definitions across translation units, and enriches the graph with derived program properties.

**V1 Target**: Q3 2026  
**V1 Focus**: C/C++ extraction, unification, and analysis for single package builds  
**First Consumers**: AI/ML data pipelines sampling from the dataset

---

## Project Documentation

### Strategic Documentation

| File | Purpose | Audience |
|------|---------|----------|
| [LONG_TERM_PLAN_DRAFT.md](LONG_TERM_PLAN_DRAFT.md) | Strategic vision, architecture, design principles | Developers, maintainers, technical leads |
| [ROADMAP.md](ROADMAP.md) | Detailed milestones, timeline, task breakdown | Project managers, contributors, agents |
| [TASKS.md](TASKS.md) | Prioritized task board, current priorities | Developers, contributors, agents |

### Technical Documentation

| File | Purpose | Audience |
|------|---------|----------|
| [AGENTS.md](AGENTS.md) | Project overview, quick start | AI agents, new contributors |
| [README.md](README.md) | High-level user-facing overview | General users |
| [QUICK_REFERENCE.md](QUICK_REFERENCE.md) | Quick command reference | All users |
| [BUILD_SYSTEM.md](BUILD_SYSTEM.md) | Build system details | Developers |
| [CODE_QUALITY.md](CODE_QUALITY.md) | Code quality standards | Developers |

---

## Architecture Highlights

### Core Design Principles

1. **Consumer agnostic** - No translation-specific concepts in data model
2. **Language agnostic core** - Normalized graph schema
3. **Language-specific depth** - Full language ASTs available
4. **Append-only** - Immutable subgraphs, versioning preserved
5. **Build-system agnostic** - Works with any build system via compiler wrappers
6. **Distro-scale** - Designed for Linux distribution analysis
7. **Fixpoint analysis via SQL** - CTEs for Datalog semantics

### System Components

| Component | Status | Notes |
|-----------|--------|-------|
| Clang Plugin | In Progress | C++ extraction with LLVM IR |
| Rustc Plugin | Planned | HIR + MIR + LLVM IR extraction |
| Compiler/Linker Wrappers | Planned | Rust binaries |
| Graph Store | In Progress | Postgres with normalized + language-specific tables |
| Build Artifact Layer | Planned | Symbol resolution, runtime resolution |
| Graph Unification | Planned | Structural hash + verification |
| Program Analysis | Planned | SQL CTE-based analyses |
| Package Registry | Planned | Package/library metadata |
| Build Integration | Planned | RHEL/Fedora (primary), Debian/Ubuntu (secondary) |
| Global Catalog | Deferred | PostgreSQL backend + parquet for offline |

---

## V1 Scope

### In Scope

- C++ Clang plugin extraction to Postgres with LLVM IR (PreOpt, PostInline, PostOpt)
- Normalized `cpg_node`/`cpg_edge` tables populated alongside `cpp_*` tables
- AST, CFG, and IR edge extraction with `visit_order`
- Linker wrapper with LLD (Rust binaries) and postlink binary
- Cross-TU unification via structural hash and candidate verification
- Declaration/definition linking via mangled name and `symbol_provider`
- Forward declarations unified with definitions
- All V1 analyses:
  - Def-use chains
  - Flow-sensitive alias analysis
  - Control Dependence Graph (CDG)
  - Dominator tree
  - Lifetime extent computation
  - Call graph construction
  - C++ ownership inference
- Package and symbol version tracking (separate)
- All analyses expressed as SQL CTEs (no Souffle for V1)

### Not in V1

- Rust extractor
- Additional language extractors (Python, JS, Go, Java)
- Context-sensitive interprocedural alias analysis
- Template instantiation merging beyond identical copies
- Global catalog infrastructure
- Souffle-based analysis authoring
- Query API abstraction layer

---

## Roadmap Summary

### V1 Timeline (Q2-Q3 2026)

| Month | Milestone |
|-------|-----------|
| April | Complete C++ extraction with LLVM IR |
| May | Basic unification working |
| June | All analyses working (def-use, alias, CDG, etc.) |
| July | Docker distribution ready (RHEL, Debian, Ubuntu) |
| August | Hero use case validation (OpenSSL, SQLite, etc.) |
| September | Documentation and external tryout |

### V2 Timeline (Q4 2026 - Q1 2027)

| Month | Milestone |
|-------|-----------|
| October | Rust extractor (HIR + MIR + LLVM IR) |
| November | Package registry and distro metadata |
| December | Build system integration (RHEL/Fedora, Debian/Ubuntu) |
| January | Context-sensitive alias analysis (V2) |
| February | Template instantiation merging |
| March | Additional language extractors (Python, JS, Go, Java) |

### Future Releases

| Release | Timeline | Features |
|---------|----------|----------|
| V3 | Q2-Q3 2027 | Global catalog infrastructure, query API abstraction |
| V4 | Q4 2027+ | Advanced analyses, AI/ML integrations |

---

## Success Criteria

### V1 Success

V1 is successful when the engine can:
1. Extract a single package build at scale (multiple TUs)
2. Create a unified graph across translation units
3. Run all V1 analyses successfully
4. Output results queryable via PostgreSQL
5. Enable first consumers (AI/ML pipelines) to sample the data

### Hero Use Cases

Target applications for V1 validation:
- OpenSSL (complex, ownership/alias test)
- SQLite (simple, validation)
- BusyBox (embedded, distro-scale)
- GCC (self-hosting, toolchain)
- Node.js (multi-TU, unification)

---

## Current State

### Completed

- C++ extraction infrastructure in place
- PostgreSQL storage design defined
- FFI layer for C++ ↔ Rust communication

### In Progress

- C++ Clang plugin extraction
- RecursiveASTVisitor implementation
- LLVM IR extraction

### Not Started

- Cross-TU unification
- All V1 analyses
- Docker distribution
- Hero use case validation
- Documentation

---

## How to Contribute

1. Review [TASKS.md](TASKS.md) for available tasks
2. Review [ROADMAP.md](ROADMAP.md) for context
3. Comment on a task to claim it
4. Create a branch: `git checkout -b task/<task-number>-<description>`
5. Submit a PR with the task number in the title

### For Agents

- Review [TASKS.md](TASKS.md) for assigned tasks
- Check [ROADMAP.md](ROADMAP.md) for milestone context
- Refer to [LONG_TERM_PLAN_DRAFT.md](LONG_TERM_PLAN_DRAFT.md) for architecture details

---

## Key Decisions (as of March 9, 2026)

| Decision | Status |
|----------|--------|
| SQL CTEs for analyses (no Souffle for V1) | ✅ Decided |
| Rust binaries for wrapper scripts | ✅ Decided |
| Docker images for distribution | ✅ Decided |
| PostgreSQL as primary storage | ✅ Decided |
| Separate tracking of package and symbol versions | ✅ Decided |
| Forward declarations unified with definitions | ✅ Decided |
| No public global catalog for V1 | ✅ Decided |

---

## Questions or Need Help?

- Review [LONG_TERM_PLAN_DRAFT.md](LONG_TERM_PLAN_DRAFT.md) for architectural questions
- Review [ROADMAP.md](ROADMAP.md) for timeline and milestone questions
- Review [TASKS.md](TASKS.md) for task-specific questions
- Check component-specific docs:
  - `reificator/AGENTS.md`
  - `reify-cpp/AGENTS.md`
  - `reify-rs/AGENTS.md`
  - `arboretum-ffi/AGENTS.md`
  - `arboretum-plugin/AGENTS.md`

---

*This project overview is maintained as part of the Arboretum project documentation.*
