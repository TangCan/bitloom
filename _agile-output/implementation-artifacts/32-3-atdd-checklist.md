# ATDD — Story 32.3 Bundle derive（FR80）

Acceptance evidence after green implementation.

## AC mapping

| AC | Test / evidence | Status |
|----|-----------------|--------|
| Derive API via prelude (AD-6) | `use bitloom_prelude::Bundle` + `#[derive(Bundle)]` in `bundle_vec_skel` | GREEN |
| Documented support/limits | language-surface + prelude; `fr80_nested_bundle` | GREEN |
| Positive → synthesizable emit | `DerivedNestedBundleSkel` elaborate/emit/tick | GREEN |
| Unsupported shapes → stable diag | trybuild enum / tuple / HwVec field → `rhdl::E0180` | GREEN |
| Design crate deps | skel Cargo.toml only `bitloom-prelude` | GREEN |

## Commands

```bash
cargo test -p bundle_vec_skel
cargo test -p bitloom --test fr80_nested_bundle
```

**Result (2026-09-09):** both PASS.
