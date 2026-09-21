# Story127.3 独立代码审查

2026-09-21，full模式，基线9393c261cdfb2ca3699edb959efd1986bbe5275e；diff `/tmp/bitloom-1273-independent-cnv5zz0i.diff`，71文件/13192行。按并发容量先三层再第四层，全部返回后裁决。blind下限min(floor(sqrt(668.279)+1),10)=10，返回10；edge=[]，verification无gap，acceptance无违约，均为有效完成，无失败层。

| ID | 来源 | verdict | 标题 / 位置 | 核实证据 / route |
|---|---|---|---|---|
| R1 | blind-hunter | medium | 失败写响应被不必要地约束rdata=0；`fr196_axi_lite_csr_formal.rs:130` | 当前if(csr_error!=0)不区分owner；合同只要求失败读为0，故排除了合法写错误。改为只约束读owner。 route=patch。 |
| R2 | blind-hunter | medium | 缺成功完成的延迟CSR响应场景；`fr196_axi_lite_csr.rs:365` | peer只要owner存在即valid；现有手工等待执行场景最终reset，缺延迟后正常响应同时捕获新请求的动态验收。 route=patch。 |
| R3 | blind-hunter | medium | reset优先未覆盖同时可握手输入；`fr196_axi_lite_csr.rs:645` | 复位输入从inputs空模板构造，仅rst=1；现有动态场景没有valid/ready同时高的取消沿。补合法状态下同步reset与各类握手竞争。 route=patch。 |
| R4 | blind-hunter | medium | reset阶段6和7重复；`fr196_axi_lite_csr.rs:608` | 所有stage分支条件在6/7取值完全相同，无额外条件区分。将末阶段改为明确断言不同占用组合。 route=patch。 |
| R5 | blind-hunter | medium | 真实组合SV刺激未接协议保持监测器；`fr196_axi_lite_csr/integration.rs:199` | 该testbench独立编写，tick只记事务/副作用，没有使用Rust Monitor；增加AW/W/AR及CSR响应跨背压保持检查。 route=patch。 |
| R6 | blind-hunter | medium | 排空即停止缺末端重复响应观察；`fr196_axi_lite_csr.rs:384` | drain在score.empty立即return，最后一个响应后不再采样，尾部重复可避开动态oracle。补有界空闲尾段并继续记账。 route=patch。 |
| R7 | blind-hunter | medium | scoreboard允许CSR响应与新提交同沿；`fr196_axi_lite_csr.rs:242` | 先owner.take再检查is_none可容纳同沿consume+submit，弱于当前保守时序文档；按沿前owner或显式禁止该组合，增加负例。 route=patch。 |
| R8 | blind-hunter | low | seed耗时字段未明确逐seed与累计；`fr196_axi_lite_csr.rs:675` | 计时起点在seed循环外，docs已披露累计所以并非伪造时长；原日志仅elapsed标签仍不便逐seed比较。直接补两种明确时间字段。 route=patch。 |
| R9 | blind-hunter | medium | 结构检测未核对FF时钟源；`fr196_axi_lite_csr_formal.rs:registered` | ff分支不读取C连接便截断依赖，现有fixture甚至无C；可把由数据输入驱动时钟的FF判为合格。核对专用clk并加入错误/缺时钟负例。 route=patch。 |
| R10 | blind-hunter | medium | monitor负例未覆盖ready升高的接受沿；`fr196_axi_lite_csr.rs:754` | 所有撤valid/改payload负例ready=false，未验证保持义务延续到接受沿。增加ready=true坏例及保持payload的合法接受正例。 route=patch。 |

10项不同根因，分别保留；0 decision-needed / 10 patch / 0 defer / 0 rejected。这是验证面补强，无确认的产品RTL缺陷。用户七步连续授权覆盖全部修补，不重新询问常规批准。修补中；尚未完成本轮review，也不提前done/commit。

## 修补验收（2026-09-21）

R1–R10全部完成，无decision-needed、defer或rejected。主代理逐项读修补diff，并检查实际ordinary日志：12行为+1结构=13 passed、0 failed、3 ignored；16seed×1000逐项完成。专用3 passed、0 failed、0 ignored，prove8/cover64 PASS；port-only控制PASS及三个mutation FAIL/2，原始综合150FF/5边界通过。原始工具status和VCD门禁已核验，详见[修补证据](127-3-independent-fix-evidence.md)。

新源6/工具405成员经主代理逐成员bytes/SHA256回读，源码live与snapshot一致，见`127-3-independent-root-archive-verification.json`。没有未解决high/medium项。on_complete解析为空。按用户七步顺序，review完成但story仍review，不能提前done：下一步automate，随后实际clean/fmt/justtest与单故事commit。
