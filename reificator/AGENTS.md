# Reificator Plugin

**Schema Generation and AST Extraction for Arboretum**

---

## 📑 Table of Contents

| Section | Description |
|---------|-------------|
| [Overview](#overview) | Plugin purpose and architecture |
| [Schema Generation](#schema-generation) | How properties.csv defines tables |
| [AST Extraction](#ast-extraction) | How AST nodes become graph records |
| [Generated Code](#generated-code) | What code gets generated |
| [Usage](#usage) | Command-line options |
| [Architecture](#architecture) | Internal implementation |

---

## Overview

The `reificator` is a Clang plugin that serves two primary functions:

1. **Schema Generation**: Analyzes Clang's AST to generate a comprehensive knowledge graph schema tailored for C/C++ analysis.

2. **Extractor Plugin**: Creates FFI bindings and table builders for extracting code properties from AST nodes.

### Key Changes

| Before | After |
|--------|-------|
| Parquet file I/O | Direct PostgreSQL INSERT |
| TCP communication | FFI direct calls |
| Buffered writes | Immediate INSERT statements |

## Schema Generation

### properties.csv Format

```
ReturnType    PropertyPath                            ExtractValue
_Bool         c:@N@clang@S@FunctionDecl@F@isVirtual#1   1
String        c:@N@clang@S@FunctionDecl@F@getName#1     1
```

- **ReturnType**: C++ return type (e.g., `_Bool`, `String`)
- **PropertyPath**: Clang USR (Unique Source Reference) for the method
- **ExtractValue**: `1` to extract, `0` to skip

### Table Generation

For each C++ class with enabled properties:
1. Creates a PostgreSQL table named `<ClassName>`
2. Adds columns: `id` (BIGINT PRIMARY KEY) + each property column
3. Determines data types from `properties.csv`

Example output for `FunctionDecl`:

```sql
CREATE TABLE FunctionDecl (
    id BIGINT PRIMARY KEY,
    name TEXT,
    is_defined BOOL,
    is_virtual BOOL,
    -- ... columns from properties.csv
);
```

## AST Extraction

### Record Types

The reificator generates Rust record types matching the schema:

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Record0 {  // FunctionDecl
    pub c0: u64,   // id
    pub c1: String, // name
    pub c2: bool,  // is_defined
    pub c3: bool,  // is_virtual
    // ...
}
```

### FFI Functions

For each table, generates an FFI function:

```rust
#[no_mangle]
pub extern "C" fn arboretum_emit_FunctionDecl(
    c0: u64,   // id
    c1: *const c_char,  // name (as C string)
    c2: bool,  // is_defined
    c3: bool,  // is_virtual
) {
    // Direct PostgreSQL INSERT
}
```

## Generated Code

The reificator generates three sets of code:

### 1. C++ Header (`arboretum_ffi.h`)

```cpp
extern "C" {
void arboretum_emit_FunctionDecl(uint64_t id, const char* name, bool is_defined, bool is_virtual);
void arboretum_emit_CXXRecordDecl(uint64_t id, const char* name);
// ... one per enabled property type
}
```

### 2. Rust FFI (`ffi.rs`)

```rust
#[derive(Debug, Serialize, Deserialize)]
pub enum Record {
    Record0(FunctionDeclRecord),
    Record1(CXXRecordDeclRecord),
    // ...
}

pub static mut TABLE_BUILDERS: Option<TableBuilders> = None;
```

### 3. Rust Table Builders (`io.rs`)

```rust
pub struct TableBuilders {
    db_url: String,
}

impl TableBuilders {
    pub fn new(db_url: impl AsRef<str>) -> Self {
        Self { db_url }
    }

    pub async fn push(&mut self, record: Record) -> Result<()> {
        // Delegates to dynamically generated builders
    }
}
```

## Usage

### Basic Compilation

```bash
clang++ -fplugin=./build/libarboretum.so \
    -c your_code.cpp
```

### Options

The plugin reads configuration from `reificator/properties.csv`:
- Edit this file to enable/disable properties
- Use `1` for enabled, `0` for disabled

## Architecture

### Code Generation Flow

```
properties.csv
    ↓
ReificatorASTConsumer::HandleTranslationUnit()
    ↓
Model::PopulateClangTables()
    ↓
emit_ffi() → emit_io() → emit_ffi_rust()
    ↓
Generated: ffi.rs, io.rs
```

### FFI Integration

```
C++ AST Consumer
    ↓ (arboretum_emit_* calls)
Rust FFI functions
    ↓ (direct INSERT)
PostgreSQL database
```

## Development

### Adding New Property Types

1. Add property to `properties.csv`
2. Rebuild: `make $(BUILD_DIR)/libreificator.so`
3. Run reificator on test code to generate updated code

### Debugging

```bash
# Enable verbose output
clang++ -v -fplugin=./build/libarboretum.so -c test.cpp

# Check generated files
cat build/reificator/arboretum_ffi.h
cat reify-rs/src/ffi.rs
```

## Notes

- Generated code is marked with `BEGIN ARBORETUM GENERATED CODE`
- Do not manually edit generated files - they're overwritten on rebuild
- Properties are extracted lazily during AST traversal
