# Arboretum
## Architecture & Design Document
*Updated — March 9, 2026*

---

## 1. Overview

The Arboretum is a general-purpose static analysis system for multi-language codebases. It extracts a semantic graph from source code during compilation, unifies shared definitions across translation units, and enriches the graph with derived program properties including ownership structure, aliasing, lifetimes, control flow, and control dependence.

The engine is designed as a standalone, reusable infrastructure component with no dependency on any downstream consumer. Translation tools, security analyzers, documentation generators, refactoring tools, and any other system that needs deep semantic understanding of source code can consume its output via a language-agnostic query API. The C++ to Rust translation tool is one such consumer.

The engine launches with C++ support and Rust as the second language. The schema and analysis plugin architecture are designed from the outset to support arbitrary additional languages without schema migration.

The engine is fully open source. A hosted global catalog provides shared storage and deduplication of analysis results across users and codebases, with a freemium model covering open source packages.

### 1.1 Current Status & V1 Focus

**Current Status (March 2026):**
- C++ Clang plugin extraction to Postgres is in progress
- Rust extraction planned for MIR/HIR/LLVM IR
- Build system integration deferred - using wrapper scripts
- All analyses use SQL CTEs (no Souffle for V1)

**V1 Scope:**
- Single package build at scale
- C/C++ extraction with LLVM IR at PreOpt/PostInline/PostOpt stages
- Cross-TU unification via structural hash + verification
- All analyses: def-use chains, alias analysis, CDG, dominator tree, lifetime extents, call graph, ownership inference
- First consumers: AI/ML data pipelines that sample from the dataset

**Success Criteria:**
- Extract and analyze the most commonly installed applications, tools, and libraries together
- Output available for downstream consumption
- Data computed rapidly enough for AI model feature engineering

---

## 2. Design Principles

- **Consumer agnostic.** The engine computes properties of programs. What consumers do with those properties is not the engine's concern. No translation-specific concepts appear in the engine's data model or APIs.
- **Language agnostic core.** The normalized graph schema and analysis plugin interface are language-independent. Language-specific extractors and analyses sit alongside the agnostic core without polluting it.
- **Language-specific depth.** Language-specific tables coexist with the normalized core, enabling analyses that require language-specific attributes without forcing all consumers through a lowest-common-denominator representation.
- **Append-only.** TU subgraphs are immutable once written. Recompilation produces a new subgraph rather than mutating an existing one. This simplifies consistency, enables versioning, and supports the content-addressed catalog.
- **Build-system agnostic.** Extraction happens inside the compiler via plugins injected through wrapper scripts. Any build system that invokes a supported compiler contributes to the graph automatically.
- **Distro-scale.** The engine is designed to analyze entire Linux distributions. Package and library metadata are first-class concepts, not afterthoughts.
- **Fixpoint analysis via SQL.** Program analyses are expressed as Postgres recursive CTEs, providing Datalog fixpoint semantics without an additional runtime dependency.

---

## 3. System Components

| Component | Status | Notes |
|---|---|---|
| Clang Plugin | In Progress | RecursiveASTVisitor + LLVM IR extraction writing directly to Postgres |
| Rustc Plugin | Planned | HIR + MIR + LLVM IR extraction for Rust TUs |
| Compiler/Linker Wrappers | Planned | Rust binaries injecting plugin flags and capturing link artifacts |
| Graph Store | In Progress | Postgres: normalized cpg_node/cpg_edge + language-specific tables |
| Build Artifact Layer | Planned | BuildArtifact, symbol_provider, runtime_resolution from linker wrapper + postlink binary |
| Graph Unification | Planned | Two-phase: structural hash via recursive CTE, then candidate verification |
| Program Analysis | Planned | SQL CTE-based analyses (no Souffle for V1) |
| Package Registry | Planned | Package, library, and distro metadata layer |
| Build Integration | Planned | RHEL/Fedora (primary), Debian/Ubuntu (secondary) via compiler wrappers |
| Global Catalog | Deferred | PostgreSQL for now; parquet for offline storage later |
| Query API | Planned | Using PostgreSQL directly - versioned by upstream |

---

## 4. Data Model

### 4.1 Subgraph

Every node in the graph has a compound ID: a 32-bit `subgraph_id` and a 32-bit `object_id`. Subgraph IDs are assigned by a Postgres auto-increment sequence — no hashing, no collision risk. The content hash of the TU is stored separately for deduplication and catalog lookup but is not used as identity. Subgraph IDs may be recycled after a subgraph is deleted.

