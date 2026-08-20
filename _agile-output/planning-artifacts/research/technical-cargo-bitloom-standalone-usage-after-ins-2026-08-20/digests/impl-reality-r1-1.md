# Digest: Implementation reality — round 1

**Granted local evidence (plan-approved):** `crates/bitloom/Cargo.toml`, `crates/bitloom/src/main.rs`, workspace `publish =` flags, plus live CLI experiment 2026-08-20.  
**Accessed:** 2026-08-20

## Claims

1. **claim:** Local package `bitloom` is `publish = true` with `[[bin]] name = "cargo-bitloom"` and runtime deps only `clap` + `sha2` — matching the crates.io published surface.  
   **source:** file:crates/bitloom/Cargo.toml (workspace grant)  
   **publisher:** Bitloom repo  
   **accessed:** 2026-08-20  
   **confidence:** high  
   **class:** version

2. **claim:** `cargo bitloom build` generates a temporary host crate under `target/rhdl-host/<package>/` whose `Cargo.toml` path-depends on the design package under `examples|crates|<name>` and on workspace crates `rhdl-vlog` and `rhdl-hir`; host `main` calls `{package}::rhdl_elaborate()` then `rhdl_vlog::emit`.  
   **source:** file:crates/bitloom/src/main.rs (`build_host_cargo`, `build_host_main`, `Commands::Build`)  
   **publisher:** Bitloom repo  
   **accessed:** 2026-08-20  
   **confidence:** high  
   **class:** process

3. **claim:** All observed library crates in this workspace (`rhdl-prelude`, `rhdl-hir`, `rhdl-vlog`, `rhdl-sim`, …) set `publish = false`; only `crates/bitloom` sets `publish = true`.  
   **source:** grep of crates/*/Cargo.toml `publish =`  
   **publisher:** Bitloom repo  
   **accessed:** 2026-08-20  
   **confidence:** high  
   **class:** version

4. **claim:** Running the installed `cargo-bitloom` outside any Bitloom checkout (`mktemp` empty dir) successfully prints help, but `cargo bitloom build --package counter_ports --manifest-dir .` fails with missing `.../examples/counter_ports/Cargo.toml`.  
   **source:** live experiment `/tmp/tmp.PunLkCjB6t` on 2026-08-20; binary `/home/richard/.cargo/bin/cargo-bitloom`  
   **publisher:** empirical run  
   **accessed:** 2026-08-20  
   **confidence:** high  
   **class:** process

5. **claim:** Subcommands that do not need the monorepo libraries (`firtool info|ensure`, `sim-engines`, and possibly `hls` status) can still be invoked from a standalone install; `build` cannot complete without a checkout that contains the design package and unpublished emit/HIR crates.  
   **source:** main.rs command match + experiment above  
   **publisher:** Bitloom repo / empirical  
   **accessed:** 2026-08-20  
   **confidence:** high for build; medium for hls (depends on whether Bambu/external tools are present)  
   **class:** process

## Honest tutorial boundary (finding)
A step-by-step “only `cargo install bitloom`” tutorial **cannot** honestly end at Verilog emission today. An honest tutorial must either (A) clone/use TangCan/bitloom (or a path checkout) as `--manifest-dir`, or (B) wait until library crates are published and `build` stops hardcoding path deps to this monorepo.

## Leads
- Product gap: publish `rhdl-prelude` (+ emit stack) and change host shim to crates.io deps
- Or ship a `cargo bitloom new` that scaffolds a design crate against git dependencies

## Gaps
- Did not re-publish or inspect the exact crates.io tarball source beyond API metadata; local main.rs assumed equal to 0.1.1 given matching size/line counts (~508 Rust LOC on crates.io).
