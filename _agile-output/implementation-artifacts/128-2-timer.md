# Story 128.2: Timer

Status: done

## Story

作为实现/集成维护者，
我希望交付具有共享模块定义、CSR 配置和原始匹配事件的 32 位 Timer，
以便软件通过已验收的 CSR 基座控制计时，并为 FR197 外设和后续 FR198 系统提供可复用部件。

估算：2–3 有效人日；维护 owner Richard，实施与证据 Codex，主代理负责串行七步与一故事一提交。估算不是墙钟时间承诺。

## Acceptance Criteria

1. **合法前置与组合。Given** 128.1、127.2 done，Epic128 NFR14有效，**When** 构造 Timer，**Then** 提供 prelude-only 公开入口、同一 `ElaborateSession` 的 `define_module` 和共用实现的独立 `elaborate`；全部模块定义结束后只 `finish` 一次，复用既有 CsrBlock，不拼 FrozenHir、不建第二 HIR。明确唯一状态owner，重复定义复用及命名/连接冲突沿用 M1 诊断。
2. **地址、访问与部分写。Given** 下表固定 Timer32 描述，**When** 经 CSR 访问，**Then** CTRL RW mask3、COUNT/COMPARE RW32、EVENT W1C mask1，reset均0；小端 WSTRB4 逐字节合并，保留位读0写忽略。局部地址00/04/08/0c与系统Timer窗口0200/0204/0208/020c明确区分；Timer本身不截高位或把0200当00。未定义/未对齐局部地址SLVERR、失败读rdata0；合法零有效mask写OKAY无访问副作用。软件地址文档/C头来自同一描述，独立黄金手写关键地址。
3. **完整32位计数。Given** 未发生有效CTRL/COUNT/COMPARE写，**When** 普通使能上升沿到来，**Then** 先 `next_count=(COUNT+1) mod 2^32`，再以该值等于沿前COMPARE判match；periodic命中COUNT归0，one-shot命中保留递增后COUNT并清enable（periodic位保留）。关闭时COUNT保持、无match。COMPARE=0只在自然回绕命中；写比较值越过当前COUNT不会立即触发。不得缩窄计数以替代32位边界验证。
4. **软件写优先。Given** 成功CSR写提交，**When** CTRL/COUNT/COMPARE的有效mask非零，**Then** 写COUNT采用合并候选值，其余有效CTRL/COMPARE写保持COUNT；这三类写均抑制该拍自动计数和match，未被写的计数配置状态保持。写相同值或写入0也算有效写。零WSTRB以及非零WSTRB仅选择CTRL保留字节时有效mask=0，不抑制自然计数/match。普通读、错误访问和EVENT写也不抑制自然计数/match。
5. **事件与W1C。Given** match与软件EVENT清除可同沿发生，**When** 更新事件，**Then** `EVENT_next=(EVENT_old & ~clear)|match`，set优先；只清选中字节的写1有效位。原始匹配事件与粘滞EVENT分开，供128.3未来IRQ0消费的是每个匹配沿的事件，而非EVENT电平。periodic COMPARE=1可每沿匹配，不能人为插入空拍；EVENT未清不阻止新match。
6. **提交、背压和共同reset。Given** 非reset沿 `req_valid && req_ready` 是唯一提交，**When** 响应受阻、自然计数继续或reset发生，**Then** CSR写/清除仅一次；提交前读快照在提交沿锁存，下一周期rsp_valid可见；背压保持rdata/error/valid，不接受新提交，消费沿不refill。reset优先于提交与match，共同清leaf/Timer/在途响应和全部状态；释放后保持关闭。安全性不得依赖最终rsp_ready，活性另列公平性与时限。
7. **真实验证和关闭边界。Given** 独立模型/定向向量/组合例完成，**When** 执行验证矩阵及七步，**Then** 保存真实RTL、适用native/generated、direct/Chisel/FIRRTL、formal prove/cover及原始综合的各自证据，核验新增API与旧API回归、SemVer minor；工具缺失或未执行不计PASS。只关闭128.2 Timer子集，不关闭FR197/M3/Epic128或Phase24；128.3–5、129、130仍按自身依赖，FR189 deferred/NFR91保持，不push/publish或改工具钉/包版本。