```
subgraph
    subgraph_id  : uint32       -- identity, auto-incremented
    content_hash : bytes        -- SHA-256 of TU source; for dedup and catalog
    file_path    : text
    language     : text         -- 'cpp', 'rust', 'python', etc.
    build_id     : uuid
    created_at   : timestamp
    kind         : Enum[TU, Unified]
```

### 4.2 Normalized Graph Schema

The core graph schema is language-agnostic. All language extractors write to these tables, enabling cross-language analysis and a uniform query interface for consumers. The normalized tables are the substrate for language-agnostic analyses.

```
cpg_node
    node_id      : uint32
    subgraph_id  : uint32
    node_kind    : text         -- 'function', 'variable', 'call', etc.
    language     : text         -- denormalized for query convenience
    visit_order  : uint32       -- canonical child ordering within subgraph
    source_file  : text
    source_line  : int
    source_col   : int
    name         : text         -- nullable; common attribute
    type_str     : text         -- nullable; surface type as text
    visibility   : text         -- nullable; public/private/etc.
    is_definition: bool
    attrs        : jsonb        -- language-specific overflow attributes

cpg_edge
    subgraph_id  : uint32
    parent_id    : uint32
    child_id     : uint32
    edge_kind    : Enum[AST, CFG, DefUse, Call, ControlDep]
    visit_order  : uint32

-- Note: Edges may be stored as JSON arrays in node attrs for performance
-- This is optimized per-access-pattern rather than a global decision
```

### 4.3 Language-Specific Tables

Language-specific tables coexist with the normalized schema, linked by `node_id`. They carry the full richness of the language's AST without forcing it through the normalized representation. Language-specific analyses consume these tables directly.

```
-- C++ specific (prefixed cpp_)
cpp_function_decl(subgraph_id, node_id, mangled_name, return_type,
                  is_virtual, is_const, is_template, visit_order, ...)
cpp_var_decl     (subgraph_id, node_id, type, storage_class, ...)
cpp_call_expr    (subgraph_id, node_id, ...)
-- ... one table per Clang node type

-- Rust specific (prefixed rust_)
rust_fn_def      (subgraph_id, node_id, mangled_name, return_type,
                  is_unsafe, is_async, lifetimes, ...)
rust_let_stmt    (subgraph_id, node_id, pattern, type, is_mut, ...)
-- ... one table per MIR/HIR node type

-- Language-specific edge tables
cpp_ast_edge     (parent_subgraph_id, parent_id, child_subgraph_id, child_id, visit_order)
cpp_cfg_edge     (src_subgraph_id, src_id, dst_subgraph_id, dst_id, edge_kind)
rust_mir_edge    (src_subgraph_id, src_id, dst_subgraph_id, dst_id, edge_kind)
```

### 4.4 LLVM IR Layer

The Clang plugin captures LLVM IR at multiple optimization stages alongside the AST. This provides a post-optimization view of what the compiler actually generates, making optimizer-exploited undefined behavior visible that is invisible at the AST level. Each stage is stored as a separate set of IR nodes linked to the AST via source location.

```
ir_function    (subgraph_id, node_id, llvm_name, opt_stage, ...)
ir_block       (subgraph_id, node_id, function_id, opt_stage, ...)
ir_instruction (subgraph_id, node_id, block_id, opcode, opt_stage, ...)

-- opt_stage: Enum[PreOpt, PostInline, PostOpt]

ir_ast_mapping
    subgraph_id  : uint32
    ir_node_id   : uint32
    ast_node_id  : uint32       -- nullable; unmapped IR has no source loc
    source_line  : int
    source_col   : int
    opt_stage    : text
```

Unmapped IR nodes — those without a source location — are typically compiler-generated: implicit constructors, inlined functions that lost debug info, optimizer-introduced temporaries. At distro scale the pattern of what gets unmapped is informative signal about which code is most compiler-transformed.

### 4.5 Build Artifact Layer

The build artifact layer captures what the linker produces and how symbols resolve at runtime. This is essential for cross-package analysis and for correctly modeling dynamic linking behavior, where the implementation of a symbol depends on the runtime environment rather than being determinable from source alone.

