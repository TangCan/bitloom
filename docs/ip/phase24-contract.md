# Phase 24 正式接口合同

批准：2026-09-20；Story125.1 / FR192；FR192–201、NFR93–99。本文固化计划中的行为与范围，不代表产品已交付。仅 M0 / Epic125 获准执行，126–130 backlog。研究保持历史原文；本合同的 GPIO32 位、IRQ5 有效位取代历史草案的 GPIO8/最多8路。

2026-09-20 M0关闭后续授权：用户要求继续，当前执行范围为 Story126.1 NFR14 → 126.2 模块组合基础；126.3/126.4及Epic127–130仍backlog，未授权自动执行。不宣称M1或Phase24已交付；上述M0批准记录保留为历史。

设计仅依赖 bitloom-prelude，单一 FrozenHir/ElaborateSession，工具钉不变。旧 API、端口和映射保持；126.2已实现ElaborateSession::define_module与Gpio::define_module并显式登记FR142清单；Story126.3已实现RvRegSlice两槽注册切片并显式登记FR142清单，使用见[注册切片](rv-reg-slice.md)；ParamSyncFifo、CSR 描述/桥/译码、Timer、IRQ、GPIO/UART wrapper仍仅为待实现命名表面。实现故事须逐符号更新 FR142 清单与 SemVer minor 说明。

首版仅单时钟、32-bit 数据、16-bit 字节地址、寄存器小 FIFO、UART8N1、GPIO32、Timer32、IRQ5，以及单独验收的一个外部小核。不包含 CPU、DMA、PWM、watchdog、SPI/I2C加深、burst/ID、多主、多时钟模拟、异步 FIFO、深RAM FIFO、通用FIRRTL内存、原生层级模拟、复杂外部核、物理签核或发布。FR189/Epic122 deferred 与 NFR91 不变。

## 既有 Axi4LiteSlave：M0 兼容边界

旧 Axi4LiteSlave 是现有寄存器 bank，保留全部现有端口、地址映射与 reset 值。未来 CSR 桥是新增独立表面，不能将它的16-bit地址、错误译码、单执行请求限制倒套到旧 bank。M0修复独立AW/W已握手却丢失、并发读写丢R的问题。

同拍 AW/W 握手后一个 tick 可观察 BVALID 的兼容时序保留；分拍分别缓存，收齐后写一次。并发接受的 AR 和完整写均须产生响应，同址读返回该写入之前的值（read-before-write）。受阻B/R保持有效及payload，响应不重复；reset 清捕获槽和待响应。手写 FL 同步修正。研究只在 native 观察缺陷；125.2须真实生成RTL红测，125.3才能记录修复关闭。

## 模块定义合同

126.2现行可编译入口与严格参数身份规则见[模块组合](module-composition.md)，实测见[验收记录](module-composition-evidence.md)。下列UartTx/Timer仅是未来IP适配形状，不能误认为这些类型已提供define_module。

```rust
// 合同中的调用形状；实现故事须逐符号登记 API，当前不是可编译的已发布 API。
UartTx::define_module(&mut session, "UartTx_baud", &config)?;
Timer::define_module(&mut session, "Timer32", &timer_config)?;
// 顶层与子模块在同一session完成定义，最终只finish一次。
let hir = session.finish()?;
```

`define_module` 与已有 `elaborate` 共用实现；旧入口创建 session、调用定义函数、finish。模块定义不得在另一个尚未 end_module 的定义内隐式覆盖 current；顶层可先定义并结束，再定义子模块。顶层名显式等于电路名。相同参数可复用一种模块定义，不同参数得到不同受控标识；参数指纹冲突、重复模块名、缺模块、实例环、端口宽向错误在 freeze 前给出诊断。

仅修复新组合路径必需的层级校验，不顺带交付通用 native 层级仿真。native/generated 接到此多模块系统继续明确拒绝；系统验收依靠真实生成 RTL。若未来需要统一 Rust 层级仿真，另立范围和成本，不以名字前缀替换实现伪装完成。

## Ready/valid 与参数 FIFO 合同

