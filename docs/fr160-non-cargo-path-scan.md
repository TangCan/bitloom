# FR160 — Non-Cargo monorepo path scan

**Product:** Bitloom (`bitloom-lsp`). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 92 / FR160 closed** (Story **92.3**). Implementation Story **92.2**.
Phase 14 FR118 Cargo workspace members syn-scan remains closed (NFR68).

## Selected face (NFR14)

Discover `#[bitloom::top]` / `#[rhdl::top]` under **explicit paths** that need not be
Cargo workspace members and need not contain `Cargo.toml`.

| Layer | Role |
|-------|------|
| **FR113** | Cargo-graph + metadata `design_roots` — **still closed**; alone ≠ FR160 |
| **FR118** | Cargo **members** syn-scan — **still closed**; alone ≠ FR160 |
| **FR160** | Non-Cargo / non-members **path list / scan root** — **this page** |

**Forbidden closes:** FR118 alone; FR113 alone; FR99 DesignFixture alone; docs-only;
silent-Ok on missing path or permission denied.

## Product API

```rust
use bitloom_lsp::{discover_design_roots, discover_design_roots_under};

// FR118 — Cargo workspace / package root (unchanged):
let cargo_roots = discover_design_roots(workspace)?;

// FR160 — explicit paths (no Cargo.toml required):
let roots = discover_design_roots_under(&[bare_design_dir.as_path()])?;
```

| Code / kind | Meaning |
|-------------|---------|
| `bitloom-lsp.path-not-found` (`NotFound`) | Path missing |
| `bitloom-lsp.path-permission-denied` (`PermissionDenied`) | EACCES / unreadable — **must not silent-Ok** |
| `bitloom-lsp.discover-failed` | Other IO failure under path |

## Proof obligation

1. Fixture tree **without** `Cargo.toml` yields ≥1 discovered top via
   `discover_design_roots_under`.
2. Missing path → readable `NotFound` / `path-not-found` (not empty Ok).
3. FR118 `discover_design_roots` members path remains green.

## Recipe

```text
cargo test -p bitloom-lsp --lib discover::tests::fr160_under_scans_dir_without_cargo_toml
cargo test -p bitloom-lsp --test fr160_non_cargo_path_scan
cargo test -p bitloom --test fr160_non_cargo_path_scan
cargo test -p bitloom-lsp --test fr118_syn_scan_design_root_discovery
```

## Fixtures

| Path | Role |
|------|------|
| `crates/bitloom-lsp/fixtures/fr160_non_cargo/` | No `Cargo.toml`; `design/top.rs` with `Fr160BareTop` |

## Cross-links

| Doc | Role |
|-----|------|
| [`fr118-syn-scan-design-root-discovery.md`](fr118-syn-scan-design-root-discovery.md) | FR118 members syn-scan (still closed; ≠ FR160 alone) |
| [`fr113-lsp-design-root-discovery.md`](fr113-lsp-design-root-discovery.md) | FR113 metadata face |
| NFR14 | `_agile-output/implementation-artifacts/nfr14-risk-epic92-non-cargo-path-scan-fr160.md` |

## Non-goals

- Unbounded whole-disk crawl
- Replacing FR118 members discovery
- Remote VFS / IDE store multi-target
- Claiming NFR59 “fully cleared” before FR157–FR165 all close
