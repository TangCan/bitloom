# Story 128.4: GPIO CSR wrapper

Status: done

## Story

作为实现/集成维护者，我希望交付可组合的32位GPIO CSR wrapper，以便软件通过DIR/OUT/IN/SET/CLEAR/RISE_EVENT控制与观察针脚，并把新上升沿送入已交付的五路IRQ，同时保留旧8位GPIO接口与手写FL。

估算2–3有效人日；维护owner Richard，实施与证据Codex，主代理执行连续七步及一故事一提交。估算不是代理墙钟时间承诺。

## Acceptance Criteria

1. **前置、API与组合。Given** 128.3、126.2及128.1 done、Epic128 NFR14有效，**When** 构建GPIO32 wrapper，**Then** 设计仅依赖bitloom-prelude，新增独立公开类型、静态CSR描述入口、共享session定义入口及共用定义体的Elaboratable入口；一次finish/freeze，复用CsrBlock，不拼FrozenHir、不建第二HIR。同定义复用、重命名定义、实例隔离、名字冲突/宽向错误沿用既有诊断。具体类型名/签名/端口名/描述字段由ATDD先冻结，本文建议不当作已经存在的API。
2. **32位寄存器与唯一owner。Given** 本地偏移00 DIR RW、04 OUT RW、08 IN RO、0c SET WO、10 CLEAR WO、14 RISE_EVENT W1C，**When** 经addr16/data32/WSTRB4访问，**Then** 全部32位有效、reset0；DIR=1输出/0输入，OUT保持数据，SET/CLEAR按WSTRB所选写1位更新同一份OUT。OUT由wrapper唯一拥有，DIR/RISE_EVENT可由CSR唯一拥有；SET/CLEAR不各存副本。IN始终读取双级同步实际针脚，包括输出方向，绝不以OUT代替。输出数据与输出使能的精确端口表达在ATDD钉死并文档化，不宣称实际三态pad/驱动强度。
3. **同步和沿前方向。Given** 沿前状态sync1、sync2、history、DIR，**When** 非reset上升沿，**Then** `sync1'=pad_in; sync2'=sync1; history'=sync2`，本沿事件为`rise=sync2 & ~history & ~DIR`；本沿IN读快照为沿前sync2。DIR写同沿用旧DIR筛事件，方向变化本身不能制造针脚上升沿。reset清同步链/历史，初始高在传播后产生一次事件；持续高、下降沿不重复置新事件，低后再高可以再产生。测试须明确针脚采样、同步输出、事件锁存各自拍号，不能只断言最终值或说“两拍”。
4. **事件与IRQ集成。Given** RISE_EVENT粘滞状态及真实新沿，**When** W1C清除/总线背压/IRQ清除相遇，**Then** `event_next=(event_old & ~committed_clear)|rise`，set胜clear；自然事件不依赖总线提交且不受响应背压阻塞。聚合原始输出为任一当拍新rise，供本地W1C和IRQ bit4同沿采样，不能接粘滞RISE_EVENT；复位期间不作为有效事件。真实GPIO→Irq RTL夹具验证：GPIO本地事件仍高时清IRQ不会自重触发，下一新沿仍能置IRQ4；五路映射不变，不增第6路。
5. **提交、错误及软件产物。Given** 非reset req_valid&&req_ready为唯一软件提交，**When** 合法或非法访问，**Then** 保持CSR既有提交沿快照、下一周期响应、背压稳定、消费沿无refill和reset优先取消；软件操作只执行一次。RW字节合并，SET/CLEAR/W1C只影响选中字节写1位；合法零WSTRB写OKAY无软件副作用，RO写（含零WSTRB）/WO读/未定义/未对齐/高地址别名均SLVERR，所有写响应及失败读rdata0。局部offset与系统基址0x0100分开，wrapper不截高位或代做decoder的DECERR。静态描述同源生成确定性Markdown/C头，关键黄金地址与期望独立手写，原文prelude-only示例真实编译/展开，C11头消费者真实编译。
6. **兼容、共同reset及边界。Given** 旧Gpio8及GpioFunctional已交付，**When** 新增wrapper，**Then** 旧类型/端口/组合pad读取/掩码写/FL/reset行为全部保留并回归；新GPIO32不假借旧8位产品完成。leaf、wrapper、集成IRQ同域同步高有效reset，清DIR/OUT/本地事件/同步级/历史/待响应；reset胜提交/上升沿，取消账本明确。双级同步是逻辑结构，不提供去抖/滤毛刺、多位相干采样、亚稳态MTBF或pad/电气/板级签核保证。系统aresetn须外部控制器同步断言和释放后转换，反相不是同步器。
7. **实际验证与交付边界。Given** 独立scoreboard及有限形式性质，**When** 七步完成，**Then** direct/FIRRTL/Chisel真实执行RTL分别保存结果，formal prove/cover/原始综合单列，负控制须检出实际DUT故障。适用native/generated执行；含Instance入口仍明确unsupported，不为测试扩产品路径。新增API逐符号登记FR142/minor，CI接普通/后端/形式/示例门禁及失败工件；实际cargo clean→cargo fmt --all→just test全workspace及必要专用检查完成，单独提交。本故事只关闭GPIO子集，128.5、M3/FR197整体、Epic129–130及整个Phase24保持未交付；FR189 deferred/NFR91保持，不push/publish、不改工具钉或版本。

