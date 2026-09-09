---
title: '46.2 SystemC TLM-2.0 生成或集成产品面（FR101）'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: '8f9abe3'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic46-systemc-tlm.md'
  - '{project-root}/_agile-output/implementation-artifacts/46-1-epic-46-nfr14-风险记录.md'
  - '{project-root}/docs/fr47-dual-sim-generation.md'
  - '{project-root}/docs/fr100-formal-equiv.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
  - '{project-root}/_agile-output/implementation-artifacts/process-one-story-one-commit.md'
warnings: []
deferred:
  - 'FR101 closeout / revoke「不承诺 TLM」/ Epic 46 done (Story 46.3)'
  - 'AT-style nb_transport MVP (explicit LT-only for 46.2)'
  - 'Default TLM≡CA formal proof (FR100 / Epic 45 — already closed; not reopened)'
  - 'Epic 47 waveform / coverage'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Story 46.1 已钉死 NFR14（D1–D4、LT/AT、修订 AD-5），但尚无可构建/可运行的 SystemC TLM-2.0 **产品路径**；若仅用 README 口号或把 host Rust FL（FR47）标成 TLM，会假绿关闭 FR101。

**Approach:** （1）钉死 **LT-only MVP**（`b_transport` + `tlm_generic_payload`；文档写明不交付 AT）。（2）在工具链交付 **生成器**：`bitloom-sim` 从 `FrozenHir`（或文档钉死的最小端口面）发射可 `#include`/链接的 SystemC TLM-2.0 LT 侧 C++ 工件 + Makefile；CLI `cargo bitloom gen-tlm` 为一等入口（D2）。（3）≥1 可构建/可运行 LT 夹具烟测；文档钉死 SystemC **2.3.3**（或本机 `pkg-config systemc` 探测）与安装说明；缺依赖失败可读（D4）。（4）`docs/fr101-systemc-tlm.md` 声明 FR101 产品完成面、交叉修订后 AD-5、明确 **≠ FR47 Rust FL**。（5）ATDD + CI 可复现；sprint `46-2: done`；**保持** `46-3` backlog；`epic-46` in-progress。**不**做 46.3 收口 / 不勾选 Epic 46 关闭。

## Boundaries & Constraints

**Always:** NFR14 D1–D4 + L1（LT）+ L3（钉死 LT-only）；可构建/可运行夹具；ATDD/CI 烟测；失败可读；品牌 Bitloom；文档交叉修订后 AD-5（NFR41）；设计 crate 只依赖 `bitloom-prelude`；one-story-one-commit；公开叙事不得把 FR47 标成 SystemC TLM。

**Ask First:** 若改选 AT-only 或砍掉可运行烟测改「仅头文件 stub」；若要把默认 TLM≡CA 并入本故事完成面。

**Never:** 仅文档口号 / 空 stub 关闭 FR101；把 `emit_functional_crate` / `GeneratedFunctional` / host Rust FL 标成 SystemC TLM；勾选 Epic 46 关闭或撤销「不承诺 TLM」收口（→46.3）；开工 Epic 47；把 SystemC 依赖泄漏进设计 crate / `bitloom-prelude`。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 生成 LT 工件 | FrozenHir（flat 端口）+ out_dir | 写出 `.hpp/.cpp` + Makefile；含 `tlm_generic_payload` + `b_transport` | 路径 IO 错误可读 |
| LT 烟测 Pass | SystemC 2.3.3 已装 | g++ 链接运行；stdout 含成功标记（如 `BITLOOM_TLM_LT_OK`） | N/A |
| 缺 SystemC | 无 pkg-config / 无 lib | 构建失败；stderr/消息含 `systemc` / 安装提示；**不得**静默 skip 却宣称绿 | ATDD |
| 文档诚实 | fr101 合同页 | LT-only；D1–D4；≠ FR47；交叉 AD-5；钉死依赖版本 | ATDD 字符串守卫 |
| 非本故事 | 46.3 / epic-46 done | sprint 保持 46-3 backlog；epic-46 in-progress | 不得误关 |

</frozen-after-approval>

## Code Map

