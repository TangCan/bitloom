# Story127.4 build 实测证据

状态：实现与本文件列明的build验证完成；独立review、automate、全workspace clean/fmt/regression和单故事commit仍由root后续执行。本文件不将127.4/M2标done，不push/publish，不交付Epic128–130/Phase24整体；FR189 deferred/NFR91保持。

实现新增固定52端口CsrDecoder与共享无捕获定义体；旧bank/CSR/桥、工具钉和包版本未改。完整契约、原文例、FR142逐符号/minor与支持矩阵见[译码文档](../../docs/ip/csr-decoder.md)。测试先激活初始ATDD，再补全API诊断、双实例RTL、真实四叶组合、独立记账/负例、无公平性形式及严格综合，不用9个原始入口缩减合同。

## 实际命令批次

工具环境前置维护工具与固定SBY路径，CARGO_PROFILE_TEST_OPT_LEVEL=1、BITLOOM_REQUIRE_RTL=1、PYTHONDONTWRITEBYTECODE=1；每批JSON记录完整argv、cwd、实际开始UTC、墙钟、退出码与日志SHA。工具路径/实际版本/可执行内容SHA见[工具元数据](127-4-build-tools.json)。

| 批次 | 墙钟 | 退出 | 实际结果 | 原始记录 |
|---|---|---|---|---|
| ordinary | 43.012s | 0 | 8 passed/0 ignored, 1 passed/4 ignored, 4 passed/0 ignored | [日志](127-4-build-ordinary.log) / [命令与元数据](127-4-build-ordinary.json) |
| integration-final | 31.332s | 0 | 5 passed/0 ignored | [日志](127-4-build-integration-final.log) / [命令与元数据](127-4-build-integration-final.json) |
| dual-final | 0.962s | 0 | 1 passed/0 ignored | [日志](127-4-build-dual-final.log) / [命令与元数据](127-4-build-dual-final.json) |
| formal | 10.071s | 0 | 4 passed/0 ignored | [日志](127-4-build-formal.log) / [命令与元数据](127-4-build-formal.json) |
| compat | 595.629s | 0 | 12 passed/0 ignored, 5 passed/0 ignored, 31 passed/0 ignored, 9 passed/0 ignored, 13 passed/0 ignored, 12 passed/0 ignored, 9 passed/0 ignored | [日志](127-4-build-compat.log) / [命令与元数据](127-4-build-compat.json) |
| compat-formal | 38.059s | 0 | 3 passed/0 ignored, 4 passed/0 ignored | [日志](127-4-build-compat-formal.log) / [命令与元数据](127-4-build-compat-formal.json) |
| example | 0.876s | 0 | 原文编译执行PASS | [日志](127-4-build-example.log) / [命令与元数据](127-4-build-example.json) |
| compat-example | 1.160s | 0 | 原文编译执行PASS | [日志](127-4-build-compat-example.log) / [命令与元数据](127-4-build-compat-example.json) |
| csr-example | 0.525s | 0 | 原文编译执行PASS | [日志](127-4-build-csr-example.log) / [命令与元数据](127-4-build-csr-example.json) |
| bridge-example | 0.426s | 0 | 原文编译执行PASS | [日志](127-4-build-bridge-example.log) / [命令与元数据](127-4-build-bridge-example.json) |

ordinary是当时13个普通入口通过、4个专用ignored未执行；此后补强集成和双实例的结果分别为integration-final/dual-final，不把旧批次当新源码证明。最终集成5入口包含新增七模块native两引擎与GeneratedFunctional的明确unsupported检查；预期catch_unwind诊断不算产品失败。最终唯一普通入口共14个（native/API/双实例8、集成/软件/monitor/unsupported5、结构负例1），不将重跑次数相加。专用4入口已另跑，ignored不是PASS。

## 实际矩阵与边界

