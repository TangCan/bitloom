# Digest: Deepen — implementation DAG & change list (r1)

**Accessed:** 2026-08-20  
**Granted:** crates/*/Cargo.toml, bitloom main.rs

## Current DAG (elaborate → .v)

```
design crate
  → rhdl-prelude → rhdl-builder → rhdl-hir
                 → rhdl-macro (proc-macro)
                 → rhdl-hir
CLI host shim
  → design
  → rhdl-vlog → rhdl-hir
  → (calls design::rhdl_elaborate / Elaboratable)
```

All of the above except CLI are `publish = false` today.

## Optional later
- `rhdl-sim` → `rhdl-hir` (dev-dep for tick tests)
- firrtl / cabi / hls / formal / float / viz — not required for “install → Verilog” MVP

## Concrete change list for true standalone

1. Rename package names (or publish aliases) to `bitloom-{hir,builder,macro,prelude,vlog}` (+ later `sim`); set `publish = true`; bump coordinated versions.
2. Revise AD-2/AD-6 text: design depends on **`bitloom-prelude`**.
3. Rewrite `build_host_cargo` to depend on crates.io `bitloom-vlog`/`bitloom-hir` (version pinned to CLI release), not `workspace/crates/...` paths.
4. Resolve `--package` via `cargo metadata` in user `--manifest-dir` (any Cargo package exporting elaborate entry), not only `examples/<name>`.
5. Add `cargo bitloom new <name>` scaffolding: Cargo.toml + lib with `#[module]` stub + `rhdl_elaborate`/`bitloom` entry.
6. ATDD: empty temp dir, `cargo install` binary, `new` + `build` produces `.v` without cloning TangCan/bitloom.
7. README / crates.io README: replace monorepo-only quickstart with standalone tutorial; keep “devs hacking the compiler clone the repo”.

## Acceptance (definition of done)
```
cargo install bitloom@<ver>
cd $(mktemp -d)
cargo bitloom new demo
cargo bitloom build --package demo --out-dir out --manifest-dir .
test -s out/*.v   # or known filename
```
No `git clone` of the toolchain in that flow.
