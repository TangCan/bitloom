# Story 129.1 只读准备：完整系统与核心证据 NFR14

日期2026-09-21；给主代理的研究输入，不是 accepted 风险门、不创建 Story、不表示 FR198/FR201 已交付。本轮只读仓库，未执行测试/工具版本探针，唯一写入为本文件。128.5尚由主代理完成最终回归与提交，不能提前当作 done。

## 权威与依赖

完整阅读 Epic129（三故事）、Phase24共用完成条件/风险模板、正式 `docs/ip/phase24-contract.md`、通用 NFR14 模板、研究 `technical-bitloom-composable-ip-ecosystem-2026-09-20/implementation-plan.md`（尤其§5）、现行桥/译码/Timer/IRQ/GPIO/UART接口文档、相关公开源码、Justfile 与 CI 相关门。架构核对 AD1–4/6–19/22–29与Stack；不声称完整重读历史架构关闭段。

权威顺序：正式合同/正式 Epic高于研究拟定；GPIO32与IRQ5已经取代草案GPIO8/最多8路，没有额外外部IRQ针脚。历史文档中的 M0-only/backlog 叙述保留为历史，整体七步授权有效。风险记录(a)上游限制、(b)估算、(c)禁止降级、(d)owner缺一不可；未解决架构/工具阻塞不能把功能故事ready。

129.1自身前置125.3/M0 done。129.2要求129.1、127.4、128.2、128.4、128.5 done；129.3要求129.2。实际系统还必须使用已交付128.3 IRQ。129.3才关闭Epic129/M4及FR198、FR201核心子集；130.3未完成时绝不宣称FR201全部或Phase24全部交付。FR189 deferred/NFR91保持，不升钉，不push/publish。

## 源码已经可用的连接面

`crates/bitloom-prelude/src/ip/axi_lite_csr.rs`、`csr_decoder.rs`、`timer.rs`、`irq.rs`、`gpio/csr.rs`、`uart_csr.rs`各有 `define_module(&mut ElaborateSession, impl Into<String>)->Result<String,Diagnostics>`，都支持调用者同session、最后一次finish。四外设各自调用私有CsrBlock定义；UART还使用同一8×4 FIFO定义的两实例。不能复制register bank伪造算法，也不能拼已freeze的多个HIR。

桥31端口：clk/rst +19个AXI针脚 +10个CSR针脚。AXI数据32、地址16、WSTRB4、AWPROT/ARPROT3（保留忽略）。桥端CSR名称带`csr_`，decoder公共CSR去此前缀；decoder52端口，四叶前缀严格`uart/gpio/timer/irq`。叶共享12端口：clk/rst及请求/响应10个信号。

额外真实外设针脚：Timer.match_event:1；Irq.raw_events:5输入与irq:1输出；GpioCsr.pad_in:32输入、pad_out:32/pad_oe:32/raw_event:1输出；UartCsr.rx:1输入、tx:1/raw_events:4输出。旧Gpio/UartTx/UartRx不是这些wrapper替代物。

## 129.1应冻结的系统与两种组合

建议采用两个有实质拓扑差别、使用原始产品模块的prelude-only设计例，而非换名的同一图：

A. 完整AXI外设子系统：独立AXI主端→AxiLiteCsrBridge→CsrDecoder→UartCsr/GpioCsr/Timer/Irq。外部串行源/解码器、GPIO针脚驱动与irq观察均经公开顶层针脚；逻辑图含真实CSR/FIFO子层。此图承担FR198。
B. 直接CSR外设子系统：独立CSR主端→相同CsrDecoder→相同四外设；不含AXI桥，复用相同描述/窗口/IRQ映射，可另做真实TX→RX连线回环。说明它面向已有CSR主端的集成，不冒充第二个AXI主端或多主互联。此图承担第二种复用实例/FR201核心证据。

第二图是本报告建议，需129.1明确选定；不能在后续只给原理图而不真实执行。两图均一次session/finish、设计依赖仅prelude；公开库是否新增完整system helper应在129.1定案。优先例子自身Elaboratable，不为拼接额外扩大稳定库面；若增加API必须逐符号FR142与minor记录。例子保持与samitbasu/rhdl无关声明。