```
build_artifact
    artifact_id     : uint32
    kind            : Enum[Executable, SharedLib, StaticLib, Object]
    path            : text
    package_id      : uint32
    build_id        : uuid
    linker          : text      -- e.g. 'lld-17.0.6'
    linker_flags    : text
    capture_status  : Enum[Complete, LinkFailed(reason),
                          PluginFailed(reason), PostlinkFailed(reason)]

artifact_tu
    artifact_id     : uint32
    subgraph_id     : uint32

symbol_provider
    artifact_id     : uint32
    mangled_name    : text
    symbol_version  : text      -- GNU symbol versioning e.g. GLIBC_2.17
    subgraph_id     : uint32
    node_id         : uint32
    binding         : Enum[Strong, Weak, Global, Local]

runtime_resolution
    consumer_id     : uint32    -- artifact referencing the symbol
    mangled_name    : text
    symbol_version  : text
    provider_id     : uint32    -- artifact providing it at runtime
    resolution_kind : Enum[Static, Dynamic, Weak, VersionedDynamic]
```

### 4.6 Package Registry

Package and library metadata provide organizational context above the subgraph level, enabling distro-scale queries.

```
package
    package_id          : uint32
    name                : text
    version             : text
    distro              : text
    source_url          : text
    content_hash        : bytes

library
    library_id          : uint32
    package_id          : uint32
    name                : text
    kind                : Enum[Static, Shared, HeaderOnly]

tu_library
    subgraph_id         : uint32
    library_id          : uint32

package_dependency
    dependent_id        : uint32
    dependency_id       : uint32
    version_constraint  : text
```

### 4.7 Unified Graph

Unified entities receive new `subgraph_id`s from the same sequence, distinguished by `kind = Unified`. A provenance table records the mapping back to source TU entities.

```
unification_map
    unified_subgraph_id : uint32
    unified_object_id   : uint32
    source_subgraph_id  : uint32
    source_object_id    : uint32
```

---

## 5. Language Extractors

Each supported language has a dedicated extractor that writes to both the normalized `cpg_node`/`cpg_edge` tables and the language-specific tables. All extractors share the same subgraph ID scheme and Postgres connection model.

### 5.1 C++ Extractor (Clang Plugin)

The C++ extractor is a Clang plugin using a `RecursiveASTVisitor`. It runs inside the compiler process for each TU, emitting AST nodes in `visit_order` alongside their language-specific `cpp_` table entries. It also triggers LLVM IR generation at three optimization stages (PreOpt, PostInline, PostOpt) and captures IR nodes and their source location mappings.

For each TU the plugin: assigns a new `subgraph_id`; walks the AST emitting `cpg_node` rows and `cpp_*` rows with a monotonically increasing `visit_order`; emits AST and CFG edges to both `cpg_edge` and `cpp_ast_edge`/`cpp_cfg_edge`; runs the LLVM pass pipeline at each stage capturing IR; writes all rows in a single transaction; commits.

If the build fails or the plugin crashes, the transaction rolls back and no partial state is written. On the next build the compiler reprocesses the TU and the plugin retries cleanly. Incremental extraction is provided by the build system — only recompiled TUs produce new subgraphs.

### 5.2 Rust Extractor (Rustc Plugin)

The Rust extractor runs as a rustc compiler plugin, operating on the MIR (Mid-level Intermediate Representation), which is Rust's primary analysis-friendly IR. MIR is already lowered from the source HIR and has explicit ownership and borrow information encoded, making it a richer extraction target than the raw source AST for ownership-related analyses.

The Rust extractor writes to `cpg_node` with `node_kind` values drawn from a Rust-specific vocabulary and to `rust_*` language-specific tables. Because Rust's ownership and lifetime information is explicit in MIR, the analysis engine can extract it directly rather than inferring it, which produces higher-confidence results than the equivalent inference from C++.

### 5.3 Build System Integration

Extractors are injected via compiler/linker wrapper scripts placed in the global compiler installation. No per-project or per-build-system changes are required:

- **CMake** — `CMAKE_C_COMPILER` / `CMAKE_CXX_COMPILER` pointing to wrapper
- **Bazel** — custom toolchain with wrapped compiler
- **Make / Ninja** — `CC` / `CXX` environment variables
- **Meson** — native file compiler override
- **RPM (RHEL/Fedora)** — `%{__cc}` / `%{__cxx}` macros in `redhat-rpm-config`; single injection point covers all packages
- **DEB (Debian/Ubuntu)** — `CC` / `CXX` via `dpkg-buildpackage`; standard packages covered

