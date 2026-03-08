# Tests Directory

**Project Test Suite with PostgreSQL Backend**

---

## 📑 Table of Contents

| Section | Description |
|---------|-------------|
| [Overview](#overview) | Test suite structure |
| [Directory Structure](#directory-structure) | Layout of test files |
| [Running Tests](#running-tests) | Test execution instructions |
| [Test Cases](#test-cases) | Test file descriptions |
| [CI/CD Integration](#cicd-integration) | Automated testing |
| [Troubleshooting](#troubleshooting) | Common issues |
| [Adding Tests](#adding-tests) | Creating new tests |

---

## Overview

The `tests/` directory contains integration tests for the Arboretum framework. Tests verify that AST extraction works correctly and that graph data is properly inserted into PostgreSQL tables.

### Architecture Changes

| Before | After |
|--------|-------|
| `arboretumd` daemon running | Direct PostgreSQL writes |
| Parquet files on disk | PostgreSQL tables |
| TCP connection | FFI direct calls |

## Structure

```
tests/
├── 0/                    # Integration test suite
│   ├── a.cc             # Simple C++ test file
│   ├── b.cc             # Basic syntax tests
│   ├── c.cc             # Complex features (templates, classes)
│   ├── d.cc             # Edge cases
│   ├── AGENTS.md        # Test documentation
│   └── run.sh           # Test runner script
└── AGENTS.md            # This file
```

## Running Tests

### Prerequisites

1. **PostgreSQL must be running**:
   ```bash
   pg_ctl -D /usr/local/var/postgres start
   ```

2. **Database must exist**:
   ```bash
   psql -d postgres -c "CREATE DATABASE arboretum;"
   ```

### Automated Testing

```bash
# Run all tests using the script
./tests/run.sh

# Or manually run each test
./tests/run.sh tests/0/a.cc
./tests/run.sh tests/0/b.cc
```

### Manual Testing

```bash
# Build the project first
make arboretum

# Compile test files with Arboretum plugin - writes to PostgreSQL
clang++ -c \
    -fplugin=./build/libarboretum.so \
    -std=c++20 \
    tests/0/a.cc

# Verify data was inserted
psql -d arboretum -c "SELECT * FROM file;"
```

## Test Cases

### Simple Syntax Tests (a.cc, b.cc)

Tests basic AST node handling:
- Function declarations
- Variable declarations
- Basic expressions

### Advanced Features (c.cc, d.cc)

Tests more complex language features:
- Template instantiations
- Class definitions and methods
- Inheritance hierarchies
- Lambda expressions

## Test Execution Flow

```bash
# 1. Ensure PostgreSQL is running
pg_ctl status

# 2. Build Arboretum plugin
make arboretum

# 3. Run test script
./tests/run.sh
```

### What the Script Does

1. Sets C++ compiler flags (`-c -fplugin=... -std=c++20`)
2. Compiles each test file with the Arboretum plugin
3. Verifies PostgreSQL tables were created and populated
4. Reports success or failure
5. Cleans up object files

## Expected Behavior

### Successful Test

```
+ clang++ -c -fplugin=../build/libarboretum.so -std=c++20 0/a.cc
Traversal complete.
+ psql -d arboretum -c "SELECT COUNT(*) FROM file;"
 count 
-------
     1
```

### Failure Cases

| Error | Cause | Solution |
|-------|-------|----------|
| Connection refused | PostgreSQL not running | Start PostgreSQL with `pg_ctl` |
| Plugin not found | Not built | Run `make arboretum` |
| Unknown parameter | Wrong version | Update plugin |
| Missing table | Properties not generated | Rebuild reificator |

## CI/CD Integration

Tests are run as part of continuous integration:

```bash
# In CI pipeline
make llvm-project/build/llvm-stamp
make arboretum
./tests/run.sh
```

## Adding New Tests

1. Create a new `.cc` file in `tests/0/`
2. Add test cases that exercise specific AST nodes
3. Update `run.sh` to include the new test
4. Verify output with manual testing
5. Check PostgreSQL tables are populated:

   ```bash
   psql -d arboretum -c "SELECT table_name FROM information_schema.tables WHERE table_schema='public';"
   ```

### Test File Template

```cpp
// tests/0/test_name.cc

// Test case 1: Function declaration
void simple_function() {
    int x = 5;
}

// Test case 2: Class definition
class TestClass {
public:
    void method() {}
};

// Test case 3: Template function
template<typename T>
T template_func(T t) { return t; }
```

## Debugging Tests

### Enable Verbose Output

```bash
set -x ./tests/run.sh
```

### Check Plugin Connection

```bash
# Start PostgreSQL in foreground for debugging
pg_ctl -D /usr/local/var/postgres start -l /tmp/pg.log

# Compile with plugin
clang++ -fplugin=./build/libarboretum.so \
    -c tests/0/a.cc

# Check PostgreSQL logs
tail -f /tmp/pg.log
```

### Verify Tables

```bash
# List all tables
psql -d arboretum -c "\dt"

# Check specific table
psql -d arboretum -c "SELECT COUNT(*) FROM file;"
```

## 📖 Additional Documentation

| Component | Documentation |
|-----------|---------------|
| [`tests/0/`](0/AGENTS.md) | Integration test details and templates |
| [`reificator/AGENTS.md`](../reificator/AGENTS.md) | Clang plugin documentation |
| [`BUILD_SYSTEM.md`](../BUILD_SYSTEM.md) | Build instructions |

## Notes

- Tests are intentionally simple to verify core functionality
- Complex tests should go in project-specific test directories
- Test files use standard C++ that exercises Clang AST nodes
- PostgreSQL tables are automatically created by reificator plugin
