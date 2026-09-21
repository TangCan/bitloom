---
title: 'Timer32 可组合外设'
type: 'feature'
created: '2026-09-21'
status: 'done'
baseline_commit: '1c4cf20c4f8e8fdcb038d57a7b3f03f9f80b5c00'
route: 'dispatch'
review_loop_iteration: 0
context:
  - '/nvme_data2/richard/2026/rhdl/_agile-output/implementation-artifacts/128-2-timer.md'
  - '/nvme_data2/richard/2026/rhdl/_agile-output/test-artifacts/atdd-checklist-128-2-timer.md'
  - '/nvme_data2/richard/2026/rhdl/_agile-output/implementation-artifacts/epic-128-nfr14.md'
---

<frozen-after-approval reason="用户已批准全部故事七步连续执行">

## Intent

交付128.2 Timer32；故事七项AC与ATDD固定接口为完整验收合同。已有CSR基座没有Timer算法；新增prelude-only可组合外设，供后续IRQ/系统复用。

## Boundaries & Constraints

保持AD-30单session/一次finish与既有API、工具钉。复用CsrBlock，CTRL/COUNT唯一外部owner，COMPARE/EVENT唯一leaf owner。新增Timer、define_module返回String、registers与Elaboratable，13端口及描述名称按ATDD。不得缩位宽、拼FrozenHir或新增native层级。只交付Timer；FR197/M3不关闭，FR189 deferred保持。无push/publish/版本变动；第七步才commit。

## I/O & Edge-Case Matrix

