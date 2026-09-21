# Epic127 / FR196 CSR 与总线 NFR14 风险记录

记录日期：2026-09-21（探针 UTC 见各原始日志）。状态：accepted（2026-09-21独立审阅及回归完成，随127.1单故事提交生效）。下文build时点记录保留为历史，当前状态见末节。范围为FR196/M2开工风险，**尚无新CSR/桥/译码交付**。模板(a)–(d)均填写；是否最终有效由独立审阅裁定。

## (a) 上游约束与依赖

权威为[正式合同](../../docs/ip/phase24-contract.md)与[Story127.1](127-1-csr-总线-nfr14.md)六项AC；本记录不修改合同。M0/125.3与M1/126.1–4已done，分别见[125关闭](epic-125-closeout.md)、[126关闭](epic-126-closeout.md)。build时真实sprint：epic-127和127.1 in-progress；127.2–4 backlog。FR189/Epic122与122.2/122.3 deferred、NFR91保留。旧M0-only注记是历史，以整体七步授权为准。

| 下一故事 | 必须满足的前置 | 交付/验证责任 |
|---|---|---|
|127.1|125.3/M0关闭，已审M1关闭|风险正文、工具发现与BFM工具探针、状态门禁；主流程七步|
|127.2|127.1 done + 126.2 done|静态描述、CSR leaf RTL、地址文档/C头文件及独立黄金值|
|127.3|127.2 done + 126.3 done|独立AW/W/AR捕获、CSR桥、背压、轮转与取消|
|127.4|127.3 done|四窗静态译码、实际组合RTL及M2完整验收|

这些跨故事依赖经人工比对；`check_phase24_gate.py`只核验M0、各Epic .1和关闭状态，**没有检查全部127.2→127.3→127.4依赖**。27个临时状态场景成功、真实sprint字节与SHA256未变，详见[build gate日志](../test-artifacts/127-1-build-gate.log)。其中21个未done风险状态×功能ready负例，3个满足前置正例，当前状态正例、M0未关闭负例及Epic127提前关闭负例均核对精确返回码与诊断。

沿用AD-1/6/7/13/18/30/31：设计仅依赖bitloom-prelude；一个ElaborateSession和FrozenHir，finish一次。描述是elaboration配置，不是第二硬件IR或运行时硬件对象；不使用capturing closure、全局可变表、网络插件。现有模块定义不在活动模块内部递归创建。单时钟同步reset；native/generated多模块层级仍unsupported，以实际RTL验证组合。Rust1.97.1/edition2024、firtool1.159.0、Chisel7.15.0产品钉不变。新API须在功能故事逐符号登记FR142并说明SemVer minor，本风险故事没有API。

### 非捕获builder的具体适配方案（127.2实现）

实际`crates/bitloom-builder/src/lib.rs:127`签名为`define_module(name, Vec<(String,u32)>, fn(&mut ElaborateSession, &[(String,u32)]) -> Result<(),Diagnostics>)`；键排序、重键拒绝，每次运行body，复用核对完整参数与完整HIR（含span），不是函数地址或摘要。

方案是验证静态配置后按offset规范排序，用固定结构键编码完整描述：`schema_version`、`reg_count`；每个索引的`offset/mask/reset/access/event`用u32值；名称用`name_len`与逐UTF-8 byte的`name_byte_0000`等索引键编码（值0..255），多个字段亦存完整count和索引。端口/字段名先校验为合法且唯一的HDL/C标识符，拒绝非法配置，不静默清洗造成碰撞。127.2须拒绝目标语言保留字，并在模块端口/内部生成名及C宏各自命名空间检测碰撞；宏大小写转换后的重名也须诊断，不能依赖源名字大小写不同。非捕获函数从索引键重建经过校验的配置，再调用共享body声明端口/过程；不begin/end模块。u32能容纳全部offset16、mask32和编码枚举；任意允许长度的名字可用有限键列表表达。使用确定性span与稳定排序，同名不同完整参数由现有诊断拒绝；摘要仅可作显示辅助，不能代替完整身份。所有name/access/event字段必须纳入重建和身份；不能只编码寄存器数量或地址。配置只在elaboration内重建/丢弃，不进入tick。

127.2需验证往返重建、同配置复用、只改名称/事件/掩码仍能区分、重复键与非法枚举、未知schema版本/缺失索引/计数不一致/name byte超出0..255、重叠/超界/未对齐offset、无效字段mask/reset及标识符碰撞；使用Diagnostics，不panic后继续或自动裁剪。该方案在现有参数类型上可表达，无需先扩builder公共API；实现证据尚未产生，若实现发现不可表达则停止并给出最小扩展分析。

