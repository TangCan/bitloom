# Story127.3 独立审查修补 R1–R10

2026-09-21。仅三个桥测试文件有代码改动；产品RTL/API、story/sprint/goal/spec均未修改，文档由根代理维护。本记录不宣称完成workspace回归或故事关闭，不clean/commit/push。此前build/review-fix/review-root证据不覆盖，现有4个带标准archive字段的旧归档SHA已重新核对，见`127-3-independent-fix-prior-archive-check.json`。

| ID | 实施与可检查证据 |
|---|---|
| R1 | formal原来`if(csr_error!=0) assume(csr_rdata==0)`改为`if(!f_owner && csr_error!=0) assume(csr_rdata==0)`，仅失败读约束0；合法失败写允许任意数据。无其他环境假设增加。实际prove/cover与变异全部重跑。 |
| R2 | 新独立定向测试分别用读/写做首个owner，已提交后接受下一AW/W/AR，在31拍无CSR响应期间逐拍断言owner不变、提交数1/消费数0且无新CSR请求；恢复响应后3事务精确完成。Interpreter/Compiled×读/写四种实际RTL均通过。 |
| R3 | 八个reset阶段均在复位沿同时驱动AW/W/AR valid、B/R ready、CSR req_ready；存在owner时同时给因果CSR rsp_valid。断言AW/W/AR接受、提交、CSR消费、B/R消费七计数与沿前相同，之后8拍观察无旧请求重放并恢复新事务。 |
| R4 | stage6明确断言(AW,W,AR,B,R)=(1,1,1,1,1)、owner=None。stage7只释放B，再提交新写、保留旧受阻R和AR，再捕获新partialAW，断言(1,0,1,0,1)、owner.write=true。两引擎都打印命中，绝非重复分支改名。 |
| R5 | 真实bridge+CsrBlock SV `tick`加入AW/W/AR/CSRrsp跨背压valid/payload保持监测，保持义务包含最终接受沿；同步reset清监测历史。原始真实leaf序列重新执行通过。 |
| R6 | `drain()`检测空后不立即返回，而继续8拍双方AXI ready的idle，全部经过相同Score/Monitor，再断言仍空并核对账本。此后出现的重复响应会触发unsolicited断言；这是有界尾段，不声称动态测试覆盖任意未来时刻。 |
| R7 | Score在处理response前显式拒绝同沿CSR响应消费+新请求提交；新增负例构造旧读owner及下一完整写，必须匹配该精确错误。正例把相同两事件分成两个沿，合法通过。 |
| R8 | 每seed另设seed_start，日志明确打印`seed_elapsed`和`cumulative_elapsed`，两者都包含本seed实际RTL运行。完整16×1000重跑，结果单独JSON保存。 |
| R9 | 原始综合网表检查每个FF的C必须恰好连接专用单bit input clk，且C方向input；不再对任意FF盲目截断依赖。fixture补真实clk/C；错误clock=数据输入和缺C连接负例分别必须命中精确诊断，正确fixture仍通过。实际原始RTL综合/边界重新通过。 |
| R10 | 七通道monitor分别覆盖ready=false/true×撤valid/改payload四坏例，另验证保持payload的合法接受沿及接受后撤valid合法；reset取消正例保留。 |

## 执行范围与命令

仅执行编辑文件所覆盖的桥test targets。统一环境`PATH=/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin:$PATH CARGO_PROFILE_TEST_OPT_LEVEL=1 BITLOOM_REQUIRE_RTL=1`：

```text
cargo test -p bitloom --test fr196_axi_lite_csr --test fr196_axi_lite_csr_formal -- --nocapture
cargo test -p bitloom --test fr196_axi_lite_csr_formal -- --ignored --nocapture
cargo test -p bitloom --test fr196_axi_lite_csr_formal p1_structural -- --nocapture
```

普通结果在`127-3-independent-fix-ordinary.log`，专用结果在`127-3-independent-fix-formal.log`，提前单独核验R9的结果在`127-3-independent-fix-structural.log`。不是以普通ignored替代专用门禁。rustfmt --edition2024 --check和git diff --check仅作用三个编辑文件。root在返回后执行完整验收。

当前入口总数16：12行为+4形式/结构；普通13，专用ignored3。本轮新增延迟CSR完成和同沿handoff scoreboard负例两个入口，其余是原有入口补强。

## 已完成的专用与定向结果

- 专用formal/综合/变异目标exit0，3 passed/0 failed，Rust runner25.25s。
- `proof-1588087`：prove depth8完整basecase+induction PASS（无界安全）；cover64五项PASS。CSR任意响应延迟，B/R没有公平性假设。R1只放宽失败写数据值，不限制DUT义务。
- `mutations-1588087`：原始control port-only BMC10 PASS/exit0，配对/路由/offer保持三个RTL变异均有assertion counterexample、FAIL/exit2和VCD；无实现对应引理参与变异kill，不将ERROR/UNKNOWN/timeout算成功。BMC敏感性检查与独立无界proof不混为一谈。
- `synthesis-1588087`：无observer原始RTL综合/check PASS，150FF，5输出无输入组合路径；每个FF另实际核对专用clk。
- 单独R9结构入口exit0，1 passed，0.00s。日志中错误clock断言`left Some(1), right Some(3)`、`FF missing C connection`为被捕获且核对的预期负例；并非测试失败。
- 延迟正常完成四个case全部native+RTL PASS；reset阶段6/7两引擎命中不同占用并实际RTL PASS；真实CSR组合加SV monitor后仍177提交/176消费/B110/R64，差额来自既有明确reset取消；同沿Score负例与合法分沿正例通过；monitor接受沿正负例通过。

没有新确认的产品缺陷。本轮预期负例panic均被catch且断言内容；出现意外失败时须在此补记并保留原日志，不能将其改写为PASS。完整随机/普通最终结果见末尾补记。

## 持久证据

新源码/工具包统一`127-3-independent-fix-{source-snapshot,tool-artifacts}.tar.gz`，manifest记录包SHA及每成员SHA，并从压缩包回读核验。源包包含三个测试、共享RTL runner、未改产品作为上下文及根代理当前桥文档；hash是明确snapshot，不保证后续live永远相同。工具包保留本轮生成RTL/测试向量、工具版本/命令/状态JSON、证明/cover、变异反例与综合JSON；仅省略可重建Icarus simulation可执行文件。旧归档原字节保留，target清理需等待本包完成。

## 普通/随机最终结果

普通两个目标最终 **exit0，13 passed、0 failed、3 ignored**（12行为+1结构）；Rust runner分别396.05s与0.00s。专用3项已另跑exit0，不将ignored算PASS。16seed每个1000事务全部完成，总7955写、8045读、16000提交/消费，8拍尾段每事务实际执行；每seed真实耗时与累计耗时、完整账本、write-only gap/WSTRB及AW-first/W-first/B/R stall覆盖见`127-3-independent-fix-results.json`。本轮没有非预期失败；负例catch与RTL mutants FAIL2均是明确要求的预期结果。

归档已完成并逐成员回读：source6成员/29,240字节，tool405成员/27,903,641字节，共411成员SHA256核对成功；manifest同时记录压缩包整体SHA。所有本轮测试/归档进程均已退出，没有未完成的后台工具。可交根代理验收并继续automate/最终回归。
