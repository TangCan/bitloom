---
title: '1.2.0 发布收尾 CI 门禁修复'
type: 'bugfix'
created: '2026-09-23'
status: 'in-review'
route: 'oneshot'
review_loop_iteration: 0
context: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

恢复发布收尾的真实 CI 验证：FR164 使用当前仿真器支持的单模块夹具并补真实执行回归；FR200 使用可执行 bubblewrap 网络隔离的 runner，保留全部网络与文件隔离谓词。不得放宽层级仿真限制、静默跳过门禁、改动工具产品钉或重新上传 1.2.0 包。

</frozen-after-approval>

## Implementation Notes

- 基线 `7f5e9182ce00057ba70e858ceb6bc6a491c6724a`；远端运行 35860205600。
- 路径判定：需求无缺口；本地代码与 runner 配置可逆；小范围夹具/测试/工作流维护，无新公开 API。推送与 Release 属原有用户授权的后续发布收尾。
- 根因一：FR164 旧夹具 Child/u0 与现行 Sim 单模块边界冲突，本地重现 panic。保留外部 firtool 编译前置，原仿真传值关系用单模块 y <= x 等价表达。
- 根因二：FR200 bwrap 创建 loopback 时 EPERM，尚未进入隔离内执行。Ubuntu 24 非特权 namespace 限制是可能原因，不凭日志认定 AppArmor 唯一根因。只把该 job 钉为 ubuntu-22.04，避免修改宿主安全策略；真实远端重跑决定能否通过。离线重放本身不改动。
- 代码范围：`.github/workflows/ci.yml`；`crates/rhdl-firrtl/fixtures/fr164_external_circt_sim_gate.fir`；`crates/bitloom/tests/fr164_circt_external_sim_gate.rs`；`docs/fr164-circt-external-sim-gate.md`；本记录。
- 验收：默认夹具真实 import/Sim 执行成功；零输出变体失败；层级输入仍被拒绝；缺工具负向仍非零；FR164 完整本地脚本通过；FR200 远端所有原门禁通过才可宣布环境修复。
- 不移动已推送标签，其仍代表真实 1.2.0 上传源码；门禁修复提交单独记录。

- 实际本地验证：旧默认夹具执行失败；新回归先红（层级拒绝）后绿；`cargo test -p bitloom --test fr164_circt_external_sim_gate` 8 passed / 0 failed / 0 ignored，`cargo fmt --all -- --check` 通过，完整 `bash scripts/circt-external-sim-check.sh` 通过（firtool-1.159.0 + y=0xa5）。原始日志存仓库外 release-ci-35860205600 目录。FR200 仍待远端验证。

## Review Triage Log

- medium / patch：负向只跑复制的 helper，无法证明真实 example 的退出谓词；测试已改为执行实际 example 并验证退出码与诊断。
- medium / patch：仅观察 a5 会放过恒定 a5 故障；真实 example 改为 00/a5/5a/ff/00 输入序列，并注入常量 0 与 165。
- low / patch：runner 兼容性需真实证据，新增安装后立即运行的 network/PID namespace 探针及版本日志，完整隔离重放仍保留。

## Implementation Notes（审查补充）

- 增改 `crates/bitloom/examples/fr164_circt_sim_gate.rs`，仅门禁示例，不改变发布 API；输入序列加严保持原有 intent。所有审查项已处理，无新增延后项。

- 审查后本地复测：8 passed / 0 failed / 0 ignored，完整 FR164 门禁通过新五值序列。提交后等待 FR200 远端验收再标 done。