| 项目 | 首版合同 |
|---|---|
| 传输 | 上升沿 valid && ready 接受一次；受阻保持 valid 与 payload；reset 丢弃在途数据并明确计数边界 |
| RvRegSlice | 两槽，输出valid/data和输入ready均由寄存状态驱动，无输入到输出组合路径；稳态可每拍一笔，满后恢复允许一个等待拍，不宣称所有情况下零气泡 |
| ParamSyncFifo | WIDTH=1..64、DEPTH=1..16；覆盖非2次幂；无空直通，入队后最早下一周期可出队；首版用寄存器存储，暂不承诺块RAM推断 |
| 满/空 | 满时 input_ready=0，即使同拍有pop也不接受push；空时 output_valid=0；一般非满非空支持同拍pop+push |
| reset/flush | reset优先，其次flush，再正常传输；清占用与有效状态，不承诺清零全部payload；无效时payload不作为功能结果 |
| 非法参数 | WIDTH=0/>64、DEPTH=0/>16显式报错；不是裁剪或panic后继续生成 |
| 旧 FIFO | SyncFifo原API和8×4行为保留；如共用内部实现须以旧向量证明延迟、full/empty及输出行为兼容 |

限制小深度是首版设计取舍：消除未证明的FIRRTL内存路径对首个组合系统的依赖，不是性能结论。深FIFO/BRAM版另立存储器延迟、输出预取、碰撞与目标器件推断合同；不得更换实现后偷偷改变ready/valid可见时序。

## CSR 内部接口与提交点

内部请求：`req_valid/req_ready, write, addr[15:0], wdata[31:0], wstrb[3:0]`；响应：`rsp_valid/rsp_ready, rdata[31:0], error[1:0]`。全系统最多一个已向CSR提交、尚未消费响应的请求；AXI端的AW/W/AR分别有捕获槽，槽占用不等于CSR已提交。

CSR `req_valid && req_ready` 是唯一提交点。由CSR访问触发的寄存器写入、RX FIFO弹出、TX FIFO推入和W1C清除，只在该提交点执行；`rsp_valid` 在受阻时保持，不能因多拍BREADY/RREADY为0重复副作用。读数据在提交点形成快照并锁存。首版仅固定有限延迟的片上CSR，不允许叶节点无限等待；未来接外部可等待设备时另加超时/取消协议，不能简单超时后忽略迟到响应。

描述采用 Rust 静态配置（寄存器偏移、field mask、reset、访问类型、事件映射），同源导出地址文档和C头文件。先不设计新的描述语言、远程插件机制或另一个运行时IR。独立测试中的关键黄金地址和副作用期望手写，避免生成器与测试共享同一错误。

| 场景 | 规定 |
|---|---|
| RW | WSTRB逐字节作用；保留位读0写忽略 |
| RO | 写返回SLVERR，无副作用 |
| W1C | 只对被WSTRB选中的写1位清除；本项目事件寄存器规定硬件set优先：`next=(old & ~clear)|event` |
| 同拍事件 | 这是本项目选择，不宣称来自OpenTitan通用RW1C；相同事件多次发生只保留pending，不保证计数 |
| 零WSTRB | RW/W1C/TX_DATA合法写返回OKAY但无副作用；RO写仍SLVERR，错误优先于mask |
| 地址空洞 | 不在任何窗口：DECERR；已映射窗口内未定义/未对齐/不支持访问：SLVERR；全部无副作用 |
| RX空读/TX满写 | SLVERR，RX不pop、TX不push，不以无限总线等待隐藏状态 |

## 未来 AXI-Lite→CSR 桥与仲裁

端口保留AWPROT/ARPROT（各3位），首版不实现权限域并明确忽略，不宣称安全隔离；BRESP/RRESP仅产生OKAY(00)、SLVERR(10)、DECERR(11)。AW与W独立捕获，AW先/W先/同拍均能完成；每种最多一笔，写事务收齐两部分才可进入CSR。AR单独捕获。外部ready/valid由寄存状态控制，禁止组合依赖对端valid/ready。B/R响应各有保持状态，在形成响应前预留对应空间，避免响应覆盖。

