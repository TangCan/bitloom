# Epic128 / FR197 外设 NFR14 风险记录

日期：2026-09-21。状态：**accepted（build技术风险记录；七步最终关闭仍pending）**。维护owner Richard；实施与证据 Codex；独立审查、状态流转与一故事一提交由主代理负责。没有已知未解决的基座工具或架构表达阻塞；未来外设行为仍须各功能故事实证。accepted不允许绕过128.1 done及下表依赖，不能据此宣布FR197/M3交付。

## (a) 上游约束、支持参数及依赖

正式来源为[Phase24合同](../../docs/ip/phase24-contract.md)的完整决定、[128.1六AC](128-1-外设-nfr14.md)、epics.md Epic128–129、AD-28/30/31与风险模板。历史GPIO8、最多8路IRQ、M0-only/126.2-only叙述不覆盖后续正式合同及用户全部剩余故事七步授权。NFR14指风险门禁，不是历史crates命名别名。

支持单ACLK、data32/addr16/WSTRB4、Timer32、IRQ5、GPIO32、UART8N1/BAUD_DIV32、TX/RX各8×4寄存器FIFO。Rust1.97.1/edition2024、产品firtool1.159.0及Chisel7.15.0不变。设计只依赖bitloom-prelude，唯一FrozenHir；同一ElaborateSession定义全部模块，结束定义后最终只finish一次；完整参数身份、无捕获定义体沿用M1，不冻结后拼网表、不引入第二HIR或tick闭包。native/generated层级仍unsupported，组合验收实际执行RTL。外部IP来源准入属Epic130，本故事不下载、不变更许可或依赖。

固定BFM五包闭包由`scripts/phase24-axi-bfm-requirements.txt`决定：cocotb2.0.1、cocotbext-axi0.1.28、cocotb-bus0.3.0、find-libpython0.5.1、scapy2.7.0。Python3.12.3是本轮实测解释器环境，不是该requirements文件中的包钉；仅使用上游测试主端，不移入设计crate。高层BFM会对齐/拆分，特殊WSTRB/非法对齐需原始通道驱动；二者不可同时驱动针脚。reset返回None是取消。形式入口依据`scripts/ci-sby-pins.env`的yosys-0.47标签及完整SHA，不浮动升级；本轮仅发现形式工具，不将版本输出当证明。

以下表为build实施时快照（128.1当时in-progress）；独立审查时故事已进入review，后续状态以sprint为准。

|目标|正式前置与2026-09-21实际审计|快照时决定|
|---|---|---|
|128.1|125.3/M0 done；125/126/127关闭文档与sprint一致，M1/M2全部done|风险工作可执行；本故事七步未闭合|
|128.2 Timer|128.1、127.2 done；127.2已done，128.1 in-progress|保持backlog，等待128.1关闭|
|128.3 IRQ|128.1、127.2 done；127.2已done，128.1 in-progress|保持backlog，等待128.1关闭|
|128.4 GPIO|128.3、126.2 done，128.1有效完成；126.2已done，128.3 backlog|不得提前ready|
|128.5 UART|128.3、126.4 done，128.1有效完成；126.4已done，128.3 backlog|不得提前ready|
|M3/Epic128|128.1–5全部done|当前仅风险工作，不得关闭；UART单独完成不足|
|129.2系统|129.1、127.4、128.2、128.4、128.5 done；仅127.4已done|保持backlog，不宣称FR198|

上述逐边依据epics.md:10043–10103与sprint Phase24段人工核对。35场景脚本只覆盖M0/.1/epic-close/历史deferred，不覆盖全部跨故事边。FR189/Epic122 deferred、NFR91及所有既有done保留；Epic128 in-progress、128.2–5 backlog，无push/publish。

## (b) 粗工期带