- `docs/fr101-systemc-tlm.md` — **NEW** FR101 合同页（LT-only；D1–D4；AD-5；≠ FR47）
- `crates/bitloom-sim/src/systemc_tlm.rs` — **NEW** `emit_systemc_tlm_lt`（或等价）生成器
- `crates/bitloom-sim/src/lib.rs` — **UPDATE** 导出
- `crates/bitloom/src/main.rs` — **UPDATE** `GenTlm` 子命令
- `crates/bitloom/tests/fr101_systemc_tlm_product.rs` — **NEW** ATDD（生成物 + 烟测 + 文档守卫）
- `docs/fr47-dual-sim-generation.md` — **UPDATE** 交叉：FR47 ≠ FR101
- `README.md` — **UPDATE** 索引链至 fr101（不宣称 Epic 46 已关闭）
- `.github/workflows/ci.yml` — **UPDATE** `apt` 安装 `libsystemc-dev`（烟测可复现）
- `_agile-output/implementation-artifacts/nfr14-risk-epic46-systemc-tlm.md` — **UPDATE** 可勾选 46.2 关闭项（**不**勾选 Epic 46 整表 / 46.3 项）
- `_agile-output/implementation-artifacts/sprint-status.yaml` — `46-2` → done；`46-3` backlog；`epic-46` in-progress

## Story

As a 系统级验证工程师,
I want 可用的 SystemC TLM-2.0 产品路径（生成或一等集成）,
so that 字面绿包含 TLM 合同交付。

## Acceptance Criteria

1. Given Story 46.1（NFR14 D1–D4 / LT-AT / 修订 AD-5）, when 按风险记录交付可构建/可运行的 TLM-2.0 路径（至少一夹具 + 文档化依赖/版本）, then ATDD 或 CI 可复现烟测通过；失败可读
2. And 公开品牌 Bitloom；文档交叉链修订后 AD-5；不得把 host Rust FL（FR47）标成 SystemC TLM
3. And 钉死 LT-only（或 AT-only）MVP 于 docs；不实现 46.3 收口；不关闭 Epic 46

## Tasks / Subtasks

- [x] T1: `docs/fr101-systemc-tlm.md` + fr47/README/NFR14 交叉（AC: 1–2；L3 LT-only）
- [x] T2: `bitloom-sim` SystemC TLM LT 生成器（D1/D2）+ CLI `gen-tlm`（AC: 1）
- [x] T3: ATDD `fr101_systemc_tlm_product` — 生成物守卫 + 可复现 LT 烟测 + 缺依赖可读 + 文档诚实（AC: 1–2）
- [x] T4: CI 安装 `libsystemc-dev`（AC: 1）
- [x] T5: sprint `46-2: done`；保持 `46-3` backlog；`epic-46` in-progress（AC: 3）
- [x] T6: code-review / automate / `cargo clean && cargo fmt --all && just test` / 单 commit

## Dev Notes

### Decision — LT-only + generator (D2)

**Select LT-only MVP** (`b_transport` + generic payload). Document AT as non-goal for 46.2.

**Select toolchain generator** (not third-party-lib-only): `emit_systemc_tlm_lt` writes includeable/linkable C++ under `out_dir`, mirroring FR47 `emit_functional_crate` shape. CLI `cargo bitloom gen-tlm` is the first-class product entry.

Rationale: NFR14 allows LT-only; generator satisfies D1+D2 without requiring Chisel-style external TLM IP packs; SystemC 2.3.3 is the Accellera pin available via Debian/`pkg-config`.

### Architecture

- Generator lives in **`bitloom-sim`** (toolchain). Design crates stay on **`bitloom-prelude` only** (AD-6).
- Emitted artifacts are **C++ SystemC/TLM**, never Rust FL crates. Comments/Cargo.toml of FR47 generators must continue saying Not SystemC.
- Cycle-accurate path remains FrozenHir `tick` only (AD-5); TLM product does **not** replace CA and does **not** claim TLM≡CA (FR100).
- Flat / simple port map → register-style LT target is enough for MVP fixture scale.

### Previous story intelligence (46.1 / `8f9abe3`)

