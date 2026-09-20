---
title: 'Chisel 数值一致性、边界回归、性能剖析与能力证据'
type: bugfix
created: '2026-09-20'
status: done
route: dispatch
baseline_commit: 6fda33eb10ca576e0aabc649b8d6e7a172a9ec4b
review_loop_iteration: 0
context:
  - AGENTS.md
  - _agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md
---

<frozen-after-approval reason="用户已明确授权依序完成全部四项，保留完整目标">

## Intent

依序修复 Chisel 数值差异并真实编译执行生成 RTL，扩大组合/反馈/存储器/多模块边界回归，再对较大工作量采集 CPU 与分配剖析，最后建立按能力分类的验证记录。不能以源码文本断言代替 Scala/JVM/RTL 执行，也不能只写优化建议代替实际剖析。

## Boundaries & Constraints

沿用 FrozenHir、prelude-only、Rust1.97.1、Chisel7.15.0、firtool1.159.0；历史结项及 FR189/NFR91 不变。修复已支持运算和单模块时序；不新增任意精度或完整多模块仿真，不以旧实现的静默忽略当支持。内存同址读写在 FIRRTL 是 undefined，不宣称跨后端碰撞值等价；明确内部行为与限制。先测再决定是否优化，无大规模重构；不发布或推送。

## I/O & Edge-Case Matrix

| 场景 | 输入 | 期望 |
|---|---|---|
| Chisel 运算 | 现有同宽/异宽矩阵、SInt8 0xff >>1、SAR、大移位 | Scala 实际编译与降级后，同一参考值和 RTL 对拍；逻辑0x7f、算术0xff |
| 组合依赖/反馈 | 固定种子合法 DAG、反向声明链、q.d=q+input、双寄存器交换 | 声明次序不影响组合结果，边沿采样已稳定组合值，同时提交寄存器；原生/生成模型/实际 RTL 有独立预期 |
| 存储器 | 非碰撞访问、同址读写两种语句顺序、复位/使能/延迟 | 非碰撞独立预期；碰撞仅验证并文档化内部约定，不称 FIRRTL/Chisel 碰撞等价 |
| 多模块 | 父子实例、同名局部寄存器、模块排列变化 | 有层级能力的后端真实编译；不支持的仿真/生成入口明确拒绝，不能返回貌似有效的首模块结果 |
| 剖析 | 较大寄存流水线、两种引擎 | 独立 checksum、多次交替计时、真实 CPU 调用归因与分配量；工具受限则使用用户态替代并如实标注 |
| 能力证据 | workspace、JVM/RTL、formal、SystemC、发布预演等 | 命令/工具钉/最近结果/外部执行类型/限制清楚分列；未运行不标通过 |

</frozen-after-approval>

## Code Map

- `crates/rhdl-firrtl/src/chisel.rs`：共用 emit_expr；Shr/Sar 当前均直接 >>，SignExtend 返回 UInt；RegDecl 总用 UInt RegInit。沿现有 emitter 修正，避免独立旁路。
- `crates/bitloom/tests/simulator_bit_vectors.rs`：已含独立参考、Rust 生成、Verilog/firtool 实际执行，同宽1/8/32/64与异宽；复用其矩阵与 testbench，Chisel clock/reset/io_ 端口需适配。批量一次 JVM 编译降低开销。
- `scripts/chisel-fr28-compile*.sh`：现只编译固定夹具，Scala2.13.16、sbt1.10.11；新增严格数值入口与 CI/Justfile，不冒用旧门禁。
- `crates/bitloom-sim/src/{lib,engine,generate,cycle}.rs`：comb 单遍源序，tick 先 seq 后 comb；settle 现需手动调用。next_regs 已同时提交；sync pending 在 seq 前提交。cycle 生成丢失 RegDecl enable/async 标志。
- 同址写立即修改 bank，当前结果受语句顺序影响；`rhdl-firrtl/src/lib.rs` 明示 undefined，`docs/fr159-memread-full-emit.md` 不承诺 RTL 内存深化。
- Sim/GeneratedFunctional 只执行首模块；cycle 已拒绝多模块/Instance。统一明确范围，不实现层级模拟器。
- `crates/bitloom-sim/benches/baseline.rs`：复用交替顺序、独立 checksum；加32级寄存流水线、按工作量/引擎选取、可调预热。分配计数仅 benchmark 测量区，不污染正常计时。
- `.github/workflows/ci.yml`、`Justfile`、`scripts/*check*.sh`：能力证据索引；FR164 是外部编译+另一个 Rust 夹具，不能称执行其输出 RTL；style marker 验收不等于实际 lint 工具。

## Tasks & Acceptance

- [x] Chisel：先真实复现，再修复共用生成器；同一参考矩阵经 Scala/JVM→RTL 对拍；严格 CI 入口，缺工具失败。
- [x] 边界：加入上述矩阵测试，修复支持子集中的顺序/反馈/生成元数据问题；维护唯一语义，明确碰撞与层级限制；失败保留输入/种子/工具身份。
- [x] 剖析：实际运行较大流水线的多次 release 基线、CPU 调用归因和分配计数，保存原始证据与中文结论；仅在热点证据支持时作局部优化。
- [x] 证据：新增能力清单/本轮执行记录，更新相关过时文档及已解决 deferred 条目，不改写历史事实。