|故事|有效人日|主要估算假设|
|---|---:|---|
|128.1|0.5–1|已有M0/M1/M2基座、固定工具可重用；风险/探针/审查|
|128.2|2–3|Timer单owner及全宽计数/碰撞定向与形式|
|128.3|2–3|五路事件及软件TEST/W1C碰撞|
|128.4|2–3|新GPIO32同步/方向竞争、旧GPIO回归|
|128.5|4–7|全宽UART、双FIFO、串行边界/错误与独立解码|
|合计|10.5–17|中等置信度；不含Epic129/130与新增架构范围|

可单列25%集成预留约13.1–21.3有效人日，不替换基线，不沿用不含风险故事的旧10–16估算。不是代理墙钟时间、生产率或交付日期承诺。UART最小分频与同步采样是主要不确定性；若边界不符按停止条件修复或升级范围，不能默许抬高最小分频。

## (c) 禁止静默降级

不得在不修订批准合同/PRD/脊柱的前提下缩GPIO32、IRQ5、Timer32或BAUD_DIV32，复用旧8位baud端口替代、增加第6路IRQ、删溢出/帧错误/部分写/零mask场景、用粘滞EVENT代替原始脉冲、引入状态双owner、用无限等待隐藏满/空/忙错误、修改共同reset前提或旧API行为。不得把工具发现/BFM stub/旧CSR peer/七模块夹具称作新外设交付，不把skip/ignored/未执行形式或综合当PASS，不用空壳、第二HIR、生成黄金自证或错误被过滤的成功命令冒充验收。新增API须功能故事逐符号登记FR142与SemVer minor；本风险故事不新增API、不改版本、不发布。

## (d) 负责人、失败动作与停止条件

Richard维护需求与架构边界；Codex逐故事实施、失败复现、测试和证据；主代理独立review/automate、clean/fmt/just test、状态核对和单故事提交。每个下表风险由其责任故事实施者Codex修复，Richard为升级owner，主代理执行停止闸门。

|风险|责任/失败动作|停止条件|
|---|---|---|
|单owner、candidate/commit反馈、响应重复副作用|128.2–5；检查状态图和RTL驱动，按提交前候选值重构组合方向，重跑CSR探针与新定向|双owner、组合环或状态更新不能表达时不得开下游|
|Timer软件写/自然计数优先级和32位回绕|128.2；独立模32黄金与碰撞向量，修正唯一next-state方程|回绕/COMPARE0/写优先任一不符不得关闭Timer|
|原始事件和粘滞位混淆、IRQ TEST/RAW混合|128.3及4/5接线；逐路脉冲与clear碰撞，保留失败波形|清IRQ后本地EVENT导致自重触发或增加第6路时停止|
|GPIO同步/沿前DIR竞争|128.4；新32位定义体、明确同步历史，实测bit31与初始高/同拍改向|缩为8位、改变旧端口/FL，或无法解释同步延迟时停止|
|UART32位周期溢出/忙配置/采样边界|128.5；32位倒计时或33位周期算术，合并候选值拒绝；独立串行解码及相位向量|DIV3边界不符、全宽溢出、忙写改变活动配置时保留失败，不缩合同|
|FIFO满/空同拍边界及无效byte0|128.5；使用提交前flags和有效commit驱动FIFO，分别计push/pop及取消|同拍到达绕过空错误、满时覆盖、背压重复副作用时停止|
|工具缺失/版本不兼容、探针失败|128.1及后续每故事；保留命令/UTC/退出码/日志，恢复固定工具或修复授权范围|实际必需工具与基座探针未解失败不得ready，不静默skip|
|记录字段/owner/合同冲突及新增架构范围|主代理与Richard；补有效(a)–(d)，正式合同优先历史；新增范围升级处理|缺字段/owner、批准合同不能表达或新范围未解决时停止|

缺pip的发现命令失败已保留，改用Python标准库importlib.metadata列出完整五包并由BFM逐包断言版本成功；pip不是运行依赖，无需安装或升级。当前无未解决工具阻塞。未来行为验证仍有风险，accepted表示计划与基座可执行，不是所有风险消失。