读请求和完整写请求采用轮转仲裁；不完整AW或W不得阻塞读。首版对全CSR只有一笔执行中请求，明确这是实现容量限制而非AXI-Lite协议要求。BREADY/RREADY可长期为0；安全性仍成立，完成时限只在显式ready公平性/上界假设下验收。

系统顶层接`aresetn`，首版要求板级复位控制器已将复位断言（aresetn=0）和释放（aresetn=1）均同步到ACLK，内部转换成同步高有效reset，bridge和所有leaf同域复位；异步置位的板级reset同步器作为独立适配层，不把未保留async元数据的FIRRTL路径冒称支持。清AW/W/AR槽和待回响应；复位前已触发的UART物理发送不能承诺回滚。首版禁止只复位bridge而保留外设继续执行。针脚反相不等于完成复位同步化，输入前提、释放时序和外部控制器约束必须单独记录并验证。

## 外设寄存器窗口

地址为首版相对基址，范围均0x100字节；编译时拒绝重叠和超界。不提供 ID/version 寄存器；未定义偏移为窗口内地址空洞。

| 窗口 | 偏移 | 寄存器与语义 |
|---|---|---|
| UART 0x0000 | 00/04/08/0C/10/14 | CTRL、BAUD_DIV、STATUS(RO)、TX_DATA(WO低8位)、RX_DATA(RO且成功读pop)、EVENT(W1C) |
| GPIO 0x0100 | 00/04/08/0C/10/14 | DIR、OUT、IN(RO)、SET(WO)、CLEAR(WO)、RISE_EVENT(W1C)；首版上升沿事件，禁称完整pad/电气实现 |
| Timer 0x0200 | 00/04/08/0C | CTRL(enable/periodic)、COUNT(RW)、COMPARE(RW)、EVENT(W1C) |
| IRQ 0x0300 | 00/04/08/0C | PENDING(W1C)、ENABLE(RW)、TEST(WO置事件)、RAW(RO)；保留未使用位读0 |

#### IRQ事件与清除

IRQ编号：0 Timer、1 UART RX到达、2 UART TX腾出空间事件、3 UART溢出/错误、4 GPIO聚合，其余保留。须明确外设原始事件脉冲与本地EVENT状态分开：IRQ pending接事件脉冲，不接粘滞EVENT电平，避免清IRQ后因本地状态高而不断重触发。每层W1C同拍新事件均set优先；mask只影响输出，不清pending。IRQ输出为`|(pending & enable)`。

#### 复位与UART配置

所有首版CSR的复位值为0，CTRL关闭；GPIO的输出数据寄存器为0，方向配置为输入；UART idle输出高，不由TX_DATA复位值推导线路电平。UART忙时写BAUD_DIV返回SLVERR；组合wrapper限定BAUD_DIV>=3（配置前保持关闭），作为同步延迟预算的起点，必须由Story 128.5边界测试确认，不能直接宣称板级可靠性。

#### Timer计数合同

Timer使用32位模计数；CTRL关闭时保持，写COUNT优先于自动计数；软件写CTRL/COUNT/COMPARE的该拍不做match判定；普通使能拍先形成next_count，`next_count==COMPARE`产生一次event。periodic命中后COUNT置0，one-shot命中后enable清0。COMPARE=0定义为回绕时命中，不能隐含每拍触发；软件要求立即触发使用IRQ TEST。测试包括最大值回绕、写比较值越过当前计数及同拍W1C。

#### 异步输入与板级边界

UART RX、GPIO、外部IRQ针脚属于异步输入，即使内部单时钟也要列CDC假设。首版wrapper提供同步输入路径并测试串行采样边界；实际波特率配置需给出同步延迟预算，不能把原有baud_div=0实验直接当板级可用。GPIO双级同步不滤毛刺；开漏、pad、驱动强度和板级约束不由逻辑测试证明。


## CSR 字段与访问的完整决定

