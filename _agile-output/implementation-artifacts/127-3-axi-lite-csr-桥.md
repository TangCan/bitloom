# Story 127.3: AXI-Lite→CSR 桥

Status: done

## Story

作为实现/集成维护者，我需要可组合的 AXI4-Lite→CSR 桥，使独立到达的地址和数据可靠地转换为一次 CSR 提交，并在背压、并发和复位时保持响应与副作用正确。

估算5–7有效人日。基线 `9393c261cdfb2ca3699edb959efd1986bbe5275e`；127.1 NFR14 accepted/done、127.2 CSR 和126.3注册切片均done。全部剩余故事七步执行授权有效。本故事只交付FR196桥子集；127.4四窗译码与M2关闭、128外设、129系统另行验收。

## Acceptance Criteria

1. **可组合入口。Given** prelude-only设计，**When** 独立elaborate或在同一ElaborateSession定义桥，**Then** 同一个无捕获模块体生成固定addr16/data32/WSTRB4的桥；一个外层finish，无第二IR或FrozenHir拼接。公开入口/端口在ATDD前定名，并显式登记FR142及SemVer minor。相同定义可复用，两个实例状态独立；方向/宽度错误仍由既有freeze诊断。
2. **独立捕获与配对。Given** AW/W/AR原始通道，**When** AW先、W先或同拍、间隔0/1/7/31拍并发读写，**Then** 三者各一独立捕获槽，只有收齐AW+W才具写资格；地址和数据按各自接受顺序配对，不覆盖或跨事务错配。不完整写不阻合格读；保留完整16位字节地址、32位数据及4位WSTRB。AWPROT/ARPROT各3位存在但忽略，全部8值不改变权限/地址行为。
3. **提交、仲裁和请求保持。Given** 已捕获的读/完整写，**When** 全局CSR空闲且对应AXI响应槽可预留，**Then** reset后同时合格先读，成功CSR提交后优先另一类；捕获、offer和响应消费都不独自改变优先。无可用B/R槽的类不参与。CSR req_valid一旦提出，在req_ready为0期间必须保持valid、write、addr、wdata、wstrb及所属类别，后来到达的请求不能换掉offer。非reset上升沿req_valid&&req_ready是唯一提交点；全局最多一个已提交尚未被CSR rsp_valid&&rsp_ready消费的请求。
4. **响应预留、背压和错误。Given** 每类一个B/R响应保持槽，**When** CSR请求提交前预留该类容量并收到响应，**Then** 按提交时锁定的类别路由且恰好产生一次AXI响应，不按后来捕获的地址/类别路由。B/R受阻保持valid/payload；B受阻不阻有空间的读，R受阻不阻有空间的写（CSR执行槽空闲后）。CSR响应消费后可执行另一类，即使前一AXI响应尚未消费。CSR error00/10/11原样映射BRESP/RRESP，合法peer不提供01；失败读rdata0。零WSTRB仍提交并响应，不能丢事务；是否合法/有无副作用由CSR决定。
5. **寄存边界与复位。Given** 同步高有效内部rst，**When** 施加复位沿，**Then** 取消AW/W/AR捕获、锁定offer、执行owner、预留、B/R与仲裁状态，reset优先所有接受/提交/响应计数。复位后可接新流量，无旧响应或副作用重放。外部AWREADY/WREADY/ARREADY/BVALID/RVALID仅依赖寄存状态，无任何输入组合路径（包括rst；同步reset在沿后清状态），以HIR/综合网表依赖锥证明；不能只看稳定波形。系统aresetn断言和释放都须由外部控制器同步到ACLK，再反相到内部rst；桥和所有leaf共同复位，反相不是同步器，不支持bridge-only reset。
6. **真实组合和地址语义。Given** 同session顶层实例化桥与已交付CsrBlock，**When** 实际生成RTL执行独立黄金序列，**Then** RW/RO/WO/W1C、全部16种WSTRB、动态拒绝、提交读快照与一次副作用正确；原始未对齐地址1/2/3、洞及高位地址不被桥对齐/截断。leaf非法地址/权限产生SLVERR且无访问副作用；用独立合规测试responder验证DECERR传递。四窗地址生成/系统未命中DECERR属于127.4，不提前实现或关闭。native/generated层级仍明确unsupported，单模块仿真不能替代组合RTL。
7. **独立验证和形式证据。Given** 协议合法刺激与独立scoreboard，**When** 定向、随机、长背压和reset矩阵执行，**Then** 分开记录AW/W/AR接受、完整事务、CSR提交/消费、AXI响应和reset取消，检测配错、重复、丢失和覆盖。原始RTL实际sby safety证明AW/W配对、单CSR在途、响应防覆盖/保持及reset边界，并有读写竞争和背压恢复cover；安全不假设BREADY/RREADY最终为1。活性若声明则单列CSR有限响应、ready最大等待/公平性与深度。无observer原始RTL实际Yosys综合/check，无latch或未知单元蒙混。保存工具版本、命令、实际seed/场景命中/墙钟、状态码、日志及失败证据；缺工具/超时/UNKNOWN不算通过。
8. **兼容和七步关闭。Given** 中文文档与真实可编译prelude-only示例，**When** 完成独立review、automate、实际cargo clean + cargo fmt --all + just test和单故事commit，**Then** 旧Axi4LiteSlave ADDR8、四寄存器、恒OKAY、同拍AW/W一tick B及并发同址read-before-write不变；保留CSR/M1回归。仅127.3 done，Epic127 in-progress，127.4/M2/FR196整体/Phase24仍未完成；FR189 deferred/NFR91保持，不推送或发布。

