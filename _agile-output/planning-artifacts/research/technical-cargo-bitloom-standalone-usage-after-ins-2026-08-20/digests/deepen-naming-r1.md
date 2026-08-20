# Digest: Deepen — publish graph & naming (r1)

**Accessed:** 2026-08-20  
**Granted:** ARCHITECTURE-SPINE AD-2; naming research 2026-08-19; crates.io probes

## Claims

1. **claim:** AD-2 forbids publishing `rhdl` / `rhdl-bits`, forbids `rhdl-rs` as publish name, locks public brand Bitloom / package `bitloom`; design crates currently must depend only on `rhdl-prelude`; internal packages may remain `rhdl-*` until new public names are checked on crates.io.  
   **source:** file:ARCHITECTURE-SPINE.md AD-2  
   **accessed:** 2026-08-20 **confidence:** high **class:** policy

2. **claim:** Naming research locked Bitloom/`bitloom` and treats another `rhdl*` public surface as permanent confusion tax vs samitbasu/rhdl.  
   **source:** technical-rhdl-rename-alternatives-product-naming-2026-08-19/research.md  
   **accessed:** 2026-08-20 **confidence:** high **class:** landscape

3. **claim:** As of 2026-08-20, crates.io returns 404 for: `bitloom-prelude`, `bitloom-hir`, `bitloom-vlog`, `bitloom-sim`, `bitloom-builder`, `bitloom-macro`, and also for `rhdl-prelude`, `rhdl-hir`, `rhdl-vlog`, `rhdl-sim`, `rhdl-builder`, `rhdl-macro` (all free to claim).  
   **source:** crates.io API probes with UA bitloom-research/1.0  
   **accessed:** 2026-08-20 **confidence:** high **class:** version

4. **claim:** Publishing the language surface under `rhdl-prelude` is technically available but conflicts with the locked brand decision and SEO/confusion goals; preferred public names are `bitloom-*`.  
   **source:** synthesis of claims 1–3  
   **confidence:** high **class:** pattern

## Recommendation seed
Rename+publish public crates as `bitloom-*`; revise AD-2 design dependency to `bitloom-prelude`. Keep git path names transitional if needed.