## Tasks / Subtasks

- [x] T1 ATDD冻结与RED（AC1–7）：类型/方法/端口/描述命名、输出数据/使能、reset下原始事件输出和逐拍采样表；独立关键地址、32位模型、必达计数与测试映射。缺API编译RED只表明接口缺失。
- [x] T2 GPIO32共享CSR实现（AC1–6）：新独立wrapper、唯一OUT状态、三软件写候选合并，DIR/事件leaf、IN external、双同步级和历史；原始事件与粘滞分离；不得默改旧GPIO或CSR通用语义。
- [x] T3 真实验证（AC2–7）：全32位/16WSTRB/部分写/方向竞争/同步/初始高/reset/碰撞/背压/错误；同定义双实例、重命名双实例及真实GPIO→IRQ图三后端；原始RTL综合与独立formal/cover/行为变异负控制。
- [x] T4 软件与维护（AC1、5–7）：确定性软件产物、原文prelude-only例、C11消费者、旧GPIO/FL回归、FR142逐符号/minor、专用CI及可移植证据入口。
- [x] T5 七步闭合（AC7）：build内部审查后独立code-review→automate→实际clean/fmt/workspace回归与必要专用门禁→128.4单提交；在clean前归档并校验本次原始证据。

## Dev Notes

### 基线、依赖与范围

基线`433771cdf62fba13870ed79b22880c23e5211e59`（Story128.3）。当前445故事done、8backlog、2deferred；128.1/2/3、126.2、127.2均done，Epic128 in-progress；仅128.4由backlog进入ready-for-dev。NFR14正文保留的128.1 in-progress和128.3 backlog为历史快照，不覆盖当前sprint。用户全部未完故事七步连续授权覆盖旧M0-only/126.2-only限制及例行确认。

Epic128目标是Timer/IRQ/GPIO/UART四外设：128.2与128.3已关闭，128.5缓冲UART仍backlog，其依赖128.3/126.4已满足但不在本故事实现。M3须128.1–5全部done。129.2还需129.1、127.4、128.2、128.4、128.5，不将局部GPIO→IRQ夹具称完整系统。

### CSR owner与无环接线

|local|系统地址|名称/访问|owner建议|行为|
|---|---|---|---|---|
|00|0100|DIR RW|Leaf|32位方向，reset输入|
|04|0104|OUT RW|External|wrapper唯一OUT寄存器|
|08|0108|IN RO|External|沿前sync2快照，方向不替代输入|
|0c|010c|SET WO|None|按mask写1置OUT|
|10|0110|CLEAR WO|None|按mask写1清OUT|
|14|0114|RISE_EVENT W1C|Leaf|32位输入方向新上升沿，set胜clear|

全mask0xffffffff，无保留位、无ID/version。建议新类型如`GpioCsr`，`registers()->CsrBlock`、`define_module(&mut ElaborateSession, impl Into<String>)->Result<String,Diagnostics>`、Elaboratable；建议放`ip/gpio/csr.rs`并在gpio/mod.rs再导出。**这些是建议，尚未冻结；不得在ATDD之前宣称为现行API。** 12个标准CSR端口复用既有拼写，针脚32位、原始事件1位，输出数据/使能端口由ATDD固定。