地址固定四窗0000–00ff UART、0100–01ff GPIO、0200–02ff Timer、0300–03ff IRQ，0400–ffff DECERR。各寄存器offset/mask/reset见正式合同；软件地址=base+local，不改既有C头宏语义。洞/未对齐/RO写/WO读SLVERR，失败读0；所有16位译码，不允许8104别名0104。数据little-endian，全部16种WSTRB包括零，保留位忽略与错误优先保持。

IRQ必须由真实原始事件构造：bit0 Timer.match_event；bit1 Uart.raw_events[0]；bit2 raw_events[1]；bit3 raw_events[2]|raw_events[3]；bit4 Gpio.raw_event。本地粘滞EVENT绝不能接IRQ。mask不清pending；hardware/test set胜W1C；RAW只硬件，软件TEST不能伪造RAW。单一总线不能同时提交本地clear与IRQclear两个写，系统测试不要提出不可达双写条件；每层分别对齐真实事件碰撞即可。

## 关键未解决探针：aresetn表达与后端类型

正式合同要求顶层aresetn，断言与释放事先由外部控制器同步到ACLK，内部共同同步高有效reset。仅反相不是同步化。现有桥/译码文档例只有高有效rst；本次搜索未找到aresetn实际设计例。

源码事实：`crates/bitloom-hir/src/lib.rs:419` validate_clock_reset要求每模块恰一个Clock输入和一个Reset输入；`composition.rs:311`类型兼容将Clock/Reset与UInt1严格分开。`crates/rhdl-firrtl/src/chisel.rs:1050` bit_operand将Reset转asBool.asUInt，Xor输出UInt；Wire(Reset())赋值路径未见对该UInt结果转换。FIRRTL端口Reset发为UInt1并附元数据，但Reset wire类型另需实际检查。

因此不要仅凭宽度相同宣布 `rst = aresetn ^ 1` 在三后端可用。129.1应实际执行最小aresetn适配探针（direct、FIRRTL、Chisel），并记录顶层Reset输入语义、内部Reset wire类型及自动寄存器reset选取。选项可能是明确RTL边界适配wrapper+prelude核心，或必要的最小类型转换支持；两者都须先明确产品/例子边界，不能测试台偷偷反相然后宣称产品顶层符合aresetn。若现有HIR无法诚实表达，记录阻塞及范围处理，129.2不得ready。这是只读发现的风险，未实跑，不能定性为已证实bug。

reset测试按同步沿取消：AW-only、W-only、AR捕获、CSR已提出、已提交待响应、B/R背压、UART队列/半帧、GPIO同步链、Timer活动全部共同清。沿前ready/valid可能仍高不计握手；区分已接受、已提交、响应消费、复位取消；串行已发物理位不可回滚。恢复后再执行真实读写/收发证明无旧响应重放。

## 独立驱动与可判定AC

复用已锁定五包独立主端环境：scripts/phase24-axi-bfm-requirements.txt的cocotb2.0.1、cocotbext-axi0.1.28、cocotb-bus0.3.0、find-libpython0.5.1、scapy2.7.0。Python3.12.3为历史实测而非包锁。主端高层调用会拆分/对齐；未对齐、特殊WSTRB、半写复位必须raw channel。两驱动分时互斥，不能同时驱针脚；BFM reset返回None记取消。