全部寄存器均为32位、4字节对齐、little-endian；WSTRB bit n 对应 wdata[8n+7:8n]。所有CSR复位值为0；动态RO在reset时为0，reset释放后反映采样/状态。所有未列位读0、写忽略。全部WO读返回SLVERR、rdata=0，无副作用；所有失败读rdata=0。RW/W1C及WO合法零WSTRB写返回OKAY且无副作用；地址/访问类型错误优先。TX_DATA未选择byte0时不push，即使FIFO满也返回OKAY；非零有效字节写满FIFO才SLVERR。ID/version寄存器不提供。

| 窗口/偏移 | 名称 / 访问 | 有效位与行为（reset全部0） |
|---|---|---|
| UART 0x0000 | CTRL RW | bit0 enable，其余保留 |
| UART 0x0004 | BAUD_DIV RW | bits31:0；每串行bit为BAUD_DIV+1个ACLK；启用要求>=3 |
| UART 0x0008 | STATUS RO | bit0 rx_nonempty、bit1 tx_full、bit2 tx_busy、bit3 rx_busy；其余保留 |
| UART 0x000c | TX_DATA WO | bits7:0，成功有效写push一个byte；高位忽略 |
| UART 0x0010 | RX_DATA RO | bits7:0，成功读返回并pop队首；空读SLVERR |
| UART 0x0014 | EVENT W1C | bit0 RX到达、bit1 TX腾空位、bit2 RX溢出、bit3 framing error |
| GPIO 0x0100 | DIR RW | bits31:0，每位1输出/0输入 |
| GPIO 0x0104 | OUT RW | bits31:0，输出数据锁存 |
| GPIO 0x0108 | IN RO | bits31:0，经双级同步的针脚采样 |
| GPIO 0x010c | SET WO | WSTRB所选写1位对OUT置1 |
| GPIO 0x0110 | CLEAR WO | WSTRB所选写1位对OUT清0 |
| GPIO 0x0114 | RISE_EVENT W1C | bits31:0，输入方向的同步针脚上升沿逐位粘滞 |
| Timer 0x0200 | CTRL RW | bit0 enable、bit1 periodic，其余保留 |
| Timer 0x0204 | COUNT RW | bits31:0，模2^32计数 |
| Timer 0x0208 | COMPARE RW | bits31:0，比较值 |
| Timer 0x020c | EVENT W1C | bit0 match，其余保留 |
| IRQ 0x0300 | PENDING W1C | bits4:0，事件脉冲锁存 |
| IRQ 0x0304 | ENABLE RW | bits4:0，仅控制irq输出 |
| IRQ 0x0308 | TEST WO | bits4:0，WSTRB所选写1产生对应软件事件 |
| IRQ 0x030c | RAW RO | bits4:0，当前同步硬件事件脉冲的提交点快照，不含TEST |

窗口末端分别为0x00ff/0x01ff/0x02ff/0x03ff；0x0400–0xffff均DECERR。窗口内未定义、未对齐或访问类型错误为SLVERR。译码不允许截断高地址位形成别名。

UART固定8N1、LSB first、无奇偶校验、TX/RX各8-bit×4寄存器FIFO。TX idle高；enable=0不启动新收发，允许软件预填TX或读取已有RX。启用前配置有效分频；使能且有效分频不足返回SLVERR、不改CTRL。忙时有效写BAUD_DIV或更改enable返回SLVERR；空闲写分频可保存0..2但使能状态下不得改为无效值。所有校验针对WSTRB合并后的值，零WSTRB无作用且不触发busy配置错误。关闭不清FIFO，reset清FIFO/状态。

RX同步后下降沿开始检测，半bit处确认start，随后每bit中心采样8个data及stop；stop低触发framing error并丢弃该帧。RX完成合法帧且FIFO未满时push并产生RX到达；满时丢弃新byte并产生overflow，不覆盖旧数据。TX每次从FIFO取byte进入串行发送器产生一次腾出空间事件，不将线路busy结束当作FIFO释放。TX满时即使同拍取走仍按提交前full拒绝写；RX空时即使同拍到达仍按提交前empty拒绝读。普通非空非满允许同时push/pop。两级RX同步延迟包含在>=3分频预算，边界必须由128.5实测，逻辑仿真不证明板级可靠性。