## Tasks / Subtasks

- [x] T1 确切API与ATDD红阶段（AC1–8）：锁定签名/端口，独立通道和CSR oracle，记录真实缺API红，不冒充行为红。
- [x] T2 实现捕获、锁定offer、轮转、执行owner和B/R预留/保持（AC1–5）；完整身份、独立入口共用模块体。
- [x] T3 真实桥+CSR组合、独立错误peer和原始通道矩阵（AC2–6）；reset各阶段与恢复、16WSTRB、PROT、地址高位。
- [x] T4 prove/cover、结构依赖锥与无observer综合（AC5、7）；CI显式运行专用formal并保存失败产物。
- [x] T5 中文使用文档、prelude-only示例检查、FR142与索引追加（AC1、8）；记录时序气泡及支持边界。
- [x] T6 独立code-review→automate→clean/fmt/just test→单故事commit（AC8），更新本story/sprint真实状态，goal由主代理维护。


### Review Findings

四层独立审查结果与逐项证据见[review报告](../test-artifacts/127-3-independent-review.md)。

- [x] [Review][Patch] R1 失败写响应被不必要地约束rdata=0 [`crates/bitloom/tests/fr196_axi_lite_csr_formal.rs:130`]
- [x] [Review][Patch] R2 缺成功完成的延迟CSR响应场景 [`crates/bitloom/tests/fr196_axi_lite_csr.rs:365`]
- [x] [Review][Patch] R3 reset优先未覆盖同时可握手输入 [`crates/bitloom/tests/fr196_axi_lite_csr.rs:645`]
- [x] [Review][Patch] R4 reset阶段6和7重复 [`crates/bitloom/tests/fr196_axi_lite_csr.rs:608`]
- [x] [Review][Patch] R5 真实组合SV刺激未接协议保持监测器 [`crates/bitloom/tests/fr196_axi_lite_csr/integration.rs:199`]
- [x] [Review][Patch] R6 排空即停止缺末端重复响应观察 [`crates/bitloom/tests/fr196_axi_lite_csr.rs:384`]
- [x] [Review][Patch] R7 scoreboard允许CSR响应与新提交同沿 [`crates/bitloom/tests/fr196_axi_lite_csr.rs:242`]
- [x] [Review][Patch] R8 seed耗时字段未明确逐seed与累计 [`crates/bitloom/tests/fr196_axi_lite_csr.rs:675`]
- [x] [Review][Patch] R9 结构检测未核对FF时钟源 [`crates/bitloom/tests/fr196_axi_lite_csr_formal.rs:registered`]
- [x] [Review][Patch] R10 monitor负例未覆盖ready升高的接受沿 [`crates/bitloom/tests/fr196_axi_lite_csr.rs:754`]

## Dev Notes

### API / 接线建议（ATDD必须固化后再实施）

最小新增ZST `ip::AxiLiteCsrBridge`，`Elaboratable::elaborate() -> Result<FrozenHir,Diagnostics>`，`define_module(&mut ElaborateSession, impl Into<String>) -> Result<String,Diagnostics>`。不需要可变宽度配置或新的builder公共接口。建议固定端口如下；ATDD正式契约可统一命名，不能弱化宽度/行为。