已全文读CSR rtl.rs：External RW的out_value输入来自wrapper当前OUT，candidate已按WSTRB合并；SET/CLEAR WO candidate为wdata&write_mask，**没有value端口**。只在相应write_commit更新OUT。三写互斥：单CSR请求不能同沿同时OUT/SET/CLEAR，不构造不可达多写cover，不发明优先级作为公开承诺。当前value/wdata/WSTRB→candidate→成功commit→唯一OUT next-state；不能让candidate依赖commit形成环。DIR和RISE_EVENT的value由leaf输出，wrapper不另存一份。IN external是sync2，无candidate；所有真实生成端口合法连接。

同步拍表以非reset且初始全部0、针脚自edge e前保持高为例：e后sync1=1/sync2=0/history=0；e+1后sync1=1/sync2=1/history=0，此时组合rise为1；e+2采样沿前rise，将本地事件及已接IRQ置位，同时history'=1；e+2后的rise回0。IN在e+1沿提交读仍返回0，在e+2沿提交读返回1。改变DIR的提交沿必须取旧DIR；使用sync2'计算事件会提前一拍，额外寄存raw又会迟一拍。独立模型须从原始pad_in演算，不读DUT内部同步状态当黄金。

### 必达向量

|风险|定向要求|
|---|---|
|宽度/映射|逐bit0..31 one-hot，包括bit31与各字节边界7/8/15/16/23/24；全零、全一、交错值；独立地址黄金，0x0100及0x8000不得alias local0|
|WSTRB|全部16种×DIR/OUT/SET/CLEAR/RISE_EVENT；SET从0与混合OUT开始，CLEAR从全1与混合OUT开始；W1C从已置位状态再制造同位/异位新沿；写0与零WSTRB分开，避免幂等初值掩盖故障|
|针脚/IN|OUT与pad_in相反，DIR输入/输出/混合，IN都保持实际sync2；raw针脚变化不能直接透到IN；逐拍验证初始高、持续高、下降、低后再高、reset中变化|
|方向竞争|上升事件采样沿DIR输入→输出仍记录，输出→输入不记录该沿；稳定高时改方向不得凭空产生事件；IN不随DIR代换|
|输出|ATDD固定数据/使能定义后，OUT/SET/CLEAR及DIR独立改变的每拍结果，不能仅检查CSR读回|
|软件提交|req_valid=0但SET/CLEAR地址数据变化不作用；响应stall及持续合法valid不重放；消费沿无refill；提交/消费/reset取消账本守恒|
|自然事件|响应stall时仍锁存新沿；同位W1C与新沿set胜clear，异位clear/rise并发；RISE_EVENT读与事件同沿返回沿前值|
|错误|IN写包括WSTRB0、SET/CLEAR读、洞0x18/窗口末端、未对齐和高位地址；SLVERR/rdata0无软件变化，自然事件仍正常|
|reset|非零DIR/OUT/events、部分同步传播、初始高、受阻读写响应、有效写与新沿同时reset；响应取消，之后真实新沿可用|
|集成|真实GPIO原始聚合仅接IRQ4，其余四位0；本地RISE_EVENT保持高，清IRQ后不重触发；下一新沿置IRQ；IRQ mask不清pending|
|复用|同定义双实例、不同名字共享私有CSR定义，非对称输入/CSR背压/状态；同一session一次finish，隔离及共同reset|
|兼容|旧Gpio8精确端口、pad_out=out&dir、rd_data=(out&dir)|(pad_in&~dir)、masked write与reset；现有手写GpioFunctional对照及已有GPIO VIP/SocPad/ChipPadRing回归|

### 验证与证据纪律

