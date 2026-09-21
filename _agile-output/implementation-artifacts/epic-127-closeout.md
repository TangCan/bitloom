# Epic127 / FR196 / M2 验收映射（Story127.4 最终关闭）

状态：**Epic127 / FR196 / M2 完成，随Story127.4单独本地提交关闭**。127.1风险门、127.2 CSR、127.3桥是已完成历史；127.4独立review、automate、实际clean/fmt/workspace和专用验证全部通过。最终470结果块、1823 passed / 0 failed / 25 ignored；ignored单独说明，未算PASS。完整证据见[最终验收](../test-artifacts/127-4-final-verification.md)、[自动化汇总](../test-artifacts/127-4-automation-summary.md)、[独立审查及重验](../test-artifacts/127-4-code-review-triage.json)。

| FR196能力 | 产品与本轮验证 | 明确历史依据 |
|---|---|---|
| 静态CSR描述/同源RTL与Markdown/C头 | 四个正式布局真实CsrBlock，独立局部offset/mask/global base黄金，C11共同include、重复生成；本轮CSR普通及专用回归 | [127.2最终验证](../test-artifacts/127-2-final-verification.md) |
| RW/RO/WO/W1C、唯一状态owner、动态拒绝、快照与副作用 | 七模块真实RTL独立AXI接受/CSR提交/CSR响应/AXI消费/取消记账，全部16WSTRB、保留位、同沿set胜clear、自然事件、读提交快照；不是Epic128外设算法 | 127.2 CSR语义，127.3真实leaf联验 |
| 独立AW/W/AR捕获、响应容量、轮转/锁定offer | 本轮重跑完整桥native/真实RTL/形式；组合partial AW和partial W不挡读，B堵时实际完成新读、R堵时实际完成新写；16seed各128写+128读独立通道并发 | [127.3最终验证](../test-artifacts/127-3-final-verification.md) |
| 固定四窗与唯一提交owner | 新CsrDecoder，52端口、共享无捕获body、完整16位匹配/低位保持、同沿叶提交、单在途、DECERR持久响应；两native×65536地址、双实例真实RTL、实例连接诊断 | 新增127.4，不借历史bank证明 |
| reset取消、背压与气泡 | 共同同步reset，独立decoder长等待/owner/DECERR取消、真实桥各槽阶段取消与恢复、尾部重复观察；非法producer及consume/refill负例实际拒绝 | 127.3桥取消合同沿用 |
| 独立形式与原始综合 | decoder prove基例+归纳、cover分别PASS；port-only mutant反例；原始decoder/七模块Yosys synth/check、FF专用clk和桥5输出依赖锥；工具失败保留 | 本轮同时重跑旧CSR/桥专用证明，不将ignored计PASS |
| 模块组合与软件接口 | [七模块prelude-only原文例](../../docs/ip/csr-decoder.md)编译运行；逐符号FR142/minor；无包版本或工具钉变化 | AD-30、M1/FR194；本轮适用FR193/194/195回归 |

本轮新增长延迟回放4seed×128完成/各9取消，两native与实际单模块RTL一致；四个WO control和八个实际RTL输出突变均按预期判定。形式17cover、6个port-only反例，七模块256次双类有资格轮转决策已核对。

以上证据区分历史完成与本轮实跑；具体命令、退出码、种子和源码/工具SHA以build日志和归档索引为准。总线/CSR夹具只提供明确标识的value/event/reject测试peer，不交付UART串行、GPIO同步、Timer计数或IRQ汇聚。native/generated层级仍明确unsupported；真实RTL才是层级行为证据。

关闭限制保持：Epic128–130 / FR197–201、整个Phase24未交付；FR189/Epic122 deferred，NFR91未清空；不push、不publish。Bitloom与samitbasu/rhdl无关。