GPIO reset为输入、OUT=0；双级同步无滤毛刺。上升沿采用前一拍同步值与当前同步值，事件按该沿前DIR选择输入位，reset清同步链与历史值；输入初始高在同步后会形成上升沿。GPIO聚合IRQ为当拍任一新上升沿，不取粘滞RISE_EVENT电平。

IRQ五路严格为0 Timer match、1 UART成功RX到达、2 UART TX取走队首腾空间、3 UART overflow或framing error、4 GPIO新上升沿聚合；没有额外外部IRQ针脚或第6路。异步输入边界说明不构成新增外部IRQ功能。TEST将事件直接并入pending，RAW不含软件TEST；pending清除与硬件/软件set冲突时set优先，重复事件不计数。

Timer软件有效写CTRL/COUNT/COMPARE时保持未被写的计数状态并抑制该拍match；零WSTRB不算软件有效写，不抑制自然计数。EVENT清除不抑制计数或match；同拍match set优先。所有RW部分写均先WSTRB合并。

CSR叶节点固定在提交沿锁存响应，下一周期rsp_valid可见，保持至rsp_ready。只有提交时产生副作用，reset优先于提交；受阻期间无新CSR提交。未来桥读/完整写轮转，reset后同时就绪时优先读，成功提交后优先另一类；无可用响应槽的一类不参与仲裁，不阻塞另一类。CSR error编码同AXI响应：00 OKAY、10 SLVERR、11 DECERR，01不生成。

## 验证与外部核边界

正式故事/AC见 epics.md Phase24 Inventory；适用验证采用历史 implementation-plan.md §5，保存命令、工具版本、种子与结果，分别记录native、真实RTL、综合及formal。缺工具不得skip冒充pass。新系统必须实际层级RTL执行；旧bank本故事不改RTL。

外部核以源清单/完整commit/递归依赖与SHA256、文件顺序/include/宏、端口参数/clock/reset映射、许可证原文hash与NOTICE、wrapper版本、工具锁、维护人及证据为准；CLI构建层获取，prelude不拉源码。官方原生、外部适配、仅收录分别标识；catalogued/locked/compiled/behavior-tested/maintained逐级有证据，native无模型明确unsupported。首选PULP稳定版仍待准入，不以master阅读记录代替稳定版验证。API新增不等于进入FR142稳定清单。

2026-09-20 Story126.2 / FR194 已完成：同一session模块定义复用、参数专门化与实例图/连接诊断；首批Gpio共用定义体，实际RTL验证通过。实现与证据见 docs/ip/module-composition.md 和 docs/ip/module-composition-evidence.md。126.1/126.2 done；126.3/126.4及Epic127–130保持backlog，Epic126/M1未关闭，FR195–201未交付；FR189 deferred/NFR91保持。

## 当前整体执行授权（2026-09-20）

用户明确要求按create-story→ATDD→build→code-review→automate→clean/fmt/regression→commit七步连续处理sprint-status全部未完成Story。此授权取代旧的M0-only或仅126.1/126.2执行安排；各故事仍须满足自身依赖、NFR14及真实验证，一故事一提交。已有done保留；deferred不得假交付。122.2/122.3在2026-09-20实时核验后仍因没有已发布firtool>1.159.0而阻塞；继续其他17个可执行故事，待前置满足后再恢复FR189。整体目标尚未完成，不推送或发布。

## Story126.3 实现范围

FR195 的 RvRegSlice 子集已实现：WIDTH1..64、两槽寄存、无输入组合直通、reset/flush取消、共享定义与独立入口。真实native/RTL及formal证据见 `_agile-output/test-artifacts/126-3-build-evidence.md`。故事状态由后续独立审阅、automate、clean/fmt/regression与单故事提交闭合；ParamSyncFifo/Story126.4未由本实现交付，FR195整体、M1和Phase24不得据此关闭。此前126.2时点的backlog记录保留为历史；当前整体执行授权见上节。

