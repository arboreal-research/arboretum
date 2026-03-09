# 🌳 Arboretum

<p align="center">
  <img src="./media/arboretum2.webp" width="700" alt="An arboretum in decopunk style">
</p>

---

A general-purpose static analysis system for multi-language codebases.

**Extract. Unify. Analyze.**

[![Project Status](https://img.shields.io/badge/status-active-success)](https://github.com/arboretum/arboretum)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![Documentation](https://img.shields.io/badge/docs-read-green)](AGENTS.md)

---

## 📖 Overview

Arboretum is a static analysis system that extracts semantic graphs from source code during compilation. It unifies shared definitions across translation units and enriches the graph with derived program properties including:

- Ownership structure
- Aliasing relationships
- Lifetime analysis
- Control flow
- Control dependence

The extracted data forms a code property graph that can be queried via PostgreSQL for downstream analysis tools, security analyzers, documentation generators, refactoring tools, and AI/ML pipelines.

---

## 🎯 Goal

Arboretum is designed as a **standalone infrastructure component** with no dependency on any downstream consumer. Translation tools, security analyzers, documentation generators, refactoring tools, and any other system that needs deep semantic understanding of source code can consume its output via a language-agnostic query interface.

The C++ to Rust translation tool is one such consumer, but Arboretum is built to serve any consumer that needs comprehensive code understanding.

---

## 🚀 Features

| Feature | Description |
|---------|-------------|
| **Multi-language Support** | C/C++ currently, with Rust, Python, JavaScript, Go, Java coming soon |
| **Semantic Extraction** | Full AST extraction with LLVM IR at multiple optimization stages |
| **Cross-TU Unification** | Identical definitions across translation units are unified into single entities |
| **Program Analysis** | Def-use chains, alias analysis, control dependence, dominator trees, lifetime analysis |
| **Package Integration** | Track package versions, symbol versions, and dependencies |
| **Distro-scale** | Designed to analyze entire Linux distributions |
| **PostgreSQL Backend** | Query results via SQL with recursive CTEs for fixpoint analysis |

---

## 📊 Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Source Code                              │
└─────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Compiler Plugin                              │
│  - Clang (C/C++)                                                │
│  - Rustc (Rust - future)                                        │
└─────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────┐
│                  PostgreSQL Store                               │
│  - Normalized Graph (cpg_node, cpg_edge)                       │
│  - Language-Specific Tables                                     │
│  - Build Artifacts                                              │
│  - Analysis Results                                             │
└─────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────┐
│                  Query Interface                                │
│  - SQL with PostgreSQL                                          │
│  - pgvector extension support                                   │
│  - Recursive CTEs for analysis                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 📦 Quick Start

### Prerequisites

```bash
# Install dependencies
sudo apt install cmake postgresql postgresql-contrib
curl --proto '=https' --tlsonly -sSf https://sh.rustup.rs | sh
```

### Build

```bash
# Build LLVM (first time only - ~15-20 minutes)
make llvm-project/build/llvm-stamp

# Build the project
make arboretum
```

### Usage

```bash
# Compile with Arboretum plugin
clang++ -fplugin=./build/libarboretum.so \
    -std=c++20 \
    your_code.cpp

# Query the results
psql -d arboretum -c "SELECT * FROM cpg_node LIMIT 10;"
psql -d arboretum -c "SELECT * FROM FunctionDecl WHERE name = 'main';"
```

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| **[AGENTS.md](AGENTS.md)** | Project overview for AI agents and developers |
| **[ROADMAP.md](ROADMAP.md)** | Detailed milestones and release timeline |
| **[TASKS.md](TASKS.md)** | Prioritized task board for contributors |
| **[PROJECT_OVERVIEW.md](PROJECT_OVERVIEW.md)** | Executive summary and architecture highlights |
| **[PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md)** | Directory layout and data model |
| **[DOCS_INDEX.md](DOCS_INDEX.md)** | Documentation navigation guide |

### Component Documentation

| Component | Description |
|-----------|-------------|
| **reificator** | Clang plugin for schema generation and AST extraction |
| **reify-cpp** | C++ AST visitor library |
| **reify-rs** | Rust AST reification with PostgreSQL I/O |
| **arboretum-ffi** | FFI bindings for C++ ↔ Rust communication |
| **arboretum-plugin** | Clang plugin integration |

---

## 🎨 Vision

### V1 (Q3 2026) - Current Focus
- C/C++ extraction with LLVM IR
- Cross-TU unification
- All V1 analyses (def-use, alias, CDG, etc.)
- Docker distribution (RHEL, Debian, Ubuntu)

### V2 (2027) - Expansion
- Rust extraction
- Additional language support (Python, JS, Go, Java)
- Package registry
- Distro build system integration

### V3+ - Future
- Global catalog for shared analysis
- Enterprise features
- Advanced analyses
- AI/ML integrations

---

## 💡 Use Cases

Arboretum enables:

| Use Case | Example |
|----------|---------|
| **Security Analysis** | Identify vulnerabilities across dependencies |
| **Code Translation** | Refactor C++ to Rust with semantic awareness |
| **Documentation** | Generate accurate documentation from semantic graph |
| **Refactoring** | Safe, semantic-aware code transformations |
| **AI/ML Training** | Provide structured code data for model training |
| **Legacy Migration** | Understand dependencies before modernization |

---

## 🤝 Contributing

We welcome contributions! Here's how to get involved:

1. Review [TASKS.md](TASKS.md) for available tasks
2. Review [ROADMAP.md](ROADMAP.md) for project direction
3. Read [AGENTS.md](AGENTS.md) for project architecture
4. Comment on a task to claim it
5. Submit a PR

---

## 📄 License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

## 🌟 Acknowledgments

Arboretum is designed to serve the broader software engineering community. We're grateful to the LLVM, Rust, and PostgreSQL communities whose tools make this project possible.

---

*This is a research and development project. While we strive for quality, the API and schema may change as we iterate toward V1.*
