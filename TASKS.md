# Arboretum Task Board

**Last Updated**: March 9, 2026  
**V1 Focus**: See [ROADMAP.md](ROADMAP.md) for detailed milestones

---

## 📑 Table of Contents

| Section | Description |
|---------|-------------|
| [Overview](#overview) | Purpose and scope |
| [Priority Levels](#priority-levels) | How tasks are categorized |
| [Current Priority Tasks](#current-priority-tasks) | V1 critical work |
| [Medium Priority Tasks](#medium-priority-tasks) | Important but not urgent |
| [Low Priority Tasks](#low-priority-tasks) | Nice-to-have improvements |
| [Long-term Vision](#long-term-vision) | Future direction |
| [How to Contribute](#how-to-contribute) | Getting started |

---

## Overview

This file serves as the central task board for the Arboretum project. It's designed to help:

- **Agents**: Find work to do and track progress
- **Contributors**: Identify meaningful tasks to contribute to
- **Maintainers**: Plan roadmap and prioritize work

### Task Categories

1. **Priority Tasks** - Must-do items blocking progress
2. **Medium Priority** - Important improvements
3. **Low Priority** - Enhancements and polish
4. **Long-term Vision** - Strategic goals

### Priority Levels

| Level | Label | Description |
|-------|-------|-------------|
| 🔴 P0 | Critical | Blocking production use, must be done first |
| 🟠 P1 | High | Important for stability and core functionality |
| 🟡 P2 | Medium | Enhancements and documentation |
| 🟢 P3 | Low | Polish, refactoring, nice-to-have |

---

## Current Priority Tasks

### 🔴 P0 - V1 Critical (Q2-Q3 2026)

#### 1. Complete C++ Clang Plugin Extraction
**Priority**: Critical  
**Status**: In Progress  
**Tags**: `clang`, `cpp`, `extraction`

**Goal**: Complete the C++ extractor with LLVM IR support at all three stages.

**Tasks**:
- [ ] Finalize RecursiveASTVisitor implementation
- [ ] Complete LLVM IR extraction (PreOpt, PostInline, PostOpt)
- [ ] Test on sample C++ projects
- [ ] Validate against test suite
- [ ] Performance profiling

**Where to Start**:
- `reificator/src/` - Extractor implementation
- `reify-cpp/src/` - AST visitor
- `arboretum-plugin/src/` - Plugin integration

**Difficulty**: Medium  
**Time Estimate**: 2-4 weeks

---

#### 2. Cross-TU Unification
**Priority**: Critical  
**Status**: Not Started  
**Tags**: `unification`, `graph`

**Goal**: Implement structural hash and candidate verification for cross-TU unification.

**Tasks**:
- [ ] Implement structural hash computation via Postgres CTE
- [ ] Implement candidate verification algorithm
- [ ] Create unified graph tables
- [ ] Implement declaration/definition linking
- [ ] Handle forward declarations

**Where to Start**:
- Review `LONG_TERM_PLAN_DRAFT.md` section 8
- Implement in `arboretum-ffi/src/` or `reify-rs/src/`
- Test with small codebase

**Difficulty**: High  
**Time Estimate**: 3-5 weeks

---

#### 3. All V1 Analyses Implementation
**Priority**: Critical  
**Status**: Not Started  
**Tags**: `analysis`, `cpp`

**Goal**: Implement all V1 analyses (def-use, alias, CDG, dominator tree, lifetime, call graph, ownership).

**Tasks**:
- [ ] Def-use chain analysis
- [ ] Flow-sensitive alias analysis
- [ ] Control Dependence Graph
- [ ] Dominator tree
- [ ] Lifetime extent computation
- [ ] Call graph construction
- [ ] C++ ownership inference

**Where to Start**:
- Implement analyses as Postgres recursive CTEs
- Start with simpler analyses (dominator tree)
- Move to complex analyses (alias analysis)
- All analyses in `reify-rs/src/` or `arboretum-ffi/src/`

**Difficulty**: High  
**Time Estimate**: 4-6 weeks

---

#### 4. Docker Distribution Images
**Priority**: Critical  
**Status**: Not Started  
**Tags**: `docker`, `build`

**Goal**: Create Docker images for RHEL, Debian, and Ubuntu with tooling pre-configured.

**Tasks**:
- [ ] Create RHEL Dockerfile
- [ ] Create Debian Dockerfile
- [ ] Create Ubuntu Dockerfile
- [ ] Test images on hero use cases
- [ ] Performance validation

**Where to Start**:
- Create `docker/` directory
- Start with minimal base image
- Add tooling incrementally

**Difficulty**: Medium  
**Time Estimate**: 1-2 weeks

---

### 🟠 P1 - V1 High Priority

#### 5. Hero Use Case Validation
**Priority**: High  
**Status**: Not Started  
**Tags**: `validation`, `testing`

**Goal**: Validate V1 on OpenSSL, SQLite, BusyBox, GCC, and Node.js.

**Tasks**:
- [ ] Extract OpenSSL (complex, ownership/alias test)
- [ ] Extract SQLite (simple, validation)
- [ ] Extract BusyBox (embedded, distro-scale)
- [ ] Extract GCC (self-hosting, toolchain)
- [ ] Extract Node.js (multi-TU, unification)

**Where to Start**:
- Create `tests/hero/` directory
- Create extraction scripts for each
- Validate results

**Difficulty**: Medium  
**Time Estimate**: 2-3 weeks

---

#### 6. Documentation
**Priority**: High  
**Status**: Not Started  
**Tags**: `docs`, `guide`

**Goal**: Create comprehensive documentation for users and developers.

**Tasks**:
- [ ] User guide (installation, usage, examples)
- [ ] Developer guide (contributing, code organization)
- [ ] API reference (PostgreSQL schema, queries)
- [ ] Tutorial (end-to-end example)
- [ ] FAQ (common questions)

**Where to Start**:
- Review existing docs in `reificator/`, `reify-cpp/`, etc.
- Create user-focused documentation first
- Add developer documentation

**Difficulty**: Low-Medium  
**Time Estimate**: 2-3 weeks

---

#### 7. Wrapper Script Implementation (Rust)
**Priority**: High  
**Status**: Not Started  
**Tags**: `build`, `rust`

**Goal**: Implement Rust wrapper binaries for cc/ld wrapper scripts.

**Tasks**:
- [ ] Implement cc-wrapper binary
- [ ] Implement cxx-wrapper binary
- [ ] Implement ld-wrapper binary
- [ ] Handle build_id linking
- [ ] Postlink binary integration

**Where to Start**:
- Review current wrapper design in `LONG_TERM_PLAN_DRAFT.md`
- Create Rust project in `wrappers/`
- Implement core functionality

**Difficulty**: Medium  
**Time Estimate**: 2-3 weeks

---

## 🟡 P2 - Medium Priority Tasks

### 8. Code Quality & Linting Tools

#### 8a. Set Up clang-tidy for C++
**Status**: Planned  
**Tags**: `quality`, `clang-tidy`

**Tasks**:
- [ ] Create `.clang-tidy` config
- [ ] Add `make analyze-cpp` target
- [ ] Fix all warnings
- [ ] Integrate into CI

**Difficulty**: Low  
**Time**: 1-2 days

---

#### 8b. Configure Clippy Warnings
**Status**: Planned  
**Tags**: `rust`, `clippy`, `quality`

**Tasks**:
- [ ] Run `cargo clippy --workspace`
- [ ] Fix all warnings
- [ ] Add to CI pipeline

**Difficulty**: Low  
**Time**: 1-2 days

---

#### 8c. Implement Code Formatting
**Status**: Planned  
**Tags**: `formatting`, `clang-format`, `rustfmt`

**Tasks**:
- [ ] Create `.clang-format` config
- [ ] Create `rustfmt.toml` config
- [ ] Format all code
- [ ] Add format check to CI

**Difficulty**: Low  
**Time**: 1 day

---

### 9. Testing Infrastructure

#### 9a. Integration Tests for Extraction
**Status**: Planned  
**Tags**: `testing`, `integration`

**Tasks**:
- [ ] Extract test cases
- [ ] Unification test cases
- [ ] Analysis test cases
- [ ] Integration test suite

**Difficulty**: Medium  
**Time**: 2-3 weeks

---

#### 9b. Unit Tests for Core Components
**Status**: Planned  
**Tags**: `testing`, `unit`

**Tasks**:
- [ ] C++ unit tests (GoogleTest)
- [ ] Rust unit tests
- [ ] Property-based tests
- [ ] Fuzzing targets

**Difficulty**: Medium  
**Time**: 2-3 weeks

---

## 🟢 P3 - Low Priority Tasks

### 10. Polish & Enhancements

#### 10a. Add Command-Line Examples
**Status**: Planned  
**Tags**: `docs`, `examples`

**Tasks**:
- [ ] Create `examples/` directory
- [ ] Add real-world C++ examples
- [ ] Document extraction process
- [ ] Document analysis queries

**Difficulty**: Low  
**Time**: 2-4 hours

---

#### 10b. Improve Error Messages
**Status**: Planned  
**Tags**: `ux`, `error-handling`

**Tasks**:
- [ ] Improve Clang plugin error messages
- [ ] Improve Rust error messages
- [ ] Add helpful suggestions

**Difficulty**: Low  
**Time**: 2-4 hours

---

#### 10c. Add Project Status Dashboard
**Status**: Planned  
**Tags**: `docs`, `maintenance`

**Tasks**:
- [ ] Create status page
- [ ] Build status
- [ ] Test coverage
- [ ] Release roadmap

**Difficulty**: Medium  
**Time**: 2-4 hours

---

## 🔵 Long-term Vision

### 11. Future Releases (V2+)

#### 11a. Rust Extractor
**Status**: Future  
**Tags**: `rust`, `extractor`, `hir`, `mir`

**Tasks**:
- [ ] Implement HIR extraction
- [ ] Implement MIR extraction
- [ ] Implement LLVM IR extraction
- [ ] Test on Rust projects

**Difficulty**: High  
**Time**: 4-6 weeks

---

#### 11b. Additional Language Extractors
**Status**: Future  
**Tags**: `python`, `javascript`, `go`, `java`

**Tasks**:
- [ ] Python extractor (priority 1)
- [ ] JavaScript/TypeScript extractor (priority 2)
- [ ] Go extractor (priority 3)
- [ ] Java extractor (priority 4)

**Difficulty**: High  
**Time**: 4-6 weeks per language

---

#### 11c. Global Catalog Infrastructure
**Status**: Future  
**Tags**: `catalog`, `infrastructure`

**Tasks**:
- [ ] PostgreSQL backend
- [ ] Content-addressable storage
- [ ] Caching layer
- [ ] Distribution model

**Difficulty**: High  
**Time**: 6-12 weeks

---

## How to Contribute

### Finding Work

1. **New Contributors**: Start with P2 or P3 tasks - they're well-defined and less risky
2. **Experienced Contributors**: Look at P0/P1 tasks for high-impact work
3. **Agents**: Review this file and ROADMAP.md for task assignment

### Claiming a Task

1. Add a comment showing intent to work
2. Create a branch: `git checkout -b task/<task-number>-<description>`
3. Make your changes
4. Submit a PR referencing this task

### Task Completion Checklist

Before marking a task complete:

- [ ] Code compiles without warnings
- [ ] Tests pass (unit + integration)
- [ ] Documentation updated
- [ ] Code formatted (clang-format, rustfmt)
- [ ] No linting errors (clippy, clang-tidy)
- [ ] PR reviewed and approved

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

---

## Quick Reference

| Priority | Label | Example Tasks |
|----------|-------|---------------|
| Critical | 🔴 P0 | C++ extraction, Unification, All analyses, Docker images |
| High | 🟠 P1 | Hero use cases, Documentation, Wrapper scripts |
| Medium | 🟡 P2 | Code quality, Testing |
| Low | 🟢 P3 | Polish, Examples, Linting |
| Future | 🔵 Long-term | Rust extractor, Additional languages, Global catalog |

---

## Project Status

### Current State (March 2026)
- ✅ C++ Clang plugin extraction (in progress)
- ⏳ Cross-TU unification (not started)
- ⏳ All V1 analyses (not started)
- ⏳ Docker distribution (not started)

### V1 Target
- **Goal**: Q3 2026
- **Focus**: C/C++ only, single package build at scale
- **Key**: Data availability for downstream consumption

---

*This task board is maintained as part of the Arboretum project documentation.*