### 5.4 Parallel Builds

Each extractor invocation is a separate compiler process with its own Postgres connection and transaction. There is no row-level contention between TUs since each writes to its own `subgraph_id` partition. Parallel builds up to `-j 100` are handled by Postgres natively. PgBouncer can be added transparently if larger parallelism is needed.

---

## 6. Linker Wrapper & Build Artifacts

Source-level analysis alone cannot determine how symbols resolve at runtime. The h/cc split means declarations and definitions have different AST shapes and must be linked by symbol identity rather than structural equality. Dynamic linking means the correct implementation of a symbol depends on the runtime environment — `LD_PRELOAD`, `RPATH`/`RUNPATH`, symbol versioning — not just the source graph. The linker wrapper captures this information.

### 6.1 Wrapper Architecture

Three Rust binaries are injected alongside the compiler extractor:

```
our-cc-wrapper  (Rust binary)
    -- inject extractor plugin flags
    -- record build_id
    -- exec real compiler

our-cxx-wrapper (Rust binary)
    -- same as cc-wrapper for C++

our-ld-wrapper  (Rust binary)
    -- force LLD via -fuse-ld=lld
    -- inject --Map=<tmpfile> to capture linker map
    -- exec real linker
    -- on success: exec cpp-analysis-postlink binary
    -- on failure: write LinkFailed(reason) to build_artifact
```

**Build ID Linking**: Each wrapper binary uses a unique `build_id` key to link together all related TU extraction runs. This ensures atomicity - the entire build comes at once or it doesn't. Each individual TU extraction runs in a separate transaction to allow rollback on failure.

### 6.2 LLD as Standard Linker

LLD is the standard linker for this toolchain. It is the default for LLVM/Clang, significantly faster than BFD at distro scale, and has a stable, well-documented map file format. The linker wrapper forces LLD via `-fuse-ld=lld` for all link steps.

Packages that fail due to LLD incompatibility have their `build_artifact` record written with `capture_status = LinkFailed` and a descriptive reason. They are not silently skipped — the failure is visible and queryable. At distro scale the set of failing packages is itself informative about the scope of LLD incompatibility.

The `linker` column on `build_artifact` is the extensibility point for future linker support. When BFD or gold support is added, the schema already has a place for it and artifacts can be queried by which linker produced them.

### 6.3 Postlink Binary

The postlink analysis is implemented as a standalone Rust binary (`cpp-analysis-postlink`) invoked by the linker wrapper after a successful link. Shell is insufficient for this step due to the complexity of ELF parsing and GNU symbol versioning. The binary uses the `goblin` library for ELF analysis and writes directly to Postgres under the same `build_id` as the compilation steps.

The postlink binary performs:

- Parse LLD map file → `symbol_provider` entries (which TU provides each symbol)
- Parse ELF via goblin → RPATH, RUNPATH, symbol versioning, binding (strong/weak/global/local)
- Run `ldd` against the artifact in the build environment → `runtime_resolution` entries
- Write `build_artifact` record with `capture_status = Complete`

### 6.4 Declaration / Definition Linking

The h/cc split produces declarations (`FunctionDecl` with no body) and definitions (`FunctionDecl` with `CompoundStmt`) as structurally different AST nodes. They cannot be unified by structural hash. Instead they are linked by mangled name after unification, using the `symbol_provider` table as the authority on which TU's definition is canonical for each symbol. This linking step runs as part of the analysis enrichment pass after unification.

---

## 7. Graph Store

### 7.1 Append-Only Design

The graph store is append-only. TU subgraphs are never mutated. When a file is recompiled a new subgraph is written with a new `subgraph_id`. The old subgraph remains until explicitly garbage collected. Unification identifies shared structure between old and new versions; once a new subgraph is confirmed equivalent the old one can be pruned.

This design provides implicit versioning — the full history of every TU is preserved — and aligns with the content-addressed catalog, where every unique content hash is a permanent entry.

### 7.2 Garbage Collection

Garbage collection policy is deferred. Options include eager pruning of superseded subgraphs on confirmation of a new version, retention of a configurable number of historical versions, and indefinite retention for catalog entries. The append-only design means GC can be added later without changing the core schema.

---

## 8. Cross-TU Unification

