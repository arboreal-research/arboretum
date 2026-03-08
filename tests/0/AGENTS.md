# Integration Tests

**C/C++ Test Files for Arboretum with PostgreSQL Backend**

---

## 📑 Table of Contents

| Section | Description |
|---------|-------------|
| [Overview](#overview) | Test files description |
| [Test Files](#test-files) | Individual test file details |
| [Test Execution](#test-execution) | Running tests |
| [Test Template](#test-template) | Creating new tests |

---

## Overview

The `tests/0/` directory contains C/C++ test files used to verify Arboretum's AST extraction functionality. Each test file exercises different aspects of the Clang AST and generates corresponding PostgreSQL tables.

### Test Files

| File | Purpose | AST Nodes Tested |
|------|---------|------------------|
| `a.cc` | Simple declarations | Functions, variables |
| `b.cc` | Basic syntax | Statements, expressions |
| `c.cc` | Complex features | Templates, classes, inheritance |
| `d.cc` | Edge cases | Lambda, macros, pragmas |

## Test Files

### a.cc - Simple Declarations

Tests basic AST node handling:

```cpp
void simple_function() {
    int x = 5;
}

class SimpleClass {
public:
    void method() {}
};
```

**Generates**: `FunctionDecl`, `CXXRecordDecl` tables

### b.cc - Basic Syntax

Tests statement and expression handling:

```cpp
int add(int a, int b) {
    return a + b;
}

if (x > 0) {
    do_something();
}
```

**Generates**: `FunctionDecl`, `IfStmt`, `ReturnStmt` tables

### c.cc - Complex Features

Tests advanced language features:

```cpp
template<typename T>
T template_func(T t) { return t; }

class Base {
public:
    virtual void vfunc() {}
};

class Derived : public Base {
public:
    void vfunc() override {}
};
```

**Generates**: `FunctionTemplateDecl`, `CXXRecordDecl`, `CXXBaseSpecifier` tables

### d.cc - Edge Cases

Tests corner cases:

```cpp
auto lambda = [](int x) { return x * 2; };

#pragma once

#define MAX(a, b) ((a) > (b) ? (a) : (b))
```

**Generates**: `LambdaExpr`, `PragmaDirective` tables

## Test Execution

### Running Tests

```bash
# Build project first
make arboretum

# Run all tests
./tests/run.sh

# Or test individual files
clang++ -fplugin=./build/libarboretum.so \
    -std=c++20 \
    -c tests/0/a.cc
```

### Verifying Output

After compilation, check PostgreSQL tables:

```bash
# List tables created by reificator
psql -d arboretum -c "\dt"

# Count records in each table
for table in FunctionDecl CXXRecordDecl IfStmt; do
    psql -d arboretum -c "SELECT COUNT(*) FROM $table;"
done
```

## Test Template

### Minimal Test File

```cpp
// your_test_name.cc

// Define test cases here
void test_function() {
    int x = 5;
}

class TestClass {
public:
    void test_method() {}
};
```

### Add to Test Suite

1. Create new `.cc` file in `tests/0/`
2. Add test cases as shown above
3. Update `tests/run.sh`:

   ```bash
   clang++ -c -fplugin=../build/libarboretum.so -std=c++20 0/your_test_name.cc
   ```

4. Run tests to verify

## Debugging Tests

### Enable Verbose Output

```bash
clang++ -v -fplugin=./build/libarboretum.so \
    -std=c++20 \
    -c tests/0/a.cc
```

### Check PostgreSQL Logs

```bash
# Find PostgreSQL log location
psql -d postgres -c "SHOW log_file;"
tail -f /var/log/postgresql/*.log
```

## Notes

- Tests are designed to be minimal and fast
- Each test exercises specific AST node types
- Results are stored in PostgreSQL tables for verification
