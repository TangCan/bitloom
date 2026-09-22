---
title: 'Story 130.1：外部 IP 准入 NFR14'
type: 'chore'
created: '2026-09-22'
status: 'done'
route: 'dispatch'
review_loop_iteration: 0
baseline_commit: 'baf07e739af2b36f4e0befddda9826964d16118e'
context:
  - '{project-root}/AGENTS.md'
  - '{project-root}/_agile-output/implementation-artifacts/130-1-外部-ip-准入-nfr14.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-130-context.md'
  - '{project-root}/docs/ip/phase24-contract.md'
  - '{project-root}/_agile-output/test-artifacts/130-1-atdd-manual-acceptance.md'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**问题：** Epic130 在外部源码锁定、离线重放和真实小核适配开工前缺少有效 NFR14，现有空黑盒机制不能证明 FR199/FR200 或提升外部支持等级。

**方法：** 交付一份可执行的 Epic130 风险记录，冻结来源、许可证、离线重放、真实行为、支持等级、负责人和停止条件；实跑工具/空壳绑定机制与状态门探针，但不下载或绑定候选外部核。

## Boundaries & Constraints

**始终：** 填满 NFR14(a)–(d)，明确 130.2/130.3 责任、stable-vs-master 规则、manifest/lock、许可证、禁网重放、支持等级及停止动作。实际探针保存命令、UTC、耗时、退出码、版本、路径、日志与源身份，缺工具硬失败。

**禁止：** 下载、vendoring 或选择最终候选；修改产品源码/API、工具 pin、包版本或支持矩阵外部行；把空壳、compile-only、网络可达、上游自测或核心 FR201 证据称为 FR199/FR200。不得推进 130.2/130.3、关闭 Epic130、push 或 publish。

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip/blackbox.rs` 与 `crates/bitloom/tests/fr82_fifo_uart_baseline.rs`：现有端口壳和无行为 vendor stub，只读并用于机制探针。
- `_agile-output/implementation-artifacts/nfr14-risk-record-template.md`、`epic-130-context.md`：风险结构与 Epic 边界。
- `scripts/check_phase24_gate.py`、`130-1-atdd-{presence,gate}.py`：现有状态门、缺正文 RED 和 84 场景过程验收。
- `docs/ip/phase24-support-matrix.md`：外部行必须保持全 no/not delivered。
- `_agile-output/test-artifacts/129-1-root-foundation.py`：工具身份与证据记录模式，可复用原则但不复制系统/形式宣称。

## Tasks & Acceptance

**执行：**
- [x] `epic-130-nfr14.md`：写完整风险记录，冻结来源/许可证/离线/真实适配/支持等级/owner/停止条件和维护叠加。
- [x] `_agile-output/test-artifacts/130-1-*`：实现并运行工具、禁网能力和真实空壳 RTL 编译/综合机制探针，保存结构化结果与边界说明。
- [x] `_agile-output/test-artifacts/130-1-atdd-manual-acceptance.md`：逐项引用真实证据完成 M01–M21；M22 七步最终关闭留主代理。
- [x] Story Dev Record 与 spec：记录实现、验证、未交付边界和文件清单；不改产品源码/支持等级。

**验收标准：**
- Given Story130.2/130.3 尚未开始，when 风险门完成，then FR199/FR200 与外部支持行仍未交付，后续依赖和停止条件可执行。
- Given 当前环境与空壳机制，when 探针运行，then Git/SHA256/禁网/Icarus/vvp/Yosys/闭包工具身份和真实 RTL compile/sim/synthesis 有原始结果，且明确不构成外部行为证明。
- Given 正文和状态门，when presence、普通及优化 Python gate 与人工验收执行，then正文存在性转绿、两次各84场景通过、真实 sprint 不变，内容质量由逐项证据而非关键词确认。

## Implementation Notes

- Intent gaps: none。正式 Story、Epic130 context、FR199–201/NFR98、AD-31 与 ATDD 已冻结可观察结果。
- Irreversibles: none。不发布、不 push、不联网获取候选、不迁移数据或触发外部配置。
- Footprint: 新增 NFR14 正文、build spec、工具/机制证据与审阅记录；更新 Story Dev Record。`blackbox.rs`、支持矩阵、产品 API、工具 pin、包版本和 130.2/130.3 状态保持不变。
- Dispatch implementation agent timed out without edits; main workflow used the allowed inline fallback.
- Build probe ran under normal and optimized Python. Each result contains 17 commands and records the unavailable host `unshare -n` plus the successful loopback-only bwrap network namespace.
- Empty vendor RTL compiled, launched and synthesized, but its explicit `behavior=absent` marker prevents promotion to FR199/FR200 evidence.
- M01–M21 passed substantive review; M22 remains with the parent seven-step workflow.

## Review Triage Log

- `medium / patch`：fallback diff 审计确认 Rust 合同命令未显式锁定和禁网，可能隐式解析依赖；补 `cargo test --locked --offline` 并重跑两种 Python 模式。
- `low / patch`：fallback diff 审计确认只记录了 `sha256sum --version`，未执行工具交叉校验；新增 blackbox 源文件摘要并与 Python SHA256 比较。
- `incomplete review infrastructure`：Blind Hunter、Edge Case Hunter、Verification Gap 三层均在两轮等待和收敛请求后超时，无可采信 finding；未将其记录为 clean review，以上为主线程 fallback 审计结果。

## Verification

- `python3 _agile-output/test-artifacts/130-1-atdd-presence.py`
- `python3 _agile-output/test-artifacts/130-1-atdd-gate.py`
- `python3 -O _agile-output/test-artifacts/130-1-atdd-gate.py`
- Story130.1 build probe normal 与 `python -O`；检查结构化结果和负向禁网能力。
- `python3 scripts/check_phase24_gate.py`、`git diff --check`，并人工核对支持矩阵与产品源码未变。
