# Expected Schema

The schema is **dynamically generated** by the reificator plugin based on `properties.csv`.

## Always-Created Tables (Meta Tables)

These tables are created regardless of which properties are enabled:

| Table | Description |
|-------|-------------|
| `file` | File metadata (filename, source locations) |
| `source_loc` | Source location (file ID, line, column) |
| `source_range` | Source range (begin/end locations) |
| `QualType` | Type information |

## Base AST Tables

| Table | Description |
|-------|-------------|
| `Decl` | Base declaration table |
| `Stmt` | Base statement table |
| `Expr` | Base expression table |

## Generated Tables

The following tables are generated based on enabled properties in `properties.csv`:

- `FunctionDecl` - Function declarations
- `VarDecl` - Variable declarations  
- `CXXRecordDecl` - C++ class/struct declarations
- `EnumDecl` - Enum declarations
- `IfStmt` - If statements
- `ForStmt` - For loops
- `WhileStmt` - While loops
- `DoStmt` - Do-while loops
- `SwitchStmt` - Switch statements
- `CaseStmt` - Case labels
- `BinaryOperator` - Binary operators
- `UnaryOperator` - Unary operators
- `ConditionalOperator` - Ternary operators
- And more based on enabled properties

## Schema Verification Strategy

Tests should **not** hardcode expected schemas. Instead:

1. **Verify base tables exist**: `file`, `source_loc`, `source_range`, `QualType`, `Decl`, `Stmt`, `Expr`
2. **Check for generated tables**: Query `information_schema.tables` to see what was created
3. **Verify columns**: For each created table, check that it has required columns like `id`, `beginLoc`, `endLoc`
4. **Record counts**: Verify that tables have expected record counts for test cases

See `tests/v2/tests/test_schema.py` for implementation.
