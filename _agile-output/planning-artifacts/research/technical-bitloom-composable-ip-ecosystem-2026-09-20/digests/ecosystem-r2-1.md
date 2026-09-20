# 第二轮：一个外部 RTL 试点与备选

访问：2026-09-20。未读本地项目。本轮 3 次 web 调用，尝试 8 个独立 URL（7 个成功、1 个 404）；保守将失败也计入 8 来源上限。本轮为最后一轮，停止理由 round cap；没有运行或接入 IP。

**建议首选方向：PULP common_cells FIFO，优先 v1.40.0 稳定发布的 `fifo_v3`（必须先做 tag 源码与完整 SHA 准入核对）；本轮实际审查的仅为 master `cc_fifo`，不能替代稳定版证据。备选：Taxi `taxi_axis_register`，用于随后验证 SystemVerilog interface flatten 和 AXI-stream 侧带。** 选择是有条件的试点建议，不是质量、性能或许可优劣排名。两者均未验证与 Bitloom 兼容。

## 来源表

所有来源 accessed=2026-09-20。源代码的版权年不是发布日期。各 claim 的 publisher/pub_date/URL 继承此表。

| ID | URL | publisher | pub_date | class |
|---|---|---|---|---|
| C1 | https://raw.githubusercontent.com/pulp-platform/common_cells/master/Bender.yml | PULP Platform | 未知 | primary-manifest |
| C2 | https://raw.githubusercontent.com/pulp-platform/common_cells/master/LICENSE | PULP Platform | 未知 | primary-license |
| C3 | https://raw.githubusercontent.com/pulp-platform/common_cells/master/src/cc_fifo.sv | PULP Platform | 未知 | primary-code |
| C4 | https://raw.githubusercontent.com/pulp-platform/common_cells/master/test/cc_fifo_tb.sv | PULP Platform | 未知 | primary-test-code |
| C5 | https://github.com/pulp-platform/common_cells/releases | PULP Platform | 多事件，见时间线 | upstream-release-notes |
| T1 | https://raw.githubusercontent.com/fpganinja/taxi/master/src/axis/rtl/taxi_axis_register.sv | FPGA Ninja | 未知 | primary-code |
| T2 | https://raw.githubusercontent.com/fpganinja/taxi/master/src/axis/tb/taxi_axis_register/test_taxi_axis_register.py | FPGA Ninja | 未知 | primary-test-code |
| F1 | https://raw.githubusercontent.com/pulp-platform/common_cells/master/src/fifo_v3.sv | PULP Platform URL | 不适用 | 本轮请求返回 404，不作为产品失败证据 |

继承同次研究第一轮已读证据：`ecosystem-r1-1.md` S7 的 Taxi LICENSE URL 为 https://raw.githubusercontent.com/fpganinja/taxi/master/LICENSE ，publisher FPGA Ninja（载明 CERN 文本），pub_date 未知，accessed=2026-09-20。该来源不在第二轮重复打开。

## 源码与测试事实

| Claim | source | confidence | 支持摘意与边界 |
|---|---|---|---|
| P1：`cc_fifo` 有可收窄成位向量的离散端口 | C3 | high：源码观察 | 参数 FallThrough、DataWidth、Depth、data_t；UsageWidth 为 cc_pkg 派生 localparam。端口 clk_i/rst_ni/clr_i/flush_i、full_o/empty_o/usage_o、data_i/push_i/data_o/pop_i；rst_ni 注释为异步低有效。没有 interface modport。 |
| P2：它不是独立单文件 | C3,C1 | high：源码观察 | include `common_cells/assertions.svh`、`common_cells/registers.svh`；引用 cc_pkg::cnt_width/idx_width。Bender 导出 include，声明 common_verification version 0.2.0 与 tech_cells_generic version 0.2.14。这些是版本约束，不能当 resolved commit；宏/包递归依赖尚未全部读取。 |
| P3：存在参数化上游 FIFO 测试，但不是本轮通过证据 | C4 | high：测试源码观察 | `test/cc_fifo_tb.sv` 构造 Depth 1/8/9 × FallThrough 0/1；默认 DataWidth=8、NumChecks=100000，以 queue 比较输出。依赖 rand_verif_pkg::rand_wait 和 clk_rst_gen；有随机化 class。clr_i 固定 0，usage_o 未连接，不能把它称作 clear/status 全覆盖。 |
| P4：必须审查边界与工具分支 | C3,C4 | high：代码观察；行为未验证 | RTL 内有 Depth==0 分支，同时 assertions 未关闭时要求 Depth>0，试点不选 0。TB 中 fall-through property 被 `ifndef VERILATOR` 包裹。未依据这些代码推导某版本工具必失败，亦未独立复现。 |
| P5：当前模块/许可证可直接定位 | C2,C3 | high：标识观察 | C3 文件头指定 Solderpad Hardware License Version 0.51；C2 标题原文 “SOLDERPAD HARDWARE LICENSE version 0.51”。本轮不推导全部依赖许可；若需要 SPDX 机器标识，须另核 SPDX 对应项，不能把标题擅改成 Apache-2.0。 |
| T3：Taxi register 的外部端口是接口类型 | T1 | high：源码观察 | 参数 REG_TYPE 默认 2，注释 0=bypass、1=simple buffer、2=skid；clk/rst 与 `taxi_axis_if.snk s_axis`、`taxi_axis_if.src m_axis`。宽度及 KEEP/STRB/LAST/ID/DEST/USER 从接口参数取。源码存在 DATA_W/KEEP_W 一致性检查，不能推导全侧带组合均支持。 |
| T4：Taxi 有具体自动测试入口和矩阵 | T2 | high：测试源码观察 | pytest 枚举 reg_type 0/1/2 × data_w 8/16/32；调用 cocotb_test.simulator、simulator="verilator"。源列表有测试 SV wrapper、register.sv、taxi_axis_if.sv；测试有 idle/backpressure、随机长度、payload/id/dest/user 检查。Python/模拟器精确版本及实际通过结果未取证。 |
| T5：Taxi RTL 与测试声明相同 SPDX | T1,T2；r1 S7 | high：许可证标识 | 两文件首部为 CERN-OHL-S-2.0。第一轮读取 LICENSE 标题原文为 “CERN Open Hardware Licence Version 2 - Strongly Reciprocal”。不提供法律效果结论，不由这两个文件概括全部依赖。 |