## 组合方案与共同CSR验收

源码已审核`ip/csr/{mod,codec,rtl}.rs`及`docs/ip/csr.md`：合法owner组合受validate约束，配置全量编码进入共享定义身份。rtl.rs为External RW只声明value输入、不声明leaf存储；candidate只来自当前value/wdata/扩展WSTRB，write_mask不依赖握手；reject由candidate和提交前状态形成，然后决定commit。

数据方向为 `current + wdata + WSTRB → candidate/mask → reject → commit → 上升沿唯一owner状态`，不存在commit回流到candidate/reject的组合路径。`R_*_commit`为沿前组合脉冲，同上升沿消费，不能延迟到响应拍；合法有效mask非零成功写才有write_commit。已复跑三模块peer证明现有接口可用，未证明未来Timer/UART算法。

Timer CTRL/COUNT由外设External RW唯一拥有（CTRL有one-shot自主清除）；COMPARE可由leaf RW唯一保存。GPIO OUT须External RW以合并OUT/SET/CLEAR，DIR可由leaf RW保存并以沿前值判事件。UART CTRL/BAUD可leaf RW加候选值动态reject，STATUS/RX为External RO，TX为WO；每个FIFO只存一份payload。IRQ ENABLE可leaf RW，PENDING leaf W1C事件输入由硬件脉冲与成功TEST commit所选candidate组合，RAW仅硬件脉冲。所有本地EVENT/W1C使用leaf单owner，事件自然更新不依赖CSR提交。这些是可行实现策略；具体功能实现和独立证明留各故事，不是新增API承诺。

共同验收：addr16字节/data32/WSTRB4，小端4字节对齐。四窗UART/GPIO/Timer/IRQ=0000/0100/0200/0300、各0x100；窗内洞/未对齐/权限错误SLVERR，0400–ffff DECERR，不截断高位，无ID/version。所有CSR reset0、CTRL关闭；保留位读0写忽略；RO写、WO读SLVERR，失败读rdata0。RW按字节合并，合法零有效mask写OKAY无副作用且不触发busy拒绝，权限错误仍优先。

唯一提交为非reset上升沿req_valid && req_ready，响应该沿锁存、下一周期valid，背压保持快照且不接新提交，消费沿无refill。所有push/pop/W1C仅一次。reset优先，共同清bridge/leaf/wrapper/槽和在途响应；aresetn断言和释放须外部控制器同步到ACLK，内部反相不是同步器。安全性在无限背压下也须成立；活性另列ready公平性与上界。

## 逐外设精确合同和后续验收责任

以下均为未来必须通过的验收，非本故事产品PASS。

