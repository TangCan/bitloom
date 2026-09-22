# Epic129 组合系统与核心证据 NFR14 风险记录

日期：2026-09-21。范围：FR198 / FR201核心，NFR14 / NFR94，AD-28/30。状态：build已形成，待主代理独立审阅及七步关闭后accepted。本记录不是FR198系统行为PASS，也不是FR201关闭声明。维护owner Richard；实施和证据Codex；七步状态/单故事提交由主代理负责。

## (a) 上游约束与依赖

权威依次为用户持续七步授权、正式 `docs/ip/phase24-contract.md`、架构AD-30、正式Epic129/Story129.1；研究 `technical-bitloom-composable-ip-ecosystem-2026-09-20/implementation-plan.md` 仅作验证预算参考。研究旧GPIO8/最多8IRQ、历史M0-only与旧API形状不得覆盖现行GPIO32/IRQ5或持续授权。产品Bitloom，与samitbasu/rhdl无关；设计只依赖bitloom-prelude。

基线`0ab222203916b1b0583c2c673558d5333cb05552`已关闭M3/128.5；M0/M1/M2/M3对应epic-125/126/127/128-closeout及128-5-final-verification。历史关闭保持有效。129.1只解自身风险门，129.2/129.3仍backlog，Epic129 in-progress；FR198和FR201核心、Epic130与Phase24整体尚未交付，FR189/Epic122 deferred、NFR91保留。

|后续入口|必须先done的逐边审计|当前状态 / 动作|
|---|---|---|
|129.1|125.3/M0|已done；M1–3是已有集成基座，不追改正式前置|
|129.2|129.1、127.4、128.2、128.4、128.5|后四项done；129.1须本故事七步完成。实际集成也用已done的128.3 IRQ|
|129.3|129.2|尚未满足，不ready|
|M4/Epic129关闭|129.1、129.2、129.3全部done|未满足；核心关闭不依赖130.3|
|整个Phase24关闭|核心与Epic130独立外部验收等完整合同|未满足；核心绿不能给外部行PASS|

源接口已逐一核对：`crates/bitloom-prelude/src/ip/axi_lite_csr.rs`、`csr_decoder.rs`、`uart_csr.rs`、`gpio/csr.rs`、`timer.rs`、`irq.rs`均具有同session `define_module`入口。UART共享8×4参数FIFO；不使用旧UartTx/Gpio8代替新wrapper，不复制外设算法。源调查见`129-1-system-recon.md`及真实探针绑定源码。唯一FrozenHir、定义完成后一次finish；native Interpreter/Compiled和GeneratedFunctional层级保持unsupported，实际层级验收依靠RTL。

工具不升钉：Rust/Cargo1.97.1、edition2024、firtool1.159.0、Chisel7.15.0、Scala2.13.18/sbt1.10.11既有执行配方；BFM固定cocotb2.0.1、cocotbext-axi0.1.28、cocotb-bus0.3.0、find-libpython0.5.1、scapy2.7.0，Python3.12。SBY核`scripts/ci-sby-pins.env`官方仓库tag对象、peeled commit、安装16文件和隔离Python闭包。host Yosys0.33/Z3 4.8.12如实列出，不能声称已用架构Stack的另一版本实跑，也不形成新产品pin。

## (b) 工期带与支持参数

129.1：0.5–1有效人日；129.2：3–5；129.3：3–5；合计6.5–11。若管理采用25%集成预留，另列8.125–13.75，非日期/代理墙钟承诺。置信度中等：三后端系统执行成本、独立scoreboard与净checkout使用成本需要实测，不能套研究旧6–10覆盖风险故事。估算包含维护证据，不把并行代理视为免费持续人力。

固定单ACLK、16位byte地址、32位数据、WSTRB4、UART8N1/LSB first、TX/RX各8×4寄存器FIFO、GPIO32、Timer32、IRQ5。UART有效DIV>=3，每bit=DIV+1拍；系统测DIV3/4/5与相位/坏stop/overflow，最大DIV仅做配置/有限时序，不谎称实跑2^32周期bit。无CPU/DMA、多主、burst/ID、多时钟、深RAM或物理签核。

## 两种正式拓扑与复位路线

129.2交付两个prelude-only例子，共用私有定义/事件helper，不新增公共稳定类型：

1. **AXI完整系统**：独立AXI主端→真实AxiLiteCsrBridge→真实CsrDecoder→真实UartCsr、GpioCsr、Timer、Irq。UART内部真实双FIFO与CSR叶全部属于同一session图。此图承担FR198。
2. **直接CSR完整系统**：独立CSR主端→同四窗CsrDecoder→同四真实外设，不实例化AXI桥。面向已有CSR主端，实际线端/针脚行为验证；不是仅改顶层名字或旧127.4寄存器银行/128.5局部UART→IRQ。这一实质拓扑差异承担第二组合复用。