## Tasks / Subtasks

- [x] T1 ATDD与接口规格（AC1–7）。
  - [x] 写可执行独立参考、关键地址与边界向量；记录真实RED原因，不把编译缺API冒充所有行为红测。
  - [x] 固定公开名称/签名、端口、owner与事件采样沿，在build spec中记录；公开表面逐符号登记。固定Timer32可沿CsrDecoder的无配置公开形状，无需因历史示例引入TimerConfig。
- [x] T2 Timer32可组合实现（AC1–6）。
  - [x] 新建`ip/timer.rs`，复用CsrBlock External RW CTRL/COUNT、Leaf RW COMPARE、Leaf W1C EVENT或有等价证据的唯一owner实现。
  - [x] 无环candidate→commit→next-state，32位模加、写优先、one-shot/periodic、raw match与EVENT分离。
  - [x] 相同session复用/独立入口、完整宽向连线、共同reset；C/Markdown产物及prelude-only示例。
- [x] T3 真实行为/属性/综合（AC2–7）。
  - [x] 必达场景见下表；随机序列保存seed、提交/消费/取消账本，黄金不复用DUT next-state。
  - [x] direct与适用Chisel/FIRRTL实际RTL；适用单模块native两引擎/生成Rust；层级拒绝边界单列。
  - [x] 先核验sby源码/安装身份，再prove和cover；原始无observer RTL综合/check，记录版本及cells。
- [x] T4 文档、兼容与七步闭合（AC7）。
  - [x] Timer使用/时序/地址说明、FR142符号清单、minor说明、证据与未支持项。
  - [x] 独立code-review→automate→clean/fmt/`just test`及适用额外门禁→一故事一提交；忽略项不计通过。

## Dev Notes

### ATDD Artifacts

Checklist: `../test-artifacts/atdd-checklist-128-2-timer.md`；实际RTL行为测试`crates/bitloom/tests/fr197_timer.rs`，结构/组合测试`crates/bitloom/tests/fr197_timer_api.rs`。固定Timer API、13端口与描述形状见checklist，构建必须沿用；当前RED执行结果由checklist记录。

### 基线、前置与前故事经验

创建基线`1c4cf20c4f8e8fdcb038d57a7b3f03f9f80b5c00`。128.1与127.2已done，Epic128 in-progress，本故事原backlog。128.1仅交付风险门、BFM stub和真实旧CSR peer探针，未交付Timer；最后workspace 1823 passed / 0 failed / 25 ignored不代表本故事通过。NFR14正文早期“七步pending”是build历史快照，当前故事状态以128.1最终关闭与sprint为准。

最近五提交依次为128.1风险门、127.4译码/M2、127.3桥、127.2 CSR、127.1风险门。沿用新IP独立文件、prelude再导出、专用`fr196_*`风格测试/证据，不重写旧AXI bank。128.1审查明确补入CTRL保留字节写不得抑制计数、形式安装身份不能靠版本字符串证明。

Epic128五故事：128.1风险门已done；本故事Timer依赖128.1/127.2；128.3 IRQ依赖相同前置；128.4 GPIO依赖128.3/126.2；128.5 UART依赖128.3/126.4。M3须128.1–5全部done。129.2系统另依赖129.1、127.4、128.2/4/5，不能用本故事Timer夹具代替完整系统。

### 精确寄存器与状态所有权