### 可写状态所有权与访问粒度（127.2定义，128适配）

首版每个寄存器只有一种RW/RO/WO/W1C访问类型，field mask不暗示同一寄存器内混合权限；描述包含不同访问类型字段时诊断拒绝，127.2补负例。合同现有寄存器均可表达，不扩大到混合权限产品。

每个可写状态在elaboration时明确且只选一个owner：普通独立bank可由CSR leaf保存；有自主更新或别名写的外设寄存器由wrapper保存，leaf只读取wrapper当前值，并在成功提交时输出字节合并后的写值/选位及一次提交脉冲，不再保存第二份状态。127.2需通过测试peer证明外部状态读回、部分写与自主更新冲突；128中Timer的COUNT/CTRL及GPIO OUT使用外部owner。wrapper按正式合同单个next-state方程合并软件与硬件更新：COUNT软件写胜自动计数，CTRL/COUNT/COMPARE有效写抑制match，GPIO SET/CLEAR更新同一OUT状态；W1C由选定唯一owner以set优先公式更新。外部owner共同reset，禁止双重寄存副本相互覆盖。具体公开端口名在127.2实现规格确定，当前不登记API。

### 接口与风险矩阵

以下是未来实现的验收合同，不是当前通过的产品测试。

| 风险 | 固定行为 / 失败动作 | 责任故事与验收 |
|---|---|---|
|同源产物错误放大|Rust静态offset、field mask、reset、access、event映射同时生成RTL、地址文档和C头文件；规范顺序确定性。黄金地址/值/副作用由独立手写，不能复用DUT译码或下一状态|127.2对重复生成逐字节比较，独立黄金文件/行为比较，并编译最小C消费者及多个生成头文件共同包含的消费者；任一不同停止，不改oracle迁就DUT|
|提交点重复执行|请求req_valid/req_ready、write、addr16字节地址、wdata32、wstrb4；响应rsp_valid/rsp_ready、rdata32、error2。非reset上升沿req_valid&&req_ready唯一提交；全CSR最多一个已提交未消费请求。提交沿读快照锁存，下一周期rsp_valid，保持到rsp_ready；受阻不再提交|127.2统计写/W1C/RX pop/TX push恰好一次，验证下一周期响应及快照稳定；reset优先提交|
|字节/权限/事件|32位4字节对齐little-endian，WSTRB[n]选择wdata[8n+7:8n]；RW合并；保留位读0写忽略。所有CSR reset0，动态RO reset0释放后采样。RO写/WO读SLVERR；失败读rdata0，错误无访问副作用。合法RW/W1C/WO零WSTRB写OKAY无副作用，地址/权限错误优先。W1C `next=(old & ~clear)|event`，set优先，重复事件仅pending不计数|127.2全部16种WSTRB、mask与事件碰撞、零WSTRB权限错误黄金序列；错误不会压掉独立硬件自然事件|
|动态leaf连接|描述提供明确RO状态/事件输入、动态拒绝输入及提交副作用输出；wrapper提供状态、将成功提交脉冲连接FIFO/外设。校验、读快照和副作用必须用同一提交前状态，不在响应拍重算。不能把只有RW bank叫作完整外设基础|127.2以测试peer验证动态RO、WO、W1C和拒绝；128实现真实wrapper，不提前称交付|
|动态错误优先级|RX空读/TX满有效写SLVERR且不pop/push，有限响应不无限等。TX_DATA未选byte0不push、满时也OKAY；UART零WSTRB不触发busy错误，配置检验针对合并值。提交前full/empty决定拒绝，同拍TX取走/RX到达不豁免；普通非满非空可同时push/pop|127.2接口peer覆盖，128.5真实串行/FIFO边界验证；若wrapper需要无限等待则停止另立超时/取消合同|
|地址别名/双选|error只00 OKAY/10 SLVERR/11 DECERR，不生成01。UART/GPIO/Timer/IRQ基址0x0000/0x0100/0x0200/0x0300，各0x100字节，末端0x00ff/0x01ff/0x02ff/0x03ff；0x0400–0xffff DECERR。窗内未定义/未对齐/非法访问SLVERR、无副作用；无ID/version。高地址不截断，配置重叠/超界编译时拒绝|127.4手写基址、末端、±1、03fc/03ff/0400/ffff、0018与高地址别名用例，断言onehot-or-zero选择和响应路由|
|AW/W配错或读饥饿|AW/W/AR各独立一槽，AW早/W早/同拍均可；收齐AW+W才具写资格，不完整写不堵读。外部ready/valid由寄存状态决定，禁止组合依赖对端valid/ready。AWPROT/ARPROT各3位保留但忽略，不宣称权限隔离|127.3除原始通道驱动外检查HIR/生成网表的端口依赖锥，确认外部ready/valid无输入组合路径；仅稳定性波形不能替代结构检查。原始驱动间隔0/1/7/31拍、并发读写；队列独立记录地址/数据，不能靠高层BFM自动对齐掩盖错误|
|响应覆盖/轮转|B/R各有保持状态，CSR提交前预留对应空间；全局一个CSR执行中，桥槽占用不是CSR提交。reset后同时合格先读，成功提交翻转优先；无响应槽的一类不参与，也不阻另一类|127.3分别停B/R、并发及容量边界，受阻payload稳定、无重复响应；127.4组合复验|
|无限背压|BREADY/RREADY可永久0，安全性无公平性假设；活性/完成上界必须显式列最大ready等待/公平性和证明深度，不能以有限随机测试声称无饥饿证明|127.3记录安全断言与有界活性假设分开；计划16seed×1000事务仅预算，实际seed/命中/墙钟另存|
|reset/取消|板级控制器把aresetn断言和释放都同步到ACLK后内部转同步高有效；反相不是同步器。bridge和所有leaf同域共同reset，优先提交，清AW/W/AR及待响应；禁止只复位bridge；已发UART物理位不承诺回滚|127.3覆盖仅AW、仅W、AR、待CSR、提交待B/R、停顿响应与恢复首拍；接受/提交/响应/取消分别计数，127.4组合重验|
|旧bank被新合同覆盖|旧Axi4LiteSlave ADDR8、四寄存器、恒OKAY、原端口/reset/映射、同拍AW/W一tick响应及同址read-before-write不变；新CSR16bit与错误译码为独立表面|旧接口已核对`ip/axi.rs`，127.2–4保留M0兼容向量，不能替换旧bank或将M0的通过当CSR证明|