Given 相同输入与支持子集，when 执行各模型及 Chisel 降级 RTL，then 对照独立参考通过。Given 不支持层级仿真，when 调用入口，then 明确失败。Given 相同剖析命令，when 重跑，then 校验和稳定且方法/环境可比较。Given 验证清单，when 审阅，then 可区分行为证明、文本验收、未执行和发布预演。

## Implementation Notes

用户要求全部四项且顺序明确，不再次请求范围批准。按上述任务顺序实施。Root 独立准备临时 JVM/Valgrind 工具与验证环境；实施 agent 拥有产品/测试/文档修改。perf_event_paranoid=4 已验证阻止 perf，采用 Callgrind 用户态调用归因，不能把其仪器化耗时当本机吞吐。可与 root 协调工具路径。

### Root 独立验证记录（实施中）

- Chisel 原始生成物先出现 SInt64 ← UInt64 连接失败；修正 JVM 环境后，从原始 mixed RTL 实测 SInt8 `ff >> 1` 得到 `ffff`（期望 `007f`），保留 `/tmp/bitloom-chisel-shr-red.log`。新严格入口首轮八组数值矩阵真实执行通过，后续新增边界仍需重新执行。
- Scala 2.13.16 无法满足 Chisel 7.15.0 已发布 POM 的 Scala 2.13.18 依赖（SIP-51）。将 Scala 更新到 2.13.18，保留 Chisel/firtool 钉；不使用不安全版本绕过。
- 旧 FR28 固定夹具通过真实 JVM 编译：`BITLOOM_CHISEL_JVM_SKIP=0 bash scripts/chisel-fr28-compile-required.sh crates/rhdl-firrtl/testdata/fr28_golden_counter.scala`；这只证明编译，不代替数值对拍。
- `cargo test -p bitloom --test fr101_systemc_tlm_product --test fr107_systemc_tlm_at -- --nocapture`：12 项通过，本机 pkg-config SystemC 2.3.4，LT/AT smoke 实际 C++ 编译运行。
- `perf stat -e task-clock -- true` 被 perf_event_paranoid=4 拒绝；临时提取的 Valgrind 3.22.0 已完成 Callgrind smoke，允许用户态剖析。CPU 指令计数与原生吞吐分列。

### 实施与矩阵审计

Root 已逐段阅读基线以来完整diff（含未跟踪文件），核对四项任务均有实施及实际执行证据。

| 矩阵行 | 已执行的覆盖与证据 |
|---|---|
| Chisel数值 | simulator_bit_vectors 同宽/异宽用例；严格门禁13 tests、18实际JVM/RTL cases，含SInt逻辑/SAR、扩展目标类型及寄存器；审阅修复后重跑通过，增加signed Inc及无边沿async reset |
| DAG/反馈 | reverse_dag_feedback_and_simultaneous_registers：固定seed、反向24节点、累加和双寄存器交换；两引擎/功能模型/两类生成crate/direct/FIRRTL/Chisel通过 |
| 内存 | collision两种statement order × async/sync内部参考；noncolliding memory独立bank/read-stage/q/downstream、混合enable及mid-reset；真实Rust/direct/Chisel通过，明确排除FIRRTL memory与跨后端碰撞 |
| 多模块 | 两种排列父子流水线实际三后端RTL通过，所有仿真构造/生成入口拒绝；CLI wave/coverage可读负测通过，无root递归返回E0002 |
| 剖析 | pipeline32独立u32数组参考；原生5交替pairs、分配3pairs及真实Callgrind两进程，各自checksum一致；原始gzip/CSV和重跑命令归档 |
| 能力清单 | 实际workspace1680通过/6既有ignored；JVM行为/编译、SystemC实编译执行、发布dry-run、三库API检查、未运行formal/lint等分别说明 |

证据索引：[后端与边界](../../docs/backend-boundary-evidence-2026-09-20.md)、[实际剖析](../../docs/benchmarks/backend-boundary-profile-2026-09-20.md)。本轮还修复FIFO手写模型与RV32旧周期假设，保留架构黄金结果；未追加tick热路径优化或扩大层级/任意精度合同。

## Spec Change Log

## Review Triage Log


三路审阅均已返回后统一判定。Blind floor算式为工具输出说明，不是缺陷。全部14项发现逐条记录如下；同根因只在逐条判定之后分组。

