# VSCode Configuration

**Developer IDE Setup**

---

## 📑 Table of Contents

| Section | Description |
|---------|-------------|
| [Overview](#overview) | Configuration purpose |
| [Configuration Files](#configuration-files) | VSCode settings explained |
| [Usage](#usage) | How to use configuration |
| [Dependencies](#dependencies) | Required extensions |
| [Features](#features) | Available IDE features |
| [Troubleshooting](#troubleshooting) | Common issues |

---

## Overview

The `.vscode/` directory contains VSCode configuration files for consistent development across team members.

## Configuration Files

### c_cpp_properties.json

Contains C/C++ extension settings:

```json
{
  "configurations": [
    {
      "name": "Arboretum",
      "compilerPath": "/workspace/llvm/bin/clang++",
      "cStandard": "c17",
      "cppStandard": "c++20",
      "includePath": [
        "/workspace/llvm/include"
      ]
    }
  ]
}
```

### settings.json

Contains workspace-specific settings:

| Setting | Purpose |
|---------|---------|
| C/C++ include paths | Points to LLVM headers |
| Compiler path | Uses built LLVM, not system Clang |
| Standard versions | C17 and C++20 |

## Usage

### VSCode Integration

1. Open the workspace in VSCode
2. The extension automatically uses these settings
3. C/C++ features (intellisense, diagnostics) work correctly

### Manual Configuration

To manually configure VSCode for this project:

```json
{
  "C_Cpp.default.cStandard": "c17",
  "C_Cpp.default.cppStandard": "c++20",
  "C_Cpp.default.includePath": [
    "/workspace/llvm/include"
  ]
}
```

## Dependencies

| Extension | Purpose |
|-----------|---------|
| C/C++ (ms-vscode.cpptools) | C/C++ language support |
| Rust-analyzer (rust-lang.rust-analyzer) | Rust language support |

## Features

### IntelliSense

- Accurate autocomplete for LLVM/Clang APIs
- Correct symbol resolution for custom code

### Diagnostics

- Compile-time error checking using built LLVM
- Proper include path resolution

## Troubleshooting

### IntelliSense Not Working

1. Reload window: `Ctrl+Shift+P` → "Reload Window"
2. Reindex C/C++: `Ctrl+Shift+P` → "C/C++: Reset C/C++ Database"

### Compiler Path Issues

Ensure LLVM is built:
```bash
make llvm-project/build/llvm-stamp
```

Update `c_cpp_properties.json` if LLVM path changes.

## Notes

- Settings reference `/workspace/llvm/` as the installation directory
- Use `make` to ensure LLVM is built before VSCode configuration
- Configuration matches Makefile build system