|本地字节offset|后续系统地址|寄存器|mask|访问/推荐唯一owner|
|---|---|---|---|---|
|0x0000|0x0200|CTRL：enable bit0、periodic bit1|0x00000003|RW External，Timer拥有（one-shot自主清enable）|
|0x0004|0x0204|COUNT|0xffffffff|RW External，Timer拥有|
|0x0008|0x0208|COMPARE|0xffffffff|RW Leaf，CSR拥有|
|0x000c|0x020c|EVENT：match bit0|0x00000001|W1C Leaf，CSR拥有|

全部reset0，无ID/version。Timer接收16位**局部**地址，不自行执行系统窗口译码；系统CsrDecoder将Timer窗地址减base后路由。leaf仅OKAY(00)/SLVERR(10)，系统窗外DECERR(11)由已交付decoder处理。局部0x0200与高位地址不得别名命中0；无需为Timer另造任意地址映射API。

`R_candidate`与`R_write_mask`不依赖commit/地址/valid；成功有效写才产生`R_write_commit`。三种写commit的OR是停止自然计数/match的条件，不能使用总线write、非零WSTRB或`candidate != old`近似。`R_*_commit`是沿前组合脉冲，同上升沿采样，不能延迟到响应拍。只有CTRL/COUNT存于Timer，COMPARE/EVENT不再复制一份；若调整内部组织，必须明确每个寄存器唯一owner和等价行为。

状态方向：当前值+WSTRB/wdata→candidate；合法提交→软件优先条件；沿前CTRL/COUNT/COMPARE→模32递增与match→唯一next-state。CSR EVENT输入接当沿match（扩展至32位bit0）；对外raw match端口须写明在该沿被下游采样。reset门控match，不因rst期间COUNT/COMPARE为0误报。EVENT读提交返回沿前粘滞值；同拍新match在之后的读中可见。one-shot终止时periodic位不能被意外清除。

### 必达定向向量与独立参考

|风险|验收向量|
|---|---|
|宽度/回绕|软件写COUNT近0xffffffff、COMPARE0，启用后检查精确回绕；覆盖高位/bit31部分写；不用运行2^32拍也不用缩窄DUT|
|周期/一次|COMPARE1、2及代表大值；periodic连续多次、one-shot恰一次且COUNT保持在命中值，再启用后的行为|
|写优先|预计match沿分别有效写CTRL/COUNT/COMPARE，含相同值、0值、各字节部分写；match不得发生，未写状态保持|
|零有效mask|预计match沿CTRL WSTRB=0与WSTRB=2/4/8，仅保留字节写，必须正常match；对照WSTRB=1有效写抑制；COUNT/COMPARE零WSTRB继续计数|
|compare越过|把COMPARE写为小于当前COUNT，写沿不match；之后只在自然next_count相等时匹配，可经COUNT近回绕设置短路径但不得伪称经历完整2^32周期|
|W1C|EVENT已置位后clear、写0、高字节无效clear、clear与match同沿set胜；EVENT写不暂停计数，持续match不被粘滞位吞并|
|读与错误|计数读在自然递增同沿返回沿前快照；EVENT读与新match碰撞；洞、未对齐、0x0200/高位无别名，失败读0且自然计数仍继续|
|背压/提交|持续req_valid与长rsp_ready=0保持一次提交/快照；受阻期间计数继续，不能重复配置暂停或W1C；消费沿无新提交|
|reset|关闭、运行、预计match、有效写提交、W1C碰撞及响应受阻时reset；共同取消响应、事件无泄漏、所有CSR0、释放后关闭|
|复用|同session重复Timer定义/多个实例、独立状态且单次finish；文档设计仅依赖prelude，地址/C产物确定|

手写参考用u32 wrapping运算与独立CSR提交模型，不能调用Timer生成器的next-state/地址helper作为黄金。随机配置固定并打印seed，累计提交、响应消费及reset取消；定向覆盖计数独立于随机命中。失败保留原始输出/波形，不允许吞掉非零状态或将filtered/ignored计PASS。

### 架构、工具和验证适用性