| 方向 | 端口 |
|---|---|
| 输入 | clk:Clock、rst:Reset；s_axi_awaddr16、s_axi_awprot3、s_axi_awvalid1；s_axi_wdata32、s_axi_wstrb4、s_axi_wvalid1；s_axi_bready1；s_axi_araddr16、s_axi_arprot3、s_axi_arvalid1；s_axi_rready1 |
| 输出 | s_axi_awready1、s_axi_wready1、s_axi_bvalid1、s_axi_bresp2、s_axi_arready1、s_axi_rvalid1、s_axi_rdata32、s_axi_rresp2 |
| CSR输出 | csr_req_valid1、csr_write1、csr_addr16、csr_wdata32、csr_wstrb4、csr_rsp_ready1 |
| CSR输入 | csr_req_ready1、csr_rsp_valid1、csr_rdata32、csr_error2 |

桥csr_*接leaf同名去前缀端口；顶层自己提供clk/rst及leaf的动态值/事件/拒绝，所有模块先定义后顶层实例化，finish一次。leaf仅产生00/10，11用独立test peer，01为非法peer输入，不发明转换规则。

### 状态与时序设计约束

捕获槽占用、已提出但未提交offer、已提交未消费CSR、已排队AXI响应是不同阶段。可以在执行上一请求时捕获下一AW/W/AR，但不能覆盖任何满槽或扩大每通道一槽限制。锁定offer使用寄存选择/有效位或等价保持机制；req_ready为0时新AR/完整写到达不能使选择变化。offer锁定时可先预留对应响应槽；全局执行owner在提交时记录，持续到CSR响应消费。已收到但AXI未消费的B/R不占全局CSR执行槽，仍占对应类别响应槽。

选择保守气泡即可：槽在消费沿不旁路接受新payload，响应消费沿不必同类重填，不承诺每拍吞吐或零气泡。文档与oracle写清实际延迟；不要从旧bank复制实时AW/W旁路或AR接受即取快照。新桥读写顺序由提交轮转决定，不继承旧bank同址read-before-write。

首次使用前至少施加一个有效reset上升沿；formal允许任意初始寄存器并明确假设初始reset，不能把未初始化状态当产品承诺。reset沿之前的寄存ready/valid可能仍高，计数器和producer monitor必须按reset优先规则排除该沿。外部输出不可组合门控rst；内部CSR提交可组合屏蔽rst以配合现有leaf，且全系统共同reset。取消释放受阻producer的保持义务；非reset时所有AXI producer与CSR responder均须满足valid/payload保持，不能撤回未接受请求。

126.3是已验收寄存边界模式的前置，不强制实例化两槽RvRegSlice来实现一槽捕获；两者容量不同。复用现有builder表达式，不为复用而引入第二IR或扩展native层级。

### 现有文件与改动界限

| 文件 | 当前行为 / 本故事改动 / 必须保留 |
|---|---|
| crates/bitloom-prelude/src/ip/mod.rs | 私有分协议模块pub use；追加axi_lite_csr模块/re-export，保留现有全部路径 |
| NEW ip/axi_lite_csr.rs | 固定桥定义体，必要时私有拆分；不得放进旧axi.rs改变旧bank |
| ip/axi.rs（只读） | 旧ADDR8 bank、同拍AW/W旁路、独立读；兼容回归，不套新CSR容量/错误语义 |
| ip/csr/{mod,rtl}.rs（复用） | CsrBlock静态配置、动态端口、pending响应；现有leaf响应下一周期，消费沿不接新请求；不改语义来适配桥 |
| ip/rv_reg_slice.rs / builder::define_module（复用模式） | 非捕获body/稳定Span、完整参数和HIR身份；helper失败poison；不嵌套begin/end/define，不改通用契约 |
| docs/ip/README.md | 现有IP与CSR叶索引；只追加桥链接与真实交付边界 |
| docs/public-api-1-0-surface.md | FR142逐符号清单；追加桥符号/minor，保留所有稳定承诺，不改版本或发布 |
| .github/workflows/ci.yml | workspace真实Icarus及单独formal-sby任务；追加桥formal命令/失败产物，保留其他门禁 |
| scripts/check_fr196_example.py | 已有CSR文档原文编译入口；优先新增桥入口调用check_fr194_example.check_example，不破坏CSR检查 |
| NEW crates/bitloom/tests/fr196_axi_lite_csr*.rs | 原始通道native/RTL、组合、formal/结构；工具驱动可以复用，不复用DUT仲裁/译码作为oracle |
| NEW docs/ip/axi-lite-csr-bridge.md | 精确端口、阶段/容量/气泡、同步reset前提、可运行示例、证据与限制 |

修改UPDATE文件前完整读取。当前已读mod、索引、API表面、CI及示例脚本；若新增其他修改面先读取。最终只更新本story/sprint/相关真实交付附记；不关127.4/M2，不更改已done故事内容。

### 必须执行的测试矩阵

