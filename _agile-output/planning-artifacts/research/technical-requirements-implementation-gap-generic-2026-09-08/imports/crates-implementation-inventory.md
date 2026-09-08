# Crates implementation inventory (D3) — named imports only

**Date:** 2026-09-08  
**Scope:** `/home/richard/richard/2026/2026/rhdl/crates/` (13 Cargo packages; 56 `.rs` files).  
**Epistemics:** Evidence from Cargo.toml / `src/*.rs` / crate tests only. No training conclusions.  
**Workspace brand:** public CLI/crates.io = `bitloom`; several `rhdl-*` crates remain unpublished workspace members.

---

## 1. Crate map (1–2 lines each)

| Crate | Path | Role (from Cargo.toml + lib.rs) |
|-------|------|----------------------------------|
| **bitloom** | `crates/bitloom/` | Published CLI binary `cargo-bitloom`: build/new/import/gen-func/gen-cycle/visualize/doc/wave/hls/firtool. Depends on hir, vlog, sim, firrtl, viz. |
| **bitloom-prelude** | `crates/bitloom-prelude/` | Design-crate surface (AD-2/AD-6): types, `Bundle`/`HwVec`, host-view markers, `ip` stubs; re-exports builder + macros. |
| **bitloom-macro** | `crates/bitloom-macro/` | Proc-macros → builder/prelude only: `module`, `combinational`/`sequential`, `functional_model`/`bridge`/`abstraction`/`both`, `hls`, `top`. |
| **bitloom-builder** | `crates/bitloom-builder/` | Sole public mutation path into HIR (`ElaborateSession`); latch/CDC/width gates; freeze via `finish`. |
| **bitloom-hir** | `crates/bitloom-hir/` | FrozenHir AST, diagnostics, ground types, mem/reg/process/instance; private freeze. |
| **bitloom-vlog** | `crates/bitloom-vlog/` | FrozenHir → Yosys-friendly Verilog (`.v`). |
| **bitloom-sim** | `crates/bitloom-sim/` | Cycle-accurate `Sim::tick`, VCD/FST, coverage, dual-view equiv; FR47 `generate_functional_sim` / cycle crate emit. |
| **rhdl-firrtl** | `crates/rhdl-firrtl/` | FrozenHir ↔ FIRRTL 6.0.0 text; `emit_chisel` (FR28/AD-27). `publish = false`. |
| **rhdl-hls** | `crates/rhdl-hls/` | Legacy/workspace HLS stub crate: Bambu C emit + invoke; no in-crate scheduling. Product path also in `bitloom` CLI. |
| **rhdl-formal** | `crates/rhdl-formal/` | FR39 SVA export (`emit_sva`) + tiny text checker. `publish = false`. |
| **rhdl-float** | `crates/rhdl-float/` | FR36 SoftF16 host helpers (round-ties-to-even). `publish = false`. |
| **rhdl-viz** | `crates/rhdl-viz/` | FR38/FR49 HTML hierarchy + timing from samples/VCD; LSP deferred. `publish = false`. |
| **rhdl-cabi** | `crates/rhdl-cabi/` | FR33 cdylib: `rhdl_sim_*` / `rhdl_abs_*` C ABI over hardcoded Counter + handwritten abs. |

---

## 2. Capability evidence matrix

