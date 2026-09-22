---
title: 'Story 129.1：组合系统与证据风险门'
type: chore
created: '2026-09-21'
status: done
route: dispatch
review_loop_iteration: 0
baseline_commit: 0ab222203916b1b0583c2c673558d5333cb05552
context:
  - '{project-root}/AGENTS.md'
  - '{project-root}/_agile-output/implementation-artifacts/129-1-组合系统与证据-nfr14.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-129-context.md'
  - '{project-root}/docs/ip/phase24-contract.md'
  - '{project-root}/_agile-output/test-artifacts/129-1-atdd-manual-acceptance.md'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**问题：** M3 已交付，但完整外设系统的复位路径、实际工具和核心证据责任仍需风险门确认。旧局部夹具不能证明 FR198。

**方法：** 写有效 Epic129 NFR14，以实际工具/BFM/已有桥译码与三后端父子复位探针支持路线；冻结未来两种拓扑和验收责任。本故事仅交付风险门。

## Boundaries & Constraints

**始终：** 遵守关联故事八项 AC、正式 AD-30 和既有工具钉。保留原始失败、源码、日志、命令、UTC、耗时、退出码和版本；归档到仓库专属证据目录，clean 后可审计。中文文档。用户已授权全部未完成故事七步执行，无意图缺口，本规格按既有授权冻结。

**禁止：** 修改产品后端/公共 API/工具钉/版本、伪造系统 PASS、把工具发现视为 formal prove、把逻辑反相视为同步器、悄悄跳过后端。129.2/3仍 backlog。实现代理不得提交或改 sprint/目标账本；主代理负责最终七步状态和提交。

## I/O & Edge-Case Matrix

