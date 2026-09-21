---
title: '127.3 AXI-Lite 到 CSR 桥'
type: 'feature'
created: '2026-09-21'
status: 'done'
route: 'dispatch'
review_loop_iteration: 0
baseline_commit: '9393c261cdfb2ca3699edb959efd1986bbe5275e'
context:
  - '{project-root}/AGENTS.md'
  - '{project-root}/_agile-output/implementation-artifacts/127-3-axi-lite-csr-桥.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-127-nfr14.md'
  - '{project-root}/_agile-output/test-artifacts/127-3-atdd-api-contract.md'
  - '{project-root}/_agile-output/test-artifacts/atdd-checklist-127-3-axi-lite-csr-桥.md'
---

<frozen-after-approval reason="用户已授权全部故事七步串行；ATDD固定接口">

## Intent

**Problem:** CSR叶已交付，但缺少可靠的独立AXI通道桥接。
**Approach:** 实现固定`AxiLiteCsrBridge`，完成故事8条AC和ATDD契约，交付可组合桥及真实验证。

## Boundaries & Constraints

**Always:** 单session/FrozenHir、无捕获共享body；AW/W/AR各一槽，锁定offer；预留B/R容量，读优先轮转只在提交后翻转；全局CSR在途到CSR响应消费结束；同步共同reset。
**Never:** 不改旧bank/CSR语义、工具钉、版本、IR或native层级；不实现四窗decoder。实现代理不改story/sprint/goal，不clean、commit、push。七步主流程最终提交。

## I/O & Edge-Case Matrix