| Theme | Status | Primary evidence paths |
|-------|--------|------------------------|
| **Macros (design surface)** | Implemented (partial depth) | `bitloom-macro/src/lib.rs` (`module` L10–81 expands to `ElaborateSession` ports only; `combinational`/`sequential` L87–121 retain markers, do not auto-record assigns; host views L136–191; `hls` L161–177; `top` L194–206) |
| **HIR / FrozenHir** | Implemented | `bitloom-hir/src/lib.rs` (`FrozenHir` L244–247, `seal_from_builder` L576–578, validation clock/reset/multi-drive/instance/special-IO) |
| **Builder / elaborate** | Implemented | `bitloom-builder/src/lib.rs` (`ElaborateSession` L32–47, `finish` L1171–1176) |
| **Verilog emit** | Implemented | `bitloom-vlog/src/lib.rs` (`emit` L9–23; comb `assign`, seq `always @(posedge …)`, mem, instances) |
| **Sim tick + VCD/FST** | Implemented | `bitloom-sim/src/lib.rs` (`Sim::tick` L296–304; `enable_vcd` L107–137; `enable_fst` L140–160); engines in `engine.rs` |
| **FIRRTL emit/import** | Implemented (subset) | `rhdl-firrtl/src/lib.rs` (`emit` L12–26; `import` L140–353; rejects Analog/assert/chirrtl L149–156) |
| **Chisel path (FR28)** | Implemented (subset; no Mem) | `rhdl-firrtl/src/chisel.rs` (`emit_chisel` L36–78; E0901 on MemDecl L47–52; targets Chisel 7.14.0 / firtool-1.155.0 L12–15) |
| **functional_model (host)** | Implemented (marker + compare) | Macro: `bitloom-macro/src/lib.rs` L136–139; trait `HostView` `bitloom-prelude/src/lib.rs` L26–30; compare `bitloom-sim/src/lib.rs` `AbstractionView` L463–509 |
| **FR47 generators (Rust crates)** | Implemented | `bitloom-sim/src/generate.rs` (`generate_functional_sim` L167+); `bitloom-sim/src/cycle.rs` (`generate_cycle_accurate_sim` L59+); CLI `bitloom/src/main.rs` `GenFunc`/`GenCycle` L80–98 |
| **HLS** | Partial (external only) | `bitloom/src/hls.rs` + `rhdl-hls/src/lib.rs`: C stub + Bambu invoke; “no scheduling in bitloom”; product CLI `Commands::Hls` |
| **Formal / SVA** | Partial (emit + toy check) | `rhdl-formal/src/lib.rs` `emit_sva` L15–48; `check_sva_text` L51–58 |
| **Viz / wave HTML** | Partial (HTML done; LSP absent) | `rhdl-viz/src/lib.rs` `to_html` L19–76, `timing_html` L79–143; LSP deferred L5, L72 |
| **Mem / SyncReadMem** | Implemented (builder+sim+vlog+fir; Chisel fail) | HIR `Stmt::MemDecl` `bitloom-hir/src/lib.rs` L198–206; builder `declare_mem*` L248–292; sim latency-1 L377–440; Chisel E0901 |
| **CDC / ClockDomain** | Partial (phantom + E0220) | prelude ZSTs `ClockDomain`/`DoubleFlop`/`SyncFIFO` `bitloom-prelude/src/lib.rs` L108–134; builder `bind_domain`/`mark_cdc_bridge` L85–93, E0220 L857–870 — no real dual-FF RTL |
| **Bundle / HwVec (FR51)** | Partial (flatten API; no derive) | `Bundle` trait + `HwVec` `bitloom-prelude/src/lib.rs` L87–106, flatten L188–279; collision gate E0152 builder L103–118; nested Bundle OUT OF SCOPE L92–94 |
| **IP stubs (FR37/48)** | Partial (smoke stubs) | `bitloom-prelude/src/ip.rs` SyncFifo/UartTx/SpiMaster/I2cMaster/Axi4LiteSlave/ExtBlackBox |
| **C ABI (FR33)** | Partial (demo Counter only) | `rhdl-cabi/src/lib.rs` hardcoded `counter_hir` L17–33 |
| **Float (FR36)** | Partial (host SoftF16 only) | `rhdl-float/src/lib.rs` — no HIR/Verilog lowering |
| **Controlled generics / closures as design generators** | **Absent** | See §3 |

---

## 3. Closure / Fn / generator-callback search (task item 3)

Searched under `crates/` for design-facing patterns: `impl Fn`, `FnMut`, `FnOnce`, `dyn Fn`, `F: Fn`, `where … Fn`, `const fn`, `LUT`/`lut_builder`/`LookupTable`, `callback`/`Callback`.

| Pattern | Result |
|---------|--------|
| `impl Fn` / `FnMut` / `FnOnce` / `dyn Fn` / `F: Fn` bounds | **NOT FOUND** (workspace-wide under `crates/`) |
| `const fn` (any) | **NOT FOUND** |
| LUT builders taking callbacks | **NOT FOUND** |
| Product API “generator patterns” accepting user closures to build hardware | **NOT FOUND** |
| Ordinary Rust closures in toolchain internals | **FOUND** (not design-surface generators) — e.g. local helper `flush_processes` at `rhdl-firrtl/src/lib.rs:165`; many `.map(|…|` / `.any(|…|` in emit/sim/CLI |

Notes:
- `syn::ItemFn` in `bitloom-macro` is AST for attribute macros on Rust `fn` items — **not** the `Fn` trait.
- FR47 “generators” (`generate_functional_sim`, `generate_cycle_accurate_sim`) take `&FrozenHir` + `Path` and emit Rust source — **no callback/`impl Fn` parameters**.
- Macro `#[functional_model]` marks host-only types; it does not elaborate hardware from closures.

---

## 4. Tests / ATDD / recipes proving end-to-end capability

### 4.1 Just recipes (`Justfile`)

| Recipe | Purpose |
|--------|---------|
| `just test` | `cargo test --workspace` |
| `just check` | fmt check + workspace tests |
| `just firtool-smoke` | `scripts/firtool-smoke.sh` |
| `just hls-smoke` | `scripts/hls-smoke.sh` (CI stub default) |
| `just chisel-fr28-jvm` | required Chisel JVM compile of golden Scala |
| `just chisel-fr28-atdd` | `scripts/test-chisel-fr28-required.sh` |