Header definitions appear redundantly across TUs. Unification identifies structurally identical definitions and merges them into a single unified entity, enabling interprocedural analysis across the full codebase without N² comparisons.

### 8.1 Algorithm

Unification proceeds in two phases:

- **Phase 1 — Structural hashing:** compute a bottom-up hash for every AST node across all subgraphs. Build a global table of `hash -> [(subgraph_id, object_id)]`. Hash buckets with a single entry are trivially unique. Only buckets with multiple entries proceed to phase 2.
- **Phase 2 — Candidate verification:** for each multi-entry hash bucket, verify structural equality by direct AST comparison. Confirmed-equal nodes are merged into the unified graph. Hash collisions that differ structurally remain separate.

### 8.2 Structural Hash

The hash is computed bottom-up via a Postgres recursive CTE. Leaf nodes are hashed from their node type and scalar fields. Interior nodes are hashed from their node type, fields, and an ordered array of child hashes. Child ordering uses `visit_order`, which is stable and canonical across TUs because it reflects the `RecursiveASTVisitor`'s source-text traversal order.

The `HAVING` clause ensures a node is hashed only once all of its children have been hashed, providing Datalog fixpoint semantics:

```sql
WITH RECURSIVE structural_hash AS (
  -- base case: leaf nodes (no children)
  SELECT subgraph_id, object_id,
         hash_leaf(node_type, fields) AS hash
  FROM ast_nodes
  WHERE object_id NOT IN (SELECT parent_id FROM ast_edges)

  UNION ALL

  -- inductive: interior nodes whose children are all hashed
  SELECT n.subgraph_id, n.object_id,
         hash_interior(n.node_type, n.fields,
             array_agg(h.hash ORDER BY e.visit_order))
  FROM ast_nodes n
  JOIN ast_edges e  ON e.parent_id = n.object_id
                   AND e.parent_subgraph_id = n.subgraph_id
  JOIN structural_hash h ON h.object_id = e.child_id
                        AND h.subgraph_id = e.child_subgraph_id
  GROUP BY n.subgraph_id, n.object_id, n.node_type, n.fields
  HAVING COUNT(*) = (
    SELECT COUNT(*) FROM ast_edges e2
    WHERE e2.parent_id = n.object_id
    AND e2.parent_subgraph_id = n.subgraph_id)
)
SELECT * FROM structural_hash;
```

### 8.3 Divergent Definitions

Definitions that differ across TUs despite appearing in the same header are kept separate and flagged as divergent. Divergence indicates a potential ODR violation, platform-specific conditional compilation, or macro-dependent behavior. Divergent definitions are surfaced to consumers as a first-class signal rather than silently picking one version.

### 8.4 CFG Exclusion

The hash computation and unification algorithm traverse AST edges only. CFG edges are excluded because the CFG may contain back edges (loop back-edges) which would cause the bottom-up hash computation to fail to terminate. The CFG is a derived view of the AST; if two AST subtrees are identical their CFGs are implicitly identical.

---

## 9. Program Analysis

The unified graph is enriched with derived analysis results stored as additional annotation and edge tables. Analyses are implemented as plugins with a declared language scope and dependency ordering. Language-agnostic analyses run first; language-specific analyses consume their results.

All analyses are expressed as Postgres recursive CTEs providing Datalog fixpoint semantics. If analyses grow complex enough to warrant it, Souffle can be used as a Datalog-to-SQL compiler, emitting Postgres-compatible SQL while allowing analyses to be authored in a more readable Datalog dialect.

### 9.1 Analysis Plugin Interface

```
AnalysisPlugin
    language  : Option[text]    -- None = language agnostic
    produces  : List[text]      -- analysis_kinds this plugin produces
    consumes  : List[text]      -- analysis_kinds this plugin requires
    run(subgraph_id) -> results

-- Results land in normalized or language-specific tables
cpg_analysis_result
    node_id       : uint32
    subgraph_id   : uint32
    analysis_kind : text
    result        : jsonb

-- Language-specific result tables e.g.
cpp_ownership_result  (node_id, subgraph_id, ownership, confidence)
rust_lifetime_result  (node_id, subgraph_id, lifetime)
```

The engine schedules plugins topologically based on their `consumes`/`produces` declarations. Language-agnostic plugins run first and produce results available to all language-specific plugins.

### 9.2 Language-Agnostic Analyses