遵守AD-1/4/6/7/15/18/28/30：runtime elaborate、唯一FrozenHir、prelude-only、严格位宽、同步高有效内部rst、无捕获定义体冻前消解。默认产品firtool1.159.0、Chisel7.15.0、Rust1.97.1/edition2024不变。Timer无新异步针脚或CDC；系统aresetn断言/释放由外部控制器同步到ACLK后转换，取反不等于同步器，所有leaf与wrapper共同复位。无PPA、板级时序或MTBF承诺。

- 真实CSR+Timer组合属于层级：必须实际运行direct RTL，并执行适用Chisel经JVM→RTL、FIRRTL→固定firtool→RTL；仅emit成功不算行为验收。
- 若存在无实例内核，native Interpreter/Compiled和受支持generated单模块验证适用；若只有含实例入口，不为测试扩展产品路径，native/generated列unsupported/不适用，由真实层级RTL验收完整行为。不得因为名为standalone就声称可native模拟。
- formal针对写优先/模32/COMPARE0/set胜clear/快照背压安全，清楚写初始reset与输入稳定假设；prove与cover分列，cover包含实际match、clear碰撞、软件抑制、保留字节不抑制、背压恢复和reset取消。需要辅助observer时检查其约束不强迫被证明结论。活性与公平性另列。
- 综合对原始生成RTL运行Yosys综合/check，排除latch、多驱动、不可综合结构；保存工具版本/cells，不把observer综合当产品综合。
- 正式证明之前，依据`scripts/ci-sby-pins.env`重新核验标签对象`bfc1c47eb786496fe794481ff88e75728f0529a6`、剥离commit `daed0e1544fd96ee7dab843e5a891d92784c6230`及所用sby入口/支持模块与已验源码安装绑定；无法绑定时受控fresh前缀重新安装。`--version`、ci-install已有工具skip、hygiene脚本均不能替代。详见128.1 build证据末节。
- BFM如用于桥接集成，沿用固定五包闭包，不更换版本；特殊WSTRB/非法对齐用原始通道，与高层BFM不可同时驱动针脚。Timer局部CSR定向可直接驱动，无需为了本故事引入新外部协议依赖。

参考既有入口`crates/bitloom/tests/fr196_csr.rs`的真实peer及`fr196_csr_formal.rs`的prove/cover/原始综合方法；新测试建议`fr197_timer*.rs`，build应记录实际最终名字与命令。最终运行`cargo clean`、`cargo fmt --all`、`just test`，以及新Timer专用形式/后端/综合和`just semver-check`等适用门禁；裸cargo test不足。清理前把target下原始证据归档至test-artifacts并记录hash。

### Project Structure Notes / UPDATE审计

|文件|当前状态|本故事变化/须保留|
|---|---|---|
|`crates/bitloom-prelude/src/ip/mod.rs`（已全文读）|私有协议模块+通配再导出，无Timer|新增timer模块与再导出；旧导出/路径全部保留|
|`crates/bitloom-prelude/src/lib.rs`（已全文读，默认READ ONLY）|已公开ip模块、builder/HIR门面、旧CDC和宏入口|通常无需改；若确需改先说明，不能引入后端依赖或更改旧接口|
|`docs/public-api-1-0-surface.md`（已全文读）|登记FR194–196逐符号和历史范围|追加实际Timer符号/签名/限制/minor；未列私有helper不自动稳定|
|`docs/ip/phase24-contract.md`（已读取）|正式Timer行为与历史关闭状态|只追加本故事真实Timer子集状态及文档链接，不改合同、不标M3完成|
|`sprint-status.yaml`（完整文件读取及全development_status解析）|128.1/127.2 done、128.2 backlog、Epic128 in-progress|create仅128.2 ready-for-dev和timestamp；最终done由七步主流程处理|