UART精确地址/访问：0x0000 CTRL RW(bit0)，0x0004 BAUD_DIV RW32，0x0008 STATUS RO(bits0..3)，0x000c TX_DATA WO低8，0x0010 RX_DATA RO低8且成功读pop，0x0014 EVENT W1C(bits0..3)。GPIO 0x0100 DIR RW、0x0104 OUT RW、0x0108 IN RO、0x010c SET WO、0x0110 CLEAR WO、0x0114 RISE_EVENT W1C均为32位；下表Timer/IRQ的04/08/0c均为所属0x0200/0x0300窗口局部偏移。所有未列位读0写忽略。
| 责任 | 必须记录的合同与未来可判定验收 |
|---|---|
|128.2 Timer|0x0200 CTRL(enable bit0/periodic bit1)、04 COUNT RW32、08 COMPARE RW32、0c EVENT W1C bit0。关闭保持；有效写CTRL/COUNT/COMPARE该拍保持未被写的计数状态并抑制match，COUNT写优先；零WSTRB不抑制自然计数。普通使能拍先模2^32形成next_count，再与COMPARE比；periodic命中COUNT归0，one-shot清enable。COMPARE0仅回绕命中。EVENT清除不抑制自然计数/match，同拍set优先。验证回绕/比较越过当前值/部分写/reset/碰撞。|
|128.3 IRQ|0x0300 PENDING W1C、04 ENABLE RW、08 TEST WO、0c RAW RO；全部mask0x1f。0 Timer match、1 UART成功RX到达、2 TX取走队首腾空间、3 overflow或framing、4 GPIO新上升沿聚合。pending接新事件脉冲，绝非本地粘滞EVENT；TEST按WSTRB写1加入pending，RAW仅提交点硬件事件快照、不含TEST。clear与硬件/软件set相遇set优先；mask只影响`|(pending & enable)`，不清pending；重复事件不计数。没有额外外部IRQ针脚。|
|128.4 GPIO|GPIO32：DIR/OUT/IN/SET/CLEAR/RISE_EVENT位于00/04/08/0c/10/14；IN RO、SET/CLEAR WO按选中字节写1更新OUT、RISE_EVENT W1C。DIR reset输入，OUT0；针脚双级同步，reset清同步链及历史。前一拍与当前同步值判上升沿，按沿前DIR选择输入位；初始高同步后形成事件。聚合IRQ仅当拍新沿，不是粘滞状态。验证32位含bit31、部分写、方向同拍改变、同步延迟、reset、event/clear碰撞和旧GPIO回归；双级同步不滤毛刺，不证明pad、电气/驱动强度/亚稳态。|
|128.5 UART配置|8N1、LSB first；CTRL bit0 enable；BAUD_DIV RW32，每bit=DIV+1个ACLK，启用须>=3。STATUS bits0 rx_nonempty/1 tx_full/2 tx_busy/3 rx_busy；TX_DATA WO低8、RX_DATA RO低8且成功读pop；EVENT W1C bits0 RX到达/1 TX腾位/2 overflow/3 framing。TX idle高。enable0不启动新收发但可预填TX/读取RX；关闭不清FIFO，reset清FIFO/状态。忙时有效写分频或更改enable返回SLVERR；空闲关闭可保存0..2，启用不能切无效分频；所有检查基于WSTRB合并值，零有效mask不报busy。|
|128.5 UART串行/FIFO|TX/RX各8×4寄存器FIFO。RX双级同步后下降沿检测，半bit确认start，每bit中心采data/stop；stop低丢帧并framing。合法帧未满push并RX事件，满丢新字节并overflow不覆盖；TX取走队首进入发送器才产生腾位事件，不能用线路busy结束替代。提交前RX空即使同拍到达仍SLVERR无pop，TX满即使同拍取走仍拒有效写；未选byte0的TX写始终无push且满时OKAY。普通非空非满可同拍push/pop。真实串行loopback加独立解码，错误帧、相位、baud边界、背压不重复副作用及共同reset验证；已开始物理发送不承诺回滚。|

UART宽分频需特别记录：`BAUD_DIV=0xffffffff`意味着2^32周期/bit，不可将DIV+1放32位helper溢出为0或复用旧8位端口。代表分频做实际串行运行；全宽极值可用明确标识的近终点状态/算术定向或形式验证补充，不冒称已逐拍模拟2^32周期。DIV>=3仅为同步延迟预算起点，128.5必须通过串行采样边界实测；逻辑仿真不证明板级可靠性。


## 验证层次、维护叠加与证据

逐故事采用独立手写关键地址/期望scoreboard，不复用DUT next-state。Timer覆盖写COMPARE跨过当前值后继续到下次自然匹配，COUNT部分写与CTRL/COMPARE有效写抑制计数/match，EVENT清除不抑制。IRQ逐路验证mask不丢事件、重复事件不计数、TEST不进入RAW、本地EVENT未清时清IRQ不自行重触发。GPIO逐位及bit31、SET/CLEAR部分字节、沿前方向变化、两级同步与初始高、reset/clear碰撞。UART实跑DIV3及代表分频、不同RX相位和错误stop；全宽边界单列近终点/算术/形式方法，不称逐拍模拟2^32周期。错误读写均检查无FIFO副作用，响应长背压保持快照；共同reset检查取消账本。