- 无实例内核若实际存在，适用Interpreter/Compiled/generated分别跑；含CSR实例的standalone入口仍是层级，维持明确拒绝诊断，不为测试新增native层级模拟。三种RTL后端须运行同一独立scoreboard，而非只emit/源码断言。各组合图也记录实际覆盖后端。
- formal使用独立sync1/sync2/history/dir/out/event/response ghost状态，对状态等价和行为做assert而非assume。只假设初始reset与合法受阻请求保持；pad输入按采样拍可任意变化，安全性不要求最终rsp_ready。prove与cover分列，cover含bit31、两种方向竞争、初始高、W1C碰撞、长背压新沿、reset取消，不要求不可达同端口双写。
- 以真实DUT变异检验observer和scoreboard：如IN误接OUT、同步旁路/采样提前、使用新DIR、clear胜set、SET/CLEAR忽略WSTRB或原始IRQ误接sticky。选取代表故障，原始控制必须PASS；变异必须在指定行为断言失败并有VCD，编译失败/timeout/UNKNOWN不计检出。测试黄金不能复用DUT next-state或描述生成地址。
- 原始无observer生成RTL做Yosys synth/check，记录实际版本/cells、无latch/多驱动。证明逻辑同步行为不等于模拟亚稳态或板级签核；多位针脚独立同步不保证原子字。
- 固定Rust1.97.1/edition2024、产品firtool1.159.0、Chisel7.15.0；Yosys/Z3实际版本单列，不把主机版本记录当新锁。SBY按scripts/ci-sby-pins.env核验官方origin、tag对象bfc1c47eb786496fe794481ff88e75728f0529a6、commit daed0e1544fd96ee7dab843e5a891d92784c6230、tracked源码及安装支持模块字节。复用128.3隔离Python加载/缓存校验，不能只看--version。
- runner依PATH/RHDL_FIRTOOL_PATH/BITLOOM_SBY_SOURCE，可移植且每次新目录。每命令记录UTC/参数/退出/耗时/日志，launch失败也记录；exact过滤必须核对目标名和实际执行数，cargo零匹配exit0不是PASS。普通与-O测试证据脚本失败分支，并接CI。
- 归档显式选择同一次完整run；起点/终点/当前相关源码SHA一致。指纹包含真实源码/工具配置/Justfile，排除target、缓存、生成证据及ignored SBY `crates/rhdl-formal/fixtures/fr119/fr119_pass/logfile.txt`，不能把运行日志变化误判产品改动。历史PASS并集不能替代当前run，formal必须验证关联状态/波形，示例比较作为独立gate；保护归档和manifest不覆盖，归档成员与源逐字节校验。clean前持久保存原始产物，原始日志不为whitespace检查改写。
- 普通workspace、独立JVM/形式/综合/软件示例分别接CI，保留失败工件；本地等价检查与远端CI实际执行须区分。最后实际clean/fmt/just test，ignored单列并实际跑专用入口，不用历史1844 PASS替代本故事回归。

### 前故事与Git经验

最近五提交：128.3 IRQ `433771c`、128.2 Timer `9d2aff7`、128.1 NFR14 `1c4cf20`、127.4 decoder `b926629`、127.3 bridge `9182861`；沿用prelude模块/CLI集成测试/故事证据组织，无新增第三方运行库需求。128.3实际1844 passed/0 failed/35 ignored只是基线；四API符号明确登记minor。

128.3审查修复选定run/源绑定、formal状态、custom输出路径、manifest保护、example comparison gate、exact测试数、CI harness；TEST矩阵用空/混合pending才能观察遗漏set的故障。GPIO SET/CLEAR同样必须选择能区分动作的初值。复用其真实变异负控制方式，不把故障发生器与期望同时改掉。最新后端已用`chisel3.util.Cat`正确限定，Timer的BigInt/Reset/UInt1 Mux修复也已合并；禁止手改生成Scala来掩盖产品失败，若发现新后端问题保留实际失败并限定必要修复。

### Project Structure Notes / UPDATE审计