- **Def-use chains** — for each definition, the set of uses reachable along the CFG. Foundation for ownership inference and dead code detection.
- **Dominance tree** — dominator tree of the CFG, used for lifetime extent computation, loop detection, and ownership heuristics.
- **Control Dependence Graph (CDG)** — captures which conditions control whether a statement executes. Dual of data dependence. Essential for tracking how attacker-controlled data influences control flow and for understanding conditional ownership patterns.
- **Call graph** — interprocedural edges built after unification using unified symbol identities. Calls to library functions resolve correctly regardless of which TU's copy of the header was used.
- **Lifetime extents** — for each pointer or reference, the minimal scope in which it must remain valid, derived from def-use chains and the dominator tree.

### 9.3 Alias Analysis

Alias analysis has two tiers with different precision/cost tradeoffs:

- **Flow-sensitive intraprocedural (V1)** — computes points-to sets within a function, tracking how pointer values flow through assignments. Pointer escape across function boundaries is flagged for consumer attention. Fast and sufficient for most ownership inference cases.
- **Context-sensitive interprocedural (V2)** — distinguishes different call sites to the same function, enabling precise alias information across function boundaries. Significantly more expensive but necessary for correct ownership inference in complex aliasing patterns. Context sensitivity is the key precision gap identified relative to state-of-the-art tools such as MATE.

### 9.4 Language-Specific Analyses

- **C++ ownership inference** — reconstructs ownership structure from def-use, alias, and lifetime results. Produces `cpp_ownership_result` annotations consumed by the translation tool.
- **C++ UB detection** — identifies patterns of undefined behavior using both AST-level analysis and the LLVM IR layer where optimizer-exploited UB is visible.
- **Rust ownership extraction** — extracts explicit ownership and lifetime information directly from MIR. Higher confidence than C++ inference since the information is stated rather than inferred.

---

## 10. Distro Integration

The engine is designed to analyze entire Linux distributions, treating the package ecosystem as the unit of analysis rather than individual projects. This enables cross-package analysis: tracing a vulnerability through all packages that depend on an affected library, understanding the full semantic impact of an API change, and providing distro-wide memory safety assessments.

### 10.1 Target Distributions

The primary targets are RHEL/Fedora (primary) and Debian/Ubuntu (secondary). RHEL was selected because the integration is easier via `%{__cc}` / `%{__cxx}` macros in `redhat-rpm-config`. Debian/Ubuntu is also supported via `CC` / `CXX` environment variables.

### 10.2 RHEL / Fedora Integration

RPM build infrastructure is well standardized. The injection point is `redhat-rpm-config`, which provides compiler flag macros used by virtually all packages. Setting `%{__cc}` / `%{__cxx}` to the wrapper scripts covers the entire distribution build from a single change. The Koji build system provides the orchestration layer; each package build in Koji automatically contributes to the graph.

### 10.3 Debian / Ubuntu Integration

`CC` and `CXX` environment variables propagate through the standard `dpkg-buildpackage` infrastructure. Well-behaved packages are covered automatically. Packages with non-standard build systems or hardcoded compiler invocations require individual attention. Coverage will be high but not complete from a single injection point.

### 10.4 Cross-Package Analysis

Symbol references that cross library boundaries are first-class edges in the unified graph. A call from one package into a shared library is an interprocedural edge that crosses a package boundary, and can be analyzed for ownership and aliasing just as intrapackage edges are. This is the core capability that makes distro-scale analysis qualitatively different from per-project analysis.

### 10.5 Distribution Model

V1 uses Docker images as the primary distribution mechanism:

- **RHEL Dockerfile**: With analysis tooling pre-configured
- **Debian Dockerfile**: With analysis tooling pre-configured
- **Ubuntu Dockerfile**: With analysis tooling pre-configured

Users invoke the analysis tooling on packages specified within the Docker image.

---

## 11. Global Catalog

**Status**: Deferred to V2+

The global catalog is a shared service that stores subgraphs and analysis results contributed by all users. It enables deduplication across codebases: if two users have the same version of a library, their subgraphs for that library produce the same content hash and share a single catalog entry.

### 11.1 Content Addressing

The catalog is keyed by content hash. Before uploading a subgraph, the local service queries the catalog with the content hash. If the subgraph is already present, the local service receives the catalog's canonical `subgraph_id` and skips the upload. Cached analysis results travel with the subgraph — a user pulling a well-known library gets the analysis for free.

