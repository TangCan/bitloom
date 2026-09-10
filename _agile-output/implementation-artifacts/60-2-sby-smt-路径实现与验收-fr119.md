---
title: '60.2 SBY/SMT 路径实现与验收（FR119）'
type: 'feature'
created: '2026-09-10'
status: 'done'
route: 'oneshot'
baseline_commit: '78b34cf'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic60-symbiyosys-smt.md'
  - '{project-root}/_agile-output/implementation-artifacts/60-1-epic-60-nfr14-风险记录.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-60-context.md'
  - '{project-root}/docs/fr100-formal-equiv.md'
  - '{project-root}/docs/fr112-generated-functional-memread-equiv.md'
  - '{project-root}/docs/fr39-formal-sva.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
warnings: []
deferred:
  - 'FR119 / Epic 60 closeout docs + NFR14 checkboxes (Story 60.3)'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Story 60.1 已钉死 (A) SymbiYosys/`sby` 绑定与 assume/assert 义务，但尚无文档化一等产品入口与夹具；易与 FR85 Verilator lint 或 FR100 F1-(i) 混淆。

**Approach:** Fork 新入口 `just formal-sby-check` / `scripts/formal-sby-check.sh`（≠ FR85）；最小 `.sby`+SV 夹具（pass + 故意 fail，含 assume/assert）；缺 `sby` 或 `BITLOOM_SBY_FORCE_MISSING=1` → 可读非零失败；`docs/fr119-*.md`；ATDD 锁合同与缺失失败语义，并守卫 FR100/FR112-B 回归面；sprint `60-2: done`；**不**勾选 Epic 60 关闭（→60.3）。

</frozen-after-approval>

## Implementation Notes

- 新入口 `scripts/formal-sby-check.sh` + `just formal-sby-check`；z3 预检；≠ FR85。
- 夹具 `crates/rhdl-formal/fixtures/fr119/{pass,fail}.{sv,sby}`（assume+assert；bmc；expect pass/fail）。
- 本机无 sby：文档记录「未安装 + FORCE_MISSING 已验」；有 sby 时可选 ATDD 跑 pass/fail。
- `.gitignore` 忽略 sby 工作目录；docs 交叉链 fr100/fr112/fr39。
- Blind Hunter 补丁：recipe 测试名、Non-goals、completion 措辞、fail ATDD、gitignore、z3 预检。

## Review Triage Log

- known-good 版本带空 — **patch**（诚实记载本机无 sby + 填写模板）。
- story Implementation Notes 空 — **patch**（本段）。
- sprint 仍 in-progress — **patch**（finalize → done）。
- docs 错误 fr112 测试二进制名 — **patch**。
- fr112 Non-goals 仍写 A deferred — **patch**（指向 FR119；C 仍 NFR51）。
- 无 live fail ATDD — **patch**（optional when sby+z3）。
- 缺 sby workdir gitignore — **patch**。
- 缺 z3 预检 — **patch**。
- docs 称 completion surface 易误关 — **patch**（product path；收口→60.3）。
- fr100「sole F1」措辞 — **patch**。
- fr112 Cross-links 缺 fr119 — **patch**。
- story context 缺 fr112 — **patch**。

## Dev Agent Record

### Completion Notes List

- 交付 FR119 `formal-sby-check` + 夹具 + docs + ATDD；缺工具可读失败
- FR100/FR112 ATDD 回归绿；未勾选 Epic 60 关闭

### File List

- `scripts/formal-sby-check.sh`
- `Justfile`
- `crates/rhdl-formal/fixtures/fr119/*`
- `docs/fr119-symbiyosys-smt.md`
- `docs/fr100-formal-equiv.md`
- `docs/fr112-generated-functional-memread-equiv.md`
- `docs/fr39-formal-sva.md`
- `crates/bitloom/tests/fr119_symbiyosys_smt_path.rs`
- `crates/bitloom/tests/fr112_epic54_closeout.rs`
- `.gitignore`
- `_agile-output/implementation-artifacts/60-2-*`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
