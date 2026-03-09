# Arboretum Testing Plan

**Comprehensive Testing Infrastructure with PostgreSQL Integration**

---

## 📑 Table of Contents

| Section | Description |
|---------|-------------|
| [Overview](#overview) | Testing goals and architecture |
| [Infrastructure](#infrastructure) | PostgreSQL setup and teardown |
| [Test Categories](#test-categories) | Testing taxonomy |
| [Test Suite Structure](#test-suite-structure) | Organization of test cases |
| [Implementation Tasks](#implementation-tasks) | What needs to be built |
| [Testing Strategies](#testing-strategies) | Approaches for validation |

---

## Overview

### Goals

This document outlines a comprehensive testing plan for the Arboretum framework to ensure:

1. **Completeness**: All AST node types are properly visited and data extracted
2. **Correctness**: PostgreSQL tables are created and records inserted correctly
3. **Reliability**: Test results are deterministic and reproducible
4. **Maintainability**: Tests are easy to add, understand, and debug

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Test Runner (Python/Shell)                                 │
│  - Sets up PostgreSQL database                              │
│  - Runs compilation with Arboretum plugin                   │
│  - Queries PostgreSQL for verification                      │
│  - Tears down database after test                           │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  PostgreSQL Database (per-test isolate)                     │
│  - Test database created before test                        │
│  - Tables created by reificator plugin                      │
│  - Records inserted during AST traversal                    │
│  - Database dropped after test                              │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  C++ Test Files                                             │
│  - Simple: a.cc, b.cc                                       │
│  - Complex: c.cc (templates, enums)                         │
│  - Edge cases: d.cc ( Lambdas, macros)                      │
│  - New tests added per feature                              │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  Arboretum Plugin                                           │
│  - Clang plugin extracts AST                                │
│  - Calls FFI functions via reify-rs                         │
│  - Records queued in memory                                 │
│  - Flushes to PostgreSQL on finalize                        │
└─────────────────────────────────────────────────────────────┘
```

---

## Infrastructure

### PostgreSQL Management

#### Test Database Isolation

**Each test should use its own database to ensure:**
- No data pollution between tests
- Clean slate for each test run
- Concurrent test execution possible

```bash
# Create test database
psql -d postgres -c "CREATE DATABASE arboretum_test_<test_id>"

# Drop test database
psql -d postgres -c "DROP DATABASE IF EXISTS arboretum_test_<test_id>"
```

#### Connection Management

```bash
# Connection string format
export ARBORETUM_DB_URL="postgres://user:pass@localhost:5432 arboretum_test_<test_id>"

# Or use PostgreSQL connection environment variable
PGHOST=localhost PGPORT=5432 PGDATABASE=arboretum_test_<test_id> psql
```

#### Connection Pool Settings

For tests, use minimal pool size (1 connection):
- Reduces resource usage
- Prevents connection exhaustion
- Simplifies debugging

### Test Execution Environment

#### Prerequisites Check

```bash
# Verify prerequisites before running tests
echo "=== Checking Prerequisites ==="
which clang++ || { echo "ERROR: clang++ not found"; exit 1; }
which psql || { echo "ERROR: psql not found"; exit 1; }
pg_ctl status || { echo "ERROR: PostgreSQL not running"; exit 1; }

# Verify plugin exists
[ -f build/libarboretum.so ] || { echo "ERROR: libarboretum.so not built"; exit 1; }

echo "All prerequisites satisfied!"
```

#### Build Requirements

```bash
# Tests require the plugin to be built
make arboretum

# Optional: Run tests with fresh build
make clean && make arboretum
```

---

## Test Categories

### 1. Functional Tests

Verify AST extraction and PostgreSQL insertion for specific node types.

#### Test Types

| Category | File | Description | Verified Tables |
|----------|------|-------------|-----------------|
| **Expressions** | `expr_*.cc` | Basic expressions | `Expr`, `SourceLocation` |
| **Statements** | `stmt_*.cc` | Control flow | `Stmt`, `IfStmt`, `WhileStmt` |
| **Declarations** | `decl_*.cc` | Variable/function decls | `Decl`, `VarDecl`, `FunctionDecl` |
| **Types** | `type_*.cc` | Type information | `QualType`, `Type` |
| **Classes** | `class_*.cc` | Class hierarchies | `CXXRecordDecl`, `CXXMethodDecl` |
| **Templates** | `template_*.cc` | Template instantiation | `TemplateDecl`, `Specialization` |
| **Enums** | `enum_*.cc` | Enum types | `EnumDecl`, `EnumConstantDecl` |
| **Namespaces** | `ns_*.cc` | Namespace handling | `NamespaceDecl` |

### 2. Schema Verification Tests

Verify that the reificator plugin correctly generates table schemas.

#### Test Scenarios

| Scenario | Test File | Expected Outcome |
|----------|-----------|------------------|
| Property enabled | Check properties.csv entry | Table exists with column |
| Property disabled | Set Enabled=0 in CSV | Table/column NOT created |
| Type mapping | Test various Clang types | Correct PostgreSQL types |
| Reference resolution | Test AST references | IDs properly resolved |

### 3. Data Integrity Tests

Verify that data is correctly inserted and can be queried.

#### Check Items

- **Record counts**: Verify expected number of records per table
- **Reference integrity**: Foreign key relationships preserved
- **Null handling**: Proper NULL values for optional fields
- **String encoding**: UTF-8 strings stored correctly
- **Boolean values**: True/false mapped correctly

### 4. Integration Tests

End-to-end tests that compile real code and verify the complete pipeline.

#### Test Files

| File | Purpose | AST Nodes |
|------|---------|-----------|
| `integration/simple.cc` | Minimal test | FunctionDecl, ReturnStmt |
| `integration/class_hierarchy.cc` | Inheritance | CXXRecordDecl, CXXBaseSpecifier |
| `integration/templates.cc` | Template usage | FunctionTemplateDecl, ClassTemplateDecl |
| `integration/mixed.cc` | Real-world scenario | All major node types |

### 5. Edge Case Tests

Verify robustness against unusual code patterns.

| Edge Case | Test File | Expected Behavior |
|-----------|-----------|-------------------|
| Empty files | `edge/empty.cc` | No crashes, empty result |
| Large files | `edge/large.cc` | Handles large input |
| Nested templates | `edge/nested_templates.cc` | Depth > 3 |
| Macro expansion | `edge/macros.cc` | Macro handling |
| Unicode identifiers | `edge/unicode.cc` | UTF-8 support |

---

## Test Suite Structure

### Directory Layout

```
tests/
├── 0/                                    # Initial integration tests
│   ├── a.cc                              # Simple declarations
│   ├── b.cc                              # Basic syntax
│   ├── c.cc                              # Complex features (templates, enums)
│   ├── d.cc                              # Edge cases
│   ├── AGENTS.md                         # Test documentation
│   └── run.sh                            # Legacy test runner
│
├── v2/                                   # Next-generation test infrastructure
│   ├── __init__.py                       # Python package marker
│   ├── conftest.py                       # Pytest fixtures
│   ├── runner.py                         # Test execution engine
│   ├── postgres.py                       # PostgreSQL management
│   ├──验证器.py                           # Verification functions
│   ├── tests/                            # Test cases
│   │   ├── test_expressions.py          # Expression tests
│   │   ├── test_statements.py           # Statement tests
│   │   ├── test_declarations.py         # Declaration tests
│   │   ├── test_types.py                # Type tests
│   │   ├── test_classes.py              # Class tests
│   │   ├── test_templates.py            # Template tests
│   │   ├── test_enums.py                # Enum tests
│   │   ├── test_namespaces.py           # Namespace tests
│   │   ├── test_schema.py               # Schema verification
│   │   ├── test_integrations.py         # End-to-end tests
│   │   └── test_edge_cases.py           # Edge case tests
│   │
│   └── testdata/                         # Test C++ source files
│       ├── expressions/
│       │   ├── literals.cc
│       │   ├── arithmetic.cc
│       │   ├── logical.cc
│       │   └── relational.cc
│       ├── statements/
│       │   ├── if_stmt.cc
│       │   ├── switch_stmt.cc
│       │   ├── loop_stmts.cc
│       │   └── try_catch.cc
│       ├── declarations/
│       │   ├── functions.cc
│       │   ├── variables.cc
│       │   ├── classes.cc
│       │   └── enums.cc
│       └── edge_cases/
│           ├── empty.cc
│           ├── large.cc
│           ├── macros.cc
│           └── unicode.cc
│
├── expected/                             # Expected test results
│   ├── schema.sql                        # Expected table schemas
│   ├── record_counts.json               # Expected record counts
│   └── reference_data.json              # Expected reference data
│
├── fixtures/                             # Test fixtures and data
│   ├── clang_args.json                  # Common compiler arguments
│   └── postgres_config.json             # Database configuration
│
├── AGENTS.md                             # This file
└── run.py                                # New test runner (Python)
```

### Test Naming Convention

```bash
# Test files
test_<category>_<feature>.py          # e.g., test_expressions_arithmetic.py
test_<feature>_schema.py              # Schema verification tests
test_<feature>_integration.py         # End-to-end integration tests

# C++ test files
<category>_<feature>.cc               # e.g., expr_arithmetic.cc
```

---

## Implementation Tasks

### Phase 1: Infrastructure (Foundation)

#### Task 1.1: Python Test Runner

**File**: `tests/run.py`

```python
#!/usr/bin/env python3
"""Test runner for Arboretum test suite."""

import argparse
import subprocess
import sys
from pathlib import Path
from tests.v2.runner import TestRunner

def main():
    parser = argparse.ArgumentParser(description="Run Arboretum tests")
    parser.add_argument("test", nargs="?", help="Specific test to run")
    parser.add_argument("-v", "--verbose", action="store_true")
    parser.add_argument("--postgres-url", default="postgres://localhost:5432")
    parser.add_argument("--cleanup", action="store_true", help="Clean up after tests")
    
    args = parser.parse_args()
    
    runner = TestRunner(
        postgres_url=args.postgres_url,
        verbose=args.verbose,
        cleanup=args.cleanup
    )
    
    exit_code = runner.run(args.test)
    sys.exit(exit_code)

if __name__ == "__main__":
    main()
```

#### Task 1.2: PostgreSQL Management Module

**File**: `tests/v2/postgres.py`

```python
"""PostgreSQL database management for tests."""

import subprocess
import tempfile
import os
from contextlib import contextmanager
from typing import Optional

class TestDatabase:
    """Manages a PostgreSQL database for testing."""
    
    def __init__(self, postgres_url: str):
        self.postgres_url = postgres_url
        self.db_name = f"arboretum_test_{os.getpid()}"
        self.connection_url = f"{postgres_url.rsplit('/', 1)[0]}/{self.db_name}"
    
    def create(self) -> None:
        """Create the test database."""
        subprocess.run(
            ["psql", "-d", "postgres", "-c", f"CREATE DATABASE {self.db_name}"],
            check=True, capture_output=True
        )
    
    def drop(self) -> None:
        """Drop the test database."""
        subprocess.run(
            ["psql", "-d", "postgres", "-c", f"DROP DATABASE IF EXISTS {self.db_name}"],
            check=True, capture_output=True
        )
    
    def execute(self, sql: str) -> str:
        """Execute SQL and return results."""
        env = os.environ.copy()
        env["PGDATABASE"] = self.db_name
        result = subprocess.run(
            ["psql", "-t", "-c", sql],
            capture_output=True,
            text=True,
            env=env
        )
        return result.stdout.strip()

@contextmanager
def postgres_session(postgres_url: str):
    """Context manager for test database."""
    db = TestDatabase(postgres_url)
    try:
        db.create()
        yield db
    finally:
        db.drop()

# Example usage
# with postgres_session("postgres://localhost:5432") as db:
#     # Run tests here
#     pass
```

#### Task 1.3: Plugin Compilation Module

**File**: `tests/v2/compiler.py`

```python
"""C++ compilation with Arboretum plugin."""

import subprocess
import tempfile
from pathlib import Path
from typing import List, Optional

class CompilationError(Exception):
    """Compilation failed."""
    pass

class TestCompiler:
    """Compiles test files with Arboretum plugin."""
    
    def __init__(self, plugin_path: str, postgres_url: str):
        self.plugin_path = plugin_path
        self.postgres_url = postgres_url
    
    def compile(self, source_file: str, output_dir: Optional[str] = None) -> str:
        """Compile a source file with the Arboretum plugin."""
        output_dir = output_dir or tempfile.mkdtemp()
        output_file = Path(output_dir) / Path(source_file).stem
        
        # Build compiler arguments
        args = [
            "clang++",
            "-c",
            f"-fplugin={self.plugin_path}",
            "-std=c++20",
            "-fno-rtti",
            f"-fplugin-arg=arboretum-connect={self.postgres_url}",
            "-o", str(output_file),
            source_file
        ]
        
        # Run compilation
        result = subprocess.run(
            args,
            capture_output=True,
            text=True
        )
        
        if result.returncode != 0:
            raise CompilationError(
                f"Compilation failed for {source_file}:\n"
                f"STDOUT: {result.stdout}\n"
                f"STDERR: {result.stderr}"
            )
        
        return str(output_file)
```

### Phase 2: Test Cases

#### Task 2.1: Expression Tests

**File**: `tests/v2/tests/test_expressions.py`

```python
"""Tests for expression AST node handling."""

import pytest
from tests.v2.postgres import postgres_session
from tests.v2.compiler import TestCompiler

@pytest.mark.expressions
class TestExpressionTests:
    """Expression-related AST tests."""
    
    def test_literal_expressions(self, postgres_url, plugin_path):
        """Test literal expression extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path, db.connection_url)
            
            # Compile test file
            source = "testdata/expressions/literals.cc"
            compiler.compile(source)
            
            # Verify records inserted
            count = db.execute("SELECT COUNT(*) FROM CXXBoolLiteralExpr;")
            assert int(count) > 0, "No bool literal records found"
    
    def test_arithmetic_operations(self, postgres_url, plugin_path):
        """Test binary operator extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path, db.connection_url)
            
            source = "testdata/expressions/arithmetic.cc"
            compiler.compile(source)
            
            # Verify binary operators captured
            count = db.execute("SELECT COUNT(*) FROM BinaryOperator;")
            assert int(count) > 0, "No binary operator records found"
```

#### Task 2.2: Statement Tests

**File**: `tests/v2/tests/test_statements.py`

```python
"""Tests for statement AST node handling."""

import pytest
from tests.v2.postgres import postgres_session
from tests.v2.compiler import TestCompiler

@pytest.mark.statements
class TestStatementTests:
    """Statement-related AST tests."""
    
    def test_if_statements(self, postgres_url, plugin_path):
        """Test if statement extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path, db.connection_url)
            
            source = "testdata/statements/if_stmt.cc"
            compiler.compile(source)
            
            count = db.execute("SELECT COUNT(*) FROM IfStmt;")
            assert int(count) > 0, "No IfStmt records found"
    
    def test_loop_statements(self, postgres_url, plugin_path):
        """Test loop statement extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path, db.connection_url)
            
            source = "testdata/statements/loop_stmts.cc"
            compiler.compile(source)
            
            for stmt_type in ["WhileStmt", "DoStmt", "ForStmt"]:
                count = db.execute(f"SELECT COUNT(*) FROM {stmt_type};")
                assert int(count) > 0, f"No {stmt_type} records found"
```

#### Task 2.3: Declaration Tests

**File**: `tests/v2/tests/test_declarations.py`

```python
"""Tests for declaration AST node handling."""

import pytest
from tests.v2.postgres import postgres_session
from tests.v2.compiler import TestCompiler

@pytest.mark.declarations
class TestDeclarationTests:
    """Declaration-related AST tests."""
    
    def test_function_declarations(self, postgres_url, plugin_path):
        """Test function declaration extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path, db.connection_url)
            
            source = "testdata/declarations/functions.cc"
            compiler.compile(source)
            
            count = db.execute("SELECT COUNT(*) FROM FunctionDecl;")
            assert int(count) > 0, "No FunctionDecl records found"
    
    def test_variable_declarations(self, postgres_url, plugin_path):
        """Test variable declaration extraction."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path, db.connection_url)
            
            source = "testdata/declarations/variables.cc"
            compiler.compile(source)
            
            count = db.execute("SELECT COUNT(*) FROM VarDecl;")
            assert int(count) > 0, "No VarDecl records found"
```

### Phase 3: Schema Verification

#### Task 3.1: Schema Tests

**File**: `tests/v2/tests/test_schema.py`

**Key Principle**: The schema is **dynamically generated** by the reificator plugin based on `properties.csv`. Tests should verify that:
1. Meta tables always exist (`file`, `source_loc`, `source_range`, `QualType`, `Decl`, `Stmt`, `Expr`)
2. Generated tables exist based on enabled properties (e.g., `FunctionDecl`, `VarDecl`, `CXXRecordDecl`)
3. All tables have required columns (`id`, `beginLoc`, `endLoc`)
4. String columns are TEXT type, boolean columns are BOOLEAN type
5. ID columns are BIGINT primary keys

```python
"""Tests for schema generation."""

import pytest
from tests.v2.postgres import postgres_session
from tests.v2.compiler import TestCompiler

@pytest.mark.schema
class TestMetaTables:
    """Tests for meta tables that should always be created."""
    
    def test_file_table_exists(self, postgres_url, plugin_path):
        """Test that file table is created."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path, db.connection_url)
            compiler.compile("testdata/declarations/functions.cc")
            tables = db.get_tables()
            assert "file" in tables, "file table not created"
    
    def test_qualtype_table_exists(self, postgres_url, plugin_path):
        """Test that QualType table is created."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path, db.connection_url)
            compiler.compile("testdata/declarations/functions.cc")
            tables = db.get_tables()
            assert "QualType" in tables, "QualType table not created"

@pytest.mark.schema
class TestGeneratedTables:
    """Tests for tables generated from properties.csv."""
    
    def test_functiondecl_table_created(self, postgres_url, plugin_path):
        """Test FunctionDecl table is created when property is enabled."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path, db.connection_url)
            compiler.compile("testdata/declarations/functions.cc")
            tables = db.get_tables()
            assert "FunctionDecl" in tables, "FunctionDecl table not created"
    
    def test_all_tables_have_id(self, postgres_url, plugin_path):
        """Test that all tables have id column."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path, db.connection_url)
            compiler.compile("testdata/declarations/functions.cc")
            tables = db.get_tables()
            for table in tables:
                columns = db.get_table_columns(table)
                assert "id" in columns, f"Table {table} missing 'id' column"
```

### Phase 4: Integration Tests

#### Task 4.1: End-to-End Tests

**File**: `tests/v2/tests/test_integrations.py`

```python
"""End-to-end integration tests."""

import pytest
from tests.v2.postgres import postgres_session
from tests.v2.compiler import TestCompiler

@pytest.mark.integration
class TestIntegration:
    """Integration tests."""
    
    def test_complete_pipeline(self, postgres_url, plugin_path):
        """Test the complete extraction pipeline."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path, db.connection_url)
            
            # Compile multiple files
            files = [
                "testdata/declarations/functions.cc",
                "testdata/expressions/arithmetic.cc",
                "testdata/statements/if_stmt.cc"
            ]
            
            for f in files:
                compiler.compile(f)
            
            # Verify data in multiple tables
            tables = {
                "FunctionDecl": "SELECT COUNT(*) FROM FunctionDecl;",
                "BinaryOperator": "SELECT COUNT(*) FROM BinaryOperator;",
                "IfStmt": "SELECT COUNT(*) FROM IfStmt;"
            }
            
            for table, query in tables.items():
                count = db.execute(query)
                assert int(count) > 0, f"No records in {table}"
    
    def test_reference_resolution(self, postgres_url, plugin_path):
        """Test that references are properly resolved."""
        with postgres_session(postgres_url) as db:
            compiler = TestCompiler(plugin_path, db.connection_url)
            
            source = "testdata/declarations/functions.cc"
            compiler.compile(source)
            
            # Verify foreign key relationships
            # Check that a FunctionDecl has a valid returnType reference
            result = db.execute("""
                SELECT f.id, f.returnType 
                FROM FunctionDecl f 
                WHERE f.returnType IS NOT NULL 
                LIMIT 1;
            """)
            
            assert result.strip(), "No FunctionDecl with returnType found"
```

### Phase 5: Test Data Files

#### Task 5.1: Create Test C++ Files

**File**: `tests/v2/testdata/expressions/literals.cc`

```cpp
// Test file for literal expressions

bool bool_true = true;
bool bool_false = false;

int int_literal = 42;
unsigned int uint_literal = 100U;

long long_val = 1000LL;
float float_val = 3.14f;
double double_val = 2.71828;

char char_val = 'A';
const char* string_literal = "Hello, World!";

// Null pointer literal
void* null_ptr = nullptr;
```

**File**: `tests/v2/testdata/expressions/arithmetic.cc`

```cpp
// Test file for arithmetic expressions

int add(int a, int b) {
    return a + b;
}

int subtract(int a, int b) {
    return a - b;
}

int multiply(int a, int b) {
    return a * b;
}

int divide(int a, int b) {
    return a / b;
}

int modulo(int a, int b) {
    return a % b;
}

int negate(int x) {
    return -x;
}

int bitwise_and(int a, int b) {
    return a & b;
}

int bitwise_or(int a, int b) {
    return a | b;
}

int bitwise_xor(int a, int b) {
    return a ^ b;
}

int bitwise_not(int x) {
    return ~x;
}

int left_shift(int x, int n) {
    return x << n;
}

int right_shift(int x, int n) {
    return x >> n;
}
```

**File**: `tests/v2/testdata/statements/if_stmt.cc`

```cpp
// Test file for if statement handling

void test_if(int x) {
    if (x > 0) {
        // positive
    }
}

void test_if_else(int x) {
    if (x > 0) {
        // positive
    } else {
        // non-positive
    }
}

void test_if_elseif(int x) {
    if (x > 0) {
        // positive
    } else if (x < 0) {
        // negative
    } else {
        // zero
    }
}

void test_nested_if(int x, int y) {
    if (x > 0) {
        if (y > 0) {
            // both positive
        }
    }
}
```

### Phase 6: Expected Results

#### Task 6.1: Expected Record Counts

**File**: `tests/expected/record_counts.json`

```json
{
  "expressions/literals.cc": {
    "CXXBoolLiteralExpr": 2,
    "IntegerLiteral": 4,
    "FloatingLiteral": 2,
    "StringLiteral": 1,
    "GNUNullExpr": 1
  },
  "expressions/arithmetic.cc": {
    "BinaryOperator": 15,
    "DeclRefExpr": 30
  },
  "statements/if_stmt.cc": {
    "IfStmt": 5,
    "BinaryOperator": 4
  }
}
```

#### Task 6.2: Expected Schema

**File**: `tests/expected/schema.md`

**Note**: The schema is **dynamically generated** by the reificator plugin. Do not hardcode expected schemas in SQL files.

The expected schema file should document:
1. Meta tables that always exist (`file`, `source_loc`, `source_range`, `QualType`, `Decl`, `Stmt`, `Expr`)
2. Common generated tables based on enabled properties
3. Expected column types for different property types (STRING→TEXT, BOOL→BOOLEAN)

Example content in `tests/expected/schema.md`:

```
# Expected Schema

The schema is dynamically generated by the reificator plugin based on properties.csv.

## Always-Created Tables (Meta Tables)

- `file` - File metadata
- `source_loc` - Source location
- `source_range` - Source range
- `QualType` - Type information
- `Decl` - Base declaration
- `Stmt` - Base statement
- `Expr` - Base expression

## Generated Tables (based on properties.csv)

- `FunctionDecl` - Function declarations
- `VarDecl` - Variable declarations
- `CXXRecordDecl` - C++ class/struct declarations
- `EnumDecl` - Enum declarations
- `IfStmt` - If statements
- And more...

## Column Type Mappings

- STRING → TEXT
- BOOL → BOOLEAN
- Numeric → BIGINT

---

## Testing Strategies

### Strategy 1: Incremental Testing

Start with simple tests and gradually add complexity:

1. **First**: Verify plugin loads and compiles
2. **Second**: Verify basic tables created
3. **Third**: Verify simple expressions
4. **Fourth**: Verify complex features (templates, inheritance)
5. **Fifth**: Verify edge cases and large files

### Strategy 2: Regression Prevention

- Store expected results for each test
- Compare actual vs expected on each run
- Alert on any changes to schema or record counts

### Strategy 3: Performance Testing

- Measure compilation time with plugin
- Verify memory usage is acceptable
- Test with large codebases (100K+ lines)

### Strategy 4: Concurrency Testing

- Run multiple tests in parallel
- Verify no database conflicts
- Test connection pooling behavior

---

## Running Tests

### Basic Usage

```bash
# Run all tests
./tests/run.py

# Run specific test file
./tests/run.py tests/v2/tests/test_expressions.py

# Run with verbose output
./tests/run.py -v

# Run with specific PostgreSQL URL
./tests/run.py --postgres-url postgres://user:pass@host:5432

# Run with cleanup
./tests/run.py --cleanup
```

### CI/CD Integration

```yaml
# .github/workflows/tests.yml
name: Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:14
        env:
          POSTGRES_PASSWORD: test
        ports:
          - 5432:5432
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Set up Python
        uses: actions/setup-python@v4
        with:
          python-version: '3.10'
      
      - name: Install dependencies
        run: |
          python -m pip install --upgrade pip
          pip install pytest
      
      - name: Build project
        run: |
          make llvm-project/build/llvm-stamp
          make arboretum
      
      - name: Run tests
        env:
          POSTGRES_URL: postgres://postgres:test@localhost:5432/postgres
        run: |
          ./tests/run.py
```

---

## Debugging Tests

### Enable Verbose Logging

```bash
# Add debug output to test runner
./tests/run.py -v

# Check PostgreSQL logs
tail -f /var/log/postgresql/*.log

# Enable FFI logging
export RUST_LOG=debug
```

### Test Isolation

If a test fails, you can manually inspect the database:

```bash
# Run test without cleanup
./tests/run.py tests/v2/tests/test_expressions.py --no-cleanup

# Connect to the test database
psql postgresql://localhost:5432/arboretum_test_<pid>

# Query the results
SELECT * FROM FunctionDecl LIMIT 10;
SELECT * FROM BinaryOperator LIMIT 10;
```

### Common Issues

| Issue | Solution |
|-------|----------|
| Database connection refused | Start PostgreSQL: `pg_ctl start` |
| Plugin not found | Build: `make arboretum` |
| Table not found | Check plugin is writing records |
| Record count mismatch | Verify test file has expected AST nodes |
| FFI crash | Check Rust FFI function signatures |

---

## Future Enhancements

### Phase 6: Advanced Features

1. **Snapshot Testing**: Compare complete database dumps
2. **Property-Based Testing**: Generate random C++ code and verify extraction
3. **Fuzz Testing**: Use libFuzzer with Arboretum plugin
4. **Coverage Tracking**: Measure AST node coverage
5. **Benchmark Suite**: Performance regression detection

### Phase 7: Documentation

1. **Test Coverage Report**: Show which AST nodes are tested
2. **Example Test Cases**: Real-world examples in documentation
3. **Troubleshooting Guide**: Common failures and solutions

---

## Summary

This testing plan provides:

- ✅ Complete test suite structure
- ✅ PostgreSQL isolation strategy
- ✅ Python-based test runner
- ✅ Modular test organization
- ✅ Integration with CI/CD
- ✅ Debugging and maintenance tools

**Next Steps:**

1. Implement Python test runner (`tests/run.py`)
2. Implement PostgreSQL management module
3. Create test data files
4. Write initial test cases
5. Add to CI/CD pipeline
