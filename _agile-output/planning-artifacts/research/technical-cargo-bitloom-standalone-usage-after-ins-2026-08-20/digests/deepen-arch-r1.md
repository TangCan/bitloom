# Digest: Deepen — architecture patterns for true standalone (r1)

**Accessed:** 2026-08-20

## Claims

1. **claim:** Spade’s recommended install is `cargo install --git …/swim` then `swim install-tools`; users do not clone the Spade monorepo to write designs — CLI orchestrates tools against user projects.  
   **source:** https://docs.spade-lang.org/guide_installation.html  
   **publisher:** Spade docs **accessed:** 2026-08-20 **confidence:** high **class:** pattern

2. **claim:** Compiler crate `spade-lang` 0.16.0 on crates.io depends on a family of version-aligned `spade-*` crates (`spade-hir`, `spade-parser`, `spade-macros`, …) — multi-crate publish under a product prefix.  
   **source:** https://crates.io/api/v1/crates/spade-lang/0.16.0/dependencies  
   **accessed:** 2026-08-20 **confidence:** high **class:** pattern

3. **claim:** wasm-bindgen splits library (`wasm-bindgen`) from CLI (`wasm-bindgen-cli` with bins) at matching versions — users depend on lib; install CLI separately.  
   **source:** crates.io API wasm-bindgen + wasm-bindgen-cli  
   **accessed:** 2026-08-20 **confidence:** high **class:** pattern

4. **claim:** AD-14 requires `cargo bitloom build` to generate a host/shim that depends on the design crate and backends and calls elaborate+emit in-process — this pattern can use **crates.io versions** of backends instead of path deps without abandoning AD-14.  
   **source:** ARCHITECTURE-SPINE AD-14  
   **accessed:** 2026-08-20 **confidence:** high **class:** process

5. **claim:** AD-6: design `[dependencies]` only prelude; `[dev-dependencies]` may add sim; design must not depend on CLI — true standalone must keep this split (like wasm-bindgen lib vs cli).  
   **source:** ARCHITECTURE-SPINE AD-6  
   **accessed:** 2026-08-20 **confidence:** high **class:** policy

## Patterns ranked for Bitloom
| Pattern | Fit | Notes |
|---------|-----|-------|
| A. Publish `bitloom-*` libs + keep AD-14 host shim on crates.io deps | Best | Matches Spade/wasm split; preserves spine |
| B. Git-dep scaffold only (`new` → git=TangCan/bitloom) | Interim | No clone of full workflow for users, but not crates.io-complete |
| C. Monolith: put emit inside `bitloom` lib and have designs depend on CLI crate | Reject | Violates AD-2/AD-6 (design must not depend on CLI) |
| D. Serialize FrozenHir as CLI protocol | Reject for MVP | AD-14 forbids as stage-1 CLI protocol |
