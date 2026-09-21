---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-generation-mode', 'step-03-test-strategy', 'step-04-generate-tests', 'step-04c-aggregate', 'step-05-validate-and-complete']
lastStep: 'step-05-validate-and-complete'
lastSaved: '2026-09-21'
storyId: '127.4'
storyKey: '127-4-静态译码与-m2-关闭'
storyFile: '_agile-output/implementation-artifacts/127-4-静态译码与-m2-关闭.md'
atddChecklistPath: '_agile-output/test-artifacts/atdd-checklist-127-4-静态译码与-m2-关闭.md'
generatedTestFiles:
  - 'crates/bitloom/tests/fr196_csr_decoder.rs'
  - 'crates/bitloom/tests/fr196_csr_decoder_formal.rs'
  - 'crates/bitloom/tests/fr196_csr_decoder_integration.rs'
inputDocuments:
  - '_bmad/tea/config.yaml'
  - '.agents/skills/bmad-testarch-atdd/SKILL.md'
  - '_agile-output/implementation-artifacts/127-4-静态译码与-m2-关闭.md'
  - 'docs/ip/phase24-contract.md'
  - 'docs/ip/csr.md'
  - 'docs/ip/axi-lite-csr-bridge.md'
  - '_agile-output/implementation-artifacts/epic-127-nfr14.md'
  - '.agents/skills/bmad-testarch-atdd/resources/tea-index.csv'
---

# Story127.4 ATDD清单

## Step01/02：前置与模式

Richard，本轮按已授权Create模式执行。resolver激活步骤、persistent_facts和on_complete均空；中文配置。故事已由主代理审阅批准，8AC明确；127.1–3done，M0/M1关闭，现有Cargo集成测试/工具helpers可复用，环境具备。

检测为backend（Rust Cargo工作区、硬件生成器）；采用AI generation，不进行浏览器录制。测试runner为cargo test + Icarus/SBY/Yosys。Playwright/Pact mandate与library-integration-mandate已先读：此Rust硬件边界不属于JS/TS Playwright/Pact，包未安装，不因默认true增加依赖或Pact；浏览器E2E不适用，以真实RTL层级集成为产品端到端。Pact MCP工具清单仅探测一次，无SmartBear工具，pact_mcp_reachable=false；无broker调用，无provider states推断需求。

已读知识data-factories/component-tdd/test-quality/test-healing-patterns/test-levels-framework/test-priorities-matrix/ci-burn-in与三项mandate/Pact MCP。采用确定seed、可证伪断言、隔离产物、适用最低测试层和失败保留；不引入UI selector/auth/network工具。现有CSR与桥实际源码、测试tools、Cargo依赖已核对。用户连续七步授权覆盖常规确认，继续自动生成。

## Step03：冻结验证计划

| AC | 优先级 / 层 | 验收与独立观察 |
|---|---|---|
|1|P0 API/native结构|52端口精确方向宽度；两入口相同HIR、定义复用/冲突、两个实例独立，prelude-only例 |
|2|P0 native+真实RTL|65536地址驱动DUT比较手写range/local oracle；边界/holes/高位/未对齐独立黄金 |
|3|P0 周期oracle+formal|accept=selected leaf commit同沿、单owner、改变输入不串响应、DECERR pending、停顿稳定、only owner ready |
|4|P0 真实层级RTL|bridge+decoder+4 CsrBlock；正式布局/权限/mask、raw AW/W0/1/7/31、partial读进展、B/R独立背压 |
|5|P0 周期+集成+formal|reset各阶段优先、计数accept/commit/response/cancel与副作用、恢复/尾部重复监视；合法producer监测器负例 |
|6|P0 专用formal/synthesis|独立prove/cover、无公平性/响应时限假设；错误写无关rdata不限；mutant反例、原始两种RTL综合与桥5FF输出依赖锥 |
|7|P1 软件产物/文档/兼容|四头local+base独立黄金、C11共同include、重复生成、旧CSR/桥/M0/M1适用回归；文档例最终build提供 |
|8|P1 发布前门禁|七步与实际clean/fmt/justtest/专用工具由主代理最后验收；本ATDD不关闭M2 |

所有行为脚手架断言期望行为，不写assert(false)/todo伪红。首红必须由实际cargo编译缺CsrDecoder API产生并保留；不把编译红称行为测试已执行。常规行为测试应可在实现后直接运行，专用工具保留项目既有显式ignored惯例，build必须另跑。随机预算16seed×1000与并发producer、全面取消/突变目标须在build闭合，ATDD脚手架不足项显式列计划，不装作已证明。

环境实际探针由root保存127-4-root-tool-preflight.json（全部exit0），仅工具可用，不是行为PASS。

## Step04：能力探测与调度

requestedMode=auto，capability_probe=true；subagent工具可用，未暴露独立agent-team创建工具，首选subagent。实际A成功启动后B与旧审计worker复用均返回thread-limit；root批准容量fallback：等A完成后复用同worker执行B，记录顺序调度，不宣称并行加速。时间标识1274-20260921，分别输出/tmp/tea-atdd-api-tests-1274-20260921.json和/tmp/tea-atdd-e2e-tests-1274-20260921.json。