NEW建议：`crates/bitloom-prelude/src/ip/timer.rs`、`crates/bitloom/tests/fr197_timer*.rs`、`docs/ip/timer.md`及128.2专属证据。现有`csr/mod.rs`与`csr/rtl.rs`已全文读，具备所需接口，默认READ ONLY；如发现必修基座问题，先补失败复现及影响面审查。私有codec、桥、decoder和旧GPIO/UART默认不改；任何新增UPDATE文件必须先全文读并记录保存行为。文档须声明Bitloom与samitbasu/rhdl无关。

### References / 输入发现与官方核验

SELECTIVE_LOAD按目标128.2：epics现有单文件中完整Epic128及129依赖；PRD/architecture默认一层glob无匹配，依据已知项目布局解析到深层正式来源；无适用UX文件（库/RTL/软件地址工作）。正式合同覆盖研究旧GPIO8/IRQ8和历史M0-only注记，用户当前七步授权覆盖历史执行限制。

- [正式合同](../../docs/ip/phase24-contract.md)：Timer、CSR、复位和授权。
- [NFR14](epic-128-nfr14.md)、[前故事](128-1-外设-nfr14.md)、[形式身份门槛](../test-artifacts/128-1-build-evidence.md)。
- [Epics](../planning-artifacts/epics.md)：Epic128所有故事、129依赖与Phase24风险模板。
- [PRD addendum](../planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md)：FR197/NFR93–99。
- [脊柱](../planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md)：AD-1/4/6/7/9/15/18/28/30。
- [实施计划§5](../planning-artifacts/research/technical-bitloom-composable-ip-ecosystem-2026-09-20/implementation-plan.md)、[CSR](../../docs/ip/csr.md)、[FR142](../../docs/public-api-1-0-surface.md)、[SemVer](../../docs/semver-1-0-policy.md)。