## 有日期的维护事件与版本陷阱

来自 C5（confidence=high，class=官方发布事件；修复效果仅是上游说法，未独立验证）：

- **2026-07-02，v2.0.0-beta**：发布说明把 `fifo_v3` 改名 `cc_fifo`，加入 `clr_i`、移除 `testmode_i`，增加旧名称 wrapper。表明当前 master 与旧调用表面不同。
- **2026-07-16，v2.0.0-beta.2**：发布说明提及 `cc_fifo` 等同步 clear 信号调整；所称 Synopsys Fusion Compiler 兼容修复无不同 publisher/运行证据，效果 unverified。
- **2026-07-29，v2.0.0-beta.3**：有正式日期的预发布事件；页面短 commit 为 `63b7c50`。这不是完整 immutable pin。
- 同页 **2026-07-02 v1.40.0** 标注 Latest；因此不能把已读 master 的 `cc_fifo` API 或 beta.3 内容归到 v1.40.0。

上述事件在访问前六个月内，支持近期维护活动，不能证明成熟度。Taxi 本轮没有读取 dated commit/release 序列，时间线仍缺证；版权年份和 repository updated 均不替代。没有使用 star 数。

## 明确、有限的试点建议

首选 common_cells FIFO 方向的理由（inference）：已读新版本可固定 data_t 为默认 logic 位向量，端口独立且现有测试有小深度/非二次幂组合，适合先验证“依赖获取 → wrapper → 编译 → 行为测试”的外部 IP 基础路径。**候选行为范围为8位数据、深度8、关闭 fall-through，后续增加深度1/9及 fall-through；稳定版参数拼写、clear/flush/status暴露与测试范围必须由其自身源码确认。** 本轮的 DataWidth/Depth/FallThrough 拼写与 clr_i 仅适用于已读 master `cc_fifo`，不是 stable `fifo_v3` 合同。

版本落点建议：优先解析 v1.40.0 的完整 commit，在该 commit 读取 `fifo_v3` 实际路径、接口、依赖与测试；本轮未读该 tag 源码，准入尚未完成。只有明确需要新 API 才考虑 v2.0.0-beta.3，且要标明实验性质并重新核对 pin 内容。不得用短 hash、浮动 master 或 release tag 单独当作完整交付锁。当前研究未拿到完整 commit/归档 checksum，不能填写已锁定。

master 新版本的 closure 起点：`src/cc_fifo.sv` + `src/cc_pkg.sv` + `include/common_cells/assertions.svh` + `include/common_cells/registers.svh` 及递归包含；不适用于未经读取的 v1.40.0。这里只是从源码引用发现的起点，后面三个文件未读取，不称闭包完整；采用上游 Bender 时还要锁其依赖，不能默默删除 manifest 依赖。上游 TB 可作为首个对照，但已读新版本还需补 clr_i、usage_o、reset、full/empty 与 flush 同周期交互测试。

备选 `taxi_axis_register` 的理由（inference）：已有明确 Verilator/Cocotb 参数矩阵，可用来练习 stream 侧带与背压验证。额外工作是读取 `src/axis/rtl/taxi_axis_if.sv`、测试 SV wrapper 并定义离散端口映射；当前 T2 列出它们不代表已经读完或验证 interface flatten。不得将 FIFO 试点通过外推为 Taxi 通过。

## 尚不能确认及结束条件

- 两个候选：未获取完整 immutable commits/checksums；未审计全部许可证/NOTICE、递归依赖；未读 CI 运行结果；未编译、仿真、综合或验证离线重建。
- 未证明任何 Bitloom × 模拟器/综合器/上游版本组合兼容；也未提供性能数字。单上游测试代码和发布声明不满足跨 publisher 的双来源兼容/性能/失败结论规则。
- common_cells beta 的 API 稳定性不能由稳定分支 Latest 标签替代；Taxi 维护时间线未补齐。
- FuseSoC stable 与其精确 lock 边界本轮未补读。第一轮 latest schema 观察仍只适用于所读文档，不转写成 stable 保证。
- 到达来源上限停止；下一步若获准实际试点，应首先完成 pin/闭包核对，然后构建可重复测试产物，而不是继续增加 IP catalog。