|文件|当前与计划|必须保留|
|---|---|---|
|crates/bitloom-prelude/src/ip/gpio/mod.rs（全文）|base/vip/socpad/chip_ring再导出；新增csr模块建议|旧公开路径不改|
|crates/bitloom-prelude/src/ip/mod.rs（全文）|已通配再导出gpio；通常无需再改|全部现有协议/Timer/IRQ/CSR路径|
|docs/public-api-1-0-surface.md（全文）|已含Timer/IRQ逐符号；追加实际GPIO表面/minor|私有helper不自动纳入、设计依赖不扩|
|docs/ip/phase24-contract.md（全文）|正式GPIO32合同与历次状态；追加本故事真实结果链接|历史状态保留、未完成范围明确|
|.github/workflows/ci.yml（全文）|普通、chisel-numeric、formal-sby已有IRQ门禁；加GPIO持久入口与失败工件|原门禁/工具钉不删不弱化|
|sprint-status.yaml（完整YAML解析，全部development_status核对）|445done/8backlog/2deferred；128.4 backlog|create仅本story ready及时间，注释/其余键不变|

NEW建议：`ip/gpio/csr.rs`、`crates/bitloom/tests/fr197_gpio{,_api,_formal}.rs`、`docs/ip/gpio-csr.md`/原文Rust/C/Markdown产物、128-4专用证据/runner。READ ONLY基座：gpio/base.rs、bitloom-sim/src/ip_dual.rs的GpioFunctional、CSR rtl、Irq/Timer/decoder/bridge。已全文读旧base和CSR rtl，读取FL实现及旧FL回归全文；旧GPIO只有out_r8状态、无同步链，不能原地改为新合同。若需改其它既有文件，实施前全文读并记录变化/保留行为和失败复现。

### References / 输入发现与官方资料

SELECTIVE_LOAD：epics.md完整Epic128、Epic129及相关前置；PRD addendum Phase24 FR192–201/NFR93–99；架构脊柱AD-1/2/4/6/7/9/15/18/28/30/31及单一HIR约束；无适用UX文档。CLI/IP接口不是新增GUI流程。Bitloom与samitbasu/rhdl无关。

- [正式合同](../../docs/ip/phase24-contract.md)、[Epic128 NFR14](epic-128-nfr14.md)、[前故事128.3](128-3-事件-irq.md)。
- [Epics](../planning-artifacts/epics.md)、[PRD addendum](../planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md)、[架构脊柱](../planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md)。
- [CSR](../../docs/ip/csr.md)、[模块组合](../../docs/ip/module-composition.md)、[IRQ](../../docs/ip/irq.md)、[FR142](../../docs/public-api-1-0-surface.md)。
- [实施计划§5](../planning-artifacts/research/technical-bitloom-composable-ip-ecosystem-2026-09-20/implementation-plan.md)、[128.3最终验收](../test-artifacts/128-3-final-verification.md)、[本故事只读核对](../test-artifacts/128-4-csr-recon.md)。

