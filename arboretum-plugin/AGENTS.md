# Arboretum Plugin

**Clang Plugin Integration for PostgreSQL Storage**

---

## 📑 Table of Contents

| Section | Description |
|---------|-------------|
| [Overview](#overview) | Plugin purpose and architecture |
| [Key Components](#key-components) | Plugin components explained |
| [Data Flow](#data-flow) | Graph data flow diagram |
| [Usage](#usage) | Command-line usage |
| [Debugging](#debugging) | Troubleshooting guide |

---

## Overview

The `arboretum-plugin` provides the Clang plugin integration that:
1. **AST Traversal**: Visits AST nodes during compilation
2. **Data Emission**: Converts AST data to FFI calls for PostgreSQL
3. **Direct Storage**: Writes graph data directly to PostgreSQL tables

## Architecture

### Components

| Component | Description |
|-----------|-------------|
| `plugin.cc` | Main plugin entry point and AST visitor |

## Key Components

### Clang Plugin Entry Point

```cpp
// Register the reificator AST consumer
class ReificatorASTConsumer : public clang::ASTConsumer {
  void HandleTranslationUnit(clang::ASTContext &ctx) override;
};
```

### AST Visitor

```cpp
class ASTVisitor : public RecursiveASTVisitor<ASTVisitor> {
  // Visits each AST node and emits data via FFI
  bool VisitFunctionDecl(FunctionDecl *D);
  bool VisitCXXRecordDecl(CXXRecordDecl *D);
};
```

## Data Flow

```
Clang Plugin (arboretum-plugin)
    ↓ AST traversal
Rust FFI layer (arboretum-ffi)
    ↓ Direct INSERT
PostgreSQL tables
```

### Connection Flow (Simplified)

```
C++ Code (with Clang plugin)
    ↓ FFI calls
Rust extern "C" functions
    ↓ deadpool_postgres
PostgreSQL connection pool
    ↓ INSERT
PostgreSQL tables
```

### Record Processing

```cpp
// When visiting a FunctionDecl:
void ASTVisitor::VisitFunctionDecl(FunctionDecl *D) {
    // Get properties from AST
    std::string name = D->getNameAsString();
    bool is_virtual = D->isVirtualAsWritten();
    
    // Emit via FFI - writes directly to PostgreSQL
    arboretum_emit_FunctionDecl(id, name.c_str(), is_virtual);
}
```

## Usage

### Compile with Plugin

```bash
# Build project first
make arboretum

# Compile with Arboretum plugin - writes to PostgreSQL
clang++ -fplugin=./build/libarboretum.so \
    -std=c++20 \
    -c your_code.cpp
```

### Verify PostgreSQL Output

```bash
# Check tables created by reificator
psql -d arboretum -c "\dt"

# Query function declarations
psql -d arboretum -c "SELECT id, name FROM FunctionDecl;"
```

## Debugging

### Enable Verbose Output

```bash
clang++ -v -fplugin=./build/libarboretum.so \
    -std=c++20 \
    -c test.cpp
```

### Check PostgreSQL Logs

```bash
# Find PostgreSQL log location
psql -d postgres -c "SHOW log_file;"

# Monitor logs
tail -f /var/log/postgresql/*.log
```

### Common Issues

| Issue | Solution |
|-------|----------|
| Plugin not found | Build with `make arboretum` first |
| Connection error | Ensure PostgreSQL is running |
| Missing tables | Rebuild reificator to generate schema |

## 📖 Additional Documentation

| Component | Documentation |
|-----------|---------------|
| [Reificator](../reificator/AGENTS.md) | Schema generation and AST extraction |
| [Reify-RS](../reify-rs/AGENTS.md) | Rust FFI and PostgreSQL I/O |
| [Tests](../tests/AGENTS.md) | Integration tests |

## Notes

- No `arboretumd` daemon required - direct PostgreSQL writes
- Uses FFI for C++ → Rust communication
- Connection pooling via deadpool_postgres for efficiency
