---
title: 'Story 129.2 外设子系统与使用配方'
type: 'feature'
created: '2026-09-22'
status: 'done'
route: 'dispatch'
review_loop_iteration: 0
baseline_commit: '46249478384360590e90be59b4709afef967d4ee'
context:
  - '{project-root}/_agile-output/implementation-artifacts/epic-129-context.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-129-nfr14.md'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** 当前提交候选仅生成层级 RTL，并在 testbench 等待后无条件打印 PASS，没有执行 AXI/CSR 事务或验证 Timer、GPIO、UART、IRQ、错误、背压和复位，因此不能关闭 FR198。

**Approach:** 保留同一 `ElaborateSession` 的真实 bridge/decoder/四叶组合，使用强制存在的 Icarus/VVP 独立 testbench 对 AXI 与直接 CSR 两种实质不同拓扑运行真实事务和外部可观察副作用，并归档可复现命令与结果。

## Boundaries & Constraints

**Always:** 设计只使用 `bitloom-prelude` 已交付模块；单时钟、16 位地址、32 位数据、四个 0x100 窗口；IRQ 位序严格为 Timer/RX/TX/error/GPIO；工具缺失、零行为断言、超时或非零退出均硬失败；一 Story 一提交。

**Never:** 不复制外设算法，不新增 native/generated 层级，不把 emit/Yosys/旧单体测试称为系统 PASS，不改公开 API、工具 pin、包版本，不关闭 129.3、Epic129、Epic130 或 Phase24。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| AXI 四窗 | AW/W 分拍或同拍，AR，B/R 背压 | 配置并读回 GPIO/Timer/UART/IRQ，受阻 payload 稳定且副作用一次 | 洞/未对齐/访问类型为 SLVERR，窗外和高位为 DECERR |
| 直接 CSR | 同样地址和外设向量，无 bridge | 四叶副作用与 AXI 图一致，连接图不含 bridge | `error` 和失败读数据符合 decoder 合同 |
| 事件与复位 | Timer、UART RX/TX/error、GPIO 新沿；待处理事务时 reset | 五路 IRQ 可达，清除后旧 sticky 不重触发；共同 reset 取消状态与事务 | 已发出的 UART 物理 bit 不承诺回滚 |

</frozen-after-approval>

## Code Map

- `crates/bitloom/tests/fr198_peripheral_system.rs` -- 当前系统图与伪行为 testbench；改成强制实际仿真和独立断言。
- `crates/bitloom-prelude/src/ip/{axi_lite_csr,csr_decoder,uart_csr,timer,irq}.rs`、`gpio/csr.rs` -- 端口、地址、副作用和事件语义权威；只读复用。
- `crates/bitloom/tests/fr196_axi_lite_csr/`、`fr196_csr_decoder/` -- 可复用 AXI/CSR 握手、背压和错误 testbench 模式。
- `crates/bitloom/tests/fr197_{timer,gpio,irq,uart}.rs` -- 可复用真实外设向量和独立 UART/IRQ 判据。
- `_agile-output/implementation-artifacts/129-2-外设子系统与使用配方.md` -- Story 状态、实现记录和验收结论。
- `_agile-output/test-artifacts/129-2-*` -- ATDD、review、automate、最终回归和复现证据。

## Tasks & Acceptance

**Execution:**
- [x] `crates/bitloom/tests/fr198_peripheral_system.rs` -- 修正连线并增加实际 AXI/CSR 事务、四叶副作用、错误、背压、IRQ 和 reset 检查；Icarus/VVP 必须存在。
- [x] `_agile-output/test-artifacts/129-2-*` -- 更新 build/automate/复现证据，明确执行数量、工具身份、退出码与范围。
- [x] `_agile-output/implementation-artifacts/129-2-外设子系统与使用配方.md` -- review、automate、clean/fmt/just test 完成后标 done。

### Review Findings

- [x] [Review][Patch] 恢复 NFR14 冻结的 16 seed x 1000 完成事务随机预算，并保存 seed/计数。
- [x] [Review][Patch] 在隔离 checkout 与隔离 Cargo/RTL 产物目录实际执行一命令配方，记录 checkout/source/tool 指纹。
- [x] [Review][Defer] FIRRTL/Chisel 四外设系统执行在 Story 与 Epic 上下文间归属冲突 — deferred: Story AC7 将系统复核留给 129.3，而 Epic129 NFR14 表仍写为 129.2 责任；不修改冻结规格，129.3 必须实际关闭该矩阵。

Rejected: direct CSR 与 AXI 外设向量不一致为 false；复核当前 direct TB 已包含 Timer periodic/wrap 与 UART overflow。

Review infrastructure note: Blind Hunter、Edge Case Hunter、Verification Gap、Acceptance Auditor 三轮均超时且未返回 findings；以上为主线程逐 AC/NFR14 fallback triage，不能记为 clean multi-layer review。

**Acceptance Criteria:**
- Given 两种真实层级图，when 独立 testbench 执行完整外设向量，then 每个拓扑都由外部可观察断言证明行为而非无条件 PASS。
- Given 非法地址、响应背压和 reset 中断，when 请求处于不同阶段，then 响应码、payload 稳定、取消和不重放符合合同。
- Given 任何必需 RTL 工具缺失或仿真未执行，when 测试运行，then 非零硬失败且不得以 Yosys 替代。

## Implementation Notes

- 2026-09-22 恢复审计：原 testbench 只等待 100ns 后打印 PASS；原 runner 在缺 Icarus 时跳过仿真。Story 从 review 退回 in-progress。

## Spec Change Log

## Review Triage Log

- 2026-09-22：2 patch、1 defer、1 rejected；patch 自动应用，defer 绑定 Story 129.3；四个独立 layer 基础设施超时已如实记录。
- 2026-09-22：两项 patch 定向复跑通过；隔离checkout使用空Cargo target，AXI/direct各完成16,000随机事务并输出源码/工具manifest。

## Design Notes

优先复用现有 FR196/FR197 testbench 约定；系统 testbench 的 oracle 只读公开端口和手写地址，不窥探 DUT 内部状态。Yosys 继续作为结构门，但不能替代 VVP 行为门。

## Verification

**Commands:**
- `cargo test -p bitloom --test fr198_peripheral_system -- --nocapture` -- 两拓扑实际 VVP 行为测试与 Yosys 结构检查全部通过。
- `python3 _agile-output/test-artifacts/129-2-atdd-gate.py && python3 -O _agile-output/test-artifacts/129-2-atdd-gate.py` -- ATDD 合同在优化模式下仍有效。
- `python3 _agile-output/test-artifacts/129-2-automate.py` -- 自动化门实际执行系统测试并检查非跳过结果。
- `PATH=/tmp/bitloom-maintenance-tools/bin:$PATH _agile-output/test-artifacts/129-2-isolated-replay.sh` -- 独立 checkout、空 Cargo target 与独立 RTL 目录实际通过。
- `cargo clean && cargo fmt --all && PATH=/tmp/bitloom-maintenance-tools/bin:$PATH just test` -- Cargo workspace 测试完成且退出0；外层Trae沙箱因受限文件探测返回1，ignored 单列不计PASS，详见最终验证证据。
