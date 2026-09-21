---
title: '可组合GPIO32 CSR wrapper'
type: feature
created: '2026-09-21'
status: done
route: dispatch
baseline_commit: '433771cdf62fba13870ed79b22880c23e5211e59'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/128-4-gpio-csr-wrapper.md'
  - '{project-root}/_agile-output/test-artifacts/atdd-checklist-128-4-gpio-csr-wrapper.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-128-nfr14.md'
---

<frozen-after-approval reason="用户已授权全部未完故事连续七步">

## Intent

交付128.4 GPIO32 CSR，提供同步针脚、软件控制和原始上升沿事件，与IRQ组合；旧GPIO8及手写FL保持兼容。

## Boundaries & Constraints

固定`ip::GpioCsr`、16端口、六寄存器及签名详见ATDD Step03。单session/一次finish，复用CsrBlock；OUT唯一wrapper owner，DIR/RISE_EVENT唯一leaf owner。prelude-only、工具钉/版本不变。无去抖/物理pad/原生层级扩展。仅关闭GPIO子集，M3/FR197待UART；FR189 deferred保留。第七步才提交，不push/publish。

## I/O & Edge-Case Matrix

|场景|预期|
|---|---|
|DIR/OUT/IN/SET/CLEAR/EVENT|local0/4/8/c/10/14，全32位；逐字节写，SET/CLEAR写1作用唯一OUT|
|同步|sync1'=pad，sync2'=sync1，history'=sync2；IN提交读沿前sync2，输出方向也读真实针脚|
|新沿|沿前sync2 & ~history & ~DIR；初始高传播后有事件，改向不制造边沿|
|输出|pad_out=OUT & DIR，pad_oe=DIR；raw_event为非reset新沿归约，非sticky|
|竞争|新沿set胜W1C，DIR写用旧DIR筛选；单请求三软件写互斥|
|响应/reset|提交一次、快照/保持/无refill；reset清所有状态并取消响应|
|错误|RO写/WO读/洞/错齐/高地址均SLVERR、rdata0；合法零WSTRB无作用|
|集成/兼容|GPIO→IRQ4清pending不重触发，下个新沿有效；旧GPIO8/FL行为不改|

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip/gpio/{base,mod}.rs`：旧8位保持，新增csr模块并再导出。
- `crates/bitloom-prelude/src/ip/csr/rtl.rs`：External RW候选/commit、WO无value、Leaf W1C；无需改基座。
- `crates/bitloom/tests/fr197_gpio{,_api}.rs`：12个ATDD入口，独立黄金、真实组合；两目标E0432首红已留存。
- `crates/bitloom/tests/fr197_irq_formal.rs`及`128-3-build-{runner,archive,harness-tests}.py`：复用真实形式/身份/证据设计，不改历史文件。
- `crates/bitloom-sim/src/ip_dual.rs`：旧GpioFunctional，兼容回归不修改。

## Tasks & Acceptance

- [x] `crates/bitloom-prelude/src/ip/gpio/csr.rs`及`mod.rs`：实现固定表面与唯一状态。
- [x] `crates/bitloom/tests/fr197_gpio{,_api,_formal}.rs`：GREEN、三后端同黄金、独立prove/cover/原始综合及DUT变异负控制。
- [x] `docs/ip/gpio-csr*`、`docs/public-api-1-0-surface.md`、`docs/ip/phase24-contract.md`：原文例/同源软件产物/FR142/minor与真实边界。
- [x] `.github/workflows/ci.yml`及`_agile-output/test-artifacts/128-4-build-*`：持续门禁、可移植runner、原始归档及防误验收回归。

Given故事七AC及合法任意背压，When实际执行验证，Then逐拍参考与三后端一致，安全性不需ready公平性；formal/cover/综合各有证据，native层级明确拒绝。Given代表DUT故障，When同一独立observer验证，Then原始PASS且变异在指定断言失败并有VCD，非编译错误/超时。

## Implementation Notes

无意图缺口、无不可逆操作；新增硬件/API/验证采用dispatch。当前脏文件全为本故事create/ATDD及前提交SHA登记，连续授权覆盖例行checkpoint。

## Spec Change Log

## Review Triage Log

|编号|判定|证据与处理|
|---|---|---|
|B1|low|runner以目录差集关联工件，并发运行确会混入目录；本故事受控串行且CI工作区分离，日常不会遇到。加入锁或路径参数超过直接修正，按技能规则拒绝。|
|B2|medium|archive只读comparison记录和nonempty，没有重算生成文件SHA；归档前文件改变可误认证。patch：重算两产物并对照记录及当前金样。|
|B3|medium|archive仅检查16文件和runtime_command，未检查记录的pin或sby-runtime成功。patch：核对既有pins及运行状态。|
|B4|medium|archive positive formal只检查formal.v和PASS，缺失cover见证仍可通过。patch：要求10个cover目标关联的见证及证明日志。|
|B5|medium|formal使用--ignored但不核对3个测试实际执行，移除ignore可零测试通过。patch：核对既有3目标名称和计数。|
|B6|medium|gpio_irq_tb未在pending=16时关闭ENABLE；故事集成矩阵明确mask不清pending。patch：增加屏蔽、读pending、重使能无新沿断言。|
|B7|medium|reset前raw已在第三tick消失，当前断言不能验证有效raw被reset压低。patch：在sync2有效窗口复位，检查立即压低及接收端取消。|
|B8|medium|3个故障只过形式observer，故事明确同时检验scoreboard。patch：代表IN误接OUT变异复用原direct黄金，要求指定行为失败与VCD且控制PASS。|
|B9|low|Rust.status启动失败在写metadata前panic；外层runner仍记录失败，不会误计PASS。timeout缺失非日常环境，增加多处错误分支超过直接修正，按技能规则拒绝。|
|B10|low|两测试文件确有JVM harness复制，未来版本更新可能漏改其中之一；现有两个入口均实际运行且pin一致。假设更新失误非当前日常缺陷，共享抽取非直接修正，按技能规则拒绝。|
|E1|low|与B1相同的并发目录差集缺陷，独立判定仍为low；受控串行/隔离CI下不常见，锁引入复杂度，拒绝。|

verification-gap返回No verification gaps found。以上逐条判定后，B2/B3/B4/B5/B6/B7/B8分别作为patch处理；无冻结意图或公开API变化，不需要重新批准。历史归档保持原字节，修补后重新完整验证和新归档。

## Verification

`cargo test --locked -p bitloom --test fr197_gpio --test fr197_gpio_api -- --nocapture`；两个dedicated后端精确运行，独立形式/综合、旧GPIO/FL、原文示例/C头和SemVer。runner记录启动失败、exact执行数、选定run起止/当前源码SHA、形式状态与原始工件；生成日志不入源码指纹。归档成员逐字节校验，clean前持久保存。最终主代理执行clean/fmt/just test及单故事commit。


独立只读调查：现有Chisel BigInt/Reset/qualified Cat足够表达此GPIO，尚无必要后端改动证据，不扩多位UInt Mux语义。复制128.3 archive需逐项映射validate_run的组合目录前缀、两个exact测试名、软件产物、formal任务/故障目录和综合顶层GpioCsr，不能仅替换irq字符串。CI普通软件例、chisel-numeric两个exact后端、formal-sby及harness普通/-O各有持续入口，保留IRQ门禁。

本机调用环境：PATH前置/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin:/tmp/bitloom-jvm-tools/bin；RHDL_FIRTOOL_PATH=/home/richard/.cache/rhdl/firtool/1.159.0/bin，BITLOOM_SBY_SOURCE=/tmp/bitloom-1263-sby-src，CARGO_PROFILE_TEST_OPT_LEVEL=1，BITLOOM_REQUIRE_RTL=1，PYTHONDONTWRITEBYTECODE=1。可移植runner依输入环境，不写死安装路径。实现代理不clean、不commit、不改sprint/总账；主代理负责独立review/automate/最终workspace与状态。原始target产物须归档以供后续clean前审核。全过程保存失败日志，不手改生成Scala，不以ignored/工具发现/emit计PASS。

Build实现交接：完整runner全部gate PASS，source_complete=true；1589成员原始归档及SHA见`128-4-build-evidence.md`，补充旧GPIO六目标32项PASS，harness普通/-O各12PASS。主代理独立review/automate/最终clean-fmt-workspace/单故事commit尚待，不改故事或sprint为done。

主代理step03验收：完整diff /tmp/128-4-build-full-9d4e_8yx.diff；全文审阅产品/API/参考/formal/runner/archive与docs/CI，harness对已审1283差分核对。ATDD仅激活/阶段注释变化，黄金无削弱；矩阵每行均有本次实际通过记录。独立逐字节审核1589归档成员及566相关源起止/当前SHA一致，见128-4-root-build-audit.json。build任务完成，进入技能内部review，尚未完成七步。

修补交接：原实现代理完成B2–B8。harness普通/-O各14通过；新scoreboard exact、GPIOIRQ direct exact、组合FIRRTL/Chisel exact各1通过。实际10个cover目标共9份VCD，校验按日志逐目标关联。主代理已审修补diff，完整runner复验进行中；历史归档不覆盖。

构建内审最终通过：修补后完整run 20260921T115845.483458Z-797187所有gate通过，678成员归档SHA 528fd853bb7104065e77a8f48f0eb5e3e1392695b8b8004740484b0c81d39274，主代理逐字节及566源起止/当前复核通过。详见128-4-build-review-root-audit.json。用户七步顺序覆盖技能默认提交，提交延至最终回归后；独立code-review开始。

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

七步最终回归已通过：479组1855通过/0失败/40忽略，clean/fmt/test均exit0，566源码未变。见128-4-final-verification.md；第七步单故事提交，无push/publish。
