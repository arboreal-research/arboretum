# Testing Infrastructure Summary

This document summarizes the testing infrastructure that was drafted for the Arboretum project.

## Key Principles

1. **Schema is dynamically generated** - The reificator plugin creates PostgreSQL tables based on `properties.csv`. Tests should NOT hardcode expected schemas.

2. **Test database isolation** - Each test should use its own database to ensure no data pollution between tests.

3. **Tests verify generated schema** - Tests should:
   - Verify meta tables always exist (`file`, `source_loc`, `source_range`, `QualType`, `Decl`, `Stmt`, `Expr`)
   - Verify generated tables exist based on enabled properties
   - Check all tables have required columns (`id`, `beginLoc`, `endLoc`)
   - Verify column type mappings (STRING→TEXT, BOOL→BOOLEAN, Numeric→BIGINT)

## Files Created

### Infrastructure (tests/v2/)

| File | Purpose |
|------|---------|
| `__init__.py` | Python package marker |
| `conftest.py` | Pytest fixtures and configuration |
| `runner.py` | Test runner with argument parsing |
| `postgres.py` | PostgreSQL database management (create/drop per test) |
| `compiler.py` | C++ compilation with Arboretum plugin |
| `verifier.py` | Verification functions for test results |

### Test Cases (tests/v2/tests/)

| File | Description |
|------|-------------|
| `test_expressions.py` | Expression handling tests (literals, arithmetic, conditionals) |
| `test_statements.py` | Statement handling tests (if, loops, switch, try-catch) |
| `test_declarations.py` | Declaration handling tests (functions, variables, classes, enums) |
| `test_types.py` | Type handling tests (QualType, pointer types, qualified types) |
| `test_classes.py` | Class handling tests (inheritance, virtual methods, constructors) |
| `test_templates.py` | Template handling tests (class templates, function templates, specializations) |
| `test_enums.py` | Enum handling tests (scoped enums, underlying types) |
| `test_namespaces.py` | Namespace handling tests (using directives, aliases) |
| `test_integrations.py` | End-to-end integration tests |
| `test_edge_cases.py` | Edge case tests (empty files, large files, Unicode, macros) |

### Test Data (tests/testdata/)

| Directory | Contents |
|-----------|----------|
| `expressions/` | Test files for expressions (literals, arithmetic) |
| `statements/` | Test files for statements (if, loops, switch) |
| `declarations/` | Test files for declarations (functions, classes, enums, templates) |

### Expected Results (tests/expected/)

| File | Purpose |
|------|---------|
| `record_counts.json` | Expected record counts per test file |
| `schema.md` | Schema documentation (not SQL - schema is generated dynamically) |

## Testing Plan Overview

### Test Categories

1. **Expression Tests** - Verify literal, arithmetic, and conditional expression extraction
2. **Statement Tests** - Verify control flow statement extraction (if, loops, switch)
3. **Declaration Tests** - Verify function/variable/class/enum declaration extraction
4. **Type Tests** - Verify type information and reference resolution
5. **Class Tests** - Verify class hierarchy, inheritance, and methods
6. **Template Tests** - Verify template instantiation and specialization
7. **Enum Tests** - Verify enum and enum class extraction
8. **Namespace Tests** - Verify namespace and using directive handling
9. **Integration Tests** - End-to-end pipeline verification
10. **Edge Case Tests** - Robustness against unusual code patterns

### Running Tests

```bash
# Install pytest
pip install pytest

# Run all tests
./tests/run.py

# Run specific test file
./tests/run.py tests/v2/tests/test_expressions.py

# Run with verbose output
./tests/run.py -v

# Run with specific PostgreSQL URL
./tests/run.py --postgres-url postgres://user:pass@host:5432

# Run with debug database (no cleanup)
./tests/run.py --no-cleanup
```

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│  Test Runner (tests/v2/runner.py)                              │
│  - Argument parsing                                              │
│  - Calls pytest for test execution                              │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  Pytest Fixtures (tests/v2/conftest.py)                        │
│  - postgres_url fixture                                          │
│  - plugin_path fixture                                           │
│  - test_db fixture (creates/drops per test)                    │
│  - test_compiler fixture                                         │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  PostgreSQL Session (tests/v2/postgres.py)                     │
│  - Creates test database: arboretum_test_<pid>                  │
│  - Drops database after test                                    │
│  - Executes SQL queries                                         │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  Compiler (tests/v2/compiler.py)                               │
│  - Compiles C++ with -fplugin=libarboretum.so                   │
│  - Sets -fplugin-arg=arboretum-connect=<postgres_url>          │
│  - Catches compilation errors                                   │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  C++ Test File                                                  │
│  - exercises specific AST nodes                                 │
│  - triggers plugin extraction                                   │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  Arboretum Plugin                                               │
│  - Extracts AST                                                 │
│  - Calls FFI functions                                          │
│  - Flushes records to PostgreSQL                                │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│  PostgreSQL Database                                            │
│  - Tables created by reificator plugin                          │
│  - Records inserted during AST traversal                        │
│  - Dropped after test                                           │
└─────────────────────────────────────────────────────────────────┘
```

## Next Steps

1. **Build the project**: `make arboretum`
2. **Run tests**: `./tests/run.py`
3. **Verify tests pass** - All tests should pass with green output
4. **Add to CI/CD** - Integrate test runner into GitHub Actions or other CI

## Notes

- The schema is dynamically generated based on `properties.csv`
- Tests verify generated schema by checking table/column existence
- Database isolation ensures tests are independent
- Expected record counts provide validation of correct extraction