### 4.2 Scripts (`scripts/`)

- `firtool-smoke.sh`, `hls-smoke.sh`, `fixtures/bambu-ci-stub.sh`
- `chisel-fr28-compile-required.sh`, `chisel-fr28-compile.sh`, `test-chisel-fr28-required.sh`, `test-just-chisel-fr28-jvm.sh`, `test-gha-fr28-chisel-jvm.sh`

### 4.3 Integration / ATDD under `crates/bitloom/tests/` (32 files)

| Test file | Proves (roughly) |
|-----------|------------------|
| `import_cli.rs` | CLI import `.fir` → emit |
| `visualize_cli.rs` / `wave_cli.rs` / `fr38_fr49_joint.rs` | visualize/doc/wave HTML+VCD |
| `fr47_functional_sim_gen.rs` / `fr47_cycle_bridge.rs` | FR47 generate + bridge |
| `ad27_compilable_chisel.rs` | AD-27 / FR28 contract docs |
| `ad5_rust_functional_sim_allowed.rs` / `fr29_handwritten_bridge_regression.rs` | AD-5/FR29 contracts |
| `ad20_bundle_vec_allowed.rs` | AD-20/FR51 Bundle/Vec policy |
| `hls_smoke.rs` / `hls_ci_smoke.rs` / `hls_supported_docs.rs` | HLS product path |
| `ip_index_fr48.rs` / `nfr14_risk_first_class_ip.rs` | IP index / risk |
| `rv32_core_build.rs` | example RV32 build path |
| `new_scaffold.rs` / `standalone_empty_dir.rs` / `readme_standalone_path.rs` | `cargo bitloom new` / standalone |
| `cli_brand_bitloom.rs` / `ad2_*` / `maturity_contract.rs` | brand / publish identity |
| `fr30_generated_path.rs` / `host_registry_shim.rs` / `bitloom_sim_publish.rs` | packaging / generated paths |
| `nfr14_risk_*` | risk-record ATDD (HLS, viz, Chisel, dual-sim) |
| `uj6_visualization_docs.rs` | viz docs |

### 4.4 In-crate unit/integration highlights

- **bitloom-builder:** latch, width, hierarchy, mem, CDC E0220 (~22 `#[test]`)
- **bitloom-sim:** tick golden, VCD/FST, mixed both, equiv, SyncReadMem latency, enable/async reset (~19)
- **bitloom-vlog:** golden passthrough / async reset / enable
- **rhdl-firrtl:** FIR round-trip, FR28 Chisel predicates, FR46 import/tick (~15)
- **bitloom-prelude/ip:** elaborate→emit→tick for each stub (~6)
- **rhdl-cabi/tests/c_harness.rs:** C harness vs Rust golden
- **rhdl-viz / rhdl-formal / rhdl-hls / rhdl-float:** unit smoke

Approx **146** `#[test]` attributes under `crates/` (grep count).

---

## 5. Theme classification counts (summary)

| Class | Count | Themes |
|-------|------:|--------|
| **Implemented** | 8 | Macros (surface), HIR, Builder, Verilog, Sim/VCD, FIRRTL subset, Chisel emit subset, FR47 Rust-sim generators |
| **Partial** | 9 | HLS (external), Formal SVA emit, Viz HTML (no LSP), CDC markers, Bundle/HwVec flatten, IP stubs, C ABI demo, SoftF16 host, process macros (markers only) |
| **Absent** | 5+ | Design-facing `impl Fn`/callback generators; `const fn` generators; LUT-from-callback; nested Bundle derive; real DoubleFlop RTL; SystemC TLM from HIR; LSP binary |

*(Counts are theme buckets for gap analysis, not FR IDs.)*

---

## 6. CLI surface snapshot (`bitloom/src/main.rs`)

Subcommands evidenced: `Build`, `New`, `Firtool`, `SimEngines`, `Hls`, `Import` (+`--also-fir`/`--also-chisel`), `GenFunc`, `GenCycle`, `Visualize`, `Doc`, `Wave`.

---

## 7. Explicit non-capabilities (code-stated)

- No HIR→SystemC TLM-2.0 (`bitloom-sim` test `no_hir_to_tlm_api`; AD-5 contracts).
- Chisel emit rejects `MemDecl` (`rhdl::E0901`).
- `#[derive(Bundle)]` not available (prelude docs).
- DoubleFlop/SyncFIFO are narrative ZSTs; CDC legality is `mark_cdc_bridge` only.
- HLS never schedules in-process.
- Viz LSP: deferred, no language-server binary.