固定接口见127-4-atdd-api-contract.md，准确52端口，无可配置map；上游/leaf提交同沿，owner与DECERR单槽，reset优先规则明确。workers仅生成测试脚手架，Rust使用#[ignore]表达技能test.skip；真实首红仍由编译缺CsrDecoder验证，不将skip当成功。

Confidence: 9。Rationale：正式phase24-contract、已交付CSR/bridge实际端口源码、主代理127-4-root-contract-audit以及固定52端口契约提供明确可观察行为；既有Icarus工具driver可复用。Unknowns：产品内部状态命名尚不存在，形式observer只依赖端口；最终证明深度、综合资源、真实运行时和随机命中须build实测，当前不能推断。补读evidence-integrity/confidence-gate，未使用不可证伪占位断言。65536扫查采用无tick的组合eval-only或逐笔合法排空，不能每周期撤回受阻请求。

## Step04C：汇总

两组worker均完成并有效JSON回读，已持久保存127-4-atdd-worker-api.json与127-4-atdd-worker-e2e.json。实际3文件/9入口（5 native/API、2专用工具、2真实RTL/软件集成），全部Rust #[ignore]，无占位失败断言。A最初报告6个native入口，实际只有5，已据源码修正元数据，不添加测试凑数。

fixture内聚在各target，无跨target依赖：输入/独立range golden、fresh Sim、4×CsrBlock工厂、test peer、timeout工具driver。无需Playwright merged fixtures；UI selectors/data-testid/auth/service mocks均N/A。工具产物按label+PID隔离并保留供失败分析，clean前由主代理归档。已修正集成peer为posedge非阻塞赋值，避免在采样沿前改动提交前状态。普通行为仍未执行；完整矩阵见下面build缺口。

## Step05：实际首红与验证结果

实际执行：

```text
cargo test -p bitloom --test fr196_csr_decoder --test fr196_csr_decoder_formal --test fr196_csr_decoder_integration -- --ignored --nocapture
```

2026-09-21T04:56:28.532025+00:00开始，0.264秒，cargo退出101。三个目标各一项E0432，均为`bitloom_prelude::ip::CsrDecoder`不存在；没有其他编译错误，相关Elaboratable unused warning是缺API导致的附带警告。原始日志127-4-atdd-red.log、完整命令/退出/环境/耗时/SHA为127-4-atdd-red.json。`--ignored`是明确激活当前红脚手架，不改变源码ignore标记。

编译失败发生在运行前：9入口均**未执行**，不是9个行为失败或PASS，也没有Icarus/SBY/Yosys/C消费者执行结果。没有将环境错误当产品红。ATDD普通ignore在build逐项移除，专用工具按已有项目约定仍显式`--ignored`执行。root工具版本预检为独立前置，不是本次行为证明。

首红对应3文件逐字节归档127-4-atdd-red-sources.tar.gz及member SHA清单。首红后仅修正一条不准确注释：reset场景实际观察captured AW不重放，未发送late W；最终源码另存127-4-atdd-sources.tar.gz，归档与worker JSON按最终字节更新，二者语义相同且差异明确，不冒称late-W场景已验收。

## AC实现清单 / build不得遗漏

| 脚手架/范围 | 下一步实际工作 | 当前状态 |
|---|---|---|
| native/API5入口 | 实现固定decoder，激活并跑两native引擎；同源独立/共享完整HIR、准确端口与连接失败、全地址、owner/DECERR断言 | 编译首红，行为未跑 |
| API诊断/复用补充 | 非法模块名、同名不同定义poison；两个实例实际RTL状态隔离，prelude-only独立消费者 | 尚无完整harness |
| 真实RTL1入口 | 激活已有7模块组合有限场景；源码NBA peer共同reset，不称真实外设 | 编译首红，RTL未跑 |
| 软件1入口 | 激活同源四头/Markdown确定性、全部正式offset/mask/独立base黄金C11消费者 | 编译首红，C未跑；兼容性验证不独立证明新decoder |
| 地址/权限矩阵 | 真RTL补边界/高alias、RO零WSTRB/WO读、16WSTRB互补值、保留位、dynamic reject、W1C set/clear、快照/副作用及不同错误 | 有限骨架，完整矩阵未实现 |
| 协议与记账 | AW/W0/1/7/31、W先/同拍、不完整W读进展；分别B/R受阻期间实际完成相反类别；独立并发producer+16seed×1000、命中/墙钟、accept/commit/response/cancel、尾部重复监视 | 当前partial AW/有限计数，其余未实现 |
| reset/monitor | 全部不同在途阶段（含owner、miss、B/R）取消与恢复、reset和可握手沿竞争、late W真实场景；producer monitor涵盖接受沿并实际负例，reset清保持历史 | 仅有限reset，monitor尚未实现 |
| formal初始1入口 | 实际运行routing/reset端口observer prove/5cover，再补完整因果叶响应模型、owner/响应稳定、miss消费、停顿恢复/reset后新请求cover | 未运行；无公平性/等待上界；完整证明未实现 |
| formal敏感性 | representative串窗/双选/高alias等mutant实际断言反例，原版PASS；assume只环境协议，内部不变量assert | 尚未实现 |
| 原始综合1入口 | 已有decoder无observer synth/check/latch检测；补未知cell/FF专用clk、真实组合综合及外部5 ready/valid无输入路径；结构检测负例 | 未运行；组合/严格结构harness尚未实现 |
| 文档/CI/兼容 | 中文准确接口例、FR142逐符号/minor、CI专用门禁与失败产物、旧M0/M1/CSR/桥定向及适用专用工具、软件产物回归 | build负责，当前未改公开docs/CI |
| 最终M2 | 独立review→automate→actualclean/fmt/justtest→单故事commit，再closeout并真实状态更新 | 主代理后续七步；不提前done |

