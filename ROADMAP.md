# Arboretum Project Roadmap

**Last Updated**: March 9, 2026  
**Document Version**: 1.0  
**Status**: Active Planning

---

## 📑 Table of Contents

| Section | Description |
|---------|-------------|
| [Executive Summary](#executive-summary) | High-level overview |
| [V1 Success Criteria](#v1-success-criteria) | What "done" means for V1 |
| [V1 Timeline](#v1-timeline) | Milestones for Q2-Q3 2026 |
| [V2 Timeline](#v2-timeline) | Milestones for Q4 2026 - Q1 2027 |
| [V3+ Timeline](#v3-timeline) | Future releases |
| [Milestones](#milestones) | Detailed task breakdowns |
| [Dependency Graph](#dependency-graph) | Task dependencies |
| [Risk Register](#risk-register) | Known risks and mitigations |

---

## Executive Summary

The Arboretum project is a general-purpose static analysis system for multi-language codebases. This roadmap covers the development from current state (C++ extraction in progress) through V1 completion and beyond.

### Current State

- ✅ C++ Clang plugin extraction to Postgres (in progress)
- ⏳ Rust extraction planned for MIR/HIR/LLVM IR
- ⏳ Build system integration deferred - using wrapper scripts
- ✅ All analyses use SQL CTEs (no Souffle for V1)

### V1 Focus

V1 targets a working extraction and unification pipeline for a single package build, with full program analysis capabilities. The goal is to produce a correct unified graph with ownership and alias annotations, queryable by downstream consumers via PostgreSQL.

### Success Definition

V1 is successful when:
1. The engine can extract a single package build at scale
2. Cross-TU unification creates a unified graph
3. All V1 analyses run successfully
4. Results are queryable via PostgreSQL
5. First consumers (AI/ML pipelines) can sample the data

---

## V1 Success Criteria

### 1. Feature Completeness

| Feature | Status | Validation |
|---------|--------|------------|
| C++ Clang plugin extraction | Complete | Extracts all AST nodes |
| LLVM IR extraction | Complete | PreOpt, PostInline, PostOpt stages |
| Normalized graph tables | Complete | cpg_node, cpg_edge populated |
| Language-specific tables | Complete | cpp_* tables populated |
| Cross-TU unification | Complete | Structural hash + verification |
| Declaration/definition linking | Complete | Via mangled name and symbol_provider |
| Forward declaration unification | Complete | Clients see unified view |
| Def-use chain analysis | Complete | Recursive CTE |
| Flow-sensitive alias analysis | Complete | Points-to sets |
| Control Dependence Graph | Complete | CDG edges |
| Dominator tree | Complete | Dominance info |
| Lifetime extent computation | Complete | Via dominator tree |
| Call graph construction | Complete | Interprocedural edges |
| C++ ownership inference | Complete | Language-specific plugin |
| Package version tracking | Complete | Separate from symbol versions |
| Symbol version tracking | Complete | GNU symbol versions |

### 2. Hero Use Cases

| Package | Reason | Validation |
|---------|--------|------------|
| OpenSSL | Complex, ownership/alias test | Successful extraction + analysis |
| SQLite | Simple, validation | Clean extraction |
| BusyBox | Embedded, distro-scale | Multiple TUs |
| GCC | Self-hosting, toolchain | Extraction toolchain |
| Node.js | Multi-TU, unification | Unification coverage |

### 3. Performance Targets

| Metric | Target | Notes |
|--------|--------|-------|
| Build time | < X minutes | Per package (TBD) |
| Storage | < X GB | Per package build (TBD) |
| Query latency | < X ms | Common queries (TBD) |
| Unification coverage | > X% | definitions unified (TBD) |

---

## V1 Timeline

### April 2026

| Week | Milestone | Tasks |
|------|-----------|-------|
| 1-2 | C++ Extraction Complete | Finalize RecursiveASTVisitor<br>Complete LLVM IR extraction<br>Test on sample projects |
| 3-4 | Basic Extraction Working | Extract a single package<br>Create first Postgres tables<br>Validation tests |

### May 2026

| Week | Milestone | Tasks |
|------|-----------|-------|
| 1-2 | Unification Prototype | Structural hash implementation<br>Candidate verification<br>Test on small codebase |
| 3-4 | Unification Working | Cross-TU unification<br>Declaration/definition linking<br>Forward declaration handling |

### June 2026

| Week | Milestone | Tasks |
|------|-----------|-------|
| 1-2 | All Analyses Working | Def-use chains<br>Alias analysis<br>CDG<br>Dominator tree |
| 3-4 | Analysis Complete | Lifetime extents<br>Call graph<br>Ownership inference<br>Integration tests |

### July 2026

| Week | Milestone | Tasks |
|------|-----------|-------|
| 1-2 | Docker Distribution | RHEL Dockerfile<br>Debian Dockerfile<br>Ubuntu Dockerfile |
| 3-4 | Docker Validation | Test on hero use cases<br>Performance validation<br>Documentation |

### August 2026

| Week | Milestone | Tasks |
|------|-----------|-------|
| 1-2 | Hero Use Case Validation | OpenSSL extraction<br>SQLite extraction<br>BusyBox extraction |
| 3-4 | Hero Use Cases Complete | GCC extraction<br>Node.js extraction<br>Full validation suite |

### September 2026

| Week | Milestone | Tasks |
|------|-----------|-------|
| 1-2 | Documentation | User guide<br>Developer guide<br>API reference |
| 3-4 | External Tryout | First external users<br>Feedback collection<br>Final V1 release |

---

## V2 Timeline

### October 2026

| Week | Milestone | Tasks |
|------|-----------|-------|
| 1-2 | Rust Extractor Design | HIR extraction<br>MIR extraction<br>LLVM IR extraction |
| 3-4 | Rust Extractor Implementation | Implement extraction<br>Test on Rust projects<br>Validation |

### November 2026

| Week | Milestone | Tasks |
|------|-----------|-------|
| 1-2 | Package Registry | Package metadata schema<br>Library metadata schema<br>Package dependency tracking |
| 3-4 | Package Registry Working | Populate from RPM/DEB<br>Query integration<br>Validation |

### December 2026

| Week | Milestone | Tasks |
|------|-----------|-------|
| 1-2 | RHEL/Fedora Integration | redhat-rpm-config<br>Koji build system<br>Package coverage |
| 3-4 | Debian/Ubuntu Integration | CC/CXX environment<br>dpkg-buildpackage<br>Package coverage |

### January 2027

| Week | Milestone | Tasks |
|------|-----------|-------|
| 1-2 | Context-Sensitive Analysis | V2 alias analysis design<br>Implementation<br>Performance evaluation |
| 3-4 | Context-Sensitive Working | All V2 analyses<br>Validation<br>Difference from V1 |

### February 2027

| Week | Milestone | Tasks |
|------|-----------|-------|
| 1-2 | Template Merging | Template instantiation merging<br>Parametric comparison<br>Validation |
| 3-4 | Template Merging Complete | All template cases<br>Performance<br>Differentiation from V1 |

### March 2027

| Week | Milestone | Tasks |
|------|-----------|-------|
| 1-2 | Additional Languages | Python extraction<br>JavaScript extraction<br>Go extraction |
| 3-4 | Additional Languages Complete | Java extraction<br>All language validation<br>Distribution |

---

## V3+ Timeline

### Q2-Q3 2027

| Release | Timeline | Features |
|---------|----------|----------|
| V3 | Q2-Q3 2027 | Global catalog infrastructure<br>Query API abstraction<br>Enterprise features |

### Q4 2027+

| Release | Timeline | Features |
|---------|----------|----------|
| V4 | Q4 2027+ | Advanced analyses<br>AI/ML integrations<br>Enterprise tooling |

---

## Milestones

### M1: C++ Extraction Complete (April 2026)

**Goal**: Complete the C++ Clang plugin extraction with LLVM IR support.

**Tasks**:
1. Finalize RecursiveASTVisitor implementation
2. Complete LLVM IR extraction at PreOpt, PostInline, PostOpt stages
3. Test on sample C++ projects
4. Validate against test suite
5. Performance profiling

**Deliverables**:
- Working C++ extractor plugin
- LLVM IR extraction working
- Test coverage for all AST node types

**Success Criteria**:
- Extracts 100% of AST nodes in test suite
- LLVM IR at all three stages
- No crashes on valid C++ code

---

### M2: Unification Working (May 2026)

**Goal**: Implement cross-TU unification.

**Tasks**:
1. Implement structural hash computation via Postgres CTE
2. Implement candidate verification
3. Create unified graph tables
4. Implement declaration/definition linking
5. Handle forward declarations

**Deliverables**:
- Working unification pipeline
- Unified graph with correct edges
- Test cases for various scenarios

**Success Criteria**:
- Unification coverage > 80% for test cases
- No duplicates in unified graph
- All edges correctly mapped

---

### M3: All Analyses Working (June 2026)

**Goal**: Implement all V1 analyses.

**Tasks**:
1. Def-use chain analysis
2. Flow-sensitive alias analysis
3. Control Dependence Graph
4. Dominator tree
5. Lifetime extent computation
6. Call graph construction
7. C++ ownership inference

**Deliverables**:
- All analyses implemented
- Results stored in Postgres
- Integration tests for each analysis

**Success Criteria**:
- All analyses complete on test cases
- Results are queryable
- Performance acceptable

---

### M4: Docker Distribution (July 2026)

**Goal**: Create Docker images for all target distributions.

**Tasks**:
1. Create RHEL Dockerfile
2. Create Debian Dockerfile
3. Create Ubuntu Dockerfile
4. Test images on hero use cases
5. Performance validation

**Deliverables**:
- RHEL Docker image
- Debian Docker image
- Ubuntu Docker image
- Documentation

**Success Criteria**:
- All images build successfully
- Extract on hero use cases works
- Performance acceptable

---

### M5: Hero Use Cases Complete (August 2026)

**Goal**: Validate V1 on real-world packages.

**Tasks**:
1. Extract OpenSSL
2. Extract SQLite
3. Extract BusyBox
4. Extract GCC
5. Extract Node.js

**Deliverables**:
- All hero use cases extracted
- Analysis results available
- Validation suite

**Success Criteria**:
- All hero use cases extract successfully
- Results are queryable
- Performance acceptable

---

### M6: Documentation Complete (September 2026)

**Goal**: Create comprehensive documentation.

**Tasks**:
1. User guide
2. Developer guide
3. API reference
4. Tutorial
5. FAQ

**Deliverables**:
- User guide
- Developer guide
- API reference
- Tutorial
- FAQ

**Success Criteria**:
- Documentation complete
- Examples work
- Questions answered

---

### M7: External Tryout (September 2026)

**Goal**: Get external users to try V1.

**Tasks**:
1. Identify external users
2. Provide access to V1
3. Collect feedback
4. Fix issues
5. Release V1

**Deliverables**:
- External user feedback
- Issues resolved
- V1 release

**Success Criteria**:
- External users engaged
- Feedback collected
- Issues resolved

---

## Dependency Graph

```
M1 (C++ Extraction)
    ↓
M2 (Unification)
    ↓
M3 (All Analyses)
    ↓
M4 (Docker Distribution)
    ↓
M5 (Hero Use Cases)
    ↓
M6 (Documentation)
    ↓
M7 (External Tryout)
    ↓
V1 Release

M7 (External Tryout)
    ↓
V2: Rust Extractor (October 2026)
    ↓
V2: Package Registry (November 2026)
    ↓
V2: Build Integration (December 2026)
    ↓
V2: Context-Sensitive Analysis (January 2027)
    ↓
V2: Template Merging (February 2027)
    ↓
V2: Additional Languages (March 2027)
    ↓
V3: Global Catalog (Q2-Q3 2027)
    ↓
V4: Advanced Features (Q4 2027+)
```

---

## Risk Register

### High Priority Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Unification performance | Medium | High | Use sampling for validation<br>Implement incremental processing<br>Profile and optimize |
| Analysis performance | Medium | High | Batch processing<br>Optimize CTEs<br>Use indexes strategically |
| Docker build complexity | Low | Medium | Start with minimal image<br>Add features incrementally<br>Use multi-stage builds |
| Hero use case failures | Low | High | Test early<br>Have fallback use cases<br>Debug tooling |

### Medium Priority Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Rust extractor complexity | Medium | Medium | Start simple (MIR only)<br>Add HIR/IR incrementally<br>Use existing libraries |
| Package registry complexity | Medium | Medium | Start with package metadata<br>Add library tracking incrementally<br>Use existing schemas |

### Low Priority Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Additional language complexity | Low | Medium | Prioritize based on demand<br>Use existing parsers<br>Gradual adoption |

---

## Appendix

### A. Success Metrics Definition

**Build Time**: Time from starting package build to extraction complete
- Measured from build start to Postgres write complete
- Includes compilation time

**Storage**: Total storage used per package build
- Includes all tables (normalized, language-specific, analysis)
- Includes indexes

**Query Latency**: Time to execute common queries
- Single TU retrieval
- Symbol lookup
- Analysis result retrieval
- Cross-TU queries

**Unification Coverage**: Percentage of definitions unified
- Count of unified definitions / total definitions
- Per-file and per-package metrics

### B. Validation Test Suite

| Test Case | Purpose | Pass Criteria |
|-----------|---------|---------------|
| simple_function | Basic extraction | All nodes extracted |
| inheritance | C++ features | All node types present |
| templates | Template extraction | Instantiations unified |
| cross_tu | Unification | Definitions unified |
| forward_decl | Forward declarations | Unified with definitions |
| complex_ownership | Ownership analysis | Correct ownership inferred |
| alias_analysis | Alias analysis | Correct points-to sets |

### C. Distribution Channels

| Channel | Platform | Use Case |
|---------|----------|----------|
| Docker Hub | Linux/macOS | Quick start, development |
| GitHub Releases | All | Production deployment |
| Package Managers | Linux | OS integration |

---

*This roadmap is maintained as part of the Arboretum project documentation.*