每功能故事按implementation-plan §5保存配置、种子与命令，分别列native两引擎/必要generated、direct/Chisel/FIRRTL实际RTL、层级RTL、formal prove/cover、原始综合及CDC/旧API兼容。129.3核心矩阵不替代128各功能验收。形式proof不混同cover，综合不等于PPA，逻辑仿真不证明板级亚稳态/pad/驱动强度；旧Gpio和UART端口/延迟/手写FL必须回归。

维护叠加：共享CSR/FIFO基座减少重复状态，同时扩大候选拒绝、时序、事件、串行采样与多后端回归面。即使故事串行，旧GPIO8/UART8bit实验用法与新GPIO32/UART32合同长期共存；不能用语义最低公分母压低新合同。新wrapper宜独立定义或有明确兼容证据的共享内核重构；CSR描述与地址文档同源，但黄金独立。既有层级unsupported、工具安装路径易失及/tmp环境需每次探测，跨故事更新须重验相关消费者。四功能owner集中于Codex/Richard，需要主代理审查串行化和25%另列预留，不能把并行代理当免费维护人力。

本轮实际结果见[build证据](../test-artifacts/128-1-build-evidence.md)：固定BFM stub一项通过，真实CSR peer一项通过（2311帧，seed12720a11ce55），状态门禁35场景通过且sprint字节不变。工具版本、所有命令/退出码/UTC及失败均在128-1-build-commands.json与分项log；RTL/stub原始产物tar.gz及SHA256已归档，最终clean不会删除。未执行新外设、formal证明、综合、Chisel/FIRRTL外设或板级测试；不从工具发现推导兼容/证明成功。

build内容可接受；独立review、automate、clean/fmt/完整workspace和单故事commit仍由主流程完成。只有128.1完成后才可依正式边推进；FR197/M3、FR198与Phase24整体仍未交付。

## 128.5 RX采样锚点与奇数周期判据（待功能故事执行）

128.5责任人Codex、维护owner Richard。选定整数实现判据：P=BAUD_DIV+1用足够宽的算术表达，H=floor(P/2)，奇数P取较早中心拍，不每bit重新取半周期。令e为接收状态机在上升沿用**沿前第二级同步值**和其上一拍历史值检测到下降沿的拍号；锚点不是原始针脚下降沿、也不是同沿刚更新的同步级。状态机始终消费同一条第二级同步流。start确认在e+H，data[k]在e+H+(k+1)P（k=0..7），stop在e+H+9P；start确认不低则取消，无RX/framing事件。

128.5须在实现前写独立预期拍表，不复用DUT倒计时helper：DIV3/P4/H2：start偏移2，data偏移6/10/14/18/22/26/30/34，stop38；DIV4/P5/H2：2，7/12/17/22/27/32/37/42，47；DIV5/P6/H3：3，9/15/21/27/33/39/45/51，57。驱动器以独立线端时钟发送0x00/0xff/0x55/0xa6并注入低stop、短start及针脚下降相位0.1/0.5/0.9 ACLK周期；scoreboard从原始针脚按两个寄存级加历史的明确逐拍更新独立计算e，核对采样拍与最终FIFO/event，而非读DUT内部timer当黄金。覆盖最小DIV3及奇数P5，证明同步延迟与该锚点没有多加/少加一拍；若失败保存向量修复，不能抬高最小分频或改成窄baud。全宽极值单列近终点/算术/形式证据，不虚称长周期串行全跑。以上是未来测试实施约定，不是现有产品PASS或板级时序保证。

## 各外设验证适用性（未来责任矩阵）