- 两种native引擎分别对实际decoder组合输出遍历全部65536字节地址，独立literal-range黄金校验onehot/完整高位/local低位；另测31拍请求与叶等待、31拍响应背压、改变已提交上游/非owner输入、DECERR持久/消费、reset取消。精确52端口/同源HIR、定义复用、非法名/冲突poison、宽度/方向诊断通过。
- 双实例真实RTL共享一个Clock/Reset，A消费不影响B持久DECERR，随后共同reset取消两实例待响应。初版两个Clock被正确拒绝，没有扩大多时钟架构。
- 七模块真实RTL采用正式20个寄存器布局（UART6/GPIO6/Timer4/IRQ4）、各自真实CsrBlock；External RW/RO value及事件/拒绝是测试peer，不是外设算法。所有叶read/write提交脉冲逐沿由独立权限/掩码oracle核对；外部RW和W1C状态每沿与独立模型核对。
- 定向覆盖四窗local独立、边界/hole/高alias、全部16WSTRB与互补值、RO零WSTRB错误/WO读错误、WO无有效byte不push、W1C零mask、保留位、动态拒绝期间自然事件、W1C同沿set胜clear；RX快照在AR接受后从35改59，在CSR提交后改a7，必须返回提交时59。
- 原始AW/W同拍与间隔0/1/7/31、W先/AW先；reset后同时合格先读；partial AW/partial W分别等待时实际完成新读；B停时完成新读、R停时完成新写；合法producer涵盖接受沿保持。monitor control PASS，AW/W/AR改变、AW接受沿改变与同沿CSR consume/refill均被实际RTL观察器拒绝。
- 每seed独立完成1000随机事务，再128写+128读独立AW/W/AR producer并发。随机阶段单独清coverage并断言四窗/未命中、16strobes、00/10/11错误、0/1/7/31写gap和先后均命中；并发另断言三路offer同时活动与AW/W/AR/B/R各自实际stall，不能用定向历史填覆盖。
- 共同reset覆盖仅AW、仅W与迟到另一半、AR捕获、offer未提交、owner/DECERR提交待响应、B/R受阻且ready同时为1及与新接受竞争；取消沿不计传输，恢复访问另一窗；排空后9/31拍尾部观察。最终守恒逐项断言acceptedAW/W=writeCommit+各自cancel、acceptedAR=readCommit+cancelAR、CSRcommit=CSRresponse+cancelCSR、CSRresponse=B/Rconsume+cancelB/R。每seed取消token计数AW2/W2/AR2/CSR2/B1/R1；不是10笔完整请求的宣称。

逐seed完成数、命中分布、墙钟、退出码和原始日志索引见[种子汇总](127-4-build-seeds.json)：合计16000随机完成事务、4096并发完成事务，额外定向序列不计入这两个预算。

## 独立形式与综合

最终decoder prove depth8：basecase PASS、induction PASS，不是仅8拍BMC。初始reset、合法上游保持及因果/稳定的叶响应是环境假设；无叶响应等待上界、无rsp_ready公平性，失败写无关rdata不限。ghost↔实现状态辅助关系为assert并一起证明，没有把DUT正确性写成assume。

cover depth8独立运行：四窗提交与ffff miss在step2可达，同笔响应背压恢复和reset取消后新请求在step4可达，共7项；f_stalled在消费清除，不由历史事务代替当前恢复。可达性不等于无条件活性。

mutation使用**移除内部对应引理**的port-only observer，BMC depth10：control PASS，cross-owner/dual-select/high-alias均实际FAIL、exit2并保存Assert failed日志与VCD反例。原版prove PASS与mutant反例分列。

原始decoder及七模块组合无observer运行Yosys synth/check；严格cell白名单拒绝latch/unknown，FF时钟必须专用clk。组合305个FF，桥AWREADY/WREADY/ARREADY/BVALID/RVALID五个输出均无输入（含rst）组合路径。结构观察器正例与错误时钟/缺clock/latch/伪FF/未知cell/未驱动/未知常量/输入直达负例实际执行。数量仅此次工具网表观测，不作PPA承诺。

## 兼容、软件和证据保存

FR193旧bank、FR194模块组合、FR195切片/FIFO、FR196 CSR配置/行为与桥普通验证为本轮实跑；CSR与桥专用形式/综合也为本轮重跑。M0/M1及127.2/3历史关闭不改，不借旧通过替代本轮。C11四头共同include、所有正式local offset/mask及手写global base/末端黄金、重复C头和Markdown逐字节一致均实跑；四份prelude-only原文例编译执行。

[初始失败索引](127-4-build-initial-failures.md)明确保留首次prove UNKNOWN、初始SV语法失败和dual Clock诊断；不将缺工具、UNKNOWN、timeout或预期失败冒称PASS。[归档manifest](127-4-build-archive.json)列每成员SHA及回读验证，[源码与工具产物归档](127-4-build-artifacts.tar.gz)位于target外可经受clean。所有decoder尝试完整保存；本轮兼容归档只收当前命令修改的文本源码/日志/网表/轨迹，排除编译二进制与嵌套Cargo target（不是删除原始证据）。工具版本与可执行SHA另存。

## 后续主流程

[M2映射草稿](../implementation-artifacts/epic-127-closeout.md)逐项区分历史与本轮证据。root仍须独立build review/code-review、automate、实际cargo clean/cargo fmt --all/just test及必要专用复验，然后一个Story127.4提交和最终状态更新。此实现代理没有执行clean、commit、push或发布，也没有改story/sprint/goal。