|场景|预期|
|---|---|
|使能普通沿|模32递增后比较；周期命中归0，一次命中保留COUNT并清enable；COMPARE0仅回绕|
|有效CTRL/COUNT/COMPARE写|COUNT合并写优先，其余COUNT保持；同值/零值写也抑制match|
|零有效mask/读/错误/EVENT写|自然计数继续；保留字节非零WSTRB不抑制|
|match与clear|set优先；raw match_event沿前采样，独立sticky EVENT|
|CSR/reset|提交前快照、持久背压、消费无refill；同步reset最高优先取消在途|
|地址|local0/4/8/c，非系统0200；洞/未对齐/高位无别名，SLVERR失败读0|

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip/csr/{mod,rtl}.rs`：已有描述/外部candidate与commit/W1C，默认不改。
- `crates/bitloom-builder/src/lib.rs::define_module`：非捕获函数、完整参数/HIR身份；callback不可嵌套定义。私有子模块先定义，名字/身份须无歧义。
- `crates/bitloom/tests/fr197_timer{,_api}.rs`：8个已生成入口，E0432真实RED；独立oracle与双实例RTL，不削弱黄金。
- `crates/bitloom/tests/fr196_csr_formal.rs`：prove/cover/原始综合模式。
- `scripts/chisel-numeric-check.sh`及`crates/bitloom/tests/simulator_bit_vectors.rs`的export_chisel_case/verify_firrtl_rtl：复用实际后端调用；独立Timer目录，Chisel仅映射clk/rst→clock/reset及io_端口，保留同一黄金。

## Tasks & Acceptance

- [x] 新建`crates/bitloom-prelude/src/ip/timer.rs`并更新`ip/mod.rs`：实现合同，保留旧导出。
- [x] 更新上述ATDD测试并新建`fr197_timer_formal.rs`及必要后端测试：真实GREEN、形式安全/独立cover、原始综合；新增测试须有明确风险。
- [x] 新建`docs/ip/timer.md`、prelude-only可编译原文例与同源C/Markdown；更新`docs/public-api-1-0-surface.md`逐符号FR142/minor、`docs/ip/phase24-contract.md`仅真实子集状态。
- [x] 新建`_agile-output/test-artifacts/128-2-build-*`：命令/UTC/exit/工具/seed/源码SHA，原始产物归档抵抗clean；更新故事build记录，不把七步未做部分勾done。

Given已批准七AC，When执行测试，Then directed边界、3seed参考与双实例实际RTL均通过；适用direct/FIRRTL/Chisel各自运行，不以emit替代。Given层级入口，When调用native两引擎/generated，Then明确拒绝；不为测试扩大产品路径。Given任意合法背压，Whenformal prove，Then安全性无公平性假设成立；cover与综合独立报告。遗漏/失败必须修复或真实记录阻塞，不能skip计PASS。

## Implementation Notes

无实质意图缺口或不可逆操作；包含硬件/API/多后端验证，采用dispatch。已调查CSR组合、builder身份与工具路径；用户授权覆盖checkpoint，当前脏文件全部来自本故事create/ATDD。

实施中实际JVM编译暴露`crates/rhdl-firrtl/src/chisel.rs`的`AssignExpr::Lit`输出超Scala Int范围字面量。为满足既定Chisel行为验收，最小修复真实emitter并补边界回归；保留失败日志，不能手改生成Scala掩盖。属于原验收必需修复，不改冻结意图/API/工具钉。后续JVM同样暴露Reset参与位运算和UInt1作为Mux selector的类型错误，按现有HIR类型显式转换；保持Bool路径及语义。

## Spec Change Log

## Review Triage Log

调度：blind/edge为新无上下文同模型子代理。第三个fresh spawn被平台thread limit拒绝，因此verification复用仅做只读后端调查、未参与产品实现的timer1282_backend_recon线程；三路全部启动后才收集/triage。未跳过层，但第三路不是fresh context，此限制显式保留。

### 本轮逐项裁定（修复前）

|发现|裁定|证据与处理|
|---|---|---|
|B1|medium|新增三个 ignored 后端/形式/综合入口未被 CI 调用，常规 workspace 无法发现退化；patch：接入现有专用 job。|
|B2|medium|build runner 读取临时 identity JSON、固定开发机工具目录及预先存在的 example manifest；新 checkout 不能复现；patch：明确工具输入并自行创建示例与核验身份。|
|B3|low|C 验证文件包含开发机绝对路径；patch：普通 include 加显式 include 目录。|
|B4|medium|新 BigInt 分支对大于 LongMax 和 memory init 仅检查字符串，现有 JVM Timer 只用32位；patch：实际 JVM/RTL 边界用例。|
|B5|medium|bit_operand 同时改变 AND/OR/XOR，Timer 实际只覆盖 Reset XOR；patch：AND/OR 双操作数位置的真实后端验证。|
|B6|low|非法名字/冲突仅 is_err，错误连接仅 nonempty，其他错误可掩盖预期诊断；patch：核对诊断类别和相关标识。|
|B7|medium|pair 仅重复 SharedTimer；未检验不同公开名字共享私有 CSR 的可组合性；patch：不同名字双实例实际 RTL。|
|B8|low|私有 BitloomTimerCsr 固定名称，缺少与预先定义不同 body 冲突的回归；patch：检验正确拒绝及具体诊断。|
|B9|medium|formal cover 将 wc/wn/wp 合并，任一命中即可而其余不可达仍通过；patch：拆为三个独立 cover。|
|B10|medium|双实例 RTL 的 access0/access1 串行完成，没有一实例堵塞时另一实例进展的场景；patch：同步请求和非对称背压。|
|E1|medium|assign_mux 的非零真值合同接受多位选择器；旧 Chisel 直接传 UInt，新代码 asBool 仍不支持多位，Timer 只用1位。旧 HEAD 同样不支持，该问题先于本故事；defer：多位选择器用归约转换及实际后端回归。|
|V1|medium|verification-gap 预核实 Reset XOR 退化不会被现有 numeric CI 检测；patch，与 B1 同根因，增加 Timer 专用后端 CI 调用。|

分组后 B1/V1 共用 CI 修复，其余分别处理；测试补强与可移植性修复不新增公开接口、不改变冻结意图。所有 patch 待实现代理修复后复核，尚未宣告关闭。

修复复核：B1–B10及V1已关闭，E1已追加deferred。主代理按Verification重跑8个命令全部exit0（8 ATDD、2专用formal、1专用后端、20 FIRRTL单测、SemVer、例子、C编译/执行），10 cover到达；归档与命令见`128-2-root-recheck-*`。独立审计181个修补归档文件及6个源码SHA。七步尚未结束，按用户要求第七步才commit。

## Verification

`CARGO_PROFILE_TEST_OPT_LEVEL=1 BITLOOM_REQUIRE_RTL=1 PYTHONDONTWRITEBYTECODE=1`。PATH前置`/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin:/tmp/bitloom-jvm-tools/bin`；firtool固定`/home/richard/.cache/rhdl/firtool/1.159.0/bin/firtool`。形式前重核源码/安装身份，根预检16文件见`/tmp/128-2-sby-identity-preflight.json`；不可仅看version。

运行`cargo test --locked -p bitloom --test fr197_timer --test fr197_timer_api`，新formal专用命令、同向量后端、原始综合/例子/semver适用门禁；旧GPIO/UART等兼容由有意义目标及最终workspace回归覆盖。主代理管理spec/sprint/总账，负责独立review、automate和实际`cargo clean && cargo fmt --all && just test`后单故事提交。实现代理不提前clean或commit。

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