建议129.1冻结下列129.2/3具名验收，而非泛称“全系统通过”：
1. 干净checkout且文档工具前提满足，一条严格命令编译原文prelude-only例、生成层级RTL、实际跑主端/串行/针脚场景并输出可核对摘要。不得引用本机/tmp工具/预生成RTL才成功；首次依赖获取是否联网要明示，不把Epic130禁网源重放强加核心例。
2. AXI五通道独立：AW早/W早/同时、间隔0/1/7/31、AR并发、不完整写不饿死读、B/R各自背压。接受/提交/响应/取消分别记账，事务排空后继续观察重复。
3. 真实主端配置Timer one-shot/periodic/回绕→IRQ0，GPIO DIR/OUT/SET/CLEAR/IN实际pad→IRQ4，UART配置DIV>=3、TX独立物理解码、外部RX字节→IRQ1、真实TX取队首→IRQ2、overflow/framing→IRQ3。通过AXI读RX验证pop，受阻R/B期间不能重复pop/push。
4. 所有五源分别产生/屏蔽/读pending/清pending，本地EVENT保持高时清IRQ不重触发，新的真实事件能重触发；后端同一独立期望。IRQ.TEST只验软件注入，不能代替真实外设源产生。
5. 固定16seed×1000完成事务起始预算+必须命中定向场景（研究§5）；若调整需记录真实运行成本/覆盖依据而不是静默删减。独立scoreboard手写黄金地址/期望，不调用DUT描述译码/next-state。UART独立u64绝对时刻/线路解码可借128.5模式，但不能从DUT内部busy/state取期望。
6. DIV3/4/5及相位、back-to-back/坏stop/溢出覆盖系统路径；超大DIV不许声称跑完2^32周期位。系统需要真实行为，不必重复128.5所有局部矩阵，但须明确分层证据不能替代系统路径。
7. 原始系统生成RTL综合，无observer，检查latch/多驱动/未知结构并记录cells；不是PPA。formal safety/cover/负控制明确性质/假设/深度/实际状态，不把128.5有限证明或工具版本视为系统全协议证明。
8. 第二组合实际执行，有不同图及独立行为；两图direct、FIRRTL→firtool→RTL、Chisel→JVM→firtool→RTL分列，不能emit-success当行为成功。native Interpreter/Compiled和GeneratedFunctional层级明确unsupported并检诊断，不隐式flatten或空壳输出0。

129.1风险故事只交付有效风险/依赖/工具接口探针，不要求先实现上述完整功能。129.2完整系统成功不自动关闭129.3维护矩阵。

## 工具与CI

产品固定Rust1.97.1/edition2024，FIRRTL6.0.0，firtool1.159.0（AD9 unpaired product pin，不宣称官方配对），Chisel7.15.0。Java17+；既有执行环境Scala2.13.18/sbt1.10.11应由本故事工具探针再次记录。禁止为集成随手升钉。

SBY scripts/ci-sby-pins.env：官方YosysHQ/sby.git，yosys-0.47 tag object bfc1c47eb786496fe794481ff88e75728f0529a6。已有runner进一步校验peeled commit与安装16文件、隔离Python缓存；129.1应沿用真正身份前置，而非仅command -v。架构Stack Yosys0.68/Verilator5.050与实际历史host Yosys0.33/Z3 4.8.12须区分，不把实录host版本当新增产品钉，亦不宣称实跑Stack工具。本次没有运行版本命令。

Justfile `test`仅cargo test --workspace，`check`加fmt，Chisel/formal/semver分别独立。CI已有test25m、chisel-numeric25m、formal-sby20m，UART等专用ignored由独立门执行；新增系统矩阵可能超过预算，应测时再决定独立系统job/复用构建缓存，不把缩seed或continue-on-error当解决。CI接线不等于远端CI已运行。archive在cargo clean前冻结原始命令、退出、种子、波形、证明、工具身份和相关源码SHA，不能复用不同源码旧PASS。贡献模板须包含owner、行为合同、独立测试、两组合/适用矩阵、来源与兼容性，核心与外部分栏。

## 估算、owner、停止条件

129.1 0.5–1、129.2 3–5、129.3 3–5有效人日，总6.5–11；单列25%集成预留约8.1–13.8，不是代理墙钟或承诺。aresetn类型边界与跨后端全系统运行成本是当前主要不确定项。维护owner Richard；逐故事实施/证据Codex；主代理七步审查/状态/单故事commit；需求/架构越界升级Richard。不要把并行代理算长期维护人力。

|风险|处理/证据|停止条件|
|---|---|---|
|aresetn类型/自动reset语义与跨后端不一致|最小真实探针，固定适配边界和共享reset|未解前129.2不得ready，不能删aresetn合同|
|IRQ错位/sticky误接/TEST代替硬件|逐源真实刺激、clear后静默、新事件再置位；建议映射突变检错|任一源缺实测或仅软件注入不得FR198通过|
|AXI/CSR不同提交时刻导致重复或快照错误|独立通道/端到端副作用计数/取消账本|任何无法解释的接受提交响应差额停止关闭|
|两组合仅换名或局部夹具冒充完整系统|图/实例及真实行为检查|127.4七模块银行、128.5双UART+IRQ单独不足FR198|
|工具/源码/原始证据不可重放|身份绑定、精确测试执行数、归档及缺工具失败|缺工具/UNKNOWN/timeout/零匹配不能PASS|
|新增例子迫使第二HIR/native层级/异步复位扩范围|坚持AD1/7/12/13与NFR97|无法在批准范围实现须记录阻塞，不偷扩|
|维护/CI成本上升|两例复用数据、首次步骤、实际CI分钟/失败修复成本|无owner或required矩阵不可运行不能maintained|