### 对后续外设的完整衔接边界

所有位分配/偏移以正式合同“CSR字段与访问的完整决定”表为准。UART窗口00/04/08/0c/10/14对应CTRL(bit0)、BAUD_DIV(32)、STATUS(4位RO)、TX_DATA(低8 WO)、RX_DATA(低8 RO读pop)、EVENT(4位W1C)；GPIO窗口同六偏移为DIR/OUT/IN/SET/CLEAR/RISE_EVENT，均32位，IN RO、SET/CLEAR WO；Timer00/04/08/0c为CTRL(2位)、COUNT32、COMPARE32、EVENT(bit0)；IRQ00/04/08/0c为PENDING/ENABLE/TEST/RAW，均5有效位，TEST WO、RAW RO。没有第6路IRQ或额外外部IRQ针脚。

UART8N1/LSB first，无parity，TX/RX各8×4寄存器FIFO；idle高，enable0不启动新收发但可预填TX/读取RX。BAUD_DIV+1为每bit拍数，开启要求>=3；闲时禁用可保存0..2，启用不能改为无效值；忙时有效写分频/改变enable报SLVERR；零WSTRB不报busy。关闭保FIFO、reset清FIFO/状态。同步RX下降沿检测、半bit确认start、每bit中心采data/stop；stop低报framing并弃帧，满时弃新字节报overflow不覆盖旧数据。TX取队首时产生腾空间事件，不等于busy结束。128.5负责>=3分频与同步延迟预算边界，不据逻辑仿真宣称板级可靠性。

GPIO reset方向输入、OUT0；双级同步不滤毛刺，reset清同步链/历史。按前一拍与当前同步值检测上升沿，按该沿前DIR选择输入位，初始高同步后可产生沿；聚合IRQ取当拍新事件，不能取粘滞状态。Timer32模计数，关闭保持，COUNT写优先；有效CTRL/COUNT/COMPARE软件写保持未写计数状态并抑制该拍match，零WSTRB不抑制自然计数。普通使能拍先算next_count再与COMPARE比较；periodic命中COUNT0，one-shot清enable；COMPARE0仅回绕命中。EVENT清除不抑制计数/match，同拍set优先。128对应故事负责溢出、跨比较值写及W1C碰撞。

IRQ路0 Timer match、1合法RX到达、2TX取字腾空间、3overflow或framing、4GPIO新沿聚合；pending锁存脉冲，不接本地粘滞EVENT电平。TEST直接并入pending，RAW仅同步硬件事件提交快照、不含TEST；mask仅影响`|(pending & enable)`，不清pending；硬件/软件set胜清除，重复不计数。同步输入不构成pad/开漏/驱动强度/板级签核。上述产品功能均归Epic128，127只确保可连接接口与有限错误响应。

## (b) 粗工期带与维护成本

