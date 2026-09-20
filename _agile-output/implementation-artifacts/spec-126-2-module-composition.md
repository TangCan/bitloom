---
title: 'Story 126.2 共享模块定义与实例校验'
type: feature
created: 2026-09-20
status: done
route: dispatch
baseline_commit: 706632a304bf7bd21df4d8ef6ea29c85525d6fe5
review_loop_iteration: 0
context:
  - AGENTS.md
  - docs/ip/phase24-contract.md
  - _agile-output/implementation-artifacts/epic-126-context.md
  - _agile-output/implementation-artifacts/epic-126-nfr14.md
  - _agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md
  - docs/public-api-1-0-surface.md
---
<frozen-after-approval>
## Intent
完成FR194：设计只依赖prelude，就能在一个ElaborateSession中复用模块定义、实例化多个IP，最后finish一次；同参数复用、不同参数专门化，命名隔离与非法组合诊断可信。用户已要求继续，126.1风险门禁完成后实施126.2。
## Boundaries & Constraints
不合并FrozenHir、不引入新IR、不实现native层级执行、不升级工具/发布版本、不执行126.3/4 FIFO或流组件。先结束顶层再定义子模块必须可用；不能覆盖活动模块。旧elaborate入口及GPIO行为兼容。新API明确登记FR142与SemVer minor策略。一故事一提交，主代理负责提交；不得git commit/push。
</frozen-after-approval>
## Code Map
- builder/src/lib.rs：ElaborateSession.current及BuilderOwnedHir是唯一状态；begin_module会覆盖current，finish会丢弃current；add_instance与generate_instances已有实例IR。
- hir/src/lib.rs：freeze及validate_instances缺全图环、重复名/连接、方向/类型与实例驱动校验；模块/语句已有PartialEq。
- prelude/src/ip/gpio/base.rs：Gpio仅60行，提取唯一模块定义体，old elaborate转helper+finish。prelude/src/lib.rs已有Session重导出。
- bitloom/tests/simulator_bit_vectors.rs层级例、fr193_axi_protocol.rs实际RTL harness可参考；不得复制其DUT逻辑作为oracle。
- add_instance.params目前后端忽略，本故事参数化由展开时专门化模块实现，不宣称实例参数覆盖已支持。
## Tasks & Acceptance
- [x] 提供公开Session模块定义helper：显式模块名、显式参数、非捕获函数指针定义体；返回可用于add_instance的模块名或等价句柄。现有HIR内定义，不保存Rust closure到tick。活动模块中调用返回Diagnostics并保证最终不能成功freeze；begin_module重入/finish未end也诊断。
- [x] 使用完整规范参数键值身份（顺序不影响，重复参数键拒绝），禁止有损哈希作为唯一身份；同名同参数请求只保留一个定义，重复请求必须核验定义内容/来源不能按名字直接短路；不同参数不得偷偷复用同名定义，需不同明确合法名字。同名不同定义/与手工模块碰撞均Diagnostics。实现可每次在同session构造候选再比较完整内容；保留单一IR。记录回调只负责模块体，不操作begin/end/finish，违规必须失败。参数专门化示例8位与16位。
- [x] Gpio::define_module提供组合入口，旧Elaboratable::elaborate调用同一helper保持唯一逻辑体，旧ABI/端口/时序不变。不要求把所有旧IP都重构。
- [x] freeze统一拒绝重复模块、重复端口/实例名、实例名与本地net冲突、重复child_port连接、未知child_port、缺模块/输入、未知父net、宽度及类型错误、非法输出目标、实例输出与实例/过程多驱动。输出只能接可驱动wire或父Output，不能驱动Input/寄存器；child Input可读父Output或reg，因此不可只比较方向标签。Clock/Reset需精确类型；Bool与UInt1保持现有有据可依兼容，明确文档化。dangling现有合法用途保持，不漏检查未知端口。
- [x] 全图循环检测含显式top及不可达环、自环；explicit circuit.name优先选top，否则仅唯一根可回退，多根诊断；特殊Analog/InOut验证使用实际选定top。既有合法单模块/导入保持。诊断使用未占用稳定code及中英文消息。
- [x] 定向测试每类正反边界：上述诊断、同参数复用、参数重排、不同参数/同名冲突、重复定义体冲突、活动模块错误、top先/后、共享局部名。测试通过公开prelude组装两个8位及一个16位寄存模块，独立输入/reset或enable，实际生成RTL/Icarus逐拍验证状态隔离与复位，保存命令、工具版本/种子及执行证据。不支持层级native仍明确拒绝，不能用成功emit代替执行。
- [x] GPIO旧elaborate与helper单模块结构/行为等价回归；两个GPIO组合实际RTL独立写入/方向/屏蔽/读回覆盖。新增真实RTL测试CI BITLOOM_REQUIRE_RTL=1缺工具失败。
- [x] docs/ip模块组合中文说明（可新建module-composition.md）：可直接编译仅依赖prelude的完整例、命名/参数/回调约束、top顺序/一次freeze、支持矩阵/局限。公开API清单登记具体符号；新增SemVer minor但不改crate版本。状态126.2由主代理收尾done，126.3/4backlog、M1未闭合；AGENTS必要状态由主代理维护。
## Implementation Notes
允许按代码调查调整小型API签名，不扩大意图。不要用函数指针地址当身份（编译器可合并），不要单按键缓存忽略定义差异。完整语义比较若忽略Span需显式理由/测试。错误要通过Diagnostics而非panic。合理防止深图递归堆栈问题（迭代遍历优先）。优先复用现有模块结构和验证，不引入泛型注册框架。所有输出说明中文，代码遵循项目风格。仅本故事相关修改。
## Spec Change Log
## Review Triage Log
- blind-1 / medium / defer：旧add_instance及导入路径就未验证实例HDL标识符，当前名称碰撞检查未改变该行为；完整实例/网名合法性属于既有生成器缺口，登记后续统一处理。新define_module自身已拒绝非法模块名，不扩大本故事到所有标识符。
- blind-2 / medium / defer：Instance.params被后端忽略是既有缺口，意图明确仅展开时专门化、排除实例参数覆盖；本次文档明确限制，登记后续诊断/后端支持契约。
- blind-3 / low / patch：嵌套helper以?返回已记录诊断，外层会重复追加；已按完整Diagnostic去重并增加嵌套传播测试。
- blind-4 / low / patch：E0244缺少定位身份；直接补模块名与原/新参数，不改变诊断代码或API。
- blind-5 / low / patch：E0255缺定位线索；补最多8个Kahn残留模块及总数，明确残留可包含受环阻塞者，非全部在环上。
- blind-6 / medium / patch：新RTL子进程无超时，回归可能挂起；使用60秒上限、kill/wait及保留日志，新增实际超时路径测试。
- blind-7 / medium / patch：任意panic可使层级拒绝测试误绿；改为核验具体unsupported hierarchy消息。
- blind-8 / medium / patch：非空回调诊断保留缺断言；新增含code/span/双语文本的诊断并核验即时返回与finish。
- blind-9 / low / patch：单层RTL不足以覆盖多个连接层级；添加顶层→wrapper→leaf实际RTL，两个8位与一个16位分支仍核验独立状态。
- blind-10 / low / patch：文档例只有临时手工执行记录；新增标准库提取编译脚本并接入现有CI的workspace测试之后。
- edge-1 / medium / patch：子进程挂起风险与blind-6同根；同一超时修补及运行测试覆盖。
- verification-1 / medium / patch：非空回调诊断保留缺口与blind-8同根；对应新测试覆盖code/span/双语文本和两处返回。
## Verification
先cargo test -p bitloom-builder -p bitloom-hir，再新增bitloom集成tests（PATH=/tmp/bitloom-maintenance-tools/bin:$PATH BITLOOM_REQUIRE_RTL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1）。cargo fmt --all -- --check；python3 scripts/check_phase24_gate.py。保存实际测试/RTL运行记录与工具命令至docs/ip/evidence下，不存二进制。主代理随后跑workspace与SemVer检查，实施代理无需重复全workspace长矩阵。填写实际结果不预填通过。完成报告含未完成/风险，spec状态保持in-progress待审。


## 实施与定向验收记录
已完成Session helper、GPIO共用体、HIR统一验证及公开API清单。具体API使用Vec<(String,u32)>显式参数和返回Result的函数指针；完整Module比较含Span，保守区别不同源码位置，已有定向测试与文档说明。Bool↔UInt1允许，其余类型精确一致；模块名拒绝HDL保留字。两个旧夹具已改为明确顶层，避免依赖原错误多根/特殊IO选择。

定向builder/HIR共71测试通过（新增10+12），3个原doc例ignored；fr194集成4通过，包括3次真实RTL编译执行、544拍/3574比较。独立prelude-only文档例cargo run成功。SemVer三包各196通过/58不适用跳过；fmt与Phase24 gate通过。详细证据见[组合验收](../../docs/ip/module-composition-evidence.md)。完整workspace回归1718通过/0失败/6原有ignored；审阅后受影响五包114通过、FR194五测试通过、prelude-only脚本通过。详细时间关系与实测以证据文件为准。

## 完成记录
三层审阅已完成；修补诊断去重/定位、超时、明确错误断言、多层RTL和CI文档例。当前变更问题均验证通过；两个既有生成器缺口登记延后。FR194及126.2完成，M1保持未关闭；不推送、不发布、不修改版本或工具钉。
