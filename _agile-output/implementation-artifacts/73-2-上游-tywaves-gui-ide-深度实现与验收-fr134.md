---
title: '73.2 上游 Tywaves GUI/IDE 深度实现与验收（FR134）'
type: 'feature'
created: '2026-09-11'
status: 'done'
route: 'oneshot'
baseline_commit: 'e95ca7c'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic73-upstream-tywaves-gui-ide.md'
  - '{project-root}/_agile-output/implementation-artifacts/73-1-epic-73-nfr14-风险记录.md'
  - '{project-root}/_agile-output/implementation-artifacts/65-2-上游-tywaves-一等路径实现与验收-fr125.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/docs/fr125-upstream-tywaves.md'
  - '{project-root}/crates/rhdl-viz/src/lib.rs'
  - '{project-root}/crates/bitloom/src/main.rs'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR134 要求超出 FR125 T1–T4 的真实上游 Tywaves GUI 安装包与/或 IDE 插件深度；仅有 sidecar / `BITLOOM_TYWAVES_BIN` 不得关闭本 FR，且缺上游工件时禁止 silent 绿。

**Approach:** 按 NFR14 **G1–G4** 交付产品路径：`cargo bitloom wave --tywaves-gui` 写出 GUI/IDE 安装描述符 + 可检查元数据 manifest（含 Bitloom / `schemaVersion` / `tywaves.gui` / `tywaves.ide-plugin`），失败语义（`BITLOOM_TYWAVES_GUI_FORCE_MISSING` / 缺 GUI root），ATDD 锁住 G1–G3；保留 FR125/117/114/104 回归（NFR56）。文档/deferred 全量收口 → Story 73.3。

## Boundaries & Constraints

**Always:** G1–G4；品牌 Bitloom；Tywaves 运行时不进设计 crate；缺上游/元数据非零失败；FR104/114/117/125 回归不破。

**Ask First:** 若强制替换默认 VCD/`typed-wave.html` 为唯一波形面，或改写 AD-6 把 Tywaves 塞进 prelude — 须修订 NFR14 + 脊柱。

**Never:** 以 FR125 T1–T4 alone / FR104 / FR114 / FR117 alone / docs-only 关闭 FR134；silent 绿；勾选 Epic 73 关闭（→ 73.3）。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| G1+G2 落地 | `wave --tywaves-gui` + stub GUI root | `tywaves.gui.manifest.json` + `tywaves.gui.install.json`（+ FR125 sidecar） | 契约字段缺失 → Err |
| G3 缺上游 | `BITLOOM_TYWAVES_GUI_FORCE_MISSING=1` | 非零 + `bitloom.tywaves*` | 禁止 silent 绿 |
| alone 禁令 | 仅 `--tywaves` 或无 GUI 旗标 | 无 FR134 manifest | FR125 alone ≠ FR134 |
| 回归 | FR104/114/117/125 测试 | 全绿（NFR56） | 回滚或修复 |

</frozen-after-approval>

## Code Map

- `crates/rhdl-viz/src/lib.rs` — `tywaves_gui_manifest` / `tywaves_gui_install_json` / `tywaves_gui_install_sh`
- `crates/bitloom/src/main.rs` — CLI `--tywaves-gui` + `run_wave` FR134 路径
- `crates/bitloom/tests/fr134_tywaves_gui_ide.rs` — ATDD G1–G4
- `docs/fr134-upstream-tywaves-gui-ide.md` — 产品路径与版本/渠道钉死
- `_agile-output/implementation-artifacts/nfr14-risk-epic73-upstream-tywaves-gui-ide.md` — G1–G4 合同

## Story

As a 调试工程师,
I want 风险记录钉死的真实上游 GUI/IDE 深度路径可用,
So that typed IDE 波形达到超出 FR125 的 GUI/插件完成面。

## Acceptance Criteria

1. Given Story 73.1 已钉死集成形状, when 交付产品路径 + ATDD/夹具（或文档化手动清单）, then 验收覆盖风险记录谓词；FR104/FR114/FR117/FR125 回归不破（NFR56）
2. And 缺上游 GUI/插件/元数据不得 silent 宣称 FR134 绿；公开品牌 Bitloom
3. And Tywaves 运行时不得进入设计 crate 依赖（延续 standing honesty）

## Tasks / Subtasks

- [x] T1: rhdl-viz FR134 emit（manifest / install 描述符 / install.sh）（AC: 1–2）
- [x] T2: CLI `--tywaves-gui` + G3 失败语义（AC: 1–2）
- [x] T3: ATDD `fr134_tywaves_gui_ide.rs` + docs/fr134（AC: 1–3）
- [x] T4: code-review Approve；automation-summary；sprint 73-2 done；不启动 73.3

## Dev Notes

- ≠ FR125 T1–T4 alone；≠ FR117 typed-wave alone。
- G1 允许树内可复现 stub 安装描述符；钉死上游 Surfer-Tywaves GUI 版本/渠道 + Surfer IDE 插件 marketplace 标识。
- G2 manifest ≠ 仅 `wave.tywaves.json`（FR125 T1）。
- 73.3：docs/deferred / Epic 73 关闭勾选。

## Testing

- `cargo test -p bitloom --test fr134_tywaves_gui_ide`
- `cargo test -p bitloom --test fr125_upstream_tywaves`
- `cargo clean && cargo fmt --all && just test`

## Dev Agent Record

### Completion Notes List

- G1–G4：`--tywaves-gui` → manifest + install.json + install.sh；钉死 Surfer-Tywaves / marketplace 渠道
- G3：`BITLOOM_TYWAVES_GUI_FORCE_MISSING` / 无效 GUI root → 非零 + `bitloom.tywaves*`
- FR125 alone ≠ FR134；Tywaves 不进 prelude；NFR56 回归保留
- code-review Approve；automation-summary；**未**启动 73.3

### File List

- `crates/rhdl-viz/src/lib.rs`
- `crates/bitloom/src/main.rs`
- `crates/bitloom/tests/fr134_tywaves_gui_ide.rs`
- `docs/fr134-upstream-tywaves-gui-ide.md`
- `_agile-output/implementation-artifacts/73-2-上游-tywaves-gui-ide-深度实现与验收-fr134.md`
- `_agile-output/implementation-artifacts/atdd-checklist-73-2-fr134-tywaves-gui-ide.md`
- `_agile-output/implementation-artifacts/73-2-code-review.md`
- `_agile-output/implementation-artifacts/73-2-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