2026-09-21实际读取[YosysHQ SBY官方reference](https://yosyshq.readthedocs.io/projects/sby/en/latest/reference.html)，确认bmc/prove/cover含义分立；在线latest文档仅作模式说明，实际使用仓库固定sby版本，不据此升钉。[cocotb2.0.1官方release notes](https://docs.cocotb.org/en/v2.0.1/release_notes.html)用于现有测试环境兼容核验，保持固定依赖；未主张这些工具钉为当前最新，无新库/升级需求。Timer set优先和next_count比较来自本项目合同，不由外部通用定时器语义推导。

## Dev Agent Record

### Agent Model Used

Codex，create-story子代理；主代理并行审计可行接口/测试基座。

### Debug Log References

创建验证：`../test-artifacts/128-2-create-story-validation.md`。已执行customization resolver（prepend/append/persistent_facts/on_complete为空），读取config/discover-inputs/template/checklist、项目指定事实及上述输入。

### Completion Notes List

- Ultimate context engine analysis completed - comprehensive developer guide created.
- 当前仅create-story完成；所有实现任务未勾选，未执行ATDD/build/RTL/formal/综合，不宣称Timer或FR197已交付。
- 没有需用户重新批准的既定产品决定。具体公开命名、证明深度和测试文件名由build在上述边界内固定并记录。

### File List

- `_agile-output/implementation-artifacts/128-2-timer.md`（NEW）
- `_agile-output/test-artifacts/128-2-create-story-validation.md`（NEW）
- `_agile-output/implementation-artifacts/sprint-status.yaml`（仅128.2状态和timestamp）


### 128.2 Build实施记录（2026-09-21）

Timer32、共享CSR leaf、13端口与同源地址产物已实现；公开符号、唯一owner、沿前raw事件与层级限制见`docs/ip/timer.md`。8项ATDD真实GREEN，专用形式prove/8cover与614cells原始综合通过，direct/FIRRTL/Chisel三seed同黄金各自实跑。完整工具、命令、SHA与原始归档见`../test-artifacts/128-2-build-evidence.md`。

适用Chisel真实编译暴露旧emitter对大UInt字面量、Reset位运算及UInt1 Mux选择器的类型错误；已在`crates/rhdl-firrtl/src/chisel.rs`作最小修复，未修补生成Scala、缩位宽或换工具钉。全文读取UPDATE文件，保留小值与Bool路径，补跨Int/Long上界和Mem初始化literal回归，复跑FIRRTL crate及Chisel实际门禁。

本段仅build；后续独立code-review、automate、clean/fmt/workspace回归与commit由主流程负责，T4整体与故事状态保持未关闭。FR197整体/M3/Phase24未关闭，FR189 deferred/NFR91保持，无push/publish/版本变化。

Build文件：`crates/bitloom-prelude/src/ip/{mod,timer}.rs`、`crates/rhdl-firrtl/src/chisel.rs`、`crates/bitloom/tests/fr197_timer{,_api,_formal}.rs`、`docs/ip/timer{,-example,-registers}.*`、`docs/public-api-1-0-surface.md`、`docs/ip/phase24-contract.md`与`128-2-build-*`证据。

### Build 内审与主代理复核

三路内审12项分别裁定，11项修补关闭、1项既有多位Mux问题记deferred。CI持久门禁、可移植runner、独立诊断/组合/背压、u64/JVM回归已补齐；主代理重跑构建Verification全绿，10 cover全部可达。见`../test-artifacts/128-2-review-fixes.md`与`128-2-root-recheck-*`。build完成，独立code-review/automate/全量clean回归/提交待执行，故事仍review。

### Review Findings

- [x] [Review][Patch] CR2 Timer例子/C检查接入持续CI — 证据见`../test-artifacts/128-2-code-review.md`。
- [x] [Review][Patch] CR3 Reset XOR双操作数实际后端覆盖 — 证据见`../test-artifacts/128-2-code-review.md`。
- [x] [Review][Patch] CR4 COMPARE最大值双模式定向命中 — 证据见`../test-artifacts/128-2-code-review.md`。
- [x] [Review][Patch] CR5 同定义与不同定义双实例均执行 — 证据见`../test-artifacts/128-2-code-review.md`。
- [x] [Review][Patch] CR6 重命名双实例多后端实际RTL — 证据见`../test-artifacts/128-2-code-review.md`。
- [x] [Review][Patch] CR7 形式observer代表性故障注入拒绝 — 证据见`../test-artifacts/128-2-code-review.md`。
- [x] [Review][Patch] CR8 复现runner涵盖numeric门禁 — 证据见`../test-artifacts/128-2-code-review.md`。
- [x] [Review][Patch] CR9 Python优化模式不得移除身份核验 — 证据见`../test-artifacts/128-2-code-review.md`。
- [x] [Review][Patch] CR12 CI形式证明前实际核验SBY身份 — 证据见`../test-artifacts/128-2-code-review.md`。
- [x] [Review][Defer] CR1/CR11 多位Mux既有问题 — deferred：旧HEAD同样不支持，当前Timer使用1位。

#### Rejected

- CR10 low：命名字段建议要求变更ATDD固定bits描述及其规格；当前bit0/bit1已文档化，按规格修改类发现拒绝。

独立code-review修补完成：9 patch均有实际相关通过证据；主代理已逐文件核验621个归档文件及对应源码SHA。1既有defer/1规格修改建议驳回保留。按用户七步关闭要求，故事暂维持review，automate和全量回归通过后才done/commit。

### Automate完成

新增1个P0直接RTL scoreboard负控制，原始PASS、错误有效mask/受阻响应损坏两种真实DUT变异均被指定周期字段断言检出；1测试真实通过。详见`../test-artifacts/128-2-automation-summary.md`。下一步完整clean/fmt/workspace及最终提交，故事仍review。

### 最终关闭

实际clean/fmt/just test全部exit0，473结果块、1833 passed/0 failed/30 ignored；本故事5个专用入口另有实际PASS。最终验收`../test-artifacts/128-2-final-verification.md`。本故事随独立提交关闭，Epic128/FR197/M3仍开放；下一128.3。此前create/build/review段落是时点记录，当前状态以本节为准。