|责任|standalone及native/generated|hierarchy与实际RTL|formal/综合重点|
|---|---|---|---|
|128.2 Timer|独立无实例的计数/CSR更新内核适用native Interpreter/Compiled；generated仅已有生成器支持的单模块子集，若未执行须单列|CSR+唯一状态peer同session层级只用真实RTL；direct及适用Chisel/FIRRTL寄存器路径记录各自结果|写优先/模32/COMPARE0、set胜clear、快照与背压安全；prove/cover及原始综合分列|
|128.3 IRQ|独立五路事件内核适用两native；generated同上，不用native结果替代|实际CSR+IRQ层级RTL验证TEST/RAW及清除；各RTL后端分列|五路mask不丢pending、硬件/软件set优先、不自重触发；证明与cover分列|
|128.4 GPIO|独立GPIO32同步/方向内核适用两native及受支持的单模块generated；旧GPIO兼容另跑|新CSR+GPIO组合需实际层级RTL，不能以旧GPIO8替代；direct/适用Chisel/FIRRTL分列|沿前DIR、同步历史/初始高、bit31、clear碰撞；逻辑证明不替代CDC/电气签核|
|128.5 UART|独立串行状态内核适用两native及受支持单模块generated；FIFO既有测试只是基座|含双FIFO/CSR的wrapper必须真实层级RTL，独立线端解码及相位/错误/满空碰撞；后端分别报告|busy候选拒绝、全宽算术、FIFO副作用一次/取消账本；有限性质prove/cover不冒充全协议串行证明|

此矩阵不要求新增native/generated层级支持：含实例的standalone入口仍是层级，必须明确unsupported，不因入口名为standalone而绕过限制。若某内核未提供无实例形式，不为测试而扩产品路径；该native/generated单模块项列不适用并用实际层级RTL验收其完整行为。129.3系统矩阵另执行，不能替代逐故事适用验收。所有行当前未执行新产品验证；128.1仍仅有工具发现与旧基座探针。形式固定安装身份前置见128-1-build-evidence.md“工具恢复来源与形式身份门槛”。

## 独立审查补充的同拍与连续输入风险

以下均是后续故事需执行的验收，不是128.1已跑的新外设测试。

- 128.2：CTRL仅bit0/1可写。WSTRB非零但只选保留字节时有效mask仍为0，必须继续自然计数/匹配；独立向量在即将match时写高字节，与真正写低字节抑制match形成对照。
- 128.4：DIR=1输出、0输入。IN始终为双级同步的实际针脚值，不是原始针脚或OUT锁存，包含DIR=1的位；方向仅影响驱动与事件筛选。向量令OUT与针脚相反、翻转bit31，逐拍核对IN延迟及沿前DIR筛选。
- 128.5：提交前RX满且同拍成功软件pop与串行新帧到达时，旧队首读出并pop，新帧仍按原full丢弃并产生overflow，不产生成功RX到达；不得以同拍腾位接纳新byte。与普通非满push/pop及空读/到达反例分别核对。
- 128.5：将空闲配置提交与TX启动/RX下降检测的竞争列为强制实现设计检查。选定软件配置在该沿生效：成功CTRL/BAUD写形成有效配置，当前仍空闲而同拍启动的新帧采用此配置；成功disable不得启动新帧。拒绝写、零有效mask写不改变有效配置。新帧须锁存使用的分频，后续busy拒绝不能让活动帧漂移。独立向量覆盖TX预填队列与RX下降同拍的enable/disable/DIV变化，核对整帧每位周期而非仅寄存器读回；实现前在128.5规格中写出配置→启动的无环组合方向，不以此计划声称现有RTL已支持。
- 128.5：RX双同步级及历史值随共同reset清0，与现有同步复位寄存器路径一致；从idle-high解除reset只有同步0→1，不得制造下降沿/start/RX事件。reset期间低输入也不得凭reset释放虚构下降沿；须先看到高再下降才检测新start。验证释放后的首个合法帧与reset中断帧取消。
- 128.5：连续至少两帧，仅一个合法stop bit、没有额外idle，覆盖DIV3和奇数周期DIV4以及各RX相位，必须依序收到全部byte；独立发送器不等待DUT ready/busy，检测stop采样后重新接收过晚的问题。