两图地址UART0000–00ff、GPIO0100–01ff、Timer0200–02ff、IRQ0300–03ff，各256字节。0400–ffff DECERR；窗内洞/未对齐/错误访问SLVERR，失败读0；8104不能别名0104。所有16种WSTRB含零及保留位/错误优先按合同；AXI写验BRESP，CSR写响应rdata依各leaf合同，不给桥额外加零约束。AWPROT/ARPROT保留忽略，非权限隔离。

IRQ接线固定bit0=Timer.match_event、bit1=UART.raw_events[0]成功RX到达、bit2=raw_events[1] TX取队首、bit3=raw_events[2]|raw_events[3] overflow/framing、bit4=GPIO.raw_event新沿聚合。没有第6路或外部IRQ针脚。各本地sticky EVENT不能接IRQ；TEST验证软件注入但不得替代真实事件。按真实硬件事件触发→本地EVENT保持→清IRQ→无新事件时pending保持清→再产生新事件置位的端到端序列验收。事件/W1C冲突set优先；单总线不能同时提交两处清写，不造不可达验收。

复位最终路线由`../test-artifacts/129-1-reset-evidence.md`及原始探针决定：typed路线与Chisel隐式reset分离实测；若失败，采用**生成core + 公开无状态aresetn RTL边界 + 明示Chisel端口拼写shim**。core全部模块使用通常clk/rst、共同同步高有效reset；适配仅`core_reset=~aresetn`。它是已批准AD-30内部复位转换的自有边界，不是第二HIR或Epic130外部IP。必须随例子交付源、filelist、SHA、综合和执行，不能称整个top由HIR生成。输入aresetn断言和释放均由外部控制器先同步到ACLK；反相不是同步器，逻辑探针不证明板级可靠性。129.2将本次已证实路线用于真实系统，不能用探针代替其全状态复位测试。

## 独立验收及事务账本

129.2负责两图实际行为、配方及使用测量；129.3负责核心矩阵复核、CI/兼容/贡献模板、有限形式与综合关闭。独立主端手写关键黄金地址和副作用，不调用DUT描述译码或next-state；UART独立驱动与物理解码，期望不取内部busy/state。必须实际配置Timer one-shot/periodic/回绕、GPIO DIR/OUT/SET/CLEAR/针脚IN与新沿、UART真实TX与外部RX/错误并清五源IRQ。

AXI覆盖AW早/W早/同拍，间隔0/1/7/31；AR与完整写并发、不完整写不阻读、B/R独立背压、全部WSTRB、错误/未对齐。高层BFM可能自动对齐/拆分；特殊请求用原始通道，BFM与raw驱动时段互斥。direct CSR同样检请求/响应保持、error、只提交一次及真实外设副作用。

账本分别计AW/W/AR捕获、CSR唯一非reset上升沿req_valid&&req_ready提交、B/R或CSR响应消费、reset取消；单次副作用只在提交发生。排空后继续观察重复响应，reset后不能重放旧请求/响应。测试reset取消AW-only、W-only、AR捕获、待提交、已提交待响应、B/R受阻，同时清桥/译码/leaf/FIFO/FSM/Timer/GPIO同步链。已发送UART物理bit不可回滚，BFM返回None计取消而非OKAY。安全性不依赖ready公平；完成界另加明确ready最大停顿和有限leaf延迟假设，记录界值。

初始随机预算固定16 seed×1000完成事务，定向必达场景单列计数（每种AW/W顺序、16种WSTRB、每个错误类别、五源真实事件、每种取消位置至少一次），seed列表由129.2 ATDD冻结并保存。随机不是概率保证；任何预算调整须保存实际测时、覆盖与原因，不能静默减少。

## 核心支持矩阵与使用证据责任

|验收项|129.2责任|129.3复核 / 限制|
|---|---|---|
|direct RTL、FIRRTL→firtool→RTL、Chisel→JVM→firtool→RTL|两图各自编译执行同一独立预期，保存源码/工具/命令/seed/波形/结果|逐行重新核证，emit/编译不能代行为；缺工具硬失败|
|native Interpreter / Compiled / GeneratedFunctional|层级明确unsupported并检诊断|不能默默flatten、空壳或零输出PASS|
|formal prove与cover|提出有限安全性质、假设/深度与可达场景|独立实际prove、cover、负控制；UNKNOWN/超时拒绝；非全AXI/全UART证明|
|原始综合|两图生成原始RTL连同无状态边界，禁observer|检查latch/多驱动/unknown及记录cells；不叫PPA或时序签核|
|兼容/API|旧API、端口、bank/手写FL保留|SemVer独立；若公开符号新增逐项FR142/minor，不自动稳定、不改包版本|
|一命令使用配方|干净独立checkout、隔离产物目录实际运行，列外部工具/环境及首次依赖联网要求|重新复现；不可读主target或/tmp生成RTL作为隐含输入|
|实际成本|记录上手步骤/耗时、手工接线及地址代码行数、修改文件数、执行耗时|CI wall time、缓存与工具准备成本分列；无实测不能宣称收益|
|贡献模板|owner、行为合同、独立测试、两组合、支持矩阵、兼容、来源/许可、证据字段|审阅模板实用性；核心与外部行分开，外部未验收不PASS|