127.1为0.5–1、127.2为4–6、127.3为5–7、127.4为3–5有效人日，总12.5–19；若加25%预留单列15.6–23.8人日。不是代理墙钟、发布日期或生产率承诺。假设现有共享模块/诊断/直接RTL工具可复用、一名维护owner串行裁定、独立review可用，无新硬件IR、pin升级、层级native或额外协议需求。

维护面包括配置合法性、三种产物确定性、独立oracle、CSR语义、桥五通道、译码组合、旧AXI bank、未来四leaf wrapper与软件头文件同步；组合×时序×mask使回归膨胀。每次修改先定向重跑触及矩阵，再workspace；RTL/formal/综合单列，不能以cargo结果覆盖。保留种子、超时、取消计数和失败向量；随机预算16×1000不代表统计保证。正式CSR安全proof/cover与原始RTL综合在功能故事建立独立harness，记录实际深度/工具/资源。127.2负责提交一次/响应稳定/错误无访问副作用安全证明及可达提交cover；127.3负责AW/W配对/响应防覆盖安全证明及读写竞争、背压恢复cover；127.4负责选择onehot-or-zero/响应路由安全证明及四窗命中、未命中cover。活性证明必须另列公平性假设；cover不是安全证明。PPA没有目标和证据，不能声称提升。若扩并行故事，Richard先评估oracle与工具维护叠加，不以最小公共语义删减合同。

## (c) 禁止静默降级与停止动作

不得以地址截断、忽略WSTRB、恒OKAY、轮转省略、无限leaf等待、响应拍副作用、捕获closure或第二HIR满足FR196；不得把新CSR16bit倒套旧bank、把版本输出/stub/历史M0/M1当新产品RTL/formal成功，或以ignored/skip替代缺证据。不得在不改PRD/脊柱和明确授权的前提下交付burst/ID/多主、异步域/深RAM/通用内存、CPU/DMA/外部复杂核或扩大稳定API与工具pin。既定合同不重提审批。

| 停止触发 | 保存与修复/升级动作 | 放行条件 |
|---|---|---|
|owner缺位/合同冲突/描述无法表达|Codex保存最小配置/冲突条款；Richard裁定最小方案，确需新范围才升级|明确owner、无未解决架构冲突；后续保持backlog|
|工具缺失、版本不兼容、probe失败|保存命令/退出码/日志，按固定requirements或ci-install-sby.sh恢复环境，不改产品钉；重跑真实probe|真实成功且失败记录仍可见，不能skip|
|oracle与RTL分歧、重复副作用、响应丢失|保存种子/向量/波形，修复实现或证明oracle错误并独立review|同一失败向量重跑成功及相关回归|
|需要等待外部设备、扩大FR142/pin/异步域|记录尚未批准的新成本/取消协议/兼容性影响，升级Richard|新范围明确决定前不推进依赖故事|

当前发现唯一探测误用是环境没有pip模块（exit1），不是固定包缺失：这是uv建立的无pip运行环境。已改用标准`importlib.metadata`查询五个固定包并审阅安装源码（exit0），BFM本身也逐版本断言并真实运行成功；无需安装pip或改版本。原失败日志保留。无待解决的工具发现或架构表达阻塞；产品RTL/formal/综合尚未实施，不应误填通过。独立review、automate、clean/fmt/just test及单故事commit未由build执行，M13待主流程，127.2仍backlog。

## (d) 负责人及验收映射

Richard为维护/范围owner；Codex负责实现、工具修复、原始证据与后续矩阵；独立reviewer负责内容有效性，不由build自签通过；主代理负责七步串行、状态、单故事commit。问题先交Richard/主代理并保持相关后续backlog，不另找未授权维护者。

AC1对应上游/依赖与M0/M1索引；AC2对应(a)–(d)、适配/维护/停止矩阵；AC3对应接口与外设边界；AC4对应[实测汇总](../test-artifacts/127-1-build-evidence.md)及原始日志；AC5对应27场景日志/真实sprint保护；AC6由[人工清单](../test-artifacts/127-1-atdd-manual-acceptance.md)及后续七步证据关闭，当前M13未完成。FR196/M2/Epic127和Phase24整体仍未交付，不push、不publish，与samitbasu/rhdl无关。

## 127.1 当前关闭状态（2026-09-21）

风险内容经三路build审查、四层独立review及主代理M01–M14核对；automate117场景/固定BFM通过，完整回归1763 passed、0 failed、14 ignored，退出0。仅127.1 done，127.2–4 backlog，Epic127 in-progress，FR196/M2未交付。七步记录见[最终验证](../test-artifacts/127-1-final-verification.md)，单故事提交由包含本文件的git记录核验。上文待审/未执行叙述为build时点历史。