### 11.2 Deployment Model

```
LocalService
    graph_store    : Postgres     -- user's own codebase
    catalog_client               -- push/pull by content hash

GlobalCatalog
    subgraph_store               -- content-addressed by content_hash
    analysis_store               -- cached analysis results per subgraph
```

### 11.3 Business Model

The catalog has a freemium structure. The free tier covers all open source packages — analyzing the open source ecosystem is a strategic investment that makes the catalog valuable to paying users. The paid tier covers private codebases, higher retention of historical versions, SLAs on analysis freshness, and API access at scale for integration into CI pipelines and security tooling.

Enterprise partnerships with Red Hat and Canonical are a natural commercial extension: licensing the catalog infrastructure for a hosted instance scoped to their package ecosystem, with open source results flowing back to the public catalog.

### 11.5 V1 Approach

For V1, there is no publicly available global catalog. Some users may fund setup of a private catalog instance. Long-term, the plan is to use PostgreSQL as the backend, potentially with parquet exports for offline storage.

### 11.6 Privacy

Sharing policy is file-level, specified by the user as glob patterns in a `.gitignore`-style configuration. The first matching rule wins; the default is all-private. Content hashes already present in the catalog are implicitly public regardless of local policy — the content is already known, and the user is only revealing that they use it. Full sharing policy design is deferred; the local service operates fully offline until the user opts in.

---

## 12. Query API

**Status**: Using PostgreSQL directly for V1

The engine exposes a query API that downstream consumers use to retrieve subgraphs, analysis results, and derived properties. The API is consumer-agnostic — it provides access to the enriched graph without any knowledge of what consumers will do with it.

The exact API design is deferred until the graph store and analysis layers are stable. The interface should support: subgraph retrieval by content hash or `subgraph_id`; symbol lookup by name and package; reachability queries over the call graph; analysis result retrieval for specific nodes; and streaming updates when new analysis results are available for a subgraph.

### 12.1 V1 Approach

For V1, consumers use PostgreSQL directly as the query interface. This provides:
- Direct SQL access to all data
- Full Postgres features (CTEs, indexes, extensions like pgvector)
- No abstraction layer overhead
- Versioning tied to upstream (llvm/clang) schema versions

Later, an abstraction layer may be added for convenience, but V1 uses PostgreSQL directly.

---

## 13. Open Questions

| Topic | Notes |
|---|---|
| Context-sensitive alias analysis | Precision vs. cost tradeoff. V1 is flow-sensitive intraprocedural. V2 escalation criteria TBD. |
| Souffle threshold | At what analysis complexity to switch from hand-written CTEs to Souffle-generated SQL. |
| Garbage collection policy | Eager vs. versioned vs. indefinite retention. Local and catalog may differ. |
| Template instantiation merging | Structural hash handles identical instantiations; parametrically related ones need further design. |
| Catalog sharing policy | Full glob-pattern design deferred. Default is all-private. |
| Query API design | Deferred until graph store and analysis are stable. |
| Debian coverage gaps | Non-standard build systems require individual handling; scope TBD. |
| Rust extractor IR layer | Rustc MIR is already post-lowering; whether to also capture LLVM IR for Rust TBD. |
| Additional language priority | After C++ and Rust, which language is highest value? Python, Java, Go are candidates. |
| LLD incompatibility scope | What fraction of distro packages fail with LLD? Measure before deciding whether to add BFD support. |

---

## 14. V1 Scope

V1 targets a working extraction and unification pipeline for a single package build, with full program analysis capabilities. The goal is a system that produces a correct unified graph with ownership and alias annotations, queryable by downstream consumers.

### 14.1 In Scope for V1

- C++ Clang plugin extraction to Postgres with LLVM IR at all three stages (PreOpt, PostInline, PostOpt)
- Normalized `cpg_node`/`cpg_edge` tables populated alongside `cpp_*` tables
- AST, CFG, and IR edge extraction with `visit_order`
- Linker wrapper with LLD (Rust binaries) and postlink binary for build artifact capture
- Cross-TU unification via structural hash and candidate verification
- Declaration/definition linking via mangled name and `symbol_provider`
- Forward declarations unified with definitions for clients
- Def-use chain analysis
- Flow-sensitive intraprocedural alias analysis
- Control Dependence Graph (CDG)
- Lifetime extent computation
- Dominator tree
- Call graph construction
- C++ ownership inference (language-specific plugin)
- Package and symbol version tracking (separate tracking for package versions and GNU symbol versions)
- All analyses expressed as SQL CTEs (no Souffle for V1)

