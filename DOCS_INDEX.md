# Arboretum Documentation Index

**Last Updated**: March 9, 2026

---

## Documentation Hierarchy

This document organizes all Arboretum project documentation by purpose and audience.

---

## For All Users

| Document | Purpose | When to Read |
|----------|---------|--------------|
| [README.md](README.md) | High-level overview | First contact with the project |
| [QUICK_REFERENCE.md](QUICK_REFERENCE.md) | Quick commands reference | When you need to run commands fast |
| [PROJECT_OVERVIEW.md](PROJECT_OVERVIEW.md) | Executive summary | Understanding project scope and goals |

---

## For Developers

| Document | Purpose | When to Read |
|----------|---------|--------------|
| [AGENTS.md](AGENTS.md) | Project overview for agents | Understanding project structure |
| [BUILD_SYSTEM.md](BUILD_SYSTEM.md) | Build system details | Before building the project |
| [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md) | Directory layout | Understanding code organization |
| [CODE_QUALITY.md](CODE_QUALITY.md) | Code quality standards | Before contributing code |

---

## For Project Contributors

| Document | Purpose | When to Read |
|----------|---------|--------------|
| [TASKS.md](TASKS.md) | Prioritized task board | Finding work to do |
| [ROADMAP.md](ROADMAP.md) | Detailed milestones | Understanding project direction |
| [LONG_TERM_PLAN_DRAFT.md](LONG_TERM_PLAN_DRAFT.md) | Strategic plan | Understanding architecture and design decisions |

---

## For AI Agents

| Document | Purpose | When to Use |
|----------|---------|-------------|
| [AGENTS.md](AGENTS.md) | Agent-focused documentation | General project understanding |
| [TASKS.md](TASKS.md) | Task board | Finding assigned work |
| [ROADMAP.md](ROADMAP.md) | Milestones | Context for task assignments |
| [LONG_TERM_PLAN_DRAFT.md](LONG_TERM_PLAN_DRAFT.md) | Architecture | Deep technical questions |

---

## Component Documentation

Each component has its own AGENTS.md file with detailed information:

| Component | Documentation | Key Files |
|-----------|---------------|-----------|
| reificator | [reificator/AGENTS.md](reificator/AGENTS.md) | properties.csv, schema generation |
| reify-cpp | [reify-cpp/AGENTS.md](reify-cpp/AGENTS.md) | AST visitor implementation |
| reify-rs | [reify-rs/AGENTS.md](reify-rs/AGENTS.md) | FFI bindings, PostgreSQL I/O |
| arboretum-ffi | [arboretum-ffi/AGENTS.md](arboretum-ffi/AGENTS.md) | FFI layer |
| arboretum-plugin | [arboretum-plugin/AGENTS.md](arboretum-plugin/AGENTS.md) | Plugin integration |
| tests | [tests/AGENTS.md](tests/AGENTS.md) | Testing documentation |

---

## Documentation Maintenance

### Creating New Documentation

When creating new documentation:

1. **Agent-facing docs**: Update [AGENTS.md](AGENTS.md) and add to [TASKS.md](TASKS.md)
2. **Technical docs**: Add to [LONG_TERM_PLAN_DRAFT.md](LONG_TERM_PLAN_DRAFT.md) and [ROADMAP.md](ROADMAP.md)
3. **User docs**: Update [README.md](README.md) and [QUICK_REFERENCE.md](QUICK_REFERENCE.md)

### Document Naming Convention

| Prefix | Purpose |
|--------|---------|
| No prefix | Core documentation |
| AGENTS.md | Agent-facing component docs |
| AGENTS.md (in subdirs) | Component documentation |

---

## Quick Start Guide

### For New Users

1. Read [README.md](README.md) - High-level overview
2. Read [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - Quick commands
3. Install prerequisites (LLVM, Rust, PostgreSQL)
4. Build the project: `make arboretum`
5. Try the examples in [tests/0/](tests/0/)

### For Contributors

1. Read [TASKS.md](TASKS.md) - Available tasks
2. Read [ROADMAP.md](ROADMAP.md) - Project direction
3. Read [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md) - Code organization
4. Read component docs in subdirectories
5. Comment on a task to claim it

### For AI Agents

1. Read [AGENTS.md](AGENTS.md) - Project overview
2. Read [TASKS.md](TASKS.md) - Current priorities
3. Read [ROADMAP.md](ROADMAP.md) - Milestone context
4. Check component-specific docs for deep technical questions

---

## Version Information

| Version | Status | Documentation |
|---------|--------|---------------|
| V1 | In Progress | [ROADMAP.md#v1-timeline](ROADMAP.md) |
| V2 | Planned | [ROADMAP.md#v2-timeline](ROADMAP.md) |
| V3 | Future | [ROADMAP.md#v3-timeline](ROADMAP.md) |

---

## Questions?

### If you need to understand...

- **What this project does**: [README.md](README.md), [PROJECT_OVERVIEW.md](PROJECT_OVERVIEW.md)
- **Where to find work**: [TASKS.md](TASKS.md)
- **What's the timeline**: [ROADMAP.md](ROADMAP.md)
- **How it's designed**: [LONG_TERM_PLAN_DRAFT.md](LONG_TERM_PLAN_DRAFT.md)
- **How to build**: [BUILD_SYSTEM.md](BUILD_SYSTEM.md)
- **How to contribute**: [CODE_QUALITY.md](CODE_QUALITY.md)

---

*This documentation index is maintained as part of the Arboretum project documentation.*