| 场景 | 输入/状态 | 预期 | 错误处理 |
|---|---|---|---|
| 分拍/停顿 | AW/W独立，CSR未ready | 正确配对，offer保持 | 不撤回或替换 |
| 响应受阻 | B或R占槽 | 另一类继续；payload稳定 | 不覆盖/重复 |
| 地址/权限 | 16位原始地址、WSTRB、PROT | 原样传递，PROT忽略 | 合规peer的00/10/11透传 |
| reset | 任意捕获/offer/执行/响应 | 清空取消，恢复新请求 | 禁止只复位桥 |

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip/axi_lite_csr.rs`：新桥；`ip/mod.rs`追加私有模块和重导出。`rv_reg_slice.rs`供共享定义模式参考，不强制使用两槽结构。
- `crates/bitloom-builder/src/lib.rs`：复用define_module、稳定Span、位运算/寄存器；callback不得begin/end模块。固定配置可用空参数，完整HIR仍核验身份。
- `crates/bitloom-prelude/src/ip/csr/rtl.rs`：现有叶响应下一周期、消费沿不接新请求；`axi.rs`旧bank保持不动。
- `crates/bitloom/tests/fr196_axi_lite_csr{,_formal}.rs`及同名辅助目录：11个ATDD入口；独立事件oracle、真实层级、严格结构检测。仅凭具体证据修复测试缺陷，不削弱合同。
- `docs/ip/axi-lite-csr-bridge.md`、`docs/ip/README.md`、`docs/public-api-1-0-surface.md`：例子、限制、逐符号FR142/minor。
- `.github/workflows/ci.yml`：保留全部既有命令，扩桥formal与失败产物；新示例脚本复用`check_fr194_example.check_example`。

## Tasks & Acceptance

**Execution:**
- [x] `ip/axi_lite_csr.rs`及mod：固定API、端口和独立/共享入口；捕获、offer锁定、执行owner、响应保持及reset。
- [x] `fr196_axi_lite_csr*`：实际native两引擎、RTL、真实CSR/双实例、16seed×1000事务、prove/cover/结构/综合通过；保留失败证据。
- [x] 文档/脚本/API表/CI：可编译prelude-only例、气泡/复位约束、专用formal执行；不提前关闭M2。
- [x] `_agile-output/test-artifacts/127-3-build-*`：命令/版本/exit/时间/seed、源码及工具归档与摘要，target清理后可核验。

**Acceptance Criteria:**
- Given 已捕获请求，when 背压、竞争与reset，then 原始地址/数据、唯一提交、正确响应、独立取消记账满足故事AC。
- Given 同session桥与真实CSR，when 实际RTL运行，then 权限/字节/事件/动态错误及副作用符合手写黄金值。
- Given 固定工具，when prove/cover和无observer综合，then 实际PASS及五输出无输入组合路径；不把ignored当通过。

## Implementation Notes

实际9普通/2专用通过，16seed×1000事务；78既有兼容通过、文档例3模块通过。四行I/O矩阵分别由raw/late-read、arbitration、真实CSR/错误peer、reset测试覆盖且实跑成功；proof基例/归纳depth8与cover64通过。主代理核对产品/测试diff及源码19/工具445成员归档，证据见[build](../test-artifacts/127-3-build-evidence.md)。三路内部review均已返回；10项验证补强已由原实现代理完成并取得各项定向证据；主代理已读修补diff并独立核验505个归档成员，统一验收通过；build完成，故事进入独立review，提交仍留第七步。

## Spec Change Log

## Review Triage Log

2026-09-21：blind 10、edge空列表、verification无gap；三路全部返回后统一裁决。以下均为本故事验证面的局部补强，无公开API/架构变化，route=patch；不把新增场景计作已通过。

| ID | verdict | evidence / route |
|---|---|---|
| B1 | medium | 组合rd只在RVALID后改变动态status，确未区分AR捕获与CSR提交快照。patch：在二者之间改变值并断言提交前值。 |
| B2 | medium | 16WSTRB都写相同黄金值，后续遗漏写可被旧值遮蔽。patch：每mask建立反值，再验证置位/清位及未选字节。 |
| B3 | medium | 组合W1C事件先于清除，缺提交同沿事件冲突。patch：同步提交沿事件并核对set胜clear。 |
| B4 | medium | 组合仅初始reset，中途reset只syntheticpeer。patch：真实leaf在offer、commit后及受阻响应中共同reset，核对取消/无重放/恢复。 |
| B5 | medium | 已披露随机逐事务排空，定向存在并发；仍缺独立多生产者随机交错压力。patch：复用scoreboard加有界并发seed矩阵，不改已有16×1000范围。 |
| B6 | medium | gaps/strobes对read也累加，不能代表写覆盖。patch：仅写入计数，另记实际AW/W先后及stall命中并验证非空。 |
| B7 | medium | 安全proof以7拍CSR响应为假设，虽文档披露限制，接口仅要求有限响应而无7拍上界。patch：安全移除该上界，保留因果/稳定/合法错误约束；cover或进展若需上界独立列明。 |
| B8 | medium | oracle在req_valid出现后验正确性，没有idle合格请求产生offer的有界义务，cover仅存在性。patch：显式reset/容量条件下断言保守实现应在下一沿提出请求，无AXIready公平性。 |
| B9 | medium | 既有Rust monitor/结构负例不能证明formal observer拒绝损坏RTL。patch：合法原版控制+配对/路由/稳定性RTL突变，必须真实assert反例，ERROR/UNKNOWN/timeout不能算通过。 |
| B10 | low | 通用helper负例已有FR194，桥仅正例，新增桥连接失败诊断缺直接验收。patch：桥非法名/冲突poison及端口方向/宽度失败小例，不新增guard/API。 |


修补进展：B1–B10均已落地，见[修补证据](../test-artifacts/127-3-review-fix-evidence.md)。主代理检查了真实集成、并发计数、诊断及port-only变异代码；原版BMC10 PASS、三突变FAIL/2，取消7拍假设后的prove8/cover64 PASS。首个整行为命令9通过/1失败（新刺激超32位）保留，修复刺激后并发定向通过；16seed×1000本轮全部完成。源6/工具499成员经主代理独立SHA256回读通过。主代理已完成完整spec Verification：11普通、3专用、78兼容全部通过，文档例通过，见[根验收](../test-artifacts/127-3-review-root-verification.md)。

## Design Notes

本实现选择无同沿capture refill及CSR响应/新请求交接；ATDD formal按此选择约束，不宣称所有未来桥都必须如此。保守气泡允许，无吞吐承诺。外部五个ready/valid不可组合依赖rst；内部CSR可reset屏蔽。CSR响应消费释放执行槽，AXI响应另占各自槽。参考模型按握手事件工作，不复制DUT逐拍状态机。形式加强只能添加经证明的不变量。

## Verification

PATH前置`/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin`；`CARGO_PROFILE_TEST_OPT_LEVEL=1`、`BITLOOM_REQUIRE_RTL=1`、`PYTHONDONTWRITEBYTECODE=1`。

- `cargo test -p bitloom --test fr196_axi_lite_csr --test fr196_axi_lite_csr_formal -- --nocapture`：9普通入口；专用2项另跑。
- `cargo test -p bitloom --test fr196_axi_lite_csr_formal -- --ignored --nocapture`：真实prove/cover与综合。
- 运行新文档例及既有FR193/194/195/196定向兼容；完整clean/fmt/just test由主代理第6步执行。

无未解决意图或不可逆动作；多文件围绕单一桥目标，dispatch。工作区仅本故事已授权create/ATDD与目标账本改动，不另行询问许可。


### Independent Review Findings


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