### 14.2 Versioning Strategy

The engine tracks multiple version dimensions:

1. **Package versions**: Standard package versioning (e.g., libc 2.31-0ubuntu9.2)
2. **Symbol versions**: GNU symbol versioning (e.g., GLIBC_2.17)
3. **Schema versions**: Versioned by upstream (llvm/clang); new version = new schema in separate tables

These are tracked separately because they are semantically different. Different versions will not unify - they are truly different.

### 14.3 Deferred to V2

- Context-sensitive interprocedural alias analysis
- Template instantiation merging beyond identical copies
- Additional language extractors (Python, JS, Go, Java)

### 14.4 Deferred for Future Releases

- Rust extractor (requires rustc plugin development)
- Package registry and distro metadata
- Distro build system integration (RPM, DEB)
- Global catalog infrastructure (PostgreSQL + parquet for offline)
- Souffle-based analysis authoring (if CTEs prove insufficient)
- Query API abstraction layer (using PostgreSQL directly for now)

---

## 15. V1 Success Criteria

### 15.1 Success Definition

V1 is considered successful when the engine can:

1. **Extract**: Process a single package build at scale (multiple TUs)
2. **Unify**: Create a unified graph across translation units
3. **Analyze**: Run all V1 analyses (def-use, alias, CDG, dominator tree, lifetime, call graph, ownership)
4. **Output**: Produce queryable results via PostgreSQL
5. **Enable**: First consumers (AI/ML data pipelines) can sample from the dataset

### 15.2 Hero Use Cases

Target applications, tools, and libraries for V1 validation:

| Priority | Package | Reason |
|----------|---------|--------|
| 1 | OpenSSL | Widely used, complex, good test for ownership/alias analysis |
| 2 | SQLite | Simple but complete, good for validation |
| 3 | BusyBox | Embedded systems, good for distro-scale validation |
| 4 | GCC | Self-hosting, tests the extraction toolchain |
| 5 | Node.js | Complex, multi-TU, good for unification testing |

### 15.3 Success Metrics

- **Build time**: < X minutes for a typical package (TBD)
- **Storage**: < X GB per package build (TBD)
- **Query latency**: < X ms for common queries (TBD)
- **Unification coverage**: > X% of definitions unified across TUs (TBD)

### 15.4 First Consumer Integration

The first consumer will be an AI/ML data pipeline that:
1. Pulls data from the database
2. Samples features for model training
3. Iterates rapidly on feature engineering

Success is measured by the pipeline's ability to:
- Extract features in minutes rather than days/weeks
- Iterate on new feature ideas quickly
- Access comprehensive semantic data

---

## 16. Roadmap

### 16.1 V1 Timeline (Q2-Q3 2026)

| Month | Milestone |
|-------|-----------|
| April | Complete C++ extraction with LLVM IR |
| May | Basic unification working |
| June | All analyses working (def-use, alias, CDG, etc.) |
| July | Docker distribution ready (RHEL, Debian, Ubuntu) |
| August | Hero use case validation (OpenSSL, SQLite, etc.) |
| September | Documentation and first external试用 |

### 16.2 V2 Timeline (Q4 2026 - Q1 2027)

| Month | Milestone |
|-------|-----------|
| October | Rust extractor (HIR + MIR + LLVM IR) |
| November | Package registry and distro metadata |
| December | Build system integration (RHEL/Fedora, Debian/Ubuntu) |
| January | Context-sensitive alias analysis (V2) |
| February | Template instantiation merging |
| March | Additional language extractors (Python, JS, Go, Java) |

### 16.3 Future Releases

| Release | Timeline | Features |
|---------|----------|----------|
| V3 | Q2-Q3 2027 | Global catalog infrastructure, query API abstraction |
| V4 | Q4 2027+ | Advanced analyses, AI/ML integrations, enterprise features |

### 16.4 Success Path

The roadmap is designed to:
1. **V1**: Prove the core value (data availability for downstream consumption)
2. **V2**: Expand language coverage (Rust + additional languages)
3. **V3**: Build infrastructure (global catalog, enterprise features)
4. **V4+**: Enable advanced use cases (AI/ML, enterprise tooling)

Each release builds on the previous, with V1 being the critical proving ground.