| ID | 来源与问题 | 判定 | 证据与路由 |
|---|---|---|---|
| B1 | Blind：同process重复Net赋值拓扑误报环 | high | validate_unique_drivers明确允许同process重复target；排序将已覆盖赋值也当边，能误报合法最终DAG。patch：保留最后有效赋值语义后排序，执行独立回归。 |
| B2 | Blind：同RegD多次MemRead生成重复临时声明 | medium | 临时名只取RegDecl位置；同process相同RegD允许出现多次，两次循环输出同名val/reg。patch：按最后有效赋值语义消除已覆盖驱动并跨模型验证。 |
| B3 | Blind：显式top名绕过递归实例拒绝 | medium | 现validate_instances只验证连接、不检测图环；baseline同样允许该输入，当前E0002仅承诺无可识别根。defer既有递归图验证缺口。 |
| B4 | Blind：多个fallback roots仍依赖声明顺序 | medium | 新代码find第一个未实例化根；baseline直接first也有相同歧义。测试/本轮合同是有明确top的父子设计，未交付多根选择策略。defer既有多根歧义，并保留边界说明。 |
| B5 | Blind：special IO与fallback top身份不一致 | medium | validate_special_io在前且仅比较circuit.name；baseline也如此，电路标签与模块名不同的Analog顶层原先已被拒绝。defer既有顶层特殊IO校验问题。 |
| B6 | Blind：新拓扑排序反向链最坏立方复杂度 | medium | 每轮扫描候选×依赖×所有remaining target并重复分配dependencies；反向链合法且随节点增大会明显拖慢构造。patch：缓存依赖/目标索引消除重复全表扫描，并检查较长链；不扩展公共API或改tick热路径。 |
| B7 | Blind：SInt寄存器Inc仍使用1.U | high | 本轮RegInit改SInt后，既有Inc无条件q+1.U；公开assign_reg_d_inc允许该寄存器，新生成Scala类型不兼容。patch：按类型生成增量，真实JVM/RTL有符号计数回归。 |
| B8 | Blind：SAR移位量SInt未转UInt | medium | builder仅检查shamt有width，Chisel SAR仍裸>>shamt；baseline同样如此。本轮逻辑移位新规范化不代表SAR所有移位端口类型均已证明。defer既有signed-shamt边界并明确复现条件。 |
| B9 | Blind：异步reset缺外部行为回归 | medium | “没有任何async fixture”子句不精确：内部collision确有async=true且生成crate已运行；但外部导出均为同步，无法抓移除withReset的回归。patch：补时钟保持低时的reset断言。 |
| B10 | Blind：builder读API仍写latency1 on tick | medium | assign_reg_d_mem_read rustdoc未区分memory stage及RegD；已改变/澄清可见时序，读者会推导错误周期。patch：直接更正文档及enable/reset语义。 |
| E1 | Edge：重复Net排序改变最后值 | high | a=b; a=input; b=other可排成a=input,b=other,a=b，最终值变other。与B1同根因，patch保留最后有效driver。 |
| E2 | Edge：SInt RegD Inc Scala失败 | high | 与B7同一类型变化；同组patch，独立实际JVM回归。 |
| E3 | Edge：重复MemRead临时声明冲突 | medium | 与B2同一RegDecl索引复用；同组patch，不能只改名字而留下模型优先级差异。 |
| V1 | Verification：未验证异步reset在无时钟边沿清零 | medium | 按该层证据规则接受：当前post-edge assertions无法区分同步/异步，移除withReset仍会通过。与B9同组patch，补实际RTL between-edge断言。 |

分组路由：B1/E1、B2/E3、B6、B7/E2、B9/V1、B10为局部patch；B3/B4/B5/B8是baseline已有缺口，逐项追加deferred。无需改冻结意图、增加公共API或实施层级模拟器。

### 审阅修复结果

- B1/E1、B2/E3：freeze 规范化最后有效 Net/RegD，MemWrite 保留；两种组合覆盖、重复内存读及混合覆盖均有独立参考，真实生成 Rust/direct/Chisel 通过；无内存夹具也执行 FIRRTL。
- B6：依赖索引/就绪集合替代重复全表扫描；4,096节点反向链在两引擎及功能模型通过。不将构造期修改宣称为tick吞吐优化。
- B7/E2：Chisel/FIRRTL Inc按源类型使用有符号常量；1/8/32/64位同宽矩阵含有符号计数回绕，真实RTL通过。
- B9/V1：实际direct/Chisel RTL在时钟保持低电平、状态非零时断言reset，未等边沿即验证清零；不宣称周期Rust模型或FIRRTL同样证明无边沿事件。
- B10：builder内存读API文档已说明独立读阶段、目的寄存器及enable/reset语义。
- B3/B4/B5/B8：已逐项登记deferred，能力文档明确排除，未扩大本轮合同。

## Verification

先红后绿的 Chisel 数值对拍；边界 integration 与实际生成 crate 编译；sim/FIRRTL 定向测试；最终 cargo test --workspace、cargo fmt --all -- --check、三库 SemVer。真实 Chisel 7.15.0/JVM+firtool1.159.0+Icarus 数值门禁必须执行。较大流水线 release 计时与 CPU/分配剖析必须保留证据；独立门禁未运行则明确列出。

最终审阅后验证：workspace 1680 passed / 0 failed / 6既有ignored（452条target结果），完整严格JVM/RTL矩阵13 tests / 18 cases，格式和三库各196项SemVer检查均exit0。三路审阅发现已完成局部修复并核对，4项既有缺口已登记deferred。最终diff已重新生成并检查；无发布或push。
