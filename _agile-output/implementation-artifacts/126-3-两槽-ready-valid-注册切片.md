# Story 126.3: 两槽 ready/valid 注册切片

Status: done

## Story

作为设计与集成维护者，我希望在单一ElaborateSession中复用两槽注册切片，以隔离上下游组合路径，并在背压下保证数据顺序、守恒和稳定。用户当前目标已授权依次完成全部未完成story的七步流程；本故事不再受旧“仅126.1/126.2”执行安排限制。完成本故事仍不能关闭M1或全部FR195。

## Acceptance Criteria

1. **Given** 126.1已done、126.2共享定义可用，**When** 通过prelude创建 `RvRegSlice<const WIDTH: u32 = 32>`（WIDTH 1..64），**Then** `Elaboratable::elaborate()` 与 `define_module(&mut ElaborateSession, name)` 共用唯一模块体；参数以WIDTH完整登记，顶层仅finish一次；WIDTH 0、65和u32::MAX以Diagnostics失败且不panic。
2. 端口固定：clk:Clock、rst:Reset（同步高有效）、flush:UInt1、input_valid:UInt1、input_data:UIntWIDTH、output_ready:UInt1；输出input_ready:UInt1、output_valid:UInt1、output_data:UIntWIDTH。模块有两个数据槽。reset > flush > 正常握手，清占用/有效信息，无效payload不作为结果；reset/flush周期的表面握手视为取消，不提交事务。
3. 仅上升沿valid&&ready传输；输出valid/data和输入ready只依赖寄存状态/常量，无任何输入（含reset/flush）到输出的组合路径。没有空直通：入队后下一周期有效；满时input_ready=0，即使同拍下游ready=1也不接受新输入。满后pop会在下一周期恢复ready；不承诺所有状态零气泡。一般占用1时允许同拍pop+push，填充后上下游持续有效可每拍输出一笔。
4. 输出valid&& !output_ready时payload/valid保持（除下一上升沿reset/flush取消）；数据不丢失、不重复、不重排。独立软件队列按实际握手记账，按epoch分离reset/flush取消，最终排空后accepted=delivered+cancelled。覆盖空、单槽、满、同时push/pop、满释放、长背压、reset/flush在途及相互重叠。
5. WIDTH=1/8/32/64分别通过Interpreter、Compiled及真实生成Verilog/Icarus逐拍检查；固定多种子随机流，生产者受阻时保持valid/data，完整排空并断言所有关键状态实际命中。验证WIDTH64高位不被截断。系统组合至少两个实例用不同payload/ready驱动并真实RTL检查隔离；native层级保持unsupported。
6. 对实际生成RTL运行小状态形式验证（至少WIDTH=1与8），使用真实Yosys/SymbiYosys+Z3而非枚举冒充：证明容量≤2、顺序/守恒、受阻稳定、无非法握手；证明或结构追踪无输入→输出组合路径。单独cover空→满→释放与同时push/pop，避免空泛证明。只证明安全性；有限随机排空不作为无限活性证明，若加入活性必须明确公平性假设。工具缺失、证明FAIL/UNKNOWN、超时必须非零失败。
7. 保存ATDD红测、真实RTL/形式日志、工具版本与种子，更新FR142显式API清单和中文使用说明。按用户七步完成独立代码审阅、自动化覆盖及clean+fmt+just test回归后，才将126.3标done并单独提交。

## Tasks / Subtasks

- [x] ATDD：从以上AC写独立红测（缺API导致编译失败可作为首个红证据；不以测试框架/工具故障冒充行为红测）。
- [x] 实现 `crates/bitloom-prelude/src/ip/rv_reg_slice.rs` 并在 `ip/mod.rs`重导出；仅使用既有builder/HIR。
- [x] 完成native、实际RTL及双实例组合验收，不修改旧SyncFifo行为。
- [x] 完成真实formal prove+cover与非组合路径验证，CI严格执行。
- [x] 新API文档、风险补充、证据、审阅和automation。
- [x] `cargo clean && cargo fmt --all && just test`（设置必需RTL工具PATH与test优化，保持断言/溢出检查）；检查差异，单故事提交。

## Dev Notes

### ATDD Artifacts

