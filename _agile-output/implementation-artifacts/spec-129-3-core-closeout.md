---
title: 'Story 129.3 FR198/FR201 核心矩阵与维护关闭'
type: 'feature'
created: '2026-09-22'
status: 'done'
route: 'dispatch'
review_loop_iteration: 1
baseline_commit: '218f0620e85ab438a3ef1351e253277552c8c410'
context:
  - '_agile-output/implementation-artifacts/epic-129-context.md'
  - '_agile-output/implementation-artifacts/epic-129-nfr14.md'
  - 'docs/ip/phase24-contract.md'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Story129.2 已交付 direct RTL 的真实四外设系统，但 FR201 核心关闭仍缺完整 direct/FIRRTL/Chisel 行为矩阵、系统级有限 formal/综合、必需 CI、兼容性证据和贡献规范。

**Approach:** 复用同一个 FR198 `FrozenHir` 与独立 SV 黄金台，把完整 AXI 四外设系统分别降为 direct、FIRRTL→firtool 和 Chisel→JVM→firtool RTL 并实际执行；再用专用 runner 聚合 formal、综合、SemVer、隔离重放和维护文档证据。

## Boundaries & Constraints

**Always:** 遵守 AD-30 单 session/单 finish；地址窗、五源 IRQ、共同同步 reset 和 16×1000 随机预算不降级；固定 Rust1.97.1、firtool1.159.0、Chisel7.15.0、Scala2.13.18、sbt1.10.11；缺工具/超时/UNKNOWN/零计数硬失败；证据保存命令、版本、SHA、退出码、日志和 VCD；设计依赖保持 prelude-only。

**Never:** 不以 129.1 reset probe、emit/编译、旧局部门禁或综合替代系统行为；不增加第二套 IR、假外设、native/generated 层级支持、工具 pin/包版本/公开 API；不把 FR201 核心关闭冒充 Epic130 外部试点、整个 Phase24、FR189 或 NFR91 完成；不 push/publish。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|---|---|---|---|
| 三后端 AXI 系统 | 同一完整 HIR、AXI TB、固定 seed | 各路线 >=16000 随机事务、非零断言/VCD，地址/错误/WSTRB/背压/reset/四外设/IRQ 全覆盖 | 任一路缺工具、marker、计数或波形即失败 |
| direct CSR 第二组合 | 无 bridge 的真实 decoder+四叶 | 保持 129.2 行为与独立连接图 | 不得被三后端 AXI 结果吞并 |
| formal/综合 | 原始系统 RTL 与独立 observer | prove/cover PASS、受控 mutant EXPECTED_FAIL；三路线 Yosys check/stat 有非零 cell | UNKNOWN、timeout、空结果硬失败 |
| 兼容/维护 | 三库 SemVer、API diff、贡献模板/支持矩阵 | 无 silent expand/版本变化；外部行保持 not delivered | registry/tool 缺失不得伪 PASS |

</frozen-after-approval>

## Code Map

- `crates/bitloom/tests/fr198_peripheral_system.rs` -- 复用 `system`、reset wrapper、固定 seed、真实 AXI/direct golden benches；提取共享 emit/run/evidence helper，不降低 129.2 门槛。
- `crates/bitloom/tests/fr197_gpio_api.rs` -- 复用 `composition_backends` 的 FIRRTL emit、firtool、Chisel Scala/sbt、端口 shim 和 VVP 模式。
- `crates/bitloom/tests/fr201_core_matrix.rs` -- 将 ATDD ignored evidence scaffold 替换为可执行三后端验收。
- `crates/bitloom/tests/fr201_core_formal.rs` -- 新系统级 bounded prove/cover、受控 RTL mutation 和三路线原始综合。
- `_agile-output/test-artifacts/129-3-build-runner.py` -- 一命令编排、机器可读 evidence、缺工具负控和严格计数审计。
- `_agile-output/test-artifacts/129-3-isolated-replay.sh` -- HEAD+diff 独立 checkout、独立 Cargo/产物根重放。
- `Justfile` / `.github/workflows/ci.yml` -- 新增 `fr198-fr201-core-check` 与必需 CI job；显式安装锁定工具并上传失败证据。
- `docs/ip/contribution-template.md` / `docs/ip/phase24-support-matrix.md` -- owner、独立 oracle、两组合、后端/formal/综合/兼容/来源许可/证据字段及核心/外部分栏。
- `docs/public-api-1-0-surface.md` / Cargo 版本 -- 只做 diff 审计，不修改，除非实现意外新增公开符号（本规格禁止）。

## Tasks & Acceptance

