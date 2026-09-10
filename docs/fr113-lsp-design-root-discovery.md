# FR113 — LSP design-root discovery deepen

**Product:** Bitloom (`bitloom-lsp`). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 55 / FR113 closed** (Story **55.3**). Discovery path delivered in Story **55.2**.
Phase 12 FR99 DesignFixture MVP remains closed (NFR44). Full workspace `#[bitloom::top]`
syn-scan without metadata stays **deferred** (NFR47 — needs new contract).

This page is the **FR113 completion surface**. It deepens keystroke elaborate **beyond**
Phase 12 FR99 DesignFixture-only MVP.

## Selected discovery strategy (NFR14)

| Strategy | Status |
|----------|--------|
| **Cargo-graph + `[package.metadata.bitloom] design_roots`** | **FR113 face — closed** |
| FR99 `DesignFixture` enum | Regression only — **alone ≠ FR113** |
| Full workspace `#[bitloom::top]` syn-scan (no metadata) | **deferred** (NFR47) |

### Metadata contract

Design packages that depend on `bitloom-prelude` declare:

```toml
[package.metadata.bitloom]
design_roots = ["Fr113OkCounter"]
```

Root ids are **documented elaborate entry names** registered with `bitloom-lsp`
(文档等价 of `.rs`/type paths for this deepen). Discovery walks a Cargo workspace
`members` list or a single-package `Cargo.toml`.

## Product API

```rust
use bitloom_lsp::{
    AnalysisMode, MVP_INTERACTIVE_BUDGET, analyze_discovered_root,
    analyze_on_did_save_at, analyze_workspace_design_roots, discover_design_roots,
};

let roots = discover_design_roots(workspace)?;
let r = analyze_discovered_root(AnalysisMode::FullElaborate, &roots[0], MVP_INTERACTIVE_BUDGET);
// or:
let r = analyze_workspace_design_roots(AnalysisMode::FullElaborate, workspace, MVP_INTERACTIVE_BUDGET);
// didSave hint:
let r = analyze_on_did_save_at(Some(path));
```

| Code | Meaning |
|------|---------|
| `bitloom-lsp.no-design-roots` | Prelude package(s) found but no metadata roots |
| `bitloom-lsp.unknown-design-root` | Metadata id not in FR113 registry |
| `bitloom-lsp.discover-failed` | IO / Cargo.toml read failure |
| `bitloom-lsp.timeout` / `oversized` | Same FR99 P3/P4 budgets |

**Shallow** mode on a discovered root still must **not** call `finish()` (≠ FR113 close).

## vs FR90 / HTML

- **FR90** rust-analyzer = Rust IDE — does **not** close FR113.
- FR38/FR49 HTML visualization ≠ LSP / ≠ FR113.

## Recipe

```text
cargo test -p bitloom --test fr113_lsp_design_root_discovery
cargo test -p bitloom --test fr113_epic55_closeout
cargo test -p bitloom --test fr99_bitloom_lsp_full_elaborate
```

## Fixtures

| Path | Role |
|------|------|
| `crates/bitloom-lsp/fixtures/fr113_meta_ok` | Metadata `Fr113OkCounter` → Pass |
| `crates/bitloom-lsp/fixtures/fr113_meta_fail` | Metadata fail root → readable elaborate Fail |
| `crates/bitloom-lsp/fixtures/fr113_meta_none` | Prelude, no metadata → `no-design-roots` |

## Cross-links

| Doc | Role |
|-----|------|
| [`fr99-bitloom-lsp.md`](fr99-bitloom-lsp.md) | FR99 DesignFixture MVP — still closed (NFR44) |
| NFR14 | `_agile-output/implementation-artifacts/nfr14-risk-epic55-lsp-design-root-discovery.md` |

## Non-goals (NFR47)

- Full workspace `#[bitloom::top]` syn-scan without metadata
- Claiming DesignFixture MVP or rust-analyzer alone closes FR113
- HTML visualization as LSP