- Checklist: `_agile-output/test-artifacts/atdd-checklist-126-3-两槽-ready-valid-注册切片.md`
- API/native/RTL tests: `crates/bitloom/tests/fr195_rv_reg_slice.rs`
- Formal tests: `crates/bitloom/tests/fr195_rv_reg_slice_formal.rs`（专用`--ignored`入口）
- Red logs: `_agile-output/test-artifacts/126-3-atdd-api-red.log`、`126-3-atdd-formal-red.log`；均exit101 / E0432缺RvRegSlice。该红测时点产品尚未实现；后续实现和真实绿测见126-3-build-evidence.md。
- E2E browser tests: N/A（Rust硬件组件无UI）。

### 技术与架构约束

Rust1.97.1/edition2024，设计依赖仅bitloom-prelude；不增加新IR或使用FrozenHir拼接；不做通用native层级模拟、异步/多时钟、BRAM或通用内存改造。保留firtool/Chisel产品钉。两槽可用front/back寄存器和占用计数，count译码可组合但只依赖寄存状态。切勿让input_ready依赖output_ready，也不要在空态把input_data直通output_data。实现定义回调非捕获函数指针；同名不同WIDTH必须通过126.2已有身份校验拒绝。

### 现有文件与变化

- `crates/bitloom-prelude/src/ip/mod.rs`：现有按协议的私有module+pub use；只新增rv_reg_slice，不改旧路径。
- `crates/bitloom-prelude/src/ip/gpio/base.rs`：126.2共享定义范式，参考即可不改。
- `crates/bitloom-prelude/src/ip/sync_fifo.rs`：旧8×4memory FIFO，时序不同，不用它代替本组件或顺带重构。
- `crates/bitloom-builder/src/lib.rs`：已有assign_*、寄存器及define_module，优先复用；如发现阻塞仅作有证据的最小修复。
- `crates/bitloom/tests/fr194_module_composition.rs`：实际RTL工具超时/日志保存、层级实例方式可复用模式；黄金期望须独立。
- `scripts/ci-install-sby.sh` / `ci-sby-pins.env`：探针确认固定SHA是annotated tag对象，HEAD是其peeled commit；现有直接比较会误判。允许修正验证为严格对象存在且peel后与HEAD匹配，不改变SHA值/上游内容；若fresh-install依赖模块布局也实际失败，按上游安装方式修复并验证。不得用stub冒充工具。
- `.github/workflows/ci.yml`：现有正式formal job使用固定sby工具，可添加本组件专用验收，不能用历史FR119 fixture替代本故事。
- `crates/rhdl-formal/fixtures/fr119/fr119_{pass,fail}.sv`、`scripts/formal-sby-check.sh`：真实工具已复现clocked SVA @语法错误；在posedge always内使用受支持的立即assume/assert property，明确初始reset前提。现有expect fail使sby对真实FAIL返回0，wrapper须读取实际status并把非PASS报告为非零；保留原FAIL文件负例意图和真实counterexample。原始日志 /tmp/bitloom-1263-existing-formal.log 与 /tmp/bitloom-1263-fr119/。这修复必要CI前置，不改变本故事产品范围或冒充新组件证明。

### 前故事经验

126.2提交62832ea：同session模块体复用，完整参数排序+内容含Span比较；每次调用helper验证体，不能靠函数地址或名字短路。module helper不能嵌套于活动模块；先定义叶子或先结束top再定义叶子都可。native层级明确拒绝。输出连接不能驱动父输入或reg；Bool↔UInt1兼容，Clock/Reset严格。测试应检查具体诊断，不以任意panic算通过；外部工具设超时并保存失败产物。

### 验证工具与上游资料

本机Rust1.97.1；Icarus12.0路径 `/tmp/bitloom-maintenance-tools/bin`；Yosys0.33、Z3 4.8.12；sby固定对象bfc1c47eb786496fe794481ff88e75728f0529a6（yosys-0.47 tag，peeled commit daed0e1544fd96ee7dab843e5a891d92784c6230），实际prove/cover探针见 `_agile-output/test-artifacts/126-3-formal-tool-probe.md`，正式make install后的PATH为 `/tmp/bitloom-1263-sby-installed/bin`。工具存在不等于组件证明通过。