**Execution:**
- [x] `fr198_peripheral_system.rs`、`fr201_core_matrix.rs` -- 提取共享完整系统生成/bench runner，实际执行 direct/FIRRTL/Chisel，保留 direct CSR 第二组合和固定随机预算。
- [x] `fr201_core_formal.rs` -- 对共同 reset 取消、响应保持、五源 IRQ 接线做有限 prove/cover，运行可证伪 mutation 与三路线 Yosys 原始综合。
- [x] `129-3-build-runner.py`、`129-3-isolated-replay.sh` -- 聚合严格 evidence、工具负控、隔离复现和归档。
- [x] `Justfile`、`ci.yml` -- 接入本地/CI 必需门，配置工具、timeout 和失败产物。
- [x] `contribution-template.md`、`phase24-support-matrix.md` -- 交付贡献合同和诚实状态矩阵。
- [x] 运行 `test-semver-check.sh` 与真实 `semver-check`，保存三库结果并证明 API/版本无变化。
- [x] 更新 Story、Epic129 closeout 草案、Phase24 contract 和 sprint status；最终七步完成后才关闭 FR198/FR201 核心。

**Acceptance Criteria:**
- Given 完整 FR198 系统，when 一命令门运行，then direct/FIRRTL/Chisel 均实际 VVP PASS 且证据计数、波形、工具和 SHA 完整。
- Given formal/综合门，when 原始与 mutant 运行，then prove/cover/综合成功且负控制真实失败，任何缺工具或 UNKNOWN 不可跳过。
- Given CI/兼容/贡献资产，when ATDD gate 与隔离 replay 运行，then 全部变绿，SemVer 无破坏/静默扩面，外部 IP 行仍未交付。

## Implementation Notes

- FIRRTL lowering 发现 direct Verilog 容忍的 `i_raw_events` 未驱动 wire；改为聚合 `irq_events` 显式驱动原实例连接，三后端随后共享同一语义。
- 系统 formal 将四个复杂 leaf blackbox，并显式假设正式 CSR leaf reset/阻塞保持合同；IRQ 使用归纳 prove，reset/response 使用 depth-8 BMC，避免把 bounded 结果冒充全协议证明。
- 隔离 worktree 内 runner 全部 PASS；外层执行器随后对 `/proc` 管道/Yosys history 发出沙箱审计并返回 1，机器 evidence 只记录内部 runner 的实际 exit 0。

## Build Review

- 三个独立 review 线程在读取完整 baseline diff 后长时间无结果，按 workflow reviewer-unavailable fallback 终止；主线程按 Blind Hunter、Edge Case Hunter、Verification Gap 三层清单复审。
- HIGH（已修复）：隔离 replay 曾用临时 evidence 覆盖主 evidence，`source_evidence` 会在 `/tmp` 清理后悬空。现只合并隔离结论，保留主工作区可验证的原始路径，并由 runner 默认实际调用隔离脚本。
- MEDIUM（已修复）：formal negative 仅检查非零和通用 `FAIL`，工具内部失败可能误判。现要求 `Assert failed in Fr198AxiCore`、`DONE (FAIL` 和 counterexample VCD 同时存在。
- MEDIUM（已修复）：runner 可受外部 `BITLOOM_FR201_BACKENDS` 污染并拾取旧 evidence。现清除此变量，并要求每个后端/formal evidence 的 mtime 不早于本轮命令起点。
- 复审后无未解决 HIGH/MEDIUM；限制已写入 build evidence，不把 bounded formal、cells 或核心关闭扩大为全协议/PPA/外部 IP 声明。

## Spec Change Log

## Review Triage Log

## Design Notes

后端对比必须共享同一 HIR 和同一独立 TB。Chisel 生成端口使用现有 `clock/reset/io_*` shim；direct/FIRRTL 保持 `clk/rst/*`。证据聚合不得根据源文本猜测 PASS，只接受实际子进程退出码、marker、计数和产物哈希。

## Verification

**Commands:**
- `python3 _agile-output/test-artifacts/129-3-build-runner.py` -- 三后端、formal/综合、负控、SemVer 和 evidence 全部 PASS。
- `PATH=/tmp/bitloom-maintenance-tools/bin:$PATH _agile-output/test-artifacts/129-3-isolated-replay.sh` -- 独立 checkout PASS。
- `python3 _agile-output/test-artifacts/129-3-atdd-gate.py` 与 `python3 -O ...` -- 均 GREEN。
- `cargo fmt --all -- --check && cargo test --workspace` -- workspace 回归成功；ignored 外部门禁另行实跑，不计普通回归 PASS。