2026-09-20 Story126.3七步完成并单独提交：1732项workspace回归通过，真实formal归纳与四cover通过。仅关闭注册切片子集；参数FIFO/126.4仍待交付，Epic126/M1继续in-progress。

## M1 / Epic126 实施关闭（2026-09-20）

2026-09-20 Story126.4七步完成，随本故事单独提交关闭Epic126/M1：126.1–126.4均done，FR194模块组合与FR195两槽注册切片/参数FIFO交付。clean + fmt + just test退出0，460个实际结果块汇总1763 passed、0 failed、14 ignored；8个专用形式测试另有真实运行证据，6个既有doc-test保持ignored。最终证据 `_agile-output/test-artifacts/126-4-final-verification.md`，关闭映射 `_agile-output/implementation-artifacts/epic-126-closeout.md`。Epic127–130 / FR196–201尚待各自七步与NFR14，整个Phase24未完成；FR189/Epic122 deferred及NFR91保持，不push、不publish。

本节为当前状态，前文M0/126.2/126.3时点的范围记录保留为历史。新增API已逐符号登记FR142，属SemVer minor，未更改包版本或发布。

## Story127.2 CSR叶子集关闭（2026-09-21）

127.1风险门与127.2静态CSR描述/组合RTL/Markdown/C产物七步完成；公开入口与单owner/提交时序见`docs/ip/csr.md`。127.2完整回归1787 passed、0 failed、18 ignored，专用CSR形式/综合4项实际另跑通过，证据`_agile-output/test-artifacts/127-2-final-verification.md`。只交付FR196叶子集，127.3桥/127.4译码及M2仍待验收，Epic127 in-progress；Epic128–130/Phase24整体未交付。新增API逐符号纳入FR142/minor，未发布；FR189/Epic122 deferred及NFR91保持。前述各时点状态保留为历史。

## M2 / Epic127 实施关闭（2026-09-21）

Story127.1–127.4随各自单故事提交完成，FR196静态CSR描述/软件地址产物、AXI-Lite桥和固定四窗CsrDecoder交付。127.4实际clean/fmt/justtest为470结果块、1823 passed / 0 failed / 25 ignored，专用decoder/CSR/bridge形式与综合本轮另跑通过，不把ignored计PASS。关闭映射 `_agile-output/implementation-artifacts/epic-127-closeout.md`，完整验收 `_agile-output/test-artifacts/127-4-final-verification.md`。新增API逐符号登记FR142/minor，未改工具钉、包版本或发布。

真实七模块夹具验证总线/CSR语义，不代表Epic128外设算法或Epic129系统；native/generated层级仍unsupported。Epic128–130/FR197–201及整个Phase24未交付；下一故事128.1 NFR14，全部未完成故事七步授权继续有效。FR189/Epic122 deferred和NFR91保持，不push、不publish。此前各时点交付状态保留为历史。

## Story128.2 Timer实现子集（2026-09-21）

Timer32固定接口已实现，完整契约和prelude-only原文例见[Timer32](timer.md)。实际签名为`Timer::define_module(&mut session, name)`，无配置参数；前述研究形状的`timer_config`不是现行API。单session复用CsrBlock，唯一owner、32位模计数、有效写优先、原始事件/W1C和共同reset保持正式合同。新增四项公开符号逐项登记FR142/minor，未改版本或发布。build验证见`_agile-output/test-artifacts/128-2-build-evidence.md`，七步最终状态以故事记录为准。128.3–5、Epic129–130、FR197整体/M3和Phase24未由此关闭，FR189 deferred/NFR91保持；历史时点记录保留。

128.2最终七步关闭：473个workspace结果块、1833 passed/0 failed/30 ignored；新5个专用入口另有实际PASS。完整验收`_agile-output/test-artifacts/128-2-final-verification.md`。Timer子集随独立提交交付，Epic128/M3及FR197整体仍开放，下一128.3；前述build段保留为历史。