实现估算沿故事3–5有效人日，不把本次脚手架写作总交付耗时估算。每行必须以实际结果勾选，当前全部GREEN项未完成。

## 模板适用性与质量检查

- Story summary：FR196固定四窗译码与M2真实验收；primary_level=Rust周期/API + actual RTL integration，非浏览器E2E。
- 已完成前置/源码/框架/合同读取、8AC分层P0/P1、两worker适用任务、9有真断言的ignore脚手架、实际缺API首红、story回链与下一步命令。
- 3源码文件均小于1000行；测试隔离fresh Sim/session或label+PID工具目录，有限超时强失败并保存日志。结构/形式可超过普通测试90秒目标，专用工具上限180秒，不声称性能测量。
- 工厂：一个正式四CSR布局工厂、独立range golden和显式输入工厂；黄金硬件地址/错误/掩码是故意手写领域常量，不用faker随机替代合同。peer是明确的外部状态/事件/拒绝输入，不是隐藏产品替身。
- 网络mock、认证、data-testid、DOM selector、headed/debug browser命令、Playwright merged fixture、Pact artifact、HTTP status/schema：均N/A。没有创建/遗留浏览器会话。知识加载profile服从mandate适用门，不在Rust套用JS imports。
- 多个硬件信号共同构成同一周期性质，需要多断言和确定性循环；没有将“one assertion”机械拆成不可读逐信号测试。所有断言可被错误路由/状态/端口改变证伪，无assert(false)、todo、空成功。
- 仍存在只能在GREEN后实测的harness/工具行为风险；首红不能证明SV已编译、golden充分或全部性质正确。上述未完成项完整交给build，不能只激活现有9入口便关闭AC6或M2。

## Red→Green→Refactor及运行指引

1. 按127-4-atdd-api-contract实现固定API；先激活当前native/RTL测试（移除普通ATDD ignore），运行对应target确认真实green。
2. 补齐上表未实现验收，修产品或有具体证据的harness缺陷；不削弱golden迁就DUT。专用入口实际执行：`cargo test -p bitloom --test fr196_csr_decoder_formal -- --ignored --nocapture`。
3. 普通入口：`cargo test -p bitloom --test fr196_csr_decoder --test fr196_csr_decoder_integration -- --nocapture`；单项用测试名过滤，工具失败查看target/fr196-decoder-*目录。
4. 绿后重构保持合同、归档所有实际工具产物与member SHA；按用户七步继续独立review/automate/clean/fmt/workspace，最后一个commit。不要执行模板中提前done/commit建议。

本ATDD结束时故事保持ready-for-dev，sprint与goal不由本步骤变更。下一流程由root执行bmad-build；FR189 deferred/NFR91不变。

## Build补齐实测（2026-09-21；不替代最终七步）

以上RED/待build表保留为ATDD历史。实现阶段已补齐固定API与52端口、两native全地址/owner/reset、定义冲突与连接诊断、双实例真实RTL、正式四叶组合与软件C消费者、逐阶段独立记账、16seed×1000随机和每seed256独立并发、完整权限/byte/event/reject/snapshot矩阵、取消/接受沿monitor负例、独立prove/cover与port-only mutation、原始decoder/组合严格综合、中文原文例/FR142/CI。实际结果逐项见[build证据](127-4-build-evidence.md)，原始文件与SHA见[归档索引](127-4-build-archive.json)。

最终唯一普通入口14个（8 native/API/双实例、5集成/软件/monitor/unsupported、1结构观察器负例），专用4个另跑通过；ordinary初批13与后续integration-final/dual-final是分批证据，重跑不重复计数。新矩阵共16000随机+4096并发完成事务，另有各seed定向序列；[每seed实际命中/墙钟](127-4-build-seeds.json)。旧FR193/194/195/CSR/桥普通91项、本轮CSR/桥专用7项、四份文档原文例均exit0。

首次SBY归纳UNKNOWN和SV词法失败保留并修复，未写作PASS；结构/刺激/RTL突变负例必须失败于明确断言，control通过。所有新增公开入口已逐符号登记minor，不改版本/工具钉。主流程仍须独立review、automate、实际clean/fmt/justtest与单故事commit后核验M2；本build未改story/sprint/goal、未clean/commit/push/publish，FR189 deferred/NFR91保持。