| 场景 | 输入/状态 | 预期行为 | 失败处理 |
|---|---|---|---|
| 层级同步复位 | 至少两子状态非零；在非有效沿改变 aresetn | 沿前保留；有效沿共同清零，reset 优先于写；释放后恢复 | 保存原始失败，不能只编译 |
| 显式/隐式区别 | Chisel 隐式 reset 与 aresetn 独立 | 记录 typed 路径实际表现；最终选定路线由 aresetn 正确控制两子 | typed 不支持则实测公开源码无状态边界；两路皆失败则风险未解 |
| 固定基座 | 已锁 BFM stub、已有真实桥/译码测试 | 实际运行非零测试成功，准确注明旧基座 | 缺工具或失败不得 skip |
| 状态门 | 129.1未done，下游 active；合法未来状态 | 现有84场景按精确诊断通过且真实状态文件不被测试改动 | 不把脚本称全依赖检查 |

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip/{axi_lite_csr,csr_decoder,uart_csr,timer,irq}.rs`、`gpio/csr.rs`：真实定义接口，只读。
- `crates/bitloom-hir/src/{lib,composition}.rs`：唯一 Reset 与类型约束。
- `crates/rhdl-firrtl/src/chisel.rs`：ref_name 把 rst 改为 reset，实例连接跳过 clk/rst，Module 继承隐式 reset；当前为待实测风险。
- `crates/bitloom/tests/fr194_module_composition.rs`：真实双子寄存器图；`fr196_csr_decoder_integration.rs`：旧基座入口；UART CSR 集成测试含 JVM runner。
- `scripts/phase24_axi_bfm_probe.py`、`scripts/ci-sby-pins.env`：固定工具接口及 SBY 身份。
- `../test-artifacts/129-1-system-recon.md`、`129-1-atdd-*`：已完成调查、84例过程验收与18项人工核验。
- 规划目录的 `nfr14-risk-record-template.md` 与研究 `technical-bitloom-composable-ip-ecosystem-2026-09-20/implementation-plan.md`：风险结构及初始16seed×1000事务预算，正式合同优先。

## Tasks & Acceptance

**执行：**
- [x] `epic-129-nfr14.md`：填上游约束、估算、owner、支持参数、依赖、维护、禁止降级、停止条件；固定完整 AXI 与无桥直接 CSR 两个真实四外设图。
- [x] `../test-artifacts/129-1-*`：可复跑工具发现、固定BFM与旧桥译码探针，保留身份和独立原始结果。
- [x] `../test-artifacts/129-1-*`：隔离探针源/runner实测父子同步复位，direct/FIRRTL/Chisel逐项编译执行；记录 typed 失败与合法最终路线（若需要薄适配则同样实际综合并绑定源码）。
- [x] `../test-artifacts/129-1-atdd-manual-acceptance.md`：逐项引用证据，最终七步项留主代理完成；复跑presence与normal/-O gate。
- [x] 关联故事 Dev Record/File List：记录真实实现和界限，最终回归/关闭交主代理。

**验收：** 关联故事 AC1–7逐项有可检查依据；AC8由主代理完成。风险记录规定129.2干净独立checkout一命令复现、129.3复核，分别记录后端、有限形式/综合、两组合/贡献模板和实测成本；不以本探针提前交付系统。

## Implementation Notes

## Spec Change Log

## Review Triage Log

2026-09-22 build三路独立审查：blind 10条、edge 2条、verification无缺口。逐项裁决如下；本轮尚待修补复验，不提前关闭build。

|编号|裁决 / 路由|核验及处理|
|---|---|---|
|B1|medium / patch|子进程确未继承-O；显式传播sys.flags.optimize，复验普通与优化各84例。|
|B2|medium / patch|probe identity只覆盖自身，不能绑定实际产品依赖；补工作区Rust/manifest/lock指纹。|
|B3|medium / patch|排除了Cargo.lock且未--locked；纳入指纹并锁定执行。|
|B4|medium / patch|Chisel环境可与PATH不同；单独核环境实际firtool并固定该路径。|
|B5|low / patch|子串比较有误接受版本的可能；精确解析三段版本。|
|B6|medium / patch|early exit缺源和终态；注册退出保存输入及failed结果。|
|B7|false / reject|当前探针只声称实际纯设计综合及检查通过；无当前无latch自动门承诺。NFR14无latch门明确归129.3，未来需显式实现。|
|B8|low / patch|输出可进入自身造成递归；在mkdir之前拒绝源码目录及子目录。|
|B9|low / reject|archive确可含旧共享目录文件，但当前PASS来自本次命令时间、exact测试和日志；未把归档全部成员宣称本次执行。无需为一次性基座引入新产物路由，证据中明示归档范围。|
|B10|low / reject|固定Icarus的本次VCD采样有效，审计已声明逻辑任意单位；没有跨sim物理时间承诺。当前无错误采样；引入单位规范超出直接修正且此一次性探针不面向跨sim日常使用。|
|E1|medium / patch|TimeoutExpired未写捕获日志；超时后排空并保存stdout/stderr。|
|E2|medium / patch|subprocess.run超时不保证子孙退出；隔离进程组并超时kill/reap后归档。|


## Design Notes

优先实测 typed aresetn→UInt1→Eq/Reset wire core_reset→两个子模块，不命名为 rst。Chisel TB 不得偷绑隐式 reset 掩盖连接。若该路线不支持，允许公开无状态 RTL 边界适配生成的常规 clk/rst core，三后端一致、纳入未来例子源与综合；明确不是整个 top 由 HIR 生成。此为已批准 AD-30 内可逆实现选择，不要求扩大后端能力。

隔离 probe 可以独立 Cargo 工具 harness 引用树内库，设计源码保持 prelude-only；不修改 workspace 产品文件。新脚本用环境/参数发现工具、设置超时、捕获失败，不硬编码本机/tmp路径作为用户配方。本机候选 PATH `/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin:/tmp/bitloom-jvm-tools/bin`，firtool `/home/richard/.cache/rhdl/firtool/1.159.0/bin`，SBY源码 `/tmp/bitloom-1263-sby-src`，BFM Python `/tmp/bitloom-phase24-bfm-py312/bin/python`。这些仅供实际发现，记录真实来源。使用 CARGO_PROFILE_TEST_OPT_LEVEL=1、BITLOOM_REQUIRE_RTL=1、PYTHONDONTWRITEBYTECODE=1。

## Verification

- `python3 _agile-output/test-artifacts/129-1-atdd-presence.py`：正文存在后0，但不声称质量验证。
- `python3 [-O] _agile-output/test-artifacts/129-1-atdd-gate.py`：两次各84例成功。
- `cargo test -p bitloom --test fr196_csr_decoder_integration p0_real_hierarchy_routes_errors_partial_write_and_common_reset -- --exact --nocapture`：非零真实旧基座通过。
- 新探针 runner：保存实际命令、版本、退出码、失败工件、VCD和 source SHA；两子状态/同步优先/恢复的断言必须执行。
- 主代理完成 build review、独立code-review、automate、实际clean→fmt→just test及单故事提交。

实施记录2026-09-22：风险门正文、隔离复位probe和真实r3三后端适配行为/综合、原始失败归档、人工M01–17及story记录完成。typed Chisel lower失败保留，采用AD-30允许的公开无状态边界；主代理七步最终验证与提交待完成。

Build内审修补复验：r4错误解析LLVM版本已修，r5因sbt缓存只读失败；r6获准访问缓存后适配路线三后端执行及综合全部PASS，typed Chisel仍按预期不支持。三VCD独立15采样PASS；新版源指纹含工作区依赖与lock且起止一致。35条基座命令复跑PASS；gate普通/优化各84场景PASS。归档129-1-review-reset-evidence.tar.gz保存r4/r5/r6；新版基座单独归档，未覆盖旧归档。build结束，用户七步优先，提交延至第七步；Story保持review待外部code-review/automate/regression。