官方SBY文档确认 `smtbmc z3` 支持 prove 与 cover，必须区分归纳证明和仅有界检查：[SBY reference](https://yosyshq.readthedocs.io/projects/sby/en/latest/reference.html)、[upstream quickstart](https://yosyshq.readthedocs.io/projects/sby/en/latest/quickstart.html)。不因最新资料改动项目工具钉。

### References

- `docs/ip/phase24-contract.md` Ready/valid与FIFO合同。
- `_agile-output/planning-artifacts/epics.md` Epic126完整故事、FR195。
- `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md` AD30/31及单HIR约束。
- `_agile-output/implementation-artifacts/epic-126-nfr14.md` 与 `spec-126-2-module-composition.md`。

## Dev Agent Record

### Agent Model Used
Codex（当前会话）。

### Debug Log References
ATDD红测、build、两轮审阅、automation与clean回归已归档；最终索引：_agile-output/test-artifacts/126-3-final-verification.md。

### Completion Notes List
create-story已依据正式合同、前故事与工具探针完成；无UI/UX工作。本文件是实现合同，不是功能完成证据。

### File List
本故事/spec/执行账本；prelude的rv_reg_slice与重导出；fr195两套测试及FR119负例；SBY安装/运行脚本和fixtures；文档例检查脚本、CI；API/使用/风险文档及test-artifacts下本故事证据。

### Review Findings（独立bmad-code-review，2026-09-20）

四层均返回：Blind 10条建议；Edge无发现；Verification无缺口；Acceptance确认AC1–6通过、AC7须继续七步。0 decision-needed、5 patch、0 defer、5 rejected；用户既有完整实施授权覆盖修补，无需重复确认。

- [x] [Review][Patch] R1 / medium：默认test job的真实RTL失败产物未上传；增加target/fr195失败归档。
- [x] [Review][Patch] R3 / medium：FR119 version/help探针没有期限，可能阻塞本地运行；补有kill-after的有限工具探针。
- [x] [Review][Patch] R4 / medium：真实负例使用固定源码目录，外部并发运行可能覆盖其证据；改为独立复制fixture与对应trace路径。
- [x] [Review][Patch] R8 / medium：文档示例目前只有手工编译记录；接入既有prelude-only文档示例门禁以防API漂移。
- [x] [Review][Patch] R10 / low：更新已完成里程碑与当前review状态；ATDD缺API语句明确为历史时点，AC7仍未完成。

#### Rejected

- R2 / false：sby非零时set-e保留真实工具错误日志并非零退出，符合FAIL/UNKNOWN/ERROR严格失败合同；统一前缀不是漏报或假通过。
- R5 / low：缺失/空status、UNKNOWN和超时均在当前路径明确非零，已保留真实FAIL与工具缺失验证；额外模拟所有工具故障组合属于可选测试扩充，未发现未处理分支，不引入产品stub或额外状态。
- R6 / low：reset/flush在formal输入未受限制，安全证明覆盖任意在途取消，native/RTL也真实命中；额外取消cover是可选证明可达性补强，不是当前验收缺失。
- R7 / low：现有实际状态命中和formal安全证明已覆盖合同；要求所有取消状态×握手组合新增矩阵超出当前明确AC，未发现未覆盖的产品错误。
- R9 / low：独立端口队列断言、真实两个cover和原始RTL/native对照已验证证明路径；再引入故障RTL库增加维护分支，未发现当前断言空泛或未运行。

技能默认review后标done由用户七步顺序覆盖：automation、clean全回归和提交前继续review，不能提前关闭故事。

独立审查收尾：四项工具修补及跟踪修正全部完成；FR119 8项、两个并发隔离负例与两个文档例均通过。详见126-3-review2-tools.md。无未决项或延期项；review完成不等于故事done，继续步骤五/六/七。

## 七步最终验收（2026-09-20）

create-story、ATDD（真实E0432红）、build、独立code-review、automate、clean/fmt/just test及本故事提交全部纳入本次交付。全workspace命令退出0，1732通过、0失败、8忽略；其中2个组件formal入口已在专用命令中真实通过，另6个为既有doc-test忽略。最终formal每宽1/8各4条cover及归纳安全证明PASS；不宣称无限活性。

RvRegSlice的FR195子集完成；126.4参数FIFO仍backlog，因此Epic126/M1未关闭。独立审查未决0、延期0。状态done对应本次单故事提交；整目标继续126.4，122.2/122.3上游条件仍未满足。
