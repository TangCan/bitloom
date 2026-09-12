# FR118 — Full-tree `#[bitloom::top]` syn-scan (no metadata)

**Product:** Bitloom (`bitloom-lsp`). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 59 / FR118 closed** (Story **59.3**). Product path delivered in Story **59.2**.
Phase 12 FR99 DesignFixture MVP and Phase 13 FR113 Cargo-graph+metadata remain closed (NFR48).

This page is the **FR118 completion surface**. It deepens design-root discovery **beyond**
FR99 DesignFixture-only and FR113 metadata `design_roots`.

## Selected discovery strategy (NFR14)

| Strategy | Status |
|----------|--------|
| **Workspace `#[bitloom::top]` syn-scan** (no metadata) | **FR118 face — closed** (this page) |
| Cargo-graph + `[package.metadata.bitloom] design_roots` | FR113 — **still closed**; coexistence |
| FR99 `DesignFixture` enum | Regression only — **alone ≠ FR118** |

### Syn-scan contract

1. Walk Cargo workspace `members` (or a single-package root).
2. Consider only packages that depend on `bitloom-prelude`.
3. If `[package.metadata.bitloom] design_roots` is present → **FR113 metadata path** (unchanged).
4. If **no** metadata roots → scan `src/**/*.rs` for `#[bitloom::top]` or `#[rhdl::top]`;
   the annotated `struct` / `enum` / `type` name is the `root_id`.
5. Elaborate via `bitloom-lsp` documented registry (文档等价). Unknown id → readable fail;
   shallow mode must **not** `finish()`.

## Product API

```rust
use bitloom_lsp::{
    AnalysisMode, MVP_INTERACTIVE_BUDGET, analyze_discovered_root,
    analyze_workspace_design_roots, discover_design_roots,
};

// No metadata package with #[bitloom::top] in src/:
let roots = discover_design_roots(workspace)?;
let r = analyze_discovered_root(AnalysisMode::FullElaborate, &roots[0], MVP_INTERACTIVE_BUDGET);
```

| Code | Meaning |
|------|---------|
| `bitloom-lsp.no-design-roots` | Prelude package(s) but neither metadata nor syn-scan tops |
| `bitloom-lsp.unknown-design-root` | Syn-scanned / metadata id not in registry |
| `bitloom-lsp.discover-failed` | IO / Cargo.toml read failure |
| `bitloom-lsp.timeout` / `oversized` | Same FR99 P3/P4 budgets |

**Shallow** mode on a discovered root still must **not** call `finish()` (≠ FR118 close).

## Forbidden closes

- FR99 DesignFixture alone
- FR113 metadata `design_roots` alone
- Shallow diagnostics pretending `finish` success
- docs-only

## Recipe

```text
cargo test -p bitloom-lsp --test fr118_syn_scan_design_root_discovery
cargo test -p bitloom --test fr118_epic59_closeout
cargo test -p bitloom-lsp --test fr113_lsp_design_root_discovery
cargo test -p bitloom-lsp --test fr99_bitloom_lsp_full_elaborate
```

## Fixtures

| Path | Role |
|------|------|
| `crates/bitloom-lsp/fixtures/fr118_syn_ok` | No metadata; `#[bitloom::top] Fr118OkCounter` → Pass |
| `crates/bitloom-lsp/fixtures/fr118_syn_fail` | No metadata; syn-scan fail root → readable Fail |
| `crates/bitloom-lsp/fixtures/fr113_meta_ok` | Metadata still works (NFR48) |
| `crates/bitloom-lsp/fixtures/fr113_meta_none` | No metadata and no top → `no-design-roots` |

## Cross-links

| Doc | Role |
|-----|------|
| [`fr113-lsp-design-root-discovery.md`](fr113-lsp-design-root-discovery.md) | FR113 metadata face — still closed (NFR48) |
| [`fr99-bitloom-lsp.md`](fr99-bitloom-lsp.md) | FR99 DesignFixture MVP — still closed |
| [`fr160-non-cargo-path-scan.md`](fr160-non-cargo-path-scan.md) | FR160 non-Cargo / non-members path scan (≠ this FR118 page alone) |
| NFR14 | `_agile-output/implementation-artifacts/nfr14-risk-epic59-syn-scan-design-root-discovery.md` |