2026-09-21实际读取[SBY官方reference](https://yosyshq.readthedocs.io/projects/sby/en/latest/reference.html)，确认prove/cover/bmc含义、cover轨迹与状态种类；该latest文档仅提供语义说明，仓库固定安装身份不升级。本故事不新增框架，不需要迁移最新库版本，也不声称既有工具钉是上游最新。GPIO事件时序以本项目正式合同为准。

### ATDD Artifacts

Checklist: `_agile-output/test-artifacts/atdd-checklist-128-4-gpio-csr-wrapper.md`。GpioCsr API、16端口和描述已由ATDD固定；测试`crates/bitloom/tests/fr197_gpio_api.rs`及`fr197_gpio.rs`。ATDD阶段以缺API编译RED留证；当前构建与独立审查的实际行为验证均已通过，最终工作区回归已通过。

## Dev Agent Record

### Agent Model Used

Codex故事上下文代理；主代理并行只读核对CSR/旧GPIO可行性。

### Debug Log References

[create-story校验](../test-artifacts/128-4-create-story-validation.md)、[主代理CSR核对](../test-artifacts/128-4-csr-recon.md)。

### Completion Notes List

- GpioCsr固定16端口、六32位CSR已实现，唯一OUT和共享CSR叶保持；旧GPIO8/FL及基座未修改。
- ATDD冻结接口并留E0432首红；direct/FIRRTL/Chisel三seed各55151帧、三组合、独立formal/10covers/990cells综合和真实负控制通过。
- 构建三路内审7修补，独立四层审查6修补/5驳回；证据harness普通/-O各17通过。自动化双路核查未发现非重复P0/P1缺口，未增加重复用例。
- 最新678成员原始归档及566源码独立复核通过；实际clean/fmt/just test全部通过：479组1855通过/0失败/40忽略，相关源码未变；第七步单故事提交。

### File List

- `.github/workflows/ci.yml`
- `_agile-output/implementation-artifacts/128-4-gpio-csr-wrapper.md`
- `_agile-output/implementation-artifacts/goal-all-stories-seven-step.md`
- `_agile-output/implementation-artifacts/spec-128-4-gpio-csr.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `_agile-output/test-artifacts/128-4-atdd-knowledge-sha.json`
- `_agile-output/test-artifacts/128-4-atdd-red.json`
- `_agile-output/test-artifacts/128-4-atdd-red.log`
- `_agile-output/test-artifacts/128-4-atdd-worker-api.json`
- `_agile-output/test-artifacts/128-4-atdd-worker-e2e.json`
- `_agile-output/test-artifacts/128-4-automate-knowledge-sha.json`
- `_agile-output/test-artifacts/128-4-automate-worker-api.json`
- `_agile-output/test-artifacts/128-4-automate-worker-backend.json`
- `_agile-output/test-artifacts/128-4-automation-summary.md`
- `_agile-output/test-artifacts/128-4-build-archive.log`
- `_agile-output/test-artifacts/128-4-build-archive.py`
- `_agile-output/test-artifacts/128-4-build-atdd-first.log`
- `_agile-output/test-artifacts/128-4-build-atdd-second.log`
- `_agile-output/test-artifacts/128-4-build-commands.json`
- `_agile-output/test-artifacts/128-4-build-compatibility-0.log`
- `_agile-output/test-artifacts/128-4-build-compatibility-1.log`
- `_agile-output/test-artifacts/128-4-build-compatibility.json`
- `_agile-output/test-artifacts/128-4-build-evidence.md`
- `_agile-output/test-artifacts/128-4-build-example-artifacts.json`
- `_agile-output/test-artifacts/128-4-build-first.log`
- `_agile-output/test-artifacts/128-4-build-formal-first.log`
- `_agile-output/test-artifacts/128-4-build-full-run.log`
- `_agile-output/test-artifacts/128-4-build-generate-software.log`
- `_agile-output/test-artifacts/128-4-build-harness-opt.log`
- `_agile-output/test-artifacts/128-4-build-harness-tests.py`
- `_agile-output/test-artifacts/128-4-build-harness.log`
- `_agile-output/test-artifacts/128-4-build-header-check.c`
- `_agile-output/test-artifacts/128-4-build-implementation-notes.md`
- `_agile-output/test-artifacts/128-4-build-raw.manifest.json`
- `_agile-output/test-artifacts/128-4-build-raw.tar.gz`
- `_agile-output/test-artifacts/128-4-build-review-archive.log`
- `_agile-output/test-artifacts/128-4-build-review-full.log`
- `_agile-output/test-artifacts/128-4-build-review-raw.manifest.json`
- `_agile-output/test-artifacts/128-4-build-review-raw.tar.gz`
- `_agile-output/test-artifacts/128-4-build-review-root-audit.json`
- `_agile-output/test-artifacts/128-4-build-review.md`
- `_agile-output/test-artifacts/128-4-build-runner.py`
- `_agile-output/test-artifacts/128-4-build-sby-identity.json`
- `_agile-output/test-artifacts/128-4-build-source-end.json`
- `_agile-output/test-artifacts/128-4-build-source-start.json`
- `_agile-output/test-artifacts/128-4-code-review-archive.log`
- `_agile-output/test-artifacts/128-4-code-review-fix-backends.log`
- `_agile-output/test-artifacts/128-4-code-review-fix-direct.log`
- `_agile-output/test-artifacts/128-4-code-review-fix-harness-opt.log`
- `_agile-output/test-artifacts/128-4-code-review-fix-harness.log`
- `_agile-output/test-artifacts/128-4-code-review-full.log`
- `_agile-output/test-artifacts/128-4-code-review-raw.manifest.json`
- `_agile-output/test-artifacts/128-4-code-review-raw.tar.gz`
- `_agile-output/test-artifacts/128-4-code-review-root-audit.json`
- `_agile-output/test-artifacts/128-4-code-review.md`
- `_agile-output/test-artifacts/128-4-create-story-validation.md`
- `_agile-output/test-artifacts/128-4-csr-recon.md`
- `_agile-output/test-artifacts/128-4-final-archive-integrity.json`
- `_agile-output/test-artifacts/128-4-final-clean.log`
- `_agile-output/test-artifacts/128-4-final-fmt.log`
- `_agile-output/test-artifacts/128-4-final-regression.json`
- `_agile-output/test-artifacts/128-4-final-test-counts.json`
- `_agile-output/test-artifacts/128-4-final-test.log`
- `_agile-output/test-artifacts/128-4-final-verification.md`
- `_agile-output/test-artifacts/128-4-oracle-selfcheck.json`
- `_agile-output/test-artifacts/128-4-post-clean-archive-audit.json`
- `_agile-output/test-artifacts/128-4-pre-clean-archive-integrity.json`
- `_agile-output/test-artifacts/128-4-pre-clean-fmt-check.log`
- `_agile-output/test-artifacts/128-4-review-fix-gpio-irq-backends.log`
- `_agile-output/test-artifacts/128-4-review-fix-gpio-irq-direct.log`
- `_agile-output/test-artifacts/128-4-review-fix-harness-opt.log`
- `_agile-output/test-artifacts/128-4-review-fix-harness.log`
- `_agile-output/test-artifacts/128-4-review-fix-scoreboard.log`
- `_agile-output/test-artifacts/128-4-root-build-audit.json`
- `_agile-output/test-artifacts/atdd-checklist-128-4-gpio-csr-wrapper.md`
- `crates/bitloom-prelude/src/ip/gpio/csr.rs`
- `crates/bitloom-prelude/src/ip/gpio/mod.rs`
- `crates/bitloom/tests/fr197_gpio.rs`
- `crates/bitloom/tests/fr197_gpio_api.rs`
- `crates/bitloom/tests/fr197_gpio_formal.rs`
- `docs/ip/gpio-csr-example.rs`
- `docs/ip/gpio-csr-registers.h`
- `docs/ip/gpio-csr-registers.md`
- `docs/ip/gpio-csr.md`
- `docs/ip/phase24-contract.md`
- `docs/public-api-1-0-surface.md`

### Review Findings

- [x] [Review][Patch] ATDD必须执行清单 — atdd仅exit0；误ignore可绕过所需测试。patch核对11个普通目标及计数。
- [x] [Review][Patch] 三seed证据完整性 — archive direct-只要求非空类，删除两seed仍可通过。patch要求三个seed/accounting覆盖与成功日志。
- [x] [Review][Patch] scoreboard反例归档 — archive未要求scoreboard-mutation目录，遗漏后仍认证。patch校验control和指定行为FAIL及VCD。
- [x] [Review][Patch] 组合与后端运行记录 — archive只要求源码/VCD，不要求日志/执行退出。patch要求已有成功marker及命令记录。
- [x] [Review][Patch] 形式反例的实际失败原因 — archive仅FAIL/非空VCD可丢失指定性质和退出证据。patch复核已有Rust要求的exit2/性质/VCD。
- [x] [Review][Patch] GPIOIRQ同沿清除和新事件 — 组合夹具未覆盖两接收端同时清除与真实rise。patch同一三后端夹具增加新事件胜双clear。

Rejected:
- 1 low: 并发工件归属 — 目录差集并发可混入；串行受控运行/CI隔离下罕见，新增锁或路径参数复杂，拒绝。
- 8 low: 增加时序变异种类 — 既有三种代表故障满足代表性负控制合同，时序由独立逐拍/形式断言覆盖；未展示盲点，额外变异和分支非直接修正，拒绝。
- 9 low: 内部launcher失败元数据 — 内层启动失败在metadata前panic，外层仍记录失败；timeout缺失罕见，新增多处分支超过直接修正，拒绝。
- 10 low: JVM harness抽取 — 复制可能让未来修改漏同步，当前两实际入口同pin均执行；尚无日常分歧，抽取复杂，拒绝。
- 11 low: 并发工件归属 — 独立判定同1：共享目录差集确可受并发影响，但当前受控串行/隔离CI，锁引入复杂度，拒绝。