工具发现、固定BFM stub、旧桥/译码探针只证明旧基座当前可用，不交付上述系统。形式工具发现不是prove；JVM版本不是Chisel系统行为。完整Phase24还须Epic130独立外部试点；核心FR201关闭不依赖130.3但不得冒充FR201全部。

## (c) 禁止静默降级与停止动作

不得缩GPIO/IRQ/地址宽度或baud合同；不得换旧IP、假CSR、空黑盒；不得只reset桥保留外设；不得sticky代原始事件；不得拼FrozenHir、第二IR、偷增native层级或异步reset；不得改工具pin/版本以过测；不得skip缺工具、零测试、UNKNOWN、timeout为PASS；不得emit代执行、综合代PPA、旧局部形式代系统证明；不得用核心关闭吞外部试点或FR189/NFR91。不push/publish。

|风险 / owner|控制与失败动作|停止条件|
|---|---|---|
|复位跨后端 / Codex实施、Richard维护|保留typed失败，三后端同边界执行和综合；源码/端口shim纳入配方|最终任一路失败，129.2不得ready；不偷修产品后端|
|IRQ错位或sticky / Codex|五源实际事件、clear后静默再触发，独立针脚观测|缺任一源或TEST替代真实来源不得FR198关闭|
|事务重复/取消差额 / Codex|接受/提交/响应/取消分账，端到端副作用和排空后观察|任何无法解释差额停止行为关闭|
|两组合无实质复用 / Richard审阅、Codex实现|AXI桥图与直接CSR图共享真实四外设，分别执行|改名/旧局部夹具不能作为第二图|
|证据/工具漂移 / Codex|原始日志归档、源前后SHA、版本/完整SBY身份、命令UTC耗时退出|缺工具、源改变、版本不符或无法重放先修范围内问题|
|净checkout失败 / Codex|独立checkout/产物目录实际配方；不依赖主target|本机临时工件依赖未解除不得配方PASS|
|架构范围/合同冲突 / Richard|记录差异并升级需求决策；保留已有事实|不能在本风险故事偷偷改产品API/backend或缩合同|
|CI与维护负担 / Richard|两图共享helper、各后端测成本，必要时拆独立required job|无长期owner或required矩阵不能运行，不称maintained|

## (d) 负责人及维护叠加

Richard是长期维护owner与需求/架构升级决策人；Codex为当次实施、测试、归档责任人；主代理承担独立审查协调、七步清单、真实clean/fmt/just test、状态与单故事提交。缺席或未解决阻塞如实保留，不能转给假想免费并行人力。

维护叠加：旧单模块API/手写FL与新共享图共存；两组合/三后端、JVM与形式工具各自依赖与缓存、来源身份、证据重放和CI预算增加回归面。通过例子私有helper复用减少接线重复，但不将语义压低到后端最低公分母。未知成本由129.2实测、129.3纳入CI；当前不承诺性能提升。

## Story129.1 AC与证据索引

AC1–4：本记录前置/估算/owner/拓扑/独立验收；AC5：`129-1-root-foundation-results.json`、其原始tar与`129-1-reset-evidence.md`；AC6：`129-1-atdd-gate.py` normal/-O各84场景及人工逐边表，gate只检查自身已实现关系，不能称全部跨故事依赖检查；AC7：上表与外部分界；AC8：主代理完成review/automate/clean/fmt/workspace regression/单故事commit后才done。存在性脚本只证文件存在，不证质量。人工18项记录`129-1-atdd-manual-acceptance.md`，未来计划有效与未来行为PASS明确分开。

根代理基座：35命令exit0、570源指纹前后相同；固定BFM 1例与既有真实decoder层级exact 1例实际PASS，seed=N/A；原始`129-1-root-foundation-20260921T145519754208Z.tar.gz` SHA256 `b19a66446c10d381d2091816c1033367deb57118887358fcaef85b41cb1c42a0`。不将它们称为系统或形式证明。


## 复位风险最终实测决议（2026-09-22 UTC）

typed direct/FIRRTL编译执行通过；typed Chisel实际在firtool lowering以abstract Reset端口不具体拒绝，未进入RTL执行。最终采用公开无状态aresetn边界+通常clk/rst生成core+明确端口shim；三个后端均实际执行双子同步清零/优先/恢复且纯设计综合通过。详细证据、原始失败和源码SHA见`129-1-reset-evidence.md`，157成员归档已逐字核验。本决定只解当前路线风险；129.2真实四外设系统仍须全状态/事务取消验证。它不是隐式reset故障的行为实证，也不放宽外部控制器同步断言/释放前提。
