---
title: 'Story 125.2 AXI 真实 RTL 红测'
type: bugfix
created: 2026-09-20
status: done
route: dispatch
baseline_commit: 0e2a873961988f3c76222d193c402017ec4d8253
review_loop_iteration: 0
context:
  - '{project-root}/AGENTS.md'
  - '{project-root}/docs/ip/phase24-contract.md'
---
<frozen-after-approval>
## Intent
为已观察的旧 Axi4LiteSlave 分拍 AW/W 和并发 AR+AW/W 丢事务建立独立协议期望测试，实际执行生成 RTL，先证明修复前失败。用户批准 M0 已包含此故事。
## Boundaries & Constraints
只写测试/证据和测试驱动，不改产品代码。保留旧8bit地址、32bit数据、四寄存器、unmapped读0/写忽略/OKAY。不要误将新16bit CSR桥规则应用旧接口。红测独立标记 ignored 并说明缺陷，125.3解除；不能用should_panic把工具失败当复现成功。产品CI状态不提前绿。
## I/O & Edge-Case Matrix
| 场景 | 输入 | 期望 |
|---|---|---|
| AW早 | AW握手后撤valid并改变addr，延迟W | 写原地址并产生一次B |
| W早 | W握手后撤valid并改变data/strb，延迟AW | 使用缓存payload，产生一次B |
| 并发读写 | AW/W/AR同拍且都ready | 两个响应；同地址读旧值 |
</frozen-after-approval>
## Code Map
- `crates/bitloom-prelude/src/ip/axi.rs` 旧电路定义，仅阅读。
- `crates/bitloom/tests/fr98_axi_near_vip.rs` 既有1tick响应兼容测试；不改期望。
- `crates/bitloom/tests/simulator_bit_vectors.rs` Icarus/vvp工件风格和BITLOOM_REQUIRE_RTL门禁可参考。
- `crates/bitloom-sim/src/ip_dual.rs` 旧FL；125.3修正。
- `/tmp/bitloom-maintenance-tools/bin` 已有iverilog/vvp，运行时加PATH，不写机器路径到仓库代码。
## Tasks & Acceptance
- [x] 新建 `crates/bitloom/tests/fr193_axi_protocol.rs`：至少AW早、W早、并发同址三场景，独立黄金期望；Sim两引擎及实际direct emitted Verilog运行同一可审计输入/断言。定向帧检查pre-edge握手与post-edge响应，避免tick重复握手。
- [x] 保存生成设计、testbench、日志到target/fr193-axi/...每场景独立目录。工具可通过PATH；严格RTL缺工具必须失败。红测隔离ignore reason明确125.2复现未修复，正常emit测试可运行。运行忽略测试应非零且错误为具体协议断言，不能工具错误。
- [x] 新建 `docs/ip/phase24-axi-red-evidence.md`：完整基线commit、工具版本、命令、每场景native/RTL失败事实、局限和125.3接续；保留可复现脚本/测试，不只日志。
- [x] 如BFM依赖暂未准备，raw-channel testbench为实际测试驱动；清楚标注独立cocotbext-axi API探针由主代理并行验证，不冒称已用高层BFM。
**Acceptance Criteria:** Given旧设计，When分别执行3场景真实RTL，Then失败源为缺失响应而非编译错误；Given相同输入native，When执行，Then对应问题同样可见；Given正常workspace，When125.2尚未修复，Then红测隔离且明确未交付。
## Implementation Notes
不提交，由主代理检查后提交。只改本故事测试及证据页，不改sprint-status或其他spec。所有日志报告实际命令结果。
## Spec Change Log
## Review Triage Log
## Verification
`PATH=/tmp/bitloom-maintenance-tools/bin:$PATH BITLOOM_REQUIRE_RTL=1 cargo test -p bitloom --test fr193_axi_protocol -- --ignored --nocapture` 应因各协议断言失败；检查输出区分native/RTL。`cargo test -p bitloom --test fr193_axi_protocol` 正常项通过、忽略项明确标示。

## 完成记录
主代理逐行审阅并独立重跑，9个协议断言失败、编译成功；正常1pass/9ignored。工具BFM实测1pass/0skip，已单独注明stub边界。红测原始日志已归档。