禁止降级：缩水GPIO/IRQ/baud宽度，换旧IP/假CSR，保持bridge独立复位，原始事件改sticky，隐式默许工具skip，emit代执行、综合代PPA、局部形式代全协议、文档矩阵代实际执行、核心关闭吞外部试点或FR189/NFR91。历史关闭保持有效但不能代替当前系统实证。

## 补充：主代理复位候选与 AD-30 判定

已全文阅读 `/tmp/bitloom-1291-story-draft.md`；补读架构AD-30/31完整正文。AD-30明确共同同步高有效内部reset及系统aresetn外部同步后转换，没有字面要求逻辑反相本身必须在HIR内。以下为静态候选，均尚未执行：

1. **既有typed路径候选**：top唯一Reset输入名aresetn且top无Reg；Xor(a_u,aresetn,zero)落UInt1，Eq(core_reset,a_u,zero)生成Bool赋Reset wire，再全部child rst接core_reset。现有assign_eq仅push表达式，Chisel Eq输出Bool，理论上较Xor结果直接赋Reset更合理。不能把内部wire命名rst，`chisel.rs:984 ref_name`无条件把这个名字替换成implicit reset。
2. **更关键的静态障碍**：`chisel.rs:1127 emit_instance_connects`对child_port clk/rst直接continue；`chisel.rs:1378`实例化仅`Module(new Child)`。所以HIR中child rst接core_reset也可能在Chisel丢失，child继承父Module implicit reset。top名aresetn不是rst，`emit_module`会输出io.aresetn，同时Module仍有implicit clock/reset；需要实际确认多余implicit reset与HIR顶层端口合同，不能测试台把两者绑定后掩盖错误连接。这个问题意味着仅Eq转换不够保障全图语义。最小三后端探针必须由child状态实际观察清/计数，不能仅编译。若失败，后续最小后端修正要保留HIR显式clock/reset连接并回归原有共享连接路径；这仍可服务AD-30而不必新增公开API，但129.1风险故事不能一边声称只读产品一边无记录修后端。
3. **外层薄RTL候选**：prelude例子生成完整Core（唯一Clock clk、Reset rst，全部真实桥/decoder/leaf同域），产品例子随源码提供可审阅的最薄Verilog边界顶层，公开aresetn/ACLK，`wire core_reset = ~aresetn;`并连接Core。direct、FIRRTL和Chisel三份Core均用此相同边界并实际跑同向量，Chisel clock/reset端口适配必须显式列出。wrapper不含寄存状态、不含同步器、不改功能算法，不在testbench外偷偷补新语义；原文wrapper与生成Core共同列入filelist、复现命令、综合和源码hash。对此我的判断：符合AD-30“内部高有效、外部同步后转换”的语义及单一HIR核心，不需要第二HIR；但须在风险合同明确系统产物=生成core+边界RTL，不能宣称所有top逻辑由prelude/HIR发射。若主代理认定正式交付要求纯HIR系统top，则需选择typed修正而非外层方案。自有薄wrapper不是Epic130外部开源核，勿误引source-fetch/admission，但同样保存来源和字节证据。

草稿明显修订建议：
- AC5把“验证aresetn反相在direct/FIRRTL/Chisel”写为明确两个候选、先最小探针选择合法路线；若typed失败而外层方案符合已批准合同，应记录失败与最终选择，不必以增加原生层级/异步复位来解决。
- Dev Notes“失败读及写响应rdata0”应改为“所有失败读rdata0；AXI写仅核BRESP，CSR写响应rdata按各leaf合同/无关”。桥与decoder正式合同明确写响应rdata无关，不应将更强测试约束倒套总线peer；真实四外设若恰为0可分别记录。
- AC4/7和Dev Notes目前把第二拓扑推至129.2 ATDD冻结，而129.1职责为明确本Epic接口/AC；建议本风险门最终固定AXI完整图+直接CSR完整图，避免accepted后仍“另一种以后定”。
- 数据预算可纳入风险记录：研究建议16seed×1000事务+必达定向不是数学证明；调整要保留测时与覆盖理由。避免仅“随机背压”没有数量或可执行覆盖标准。