- AW/W早晚0/1/7/31，独立持续valid，跨事务不同地址/数据，AR与完整/不完整写竞争；reset后读优先、每次成功提交切换，req_ready低期间锁定offer，不能靠高层BFM自动对齐。
- 分别长期停BREADY/RREADY、同时停、恢复；B占槽时读继续/R占槽时写继续；响应快照不可受新捕获或peer变化影响；满槽不接受覆盖。
- 16WSTRB×有效/保留位，零WSTRB仍一提交一响应，RO零WSTRB写错误；原始1/2/3未对齐、高位如0x8000与0x0000不别名、0xfffc/0xffff；合法动态拒绝及W1C事件交错。
- reset仅AW、仅W、AR捕获、未接受CSR offer、CSR已提交未响应、已入B/R及受阻、全部阶段同时存在、恢复第一笔；用独立接受/提交/消费/取消账本，不能只数最终bank值。
- 对非法producer构造负例证明monitor会失败；不能悄悄把曾撤回未接收valid的刺激认作产品通过。实际层级至少桥+真实CsrBlock，独立桥实例证明无共享状态。
- 随机预算16 seeds×1000事务，记录实际seed、每类场景命中与墙钟；未执行不得填成功。每个seed先规定有效事务计数/取消处理和有限完成等待，随机通过不是无条件活性证明。
- safety无B/R公平性；独立参考队列和配对、owner/保留空间、响应稳定，cover读写竞争/背压恢复/reset后新提交。记录base/induction与cover深度、实际PASS/FAIL/ERROR，给mutation/负例验证observer不会空过。
- 依赖锥从外部5个ready/valid反向追到寄存器截止，拒绝任何输入路径；综合单元用已知组合/FF白名单，latch/未知类型不得当FF绕过。参考fr195_param_sync_fifo_formal.rs白名单和fr195_rv_reg_slice_formal.rs input_path，不降低严格度。
- 旧M0、M1、CSR定向兼容，prelude-only独立示例，最后全workspace；formal与综合独立执行，不拿cargo ignored或历史证明计通过。

### 前故事学习、工具与架构

127.2提交9393c26新增CsrBlock leaf与同源软件产物；回归1787 passed/0 failed/18 ignored，其中4项CSR专用formal/综合另跑真实通过。127.2独立review发现producer monitor缺口和多字段oracle盲点，已修复；本故事必须独立监测所有输入协议且预置有效负例。历史源码应归档并保存member hash，避免普通.rs副本污染搜索；每次实际工具日志保留原字节，源码manifest对应具体快照而非误指当前live文件。

最近5提交：127.2 CSR、127.1风险门、126.4 FIFO/M1、126.3切片、126.2共享定义；没有新增运行时库需求。AD-1/6/7/13/18/30/31：设计依赖仅bitloom-prelude，单一FrozenHir、elaboration消解、无capturing closure/heap硬件对象/第二IR。Rust1.97.1/edition2024、firtool1.159.0、Chisel7.15.0不变；不承诺PPA、板级可靠性或所有后端全绿。Bitloom与samitbasu/rhdl无关。

本机候选PATH `/tmp/bitloom-maintenance-tools/bin`（Icarus/vvp12）、`/tmp/bitloom-1263-sby-installed/bin`（既有固定SBY）；Yosys0.33/Z3 4.8.12为此前实测环境，实施须重新记录实际版本；不能把脊柱可选综合版本当本机已安装。沿用ci-sby-pins.env安装策略。BFM固定requirements见scripts/phase24-axi-bfm-requirements.txt：Python3.12、cocotb2.0.1、cocotbext-axi0.1.28等，不升级。工具缺失先恢复环境、保留失败再重跑，不降低AC。

