# Reify C++ Module

**C++ AST Visitor and Data Model**

## Purpose

The `reify-cpp` module provides the C++ implementation for:
1. **AST Visitor**: Traverses Clang's AST and extracts properties
2. **Data Model**: Manages graph data structures and IDs
3. **Context**: Coordinates between AST nodes and property extraction

## Architecture

### Components

| Component | Description |
|-----------|-------------|
| `arboretum_ast_visitor.h/cc` | Recursive AST visitor implementation |
| `arboretum_context.h/cc` | AST processing context |
| `arboretum_data_model.h/cc` | Graph data model interface |
| `arboretum_source_model.h/cc` | Source-level model |

## Key Classes

### ArboretumContext

Central context object that maintains mappings and coordinates extraction:

```cpp
struct ArboretumContext {
  // Mappings from AST nodes to graph IDs
  std::unordered_map<const clang::Decl *, uint64_t> decls;
  std::unordered_map<const clang::Type *, uint64_t> types;
  std::unordered_map<const clang::Stmt *, uint64_t> stmts;
  std::unordered_map<const clang::Attr *, uint64_t> attrs;
  
  // Additional mappings
  std::unordered_map<std::pair<const clang::Type *, unsigned>, uint64_t> qualtypes;
  std::unordered_map<const clang::CFGBlock *, uint64_t> cfgblocks;
  std::unordered_map<const clang::Stmt *, llvm::SmallVector<uint64_t, 0>> stmt_cfgblocks;
  
  // References
  clang::ASTContext &ast_ctx_;
  DataModel &data_model_;
  SourceModel &source_model_;
};
```

#### ID Resolution

```cpp
uint64_t resolve(const clang::Decl *decl);
uint64_t resolve(const clang::Type *type);
uint64_t resolve(const clang::Stmt *stmt);
uint64_t resolve(const clang::Attr *attr);
```

Each `resolve()` method:
1. Checks if node already has an ID
2. If not, generates a new unique ID
3. Stores mapping for future reference

### ArboretumASTVisitor

Implements `clang::RecursiveASTVisitor` with methods for every AST node type:

```cpp
struct ArboretumASTVisitor : public clang::RecursiveASTVisitor<ArboretumASTVisitor> {
  bool Visit*Type(clang::Type* T);
  bool Visit*Decl(clang::Decl* D);
  bool Visit*Stmt(clang::Stmt* S);
};
```

All visitor methods follow this pattern:
```cpp
bool VisitSomeNodeType(clang::SomeNodeType* node) {
  uint64_t id = context_.resolve(node);
  
  // Emit properties for this node
  arboretum_emit_SomeNodeType(
      id,
      context_.resolve(node->getProperty1()),
      node->getProperty2(),
      ...
  );
  
  return true;  // Continue traversal
}
```

### DataModel

Abstracts the graph storage layer:

```cpp
class DataModel {
 public:
  uint64_t next_id();           // Generate next unique ID
  void emit_edge(uint64_t from, uint64_t to, const std::string& predicate);
  void emit_vertex(uint64_t id, const std::string& label);
};
```

### SourceModel

Provides source-level information:

```cpp
class SourceModel {
 public:
  std::string get_source_range(clang::SourceRange range);
  std::string get_filename(clang::SourceLocation loc);
};
```

## FFI Integration

The module calls into `arboretum-ffi` for all external operations:

### Property Emission Functions

```cpp
extern "C" {
void arboretum_emit_AbstractConditionalOperator(uint64_t id, ...);
void arboretum_emit_AccessSpecDecl(uint64_t id, ...);
// ... one function per AST node type
}
```

### Control Flow Graph Emission

```cpp
void emit_cfg(const clang::CFG* cfg) {
  uint64_t cfg_id = context_.resolve(cfg);
  
  for (auto* block : *cfg) {
    uint64_t block_id = context_.resolve(block);
    
    // Emit block properties
    // Emit edges between blocks
    
    for (auto& elem : *block) {
      // Process CFG elements
    }
  }
}
```

## AST Traversal Patterns

### Declaration Visitors

```cpp
bool VisitFunctionDecl(clang::FunctionDecl* decl) {
  uint64_t id = context_.resolve(decl);
  
  // Emit function properties
  arboretum_emit_FunctionDecl(
      id,
      context_.resolve(decl->getReturnType()),
      decl->isVariadic(),
      context_.resolve(decl->getBody()),
      ...
  );
  
  // Visit function body
  if (decl->getBody()) {
    decl->getBody()->visit(context_);
  }
  
  return true;
}
```

### Type Visitors

```cpp
bool VisitRecordType(clang::RecordType* type) {
  uint64_t id = context_.resolve(type);
  
  // Emit type properties
  arboretum_emit_RecordType(
      id,
      context_.resolve(type->getDecl()),
      ...
  );
  
  return true;
}
```

### Statement Visitors

```cpp
bool VisitCallExpr(clang::CallExpr* call) {
  uint64_t id = context_.resolve(call);
  
  // Emit call properties
  arboretum_emit_CallExpr(
      id,
      context_.resolve(call->getCallee()),
      ...
  );
  
  return true;
}
```

## ID Management

### USR Generation

```cpp
llvm::SmallVector<char, 512> buf;
if (!clang::index::generateUSRForDecl(decl, buf)) {
  arboretum_emit_Decl_usr(result, std::string(buf.data(), buf.size()).c_str());
}
```

### Control Flow Graph Building

When processing function definitions:
```cpp
auto bo = clang::CFG::BuildOptions();
bo.AddEHEdges = true;
bo.AddInitializers = true;
bo.AddImplicitDtors = true;
bo.AddLifetime = true;
bo.AddLoopExit = true;
bo.AddTemporaryDtors = true;
bo.AddScopes = true;

auto cfg = emit_cfg(clang::CFG::buildCFG(...));
```

## Building

### Dependencies

- Clang AST library
- arboretum-ffi (Rust FFI library)

### Build Command

```bash
./llvm/bin/clang++ -g -c \
    -std=c++20 -fno-rtti \
    -Ireify-cpp/include/ \
    -Iarboretum-ffi/src/ \
    -MMD -MF $(BUILD_DIR)/reify-cpp/$*.d \
    -o $@ $<
```

## Integration Points

| Parent | Purpose |
|--------|---------|
| `reificator/` | Schema and extraction configuration |
| `arboretum-plugin/` | Clang plugin wrapper |
| `arboretum-ffi/` | FFI to Rust storage layer |

## Testing

Test input files are in `/workspace/tests/0/`:

```bash
# Test compilation with the plugin
clang++ -fplugin=$(BUILD_DIR)/libarboretum.so \
        -c tests/0/a.cc
```

## 📖 Additional Documentation

| Component | Documentation |
|-----------|---------------|
| [Reificator](../reificator/AGENTS.md) | Schema and extraction configuration |
| [Arboretum FFI](../arboretum-ffi/AGENTS.md) | Rust FFI bindings |
| [Arboretum Plugin](../arboretum-plugin/AGENTS.md) | Clang plugin integration |

## Notes

- All generated code marked with `BEGIN ARBORETUM GENERATED CODE` (don't edit manually)
- Visitors should return `true` to continue traversal
- Use `resolve()` to ensure unique IDs for all AST nodes
- CFG generation is only done for function definitions with bodies
