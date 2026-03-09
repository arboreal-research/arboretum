# Arboretum Code Quality & Maintenance Guide

**Last Updated**: March 9, 2026

---

## 📑 Table of Contents

| Section | Description |
|---------|-------------|
| [Overview](#overview) | Purpose and scope of this guide |
| [Static Analysis Tools](#static-analysis-tools) | C++ and Rust analysis tools |
| [Build & Compilation Checks](#build--compilation-checks) | Enhanced Makefile targets |
| [Testing Infrastructure](#testing-infrastructure) | Unit, integration, and fuzzing tests |
| [Code Quality Metrics](#code-quality-metrics) | Metrics and monitoring |
| [CI/CD Pipeline](#cicd-pipeline) | Automated quality gates |
| [Code Generation Quality](#code-generation-quality) | Generated code validation |
| [Database Schema Validation](#database-schema-validation) | PostgreSQL checks |
| [Performance Monitoring](#performance-monitoring) | Benchmarking suite |
| [Security Considerations](#security-considerations) | Security best practices |
| [Maintenance Automation](#maintenance-automation) | Routine tasks |

---

## Overview

This document outlines tools, techniques, and processes for assessing, maintaining, and improving code quality across the Arboretum project. It serves as a reference for:

- **Developers**: Ensuring new code meets quality standards
- **Maintainers**: Establishing quality gates for contributions
- **Project Leads**: Defining quality policies and standards

### Key Principles

1. **Automated Quality Gates**: Every PR must pass static analysis and tests
2. **Generated Code Transparency**: Generated files must be reviewed and validated
3. **Type Safety First**: Leverage Rust's type system to prevent entire classes of bugs
4. **Documentation as Code**: Maintain documentation alongside code

---

## Static Analysis Tools

### C++ Static Analysis

#### clang-tidy

**Purpose**: Static code analysis for C++ covering style issues, potential bugs, performance problems.

**Configuration**: Create `.clang-tidy` in workspace root:

```yaml
---
Checks: >
  -modernize-*, 
  -misc-*, 
  -performance-*, 
  -readability-*, 
  -bugprone-*, 
  -clang-analyzer-*
HeaderFilterPatterns: ".*"
TidyChecks: "clang-analyzer-*,misc-*,performance-*,readability-*"
WarningAsError: true
```

**Integration**: Add to Makefile:

```makefile
.PHONY: analyze-cpp
analyze-cpp:
	@echo "Running clang-tidy on C++ sources..."
	clang-tidy reificator/src/*.cc reify-cpp/src/*.cc arboretum-plugin/src/*.cc \
		-p $(BUILD_DIR) \
		-header-filter='.*' \
		-sort-results=false
```

**Run Manually**:

```bash
make analyze-cpp
```

**IDE Integration**: Install `clang-tidy` extension in VSCode/CLion for real-time feedback.

---

#### Cppcheck

**Purpose**: Complementary static analysis to clang-tidy with different rule sets.

**Configuration**: Create `cppcheck.json`:

```json
{
  "enable": ["style", "warning", "performance", "portability", "information"],
  "platform": "unix64",
  "std": "c++20"
}
```

**Integration**:

```makefile
.PHONY: analyze-cppcheck
analyze-cppcheck:
	@echo "Running Cppcheck..."
	cpplint --recursive reificator/src/ reify-cpp/src/ arboretum-plugin/src/
```

---

#### PVS-Studio

**Purpose**: Commercial static analyzer (free for open source).

**Use Case**: Deep bug detection, especially in complex template code used in reificator.

**Integration** (after getting license):

```bash
pvs-studio-analyzer analyze -j 8 -o pvs-report.log
```

---

### Rust Static Analysis

#### rust-analyzer + IDE Integration

**Purpose**: Real-time linting and error detection in editors.

**VSCode Settings** (already configured in `.vscode/settings.json`):

```json
{
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.check.allTargets": false,
  "rust-analyzer.diagnostics.enable": true,
  "rust-analyzer.lruCapacity": 8192
}
```

**Install**: `rustup component add rust-analyzer`

---

#### Clippy

**Purpose**: Rust linter for code quality and best practices.

**Integration**: Add to Makefile:

```makefile
.PHONY: analyze-rust
analyze-rust:
	@echo "Running cargo clippy..."
	cargo clippy --workspace --all-targets -- -D warnings -A clippy::type_complexity
```

**Run Manually**:

```bash
make analyze-rust
```

**Configuration**: Create `clippy.toml` in workspace root:

```toml
# Avoid warnings on intentionally complex types for FFI
type-complexity-threshold = 1000

# Allow more complex functions (AST visitors tend to be complex)
cognitive-complexity-threshold = 80

# Disable unnecessary checks for generated code
# (add to clippy.toml if needed)
```

---

## Build & Compilation Checks

### Enhanced Makefile Targets

Add these targets to `/workspace/Makefile`:

```makefile
# Static analysis targets
.PHONY: analyze-cpp
analyze-cpp:
	clang-tidy reificator/src/*.cc reify-cpp/src/*.cc arboretum-plugin/src/*.cc \
		-p $(BUILD_DIR) -header-filter='.*'

.PHONY: analyze-rust
analyze-rust:
	cargo clippy --workspace --all-targets -- -D warnings

# Quality check target
.PHONY: check
check: analyze-cpp analyze-rust
	@echo "✓ All quality checks passed"

# Generated code validation
.PHONY: check-generated
check-generated:
	cargo check --workspace
	@test -s $(BUILD_DIR)/reify-cpp.a || echo "Error: No generated code in reify-cpp.a"

# Format checking
.PHONY: check-format
check-format:
	@echo "Checking C++ formatting..."
	! find reificator/src/ reify-cpp/src/ arboretum-plugin/src/ -name "*.cc" | xargs grep -q "^[[:space:]]*$$"
	@echo "Checking Rust formatting..."
	cargo fmt --check
```

### Compilation Flags Enhancement

Update `CXXFLAGS` in Makefile:

```makefile
CXXFLAGS := -Wpedantic -Werror -std=c++20 -fno-rtti -isystem llvm/include/ -Wall -Wextra
```

Add Rust warning-as-error by default:

```bash
RUSTFLAGS="-D warnings" cargo build
```

---

## Testing Infrastructure

### Unit Tests

#### C++ Unit Tests (GoogleTest)

**Location**: Create `tests/unit/` directory

**Structure**:

```
tests/unit/
├── CMakeLists.txt
├── test_model.cc
├── test_index_builder.cc
└── test_datatype.cc
```

**Example Test** (`tests/unit/test_model.cc`):

```cpp
#include <gtest/gtest.h>
#include "model.h"

TEST(ModelTest, PopulateMetaTables) {
  // Test that meta tables are populated correctly
  clang::ASTContext ctx;
  arboretum::Index index;
  arboretum::Model model(ctx, index);
  
  model.PopulateMetaTables();
  
  // Verify meta tables exist
  EXPECT_GT(model.tables.size(), 0);
}
```

**Add to Makefile**:

```makefile
.PHONY: test-cpp
test-cpp: $(BUILD_DIR)/libreificator.so
	cmake -S tests/unit -B tests/unit/build
	cmake --build tests/unit/build
	tests/unit/build/run_tests
```

---

#### Rust Unit Tests

**Location**: Add tests inline with source or in `tests/` directory

**Example** (`arboretum-ffi/src/lib.rs`):

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffi_argument_conversion() {
        // Test u64 → i64 conversion
        let val: u64 = 42;
        let converted: i64 = val as i64;
        assert_eq!(converted, 42);
    }
}
```

**Run Tests**:

```bash
cargo test --workspace
```

---

### Integration Tests

#### Full Pipeline Tests

**Location**: `tests/integration/`

**Test Script** (`tests/integration/run.sh`):

```bash
#!/bin/bash
set -e

echo "=== Integration Tests ==="

# 1. Build project
make arboretum

# 2. Set up test database
psql -d postgres -c "DROP DATABASE IF EXISTS test_arboretum;"
psql -d postgres -c "CREATE DATABASE test_arboretum;"
psql -d test_arboretum -c "CREATE EXTENSION IF NOT EXISTS pgvector;"

# 3. Compile test file with plugin
./llvm/bin/clang++ -fplugin=./build/libarboretum.so \
    -std=c++20 \
    -Ireify-cpp/include \
    -Iarboretum-ffi/src \
    tests/integration/sample.cpp \
    -o /tmp/sample 2>&1 | tee /tmp/clang_output.txt

# 4. Verify records were inserted
psql -d test_arboretum -c "SELECT COUNT(*) FROM file;"
psql -d test_arboretum -c "SELECT COUNT(*) FROM FunctionDecl;"

# 5. Clean up
psql -d postgres -c "DROP DATABASE test_arboretum;"

echo "=== All Integration Tests Passed ==="
```

---

### Property CSV Validation

**Python Script** (`scripts/validate_properties.py`):

```python
#!/usr/bin/env python3
"""Validate properties.csv format and content."""
import csv
import sys

def validate_properties_csv(path: str) -> bool:
    errors = []
    
    with open(path, 'r') as f:
        reader = csv.DictReader(f)
        expected_cols = {'Type', 'Predicate', 'Enabled'}
        
        if not expected_cols.issubset(set(reader.fieldnames or [])):
            errors.append(f"Missing columns: {expected_cols - set(reader.fieldnames or [])}")
        
        for i, row in enumerate(reader, 1):
            if not row.get('Type'):
                errors.append(f"Row {i}: Missing Type")
            if not row.get('Predicate'):
                errors.append(f"Row {i}: Missing Predicate")
            if row.get('Enabled') not in {'0', '1'}:
                errors.append(f"Row {i}: Invalid Enabled value: {row.get('Enabled')}")
    
    if errors:
        for e in errors:
            print(f"ERROR: {e}")
        return False
    return True

if __name__ == '__main__':
    if validate_properties_csv('reificator/properties.csv'):
        print("✓ properties.csv is valid")
        sys.exit(0)
    else:
        sys.exit(1)
```

**Add to Makefile**:

```makefile
.PHONY: check-properties
check-properties:
	@echo "Validating properties.csv..."
	python3 scripts/validate_properties.py
```

---

### Fuzzing

#### C++ Fuzzing (libFuzzer)

```bash
# Compile with fuzzer support
./llvm/bin/clang++ -fsanitize=fuzzer,address \
    -Ireificator/include \
    -L./build \
    -lreificator \
    -o fuzz_test \
    tests/fuzz/test_fuzzer.cc

# Run fuzzer
./fuzz_test
```

---

## Code Quality Metrics

### C++ Metrics

| Tool | Purpose | Integration |
|------|---------|-------------|
| `ccache` | Build speed monitoring | `make CXX='ccache clang++'` |
| `include-what-you-use` | Detect unused includes | Add to CI pipeline |
| `cpp-inspect` | Code complexity metrics | Generate reports |

### Rust Metrics

| Tool | Purpose | Integration |
|------|---------|-------------|
| `cargo-geiger` | Detect unsafe code usage | `cargo geiger` |
| `cargo-udeps` | Find unused dependencies | `cargo udeps` |
| `cargo-bloat` | Detect code bloat | `cargo bloat --release` |

---

## CI/CD Pipeline

### GitHub Actions Workflow

Create `.github/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: auto
  CC: clang
  CXX: clang++

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rust-analyzer, clippy
      
      - name: Setup C++ Tools
        run: |
          sudo apt-get update
          sudo apt-get install -y clang-tidy
      
      - name: Run Clippy
        run: cargo clippy --workspace --all-targets -- -D warnings
      
      - name: Run clang-tidy
        run: make analyze-cpp

  build:
    runs-on: ubuntu-latest
    needs: lint
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Build LLVM
        run: |
          cd llvm-project
          cmake -S llvm -B build -DLLVM_ENABLE_PROJECTS=clang -DCMAKE_BUILD_TYPE=Release
          cmake --build build -j$(nproc)
      
      - name: Build Project
        run: make arboretum
      
      - name: Check Generated Code
        run: make check-generated

  test:
    runs-on: ubuntu-latest
    needs: build
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Setup PostgreSQL
        uses: pgvector/pgvector@v1
        with:
          version: '15'
      
      - name: Run Tests
        run: make test

  integration:
    runs-on: ubuntu-latest
    needs: test
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Setup PostgreSQL
        uses: pgvector/pgvector@v1
        with:
          version: '15'
      
      - name: Build Project
        run: make arboretum
      
      - name: Run Integration Tests
        run: tests/integration/run.sh
```

---

## Code Generation Quality

### Generated Code Validation Process

1. **Diff Tracking**: Compare generated code before/after changes

```bash
# Before generating code
git diff --name-only > /tmp/before.txt

# Generate code
make refresh-generated

# After generating code
git diff --name-only > /tmp/after.txt

# Show what changed
diff /tmp/before.txt /tmp/after.txt
```

2. **Syntax Validation**: Ensure generated code compiles

```bash
cargo check --workspace
```

3. **Generated Code Tests**: Test that AST visitor produces expected results

---

### Properties CSV Schema

| Column | Type | Description |
|--------|------|-------------|
| `Type` | string | Return type of the Clang method |
| `Predicate` | string | Unique identifier for the property |
| `Enabled` | 0/1 | Whether to include in extraction |

**Validation Rules**:

1. Type must be a valid Clang/LLVM type
2. Predicate must be unique across all rows
3. Enabled must be exactly `0` or `1`

---

## Database Schema Validation

### Schema Validation Queries

**Verify Table Structure**:

```sql
-- List all tables
SELECT table_name, table_type 
FROM information_schema.tables 
WHERE table_schema = 'public' 
ORDER BY table_name;

-- Check column types
SELECT table_name, column_name, data_type 
FROM information_schema.columns 
WHERE table_schema = 'public' 
ORDER BY table_name, ordinal_position;
```

### Schema Versioning

**Create Schema Version Table**:

```sql
CREATE TABLE IF NOT EXISTS schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TIMESTAMPTZ DEFAULT NOW(),
    description TEXT
);
```

**Migration Script** (`migrations/001_add_file_table.sql`):

```sql
-- migration: 001
-- description: Add file table for source file tracking

CREATE TABLE IF NOT EXISTS file (
    id BIGSERIAL PRIMARY KEY,
    filename TEXT NOT NULL,
    md5_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

INSERT INTO schema_version (version, description) 
VALUES (1, 'Add file table') 
ON CONFLICT (version) DO NOTHING;
```

---

## Performance Monitoring

### Benchmark Suite

#### AST Visitor Speed

**Create**: `benchmarks/ast_visitor.rs`

```rust
use criterion::{criterion_group, criterion_main, Criterion};
use reify_rs::{AstVisitor, Context};

fn benchmark_ast_visitor(c: &mut Criterion) {
    let mut ctx = Context::new();
    let visitor = AstVisitor::new(&mut ctx);
    
    c.bench_function("ast_visitor", |b| {
        b.iter(|| visitor.visit_file("tests/benchmarks/sample.cpp"))
    });
}

criterion_group!(benches, benchmark_ast_visitor);
criterion_main!(benches);
```

**Run**:

```bash
cargo bench
```

---

#### PostgreSQL Insert Performance

**Test Script** (`benchmarks/pg_insert.sh`):

```bash
#!/bin/bash

# Test insert performance
for count in 100 1000 10000; do
    echo "Inserting $count records..."
    time psql -d test_arboretum -c "
        INSERT INTO file (filename, md5_hash)
        SELECT 
            'test_' || g || '.cpp',
            md5(random()::text)
        FROM generate_series(1, $count) g;
    "
done
```

---

### Build Performance Monitoring

**Track Build Times**:

```bash
# With ccache
CCACHE_SLOPPINESS=clang_hash_compile_time make clean && time make arboretum

# Without ccache (cold cache)
rm -rf ~/.ccache && time make arboretum
```

---

## Security Considerations

### Rust Unsafe Code Audit

**Use cargo-geiger**:

```bash
cargo geiger --workspace
```

**Focus Areas**:

1. **FFI Boundary** (`arboretum-ffi/src/lib.rs`)
   - Validate all `extern "C"` functions
   - Ensure proper error handling
   - Check for buffer overflows

2. **Generated Code** (`reify-rs/src/ffi.rs`)
   - Validate array bounds
   - Check null pointer handling

3. **Database Operations** (`reify-rs/src/io.rs`)
   - Use parameterized queries (already done)
   - Validate input data types

---

### Input Validation

**Property CSV**:

```python
# Validate predicate format
import re

PREDICATE_PATTERN = r'^c:@N@clang@S@[A-Za-z0-9_]+@F@[A-Za-z0-9_]+#[0-9]+$'

def validate_predicate(pred: str) -> bool:
    return bool(re.match(PREDICATE_PATTERN, pred))
```

---

## Maintenance Automation

### Code Formatting

#### clang-format

**Create** `.clang-format`:

```yaml
---
BasedOnStyle: LLVM
IndentWidth: 2
TabWidth: 2
ColumnLimit: 100
AccessModifierOffset: -1
AlignAfterOpenBracket: Align
AlignConsecutiveAssign: false
AlignConsecutiveDeclarations: false
AlignOperands: true
AllowAllParametersOfDeclOnNextLine: true
AllowShortBlocksOnASingleLine: false
AllowShortCaseLabelsOnASingleLine: false
AllowShortFunctionsOnASingleLine: All
AllowShortIfStatementsOnASingleLine: true
AllowShortLoopsOnASingleLine: true
AlwaysAfterReturnType: None
AlwaysBreakAfterReturnType: None
AlwaysBreakBeforeMultilineStrings: true
AlwaysBreakTemplateDeclarations: Yes
BinPackArguments: true
BinPackParameters: true
BraceWrapping:
  AfterClass: false
  AfterControlStatement: false
  AfterEnum: false
  AfterFunction: true
  AfterNamespace: false
  AfterObjCDeclaration: false
  AfterStruct: false
  AfterUnion: false
  BeforeConstructor: false
  BeforeDestructor: false
  BeforeLambdaBody: false
  BeforeWhile: false
BreakBeforeBinaryOperators: None
BreakBeforeBraces: Attach
BreakBeforeInheritanceComma: false
BreakBeforeTernaryOperators: true
BreakConstructorInitializers: BeforeColon
BreakInheritanceInitializers: BeforeColon
BreakStringLiterals: true
ConstructorInitializerIndentWidth: 4
ContinuationIndentWidth: 4
Cpp11BracedListStyle: true
DerivePointerAlignment: false
PointerAlignment: Left
ReflowComments: true
SortIncludes: false
SortUsingDeclarations: false
SpaceAfterCStyleCast: false
SpaceBeforeAssignmentOperators: true
SpaceBeforeParens: ControlStatements
SpaceInEmptyParentheses: false
SpacesBeforeTrailingComments: 2
Standard: Auto
```

**Run**:

```bash
clang-format -i reificator/src/*.cc reify-cpp/src/*.cc arboretum-plugin/src/*.cc
```

#### rustfmt

**Create** `rustfmt.toml`:

```toml
edition = "2021"
max_width = 100
tab_spaces = 2
```

**Run**:

```bash
cargo fmt
```

---

### Dependency Updates

**Rust Dependencies**:

```bash
# Update all dependencies
cargo update

# Check for outdated dependencies
cargo outdated

# Update specific crate
cargo update -p crate_name
```

**C++ Dependencies (LLVM)**:

```bash
# Update LLVM submodule
cd llvm-project
git fetch origin
git checkout origin/main
cd ..
make clean && make llvm-project/build/llvm-stamp
```

---

### Generated Code Refresh

**Make Target**:

```makefile
.PHONY: refresh-generated
refresh-generated:
	@echo "Refreshing generated code..."
	./llvm/bin/clang++ -fplugin=$(BUILD_DIR)/libreificator.so \
		-c $(CXXFLAGS) reificator/input.cc
```

**Run**:

```bash
make refresh-generated
```

**Review Changes**:

```bash
git diff reify-cpp/src/arboretum_ast_visitor.cc
git diff reify-rs/src/ffi.rs
```

---

## Code Review Checklist

Use this checklist for all PRs:

- [ ] **Static Analysis**: All tools pass without errors
- [ ] **Tests Pass**: Unit, integration, and fuzzing tests all pass
- [ ] **Generated Code**: Regenerated and committed if properties.csv changed
- [ ] **Documentation**: Public APIs documented with Doxygen comments
- [ ] **Rust Safety**: Unsafe blocks minimized and documented
- [ ] **Error Handling**: All error paths handled appropriately
- [ ] **Performance**: No performance regressions detected
- [ ] **Security**: No SQL injection or buffer overflow risks
- [ ] **Formatting**: Code formatted with clang-format/rustfmt
- [ ] **Dependencies**: Updated if needed, no unused dependencies

---

## Quick Reference

### Common Commands

| Task | Command |
|------|---------|
| Run all checks | `make check` |
| C++ linting | `make analyze-cpp` |
| Rust linting | `make analyze-rust` |
| Build project | `make arboretum` |
| Test project | `make test` |
| Refresh generated code | `make refresh-generated` |
| Format code | `make format` |

### Quality Gate Thresholds

| Metric | Threshold |
|--------|-----------|
| clang-tidy errors | 0 (warnings allowed) |
| Clippy warnings | 0 |
| Test coverage | ≥ 80% (new code) |
| Unsafe code | Minimize, document |
| Build time (cold) | < 30 minutes |

### Contact & Support

For questions about this guide:
- Open an issue in the project repository
- Review `docs/CODE_QUALITY.md`
- Check component-specific guides:
  - `reificator/AGENTS.md`
  - `reify-cpp/AGENTS.md`
  - `reify-rs/AGENTS.md`
  - `arboretum-ffi/AGENTS.md`

---

*This guide is maintained as part of the Arboretum project documentation.*
