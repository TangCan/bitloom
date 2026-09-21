# Story127.3 ATDD 固定API与端口

2026-09-21。此为先于产品实施的测试契约；尚无实现。无可变配置，不改旧Axi4LiteSlave。

```rust
use bitloom_prelude::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir};
use bitloom_prelude::ip::AxiLiteCsrBridge;
// public struct AxiLiteCsrBridge;
// impl Elaboratable for AxiLiteCsrBridge {
//   fn elaborate() -> Result<FrozenHir, Diagnostics>;
// }
// impl AxiLiteCsrBridge {
//   pub fn define_module(session: &mut ElaborateSession,
//                        name: impl Into<String>) -> Result<String, Diagnostics>;
// }
```

独立elaborate的电路/top名字`AxiLiteCsrBridge`；define_module返回调用者模块名，固定语义/完整HIR身份，相同名字复用，相同定义体；非捕获fn、一个outer finish。无需derive承诺。

| 方向 | 端口及宽度 |
|---|---|
|输入|clk:Clock, rst:Reset|
|输入|s_axi_awaddr:16, s_axi_awprot:3, s_axi_awvalid:1, s_axi_wdata:32, s_axi_wstrb:4, s_axi_wvalid:1, s_axi_bready:1, s_axi_araddr:16, s_axi_arprot:3, s_axi_arvalid:1, s_axi_rready:1|
|输出|s_axi_awready:1, s_axi_wready:1, s_axi_bvalid:1, s_axi_bresp:2, s_axi_arready:1, s_axi_rvalid:1, s_axi_rdata:32, s_axi_rresp:2|
|输出CSR|csr_req_valid:1, csr_write:1, csr_addr:16, csr_wdata:32, csr_wstrb:4, csr_rsp_ready:1|
|输入CSR|csr_req_ready:1, csr_rsp_valid:1, csr_rdata:32, csr_error:2|

固定Story八AC不变。AW/W/AR各一捕获槽；完整write、read按成功CSR提交轮转，reset后读优先。仲裁offer一经valid提出即锁定payload/type直到req_ready握手或reset。全局执行中只到CSR rsp握手结束；AXI B/R各一响应槽，先预留后提交，另一类别有槽时可前进。无same-edge refill/bypass承诺，测试按事件与有界等待，不复制未定产品FSM时延。

外部5个AWREADY/WREADY/ARREADY/BVALID/RVALID无任何输入组合路径（包括rst）；同步reset沿清状态。CSR输出允许按既有leaf reset屏蔽提交；所有模块共同rst，初次使用必须reset沿，formal明确initial-reset假设。非reset所有被阻valid/payload保持；每个epoch partialAW/W取消独立计数，不当作完整请求。

error00/10/11原样传递，合规peer排除01；失败读返回0由peer合同保证。csr_addr不对齐/不截高位，WSTRB4完全传递；零WSTRB同样提交并响应，leaf决定无副作用/权限错误。PROT两字段各8值忽略，不形成安全域。旧bank行为不移植到新桥。

实际组合使用现有CsrBlock（docs/ip/csr.md）；slave叶error只有00/10，DECERR用独立合规test responder；四窗decoder留127.4。独立bridge单模块可native两引擎，层级native/generated明确unsupported，实际组合RTL必须执行。formal独立reference/causal downstream环境；桥应保证的条件必须assert，不能assume。无限B/R停顿安全，无公平性assumption；有界完成另列wait预算。工具缺失/超时/UNKNOWN必须失败。完整故事是其他AC的权威。