2026-09-21上游查询：[PyPI cocotb](https://pypi.org/pypi/cocotb/json)当前2.1.0，[cocotbext-axi](https://pypi.org/pypi/cocotbext-axi/json)0.1.28；本故事仍使用已探测固定2.0.1/0.1.28，无升级必要。[官方BFM文档](https://github.com/alexforencich/cocotbext-axi)说明高层访问会拆分/对齐，因此未对齐及AW/W独立序列必须raw驱动；同一针脚不能同时由raw与高层BFM驱动。Arm最新入口重定向后未取得正文，不将其作为已核验条款；本项目具体错误/仲裁/容量语义来自已批准合同。cocotb stable文档显示开发版标题，版本结论以包索引实际响应为准，不混用浮动文档和固定工具。

### References / 输入发现

SELECTIVE_LOAD：epics.md Phase24与Epic127全部4故事；PRD addendum Phase24 FR196/NFR93–99；架构脊柱及AD-30/31；正式合同、epic-127-context/NFR14、127.2完整故事/review/CSR端口代码。无适用UX文档，属于库/硬件工作。

- [Epic127](../planning-artifacts/epics.md#epic-127-csr-与总线连接)、[PRD addendum](../planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md)。
- [正式合同](../../docs/ip/phase24-contract.md)、[风险门](epic-127-nfr14.md)、[CSR实际接口](../../docs/ip/csr.md)。
- [127.2](127-2-csr-描述-rtl-与软件地址产物.md)、[独立review](../test-artifacts/127-2-independent-review.md)、[最终证据](../test-artifacts/127-2-final-verification.md)。
- [架构脊柱](../planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md)、[模块组合](../../docs/ip/module-composition.md)。

### ATDD Artifacts

固定接口：[API与端口契约](../test-artifacts/127-3-atdd-api-contract.md)。测试策略与当前步骤：[ATDD清单](../test-artifacts/atdd-checklist-127-3-axi-lite-csr-桥.md)。脚手架入口为 `crates/bitloom/tests/fr196_axi_lite_csr.rs`（辅助文件在同名目录）和 `crates/bitloom/tests/fr196_axi_lite_csr_formal.rs`。

ATDD阶段记录：实际cargo --no-run退出101，仅两项缺失AxiLiteCsrBridge的E0432，见[真实红测](../test-artifacts/127-3-atdd-red.log)。11测试入口（9普通、2专用），14成员源码归档已回读验证；当时尚无桥实现、行为通过或形式证明通过；后续build证据另列。背压cover已要求观察受阻期间新的相反类别提交，不能用此前历史交易替代。

## Dev Agent Record

### Agent Model Used

Codex；bridge1273_story创建与ATDD，bridge1273_implementation实现及修补；三路build审查已完成，主代理负责证据验收、文档和七步状态。

### Debug Log References

创建阶段见[create-story验证记录](../test-artifacts/127-3-create-story-validation.md)，ATDD见上节。首轮实现与实测见[build证据](../test-artifacts/127-3-build-evidence.md)：9普通测试、2专用formal/综合、78兼容测试和文档例通过。三路内部审查随后提出10项验证补强，修补完成；主代理[统一验收](../test-artifacts/127-3-review-root-verification.md)11普通、3专用、78兼容与示例均通过。独立code-review与automate均已完成，见[审查](../test-artifacts/127-3-independent-review.md)与[automation](../test-artifacts/127-3-automation-summary.md)；实际clean/fmt/workspace已通过1801/0/21，详见[最终验收](../test-artifacts/127-3-final-verification.md)；本文件随Story127.3单故事提交。

### Completion Notes List

- Ultimate context engine analysis completed - comprehensive developer guide created.
- 已按checklist补齐CSR消费与AXI消费区别、offer锁定、reset结构边界、零WSTRB、DECERR测试peer和旧bank顺序隔离。
- 用户已授权七步连续执行，直接应用必要改进；七步验收完成，当前done；完整回归1801通过/0失败/21忽略，Epic127及M2保持开放。

### File List

- `.github/workflows/ci.yml`。
- `_agile-output/implementation-artifacts/127-3-axi-lite-csr-桥.md`。
- `_agile-output/implementation-artifacts/goal-all-stories-seven-step.md`。
- `_agile-output/implementation-artifacts/spec-127-3-axi-lite-csr-bridge.md`。
- `_agile-output/implementation-artifacts/sprint-status.yaml`。
- `crates/bitloom-prelude/src/ip/axi_lite_csr.rs`。
- `crates/bitloom-prelude/src/ip/mod.rs`。
- `crates/bitloom/tests/fr196_axi_lite_csr.rs`。
- `crates/bitloom/tests/fr196_axi_lite_csr/integration.rs`。
- `crates/bitloom/tests/fr196_axi_lite_csr/producer_monitor.sv`。
- `crates/bitloom/tests/fr196_axi_lite_csr/tools.rs`。
- `crates/bitloom/tests/fr196_axi_lite_csr_formal.rs`。
- `docs/ip/README.md`。
- `docs/ip/axi-lite-csr-bridge.md`。
- `docs/public-api-1-0-surface.md`。
- `scripts/check_fr196_bridge_example.py`。
- `_agile-output/test-artifacts/127-3-*`：创建/ATDD/build/review/automate/final日志、摘要、manifest与源码/工具归档。
- `_agile-output/test-artifacts/atdd-checklist-127-3-axi-lite-csr-桥.md`：ATDD历史清单及最终GREEN回链。