- Gate record: `nfr14-risk-epic46-systemc-tlm.md` — D1–D4, L1–L3, forbid slogan / FR47 fake green.
- ATDD `nfr14_risk_epic46_systemc_tlm` must stay green; do not weaken forbid wording.
- Sprint: `epic-46: in-progress`, `46-1: done`, `46-2` backlog → this story ends `done`; `46-3` stays backlog.

### Git intelligence

- Recent: `8f9abe3` Epic 46 NFR14; `ca31a7e` Epic 45 close; FR100 product path pattern = docs + thin sim API + `crates/bitloom/tests/fr*.rs`.
- Commit style: English imperative summary (“Deliver …”).

### Suggested API sketch (non-binding)

```rust
// crates/bitloom-sim/src/systemc_tlm.rs
pub fn emit_systemc_tlm_lt(hir: &FrozenHir, out_dir: &Path) -> io::Result<PathBuf>;
pub fn resolve_systemc() -> Result<SystemcToolchain, String>; // pkg-config; readable err
pub fn build_and_run_tlm_lt_smoke(out_dir: &Path) -> Result<(), String>;
```

### Testing

- Primary: `cargo test -p bitloom --test fr101_systemc_tlm_product`
- Full gate: `cargo clean && cargo fmt --all && just test`
- CI must install `libsystemc-dev` so smoke is reproducible (not silent skip).

### Project Structure Notes

- Docs: `docs/fr101-systemc-tlm.md`
- Sim: `crates/bitloom-sim/src/systemc_tlm.rs`
- ATDD: `crates/bitloom/tests/fr101_systemc_tlm_product.rs`
- No prelude changes

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 46 / Story 46.2 / FR101]
- [Source: `nfr14-risk-epic46-systemc-tlm.md` — D1–D4 / LT-AT]
- [Source: ARCHITECTURE-SPINE AD-5 revised 2026-09-09]
- [Source: `docs/fr47-dual-sim-generation.md` — Not SystemC]
- [Source: `process-one-story-one-commit.md`]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

### Completion Notes List

- LT-only MVP：`emit_systemc_tlm_lt` + `cargo bitloom gen-tlm`；SystemC 2.3.3 pin；烟测 `BITLOOM_TLM_LT_OK`
- `docs/fr101-systemc-tlm.md` 交叉修订后 AD-5；明确 ≠ FR47
- ATDD 7 项绿；审查 Approve；CI apt `libsystemc-dev`
- sprint：`46-2: done`；`46-3` backlog；`epic-46` in-progress（未做 46.3 收口）

### File List

- `docs/fr101-systemc-tlm.md`
- `crates/bitloom-sim/src/systemc_tlm.rs`
- `crates/bitloom-sim/src/lib.rs`
- `crates/bitloom/src/main.rs`
- `crates/bitloom/tests/fr101_systemc_tlm_product.rs`
- `docs/fr47-dual-sim-generation.md`
- `docs/fr29-bridge-abstraction-both.md`
- `docs/fr92-shared-stimulus-adapter.md`
- `README.md`
- `.github/workflows/ci.yml`
- `crates/bitloom/tests/fr92_shared_stimulus_adapter.rs`
- `crates/bitloom-sim/src/lib.rs` (also `no_hir_to_tlm_api` FR101 allowance)
- `_agile-output/implementation-artifacts/nfr14-risk-epic46-systemc-tlm.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `_agile-output/implementation-artifacts/46-2-systemc-tlm-2-0-产品面-fr101.md`
- `_agile-output/implementation-artifacts/46-2-code-review.md`
- `_agile-output/implementation-artifacts/46-2-automation-summary.md`
- `_agile-output/implementation-artifacts/atdd-checklist-46-2-systemc-tlm-2-0-产品面-fr101.md`

## Change Log

- 2026-09-09: Story context created (ready-for-dev)
- 2026-09-09: Deliver SystemC TLM-2.0 LT product path (FR101 / Story 46.2)

## Suggested Review Order

**docs/fr101（LT-only / AD-5 / ≠ FR47）** → **emit_systemc_tlm_lt + gen-tlm** → **ATDD 烟测** → **CI apt** → **sprint（46-3 仍 backlog）**
