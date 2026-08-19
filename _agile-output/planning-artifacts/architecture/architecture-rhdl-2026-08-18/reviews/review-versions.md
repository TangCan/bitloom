# Versions / Reality-Check Review — Architecture Spine

- **Lens:** committed decisions were web-researched or reality-checked (not asserted from training data): current library/framework versions; named tech still exists and fits; greenfield live defaults
- **Subject:** `ARCHITECTURE-SPINE.md`
- **Checked:** 2026-08-18 (independent live fetches; did not trust the spine or the research memo)
- **Greenfield:** yes — no starter template; live defaults = `cargo new` + current stable rustc + Chisel-tested firtool pair

## Verdict: **pass**

Every version in the Stack table exists on the vendor’s current channel. Every named third-party technology still exists and still fits the role the spine assigns. Nothing looks invented. Greenfield defaults match: `cargo new` edition is 2024; rustc stable is 1.97.1; the firtool pin is the live Chisel 7.14.0 pair, not a stale Cement-style tarball.

CIRCT itself has moved one tag past the pin (`firtool-1.156.0`, 2026-08-16). That is a discuss item, not a failed check: AD-9 pins the **Chisel-tested** firtool, which is still 1.155.0.

## Stack audit (live)

| Spine pin | Live check 2026-08-18 | Result |
| --- | --- | --- |
| rustc MSRV **1.97.1** | [Rust blog](https://blog.rust-lang.org/2026/07/16/Rust-1.97.1/) + [stable notes](https://doc.rust-lang.org/stable/releases.html): 1.97.1 released 2026-07-16; current stable. 1.97.0 was 2026-07-09. | **current** |
| Rust edition **2024** | Stabilized in 1.85.0 (2025-02-20). [`cargo new --edition` default is 2024](https://doc.rust-lang.org/stable/cargo/commands/cargo-new.html) on 1.97. Manifest book: *“Most manifests have the edition field filled in automatically by cargo new… 2024 edition currently.”* | **current + greenfield default** |
| FIRRTL spec **6.0.0** | [chipsalliance/firrtl-spec v6.0.0](https://github.com/chipsalliance/firrtl-spec/releases/tag/v6.0.0) Latest, 2026-05-12. Spec examples use `FIRRTL version 6.0.0`. | **current** |
| CIRCT firtool **1.155.0** | Tag [firtool-1.155.0](https://github.com/llvm/circt/releases/tag/firtool-1.155.0) published **2026-08-11T00:52:57Z** with `firrtl-bin-*.tar.gz` assets. Chisel v7.14.0 `etc/circt.json` is `{"version": "firtool-1.155.0"}`. Chisel 7.14.0 release notes name 1.155.0 as the associated firtool. | **exists; Chisel-correct pin** |
|  | CIRCT **latest** tag is [firtool-1.156.0](https://github.com/llvm/circt/releases/latest) published **2026-08-16** (changelog `firtool-1.155.0...firtool-1.156.0`). | see F1 |
| Chisel **7.14.0** (interop对照, 非依赖) | [chipsalliance/chisel latest](https://github.com/chipsalliance/chisel/releases/latest) = **v7.14.0**, 2026-08-13. | **current** |
| Yosys **0.68** (可选) | [YosysHQ/yosys latest](https://github.com/YosysHQ/yosys/releases/latest) = **v0.68**, 2026-08-05. | **current** |
| Verilator **5.050** (可选) | [verilator.org changes](https://verilator.org/guide/latest/changes.html) titled 5.050, dated 2026-07-01. Homebrew core formula `url` is `v5.050.tar.gz`. No newer stable found. | **current** |

Do not treat the Chisel [versioning HTML](https://www.chisel-lang.org/docs/appendix/versioning) 7.x table as the pairing source: the live page still renders the 5.x table (JS tabs). The 7.14.0 ↔ 1.155.0 pair is on the GitHub release and `etc/circt.json`, not that table. The `llvm-firtool % "1.153.0"` snippet on that page is an **override example**, not the pairing.

## Named technology (exists + still fits)

| Named in spine | Live? | Fits the assigned role? |
| --- | --- | --- |
| crates.io `rhdl` occupied; publish **`rhdl-rs`** | **Yes.** [crates.io/crates/rhdl](https://crates.io/crates/rhdl) 0.1.0, owner samitbasu, 2023-09-02, repo `samitbasu/rhdl`. `GET /api/v1/crates/rhdl-rs` → **404** (name free). | AD-2 is the live-correct publish identity. Occupied stub ≠ this toolchain. |
| Internal `rhdl-hir` `rhdl-vlog` `rhdl-firrtl` `rhdl-sim` `rhdl-prelude` `rhdl-builder` `rhdl-macro` | All seven `GET /api/v1/crates/<name>` → **404**. Search `q=rhdl` also lists `rhdl-bits` (samitbasu, occupied) and `csa-rhdl` (unrelated). | Seed crate names are free to publish. `rhdl-bits` is **not** in the seed; see F3. |
| Chisel cannot parse `.fir` (AD-3) | **Still true.** [chisel#4899](https://github.com/chipsalliance/chisel/issues/4899) (open): Scala FIRRTL parse dropped with CIRCT; last parser was Chisel 3.6 / FIRRTL 1.6. Path is emit `.fir` → `firtool`. | AD-3 “不依赖 Chisel 解析 `.fir`” is live-correct. |
| FIRRTL ABI: private modules must be mangled | **Yes.** [abi.md Private Modules](https://github.com/chipsalliance/firrtl-spec/blob/main/abi.md): compilers shall mangle private modules not removed; scheme implementation-defined. | Consistency convention matches spec 6 ABI. |
| Yosys-friendly Verilog = no packed arrays, no `automatic` (AD-8) | **CIRCT’s live Yosys profile still says this.** [VerilogGeneration § Yosys](https://circt.llvm.org/docs/VerilogGeneration/): `disallowLocalVariables` + `disallowPackedArrays`; “Yosys doesn’t parse `automatic`”; “Yosys doesn’t accept packed arrays.” Yosys 0.68 changelog added some packed-multidimensional-array support — that does **not** retire CIRCT’s firtool-for-Yosys flags. | AD-8 emission rule still matches firtool’s documented Yosys lowering. Do not weaken it from the Yosys changelog alone. |
| `RHDL_FIRTOOL_PATH` = directory containing `firtool` | Live analogue: Chisel `CHISEL_FIRTOOL_PATH` (same shape). CIRCT ships `firrtl-bin-*.tar.gz` for 1.155.0. | Fits AD-9 fetch/cache + override. |
| `cargo rhdl` / crate `rhdl-rs` / bin `cargo-rhdl` | Cargo subcommands still require a PATH binary named `cargo-<cmd>`. Crate name may differ from the binary. | Fits. Publish `rhdl-rs` with bin `cargo-rhdl`. |
| `tracing` (CLI logs) | crates.io **tracing 0.1.44** (2025-12-18), current. | Exists and fits CLI logging. Unpinned — F2. |
| Clash `Signal<D,T>` CDC (Deferred) | Clash 1.10 still has `Signal (dom :: Domain) a`. | Exists; deferred correctly. Notation is a Rust transcription, not a Clash API string. |
| HLS Bambu / XLS (Deferred) | [google/xls](https://github.com/google/xls/) live; [ferrandi/PandA-bambu](https://github.com/ferrandi/PandA-bambu) live (release 2024.10). | Exist; “不自研 / 可选后期” still fits. |
| TLM-2.0 (forbidden as HIR lowering) | IEEE 1666 SystemC TLM still the named standard. | Exists; ban is a product choice, not a version invention. |
| rustc **1.98.0** planned **2026-08-20** (Deferred) | [releases.rs/1.98.0](https://releases.rs/docs/1.98.0/): beta, stable on 20 August 2026. [rust-lang/rust#160700](https://github.com/rust-lang/rust/issues/160700) draft notes: “Version 1.98.0 (2026-08-20)”. | Date is live-correct. Deferring MSRV bump is right (today is 2026-08-18). |
| Cement as anti-pattern (AD-9) | Not a stack pin. Research-era claim: cmtrs/cmtc last crates.io 2025-01. Used only as “don’t pin a year-old firtool and call it current.” | Still a valid caution; 1.155.0 is five days old, not Cement-stale. |
| CHIRRTL-specific mem (AD-3 import subset) | FIRRTL spec 6 still distinguishes CHIRRTL / mem forms. | Subset exclusion is a product cut, not an invented dialect. |
| `compile_error` for proc-macro user errors (AD-10) | Still the stable proc-macro contract on edition 2024 / rustc 1.97. | Fits. |

## Greenfield live defaults

No starter (no `chisel-template`, no `cargo-generate` skeleton). Defaults that matter:

| Default | Live | Spine |
| --- | --- | --- |
| `cargo new` edition | 2024 | 2024 |
| rustc stable | 1.97.1 | MSRV 1.97.1 (patch, not 1.97.0 — correct: 1.97.1 is the miscompilation fix) |
| Next stable | 1.98.0 on 2026-08-20 | Deferred, not silently adopted |
| firtool “current” | Two answers: CIRCT HEAD **1.156.0**; Chisel-tested **1.155.0** | Pins the Chisel pair (AD-9). Correct greenfield choice for interop. |
| Publish name | `rhdl` taken | `rhdl-rs` |

## Findings

### F1 — Discuss — CIRCT HEAD is firtool-1.156.0; keep 1.155.0 as the Chisel pair

**Where:** AD-9, Stack row “CIRCT firtool”, structural mermaid `firtool 1.155.0`

**Trigger:** Live CIRCT latest is [firtool-1.156.0](https://github.com/llvm/circt/releases/tag/firtool-1.156.0) (2026-08-16). The spine pins 1.155.0 dated 2026-08-11.

**Not a fail:** 1.155.0 is still the version Chisel 7.14.0 ships (`etc/circt.json` + release notes). AD-9’s stated goal is Chisel pairing and “don’t trust PATH / don’t Cement-pin an expired tarball,” not “track CIRCT HEAD.” Bumping to 1.156.0 without a new Chisel release would **break** the documented pair.

**Guard:** Keep **firtool-1.155.0**. Optionally add one clause: “CIRCT 1.156.0 exists (2026-08-16); do not adopt until Chisel pairs it.” Re-check when Chisel > 7.14.0.

### F2 — Low — `tracing` is named and unpinned

**Where:** Consistency Conventions → 日志

**Trigger:** Named crate with no Stack version. Live default is **tracing 0.1.44** (crates.io, 2025-12-18).

**Guard:** Either add `tracing` `0.1.44` to Stack, or say explicitly that CLI logging is “any 0.1.x tracing” and is not a committed pin. As written, two epics can pick different minors.

### F3 — Low — `rhdl-*` glob vs occupied `rhdl-bits`

**Where:** AD-2 “内部包可用 `rhdl-*`”; Structural Seed (no `rhdl-bits`)

**Trigger:** AD-2 only forbids publishing `rhdl`. crates.io **`rhdl-bits`** is already samitbasu’s (0.1.0, 2023-09-09). Seed crates listed today are all 404/free.

**Guard:** When a bits/width crate appears, do not name it `rhdl-bits`. Optionally tighten AD-2: “scan crates.io before any new `rhdl-*` publish name.” Not a current seed collision.

## Not findings

- **Yosys packed arrays:** CIRCT’s live Yosys section still requires `disallowPackedArrays` / `disallowLocalVariables`. AD-8 is that firtool profile, not a claim that Yosys 0.68’s frontend never parses any packed construct.
- **Chisel versioning page 1.153.0 example:** not the 7.14.0 pair; spine did not copy that trap.
- **MSRV = latest patch:** 1.97.1 is current stable; pinning the LLVM-miscompilation point release is the live-correct stable, not an invented version.
- **No Cargo/syn/proc-macro2 pins:** those crates are not named in the spine; this lens does not invent a stack for them.

## Sources fetched this run

- https://blog.rust-lang.org/2026/07/16/Rust-1.97.1/
- https://doc.rust-lang.org/stable/releases.html
- https://doc.rust-lang.org/stable/cargo/commands/cargo-new.html
- https://doc.rust-lang.org/stable/cargo/reference/manifest.html
- https://releases.rs/docs/1.98.0/
- https://github.com/rust-lang/rust/issues/160700
- https://github.com/chipsalliance/firrtl-spec/releases/tag/v6.0.0
- https://github.com/llvm/circt/releases/latest (firtool-1.156.0)
- https://github.com/llvm/circt/releases/tag/firtool-1.155.0
- https://github.com/chipsalliance/chisel/releases/latest (v7.14.0)
- https://raw.githubusercontent.com/chipsalliance/chisel/v7.14.0/etc/circt.json
- https://www.chisel-lang.org/docs/appendix/versioning
- https://github.com/chipsalliance/chisel/issues/4899
- https://github.com/YosysHQ/yosys/releases/latest (v0.68)
- https://verilator.org/guide/latest/changes.html
- https://circt.llvm.org/docs/VerilogGeneration/
- https://github.com/chipsalliance/firrtl-spec/blob/main/abi.md
- https://crates.io/crates/rhdl
- https://crates.io/api/v1/crates/rhdl-rs (404)
- https://crates.io/api/v1/crates/rhdl-{hir,vlog,firrtl,sim,prelude,builder,macro} (all 404)
- https://crates.io/api/v1/crates?q=rhdl
- https://crates.io/api/v1/crates/tracing (0.1.44)
