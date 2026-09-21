---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-generation-mode', 'step-03-test-strategy', 'step-04-test-generation', 'step-04c-aggregate', 'step-05-validate-and-complete']
lastStep: 'step-05-validate-and-complete'
lastSaved: '2026-09-21'
storyId: '127.3'
storyKey: '127-3-axi-lite-csr-桥'
storyFile: '_agile-output/implementation-artifacts/127-3-axi-lite-csr-桥.md'
atddChecklistPath: '_agile-output/test-artifacts/atdd-checklist-127-3-axi-lite-csr-桥.md'
generatedTestFiles:
  - crates/bitloom/tests/fr196_axi_lite_csr.rs
  - crates/bitloom/tests/fr196_axi_lite_csr/tools.rs
  - crates/bitloom/tests/fr196_axi_lite_csr/integration.rs
  - crates/bitloom/tests/fr196_axi_lite_csr_formal.rs
inputDocuments:
  - _bmad/tea/config.yaml
  - _agile-output/implementation-artifacts/127-3-axi-lite-csr-桥.md
  - _agile-output/implementation-artifacts/epic-127-nfr14.md
  - docs/ip/phase24-contract.md
  - docs/ip/csr.md
  - .github/workflows/ci.yml
---

# Story127.3 ATDD

Step1：Richard，已按明确授权激活bmad-testarch-atdd Create；resolver前后步骤/持久事实为空。已批准8AC、Cargo Rust测试框架及现有RTL/formal fixtures完整，栈识别backend，原生Rust/真实RTL/sby。Playwright/Pact开关true但无适用JS运行器/消费者服务边界，按mandate不加入依赖或Pact产物；工具列表一次探测pact_mcp_reachable=false，无broker调用。用户七步授权替代常规确认。

Step2：AI generation；依据静态合同与源码，无UI/browser recording。生成方式为 AI；实际 RED 结果见末尾验证记录。

## Step3 测试策略

| AC | 层次/优先级 | 场景 |
|---|---|---|
|1|Rust API/集成 P1|固定端口、独立/共享定义复用、独立实例、prelude-only文档待build|
|2|native/真实RTL P0|AW/W/AR独立、0/1/7/31、完整16地址/16WSTRB/8PROT、配对|
|3|native/RTL/formal P0|offer锁定、读优先轮转、唯一提交与CSR outstanding|
|4|native/RTL/formal P0|B/R预留、永久背压安全、另一类进展、错误透传、zeroWSTRB|
|5|RTL/formal/结构 P0|全部reset阶段/取消epoch、五输出无输入依赖锥、首次reset|
|6|真实组合RTL P0|真实CsrBlock所有权限/动态端口/错误、独立DECERRpeer、无alias|
|7|oracle/RTL/formal/综合 P0|独立监测及负例、16×1000事务预算、prove/cover/原始综合|
|8|流程/兼容 P1|旧bank/M1/CSR回归、文档/API、最终七步；此阶段未关闭|

分层重叠用于不同风险：native验证状态机，RTL验证emit/连线，formal验证任意背压安全，结构验证无组合路径，均不能互相替代。独立oracle按接受/提交/响应事件队列验证，不逐拍复制DUT未来FSM；每个epoch记录partialAW/W取消。初始RED预计missingAPI；不使用test.skip/空断言伪造行为红。既有被测框架oracle自检可绿色且须明示不是产品通过。

## Generation context / Confidence

Confidence: 8/10。依据127.3八AC、固定API契约、已存在fr196_csr native/RTL/formal驱动及严格FR195结构检查。Unknowns：桥尚未实现，具体内部FSM/气泡延迟未知，测试只按合同事件/有界等待检验；真实formal归纳深度及墙钟待build实跑，不预填PASS。

执行mode requested=auto，probe=true，supports.subagent=true，未提供独立agent-team API，resolved=subagent。两名worker同时承担A Rust/native/RTL行为、B formal/结构集成；输出/tmp/tea-atdd-api-tests-1273.json及/tmp/tea-atdd-e2e-tests-1273.json。backend将E2E角色适配为RTL/formal集成；Rust实际编译RED取代JS test.skip脚手架，符合用户要求；专用形式测试#[ignore]仅用于显式工具命令，不可算默认PASS。

质量适配：硬件周期步进不是墙钟hard wait；种子和手写黄金寄存器常量是协议契约，不引入faker。状态机scoreboard需分支和逐拍断言，但独立于DUT内部状态；工具使用进程组timeout。产物按case/PID隔离保留供审计，不为“清理”删除失败证据。无HTTP mock、data-testid或UI组件需求。

## Story Summary

实现独立AXI-Lite→CSR桥，将AW/W/AR捕获与唯一CSR提交点分开，维持背压安全与共同reset取消。设计仅prelude；新增桥不替换旧bank，也不交付127.4四窗译码或完整FR196/M2。

## Fixtures / Factories / Mock requirements

Rust明确宽度输入结构、带覆盖项的初始值和固定seed代替JS factories；每test创建独立sim/scoreboard/工具目录。合规CSR peer是测试基础：仅在真实提交后生成一次响应、受阻保持、共同reset、error00/10/11且错误读0。实际CsrBlock组合是产品集成验证，不能由peer替代。HTTP mocks=0、data-testid=0、UI/browser=不适用；不新增Playwright/Pact/faker依赖。

## Implementation Checklist（GREEN阶段；当前全部未完成）

- [ ] 实现固定AxiLiteCsrBridge公开入口和同session定义，冻结端口，逐符号登记FR142/minor。
- [ ] AW/W/AR独立一槽、数据配对、offer锁定和提交后轮转；不完整write不堵read。
- [ ] B/R预留/持有与CSR执行owner，CSR消费释放全局busy，保留完整地址/WSTRB/error。
- [ ] 全部状态同步reset、取消账本、外部5信号无输入组合路径，初始reset前提。
- [ ] native两引擎、真实RTL、真实CSR层级、独立DECERRpeer；场景/seed/事务/墙钟完整记录。
- [ ] 专用sby prove/cover、严格结构检查和原始RTL综合真实通过，接入既有CI正式命令。
- [ ] 中文prelude-only例及独立编译检查；旧bank/M1/CSR兼容回归。
- [ ] 独立code-review、automate、实际clean/fmt/just test后单故事提交；不提前done。

估算沿用故事5–7有效人日，不等于代理墙钟。实现先跑定向P0到绿，再全矩阵；修oracle须保存原失败、具体依据及复核，不能靠弱化assert/加assume得到绿。重构只在绿色后保持合同，任何尚未实际执行的工具结果保持未验证。

## Step 04C 聚合

两个独立 worker 已完成，源文件与共享 fixtures 均落盘；共 11 个测试入口，9 个普通、2 个需要显式 `--ignored` 的工具门禁。Rust 后端按用户授权保留真实编译 RED，不加入 JavaScript skip。断言使用预期协议事件、实际端口和独立账本，无占位成功断言。摘要及两个 worker JSON 已保存。未测量并行速度收益。故事 backlink 由父任务维护。

## RED 结果与实际测试入口

基线 `9393c261cdfb2ca3699edb959efd1986bbe5275e`。真实编译 exit **101**，两个目标均只有 E0432：`bitloom_prelude::ip::AxiLiteCsrBridge` 尚不存在，另有依赖该类型的 `Elaboratable` 未使用警告。日志 `127-3-atdd-red.log`，精确命令、环境、耗时在 `127-3-atdd-red.json`。没有添加临时产品实现或 stub，也没有运行任何产品行为、RTL 或 formal；编译 RED 不保证未来补入类型后不存在其他类型检查问题。

| 测试函数（省略共同 p0_/p1_ 前缀） | AC / GREEN任务 |
|---|---|
|raw_order_gaps_strobes_prot_errors_native_and_rtl|2/4/7：独立握手、4种偏斜、16WSTRB、8PROT、错误与RTL|
|offer_lock_round_robin_and_independent_response_slots|3/4：轮转、锁定、阻塞旧响应期间新异类事务进展|
|reset_cancels_partial_offer_execution_responses_and_recovers|5/7：8阶段取消及恢复账本|
|sixteen_reproducible_seeds_each_complete_one_thousand_transactions|7：16固定seed×1000真实完成事务预算；实际完成数当前为未执行|
|protocol_monitor_rejects_withdrawal_mutation_and_accepts_reset|7：7通道合法驱动独立监测器及负例|
|late_read_cannot_steal_locked_write_and_partial_write_never_blocks_read|2/3：CSR offer受阻后锁定，partial AW/W不堵read|
|same_session_bridge_real_csr_accesses_snapshots_and_single_side_effects|1/6：真实CsrBlock组合、黄金寄存器值、权限、动态拒绝与副作用计数|
|public_ports_and_two_same_session_bridge_instances_have_private_state|1：全部端口类型/宽度/方向、复用定义、两实例独立RTL|
|port_oracle_safety_induction_and_nonvacuous_covers（ignored）|2–5/7：实际SBY prove32/cover64，五类cover，不假定B/R公平性|
|original_rtl_synthesis_check_and_all_five_registered_boundaries（ignored）|5/7：原始RTL综合/check与严格组合依赖锥|
|structural_observer_rejects_reset_paths_latches_unknowns_and_missing_drivers|5/7：纯结构检查器正例及reset路径/latch/未知单元等负例|

共 11 项：9 项 P0、2 项 P1；行为8项，formal/结构3项。9项普通入口、2项专用工具入口。AC8文档/API/兼容/CI及完整七步为明确 GREEN/后续流程义务，尚无通过证据，不用ATDD替代关闭。AC1 prelude-only文档编译与层级原生模拟不支持诊断仍待实现阶段补验证。

| 文件 | 行数 |
|---|---:|
|`crates/bitloom/tests/fr196_axi_lite_csr.rs`|831|
|`crates/bitloom/tests/fr196_axi_lite_csr/tools.rs`|112|
|`crates/bitloom/tests/fr196_axi_lite_csr/integration.rs`|390|
|`crates/bitloom/tests/fr196_axi_lite_csr_formal.rs`|406|

## 执行命令与激活

```bash
export PATH=/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin:$PATH
export CARGO_PROFILE_TEST_OPT_LEVEL=1 PYTHONDONTWRITEBYTECODE=1 BITLOOM_REQUIRE_RTL=1
cargo test -p bitloom --test fr196_axi_lite_csr --test fr196_axi_lite_csr_formal --no-run
cargo test -p bitloom --test fr196_axi_lite_csr -- --nocapture
cargo test -p bitloom --test fr196_axi_lite_csr_formal -- --nocapture
cargo test -p bitloom --test fr196_axi_lite_csr_formal -- --ignored --nocapture
# 单场景调试：在 -- 前加测试函数过滤字符串，必要时 RUST_BACKTRACE=1。
```

第一条 cargo 命令已真实执行并符合预期 RED；其余命令是 GREEN 阶段待执行入口。普通目标无需解除 ignore；formal/综合使用显式 ignored 命令，必须接入 CI，不能把默认 ignored 当通过。无 headed/browser 调试需求。

## 验证清单结论与明确适配

- [x] 前置：故事批准、8AC、Rust/Cargo框架及已有RTL/formal工具配置齐备；主层次是原生协议/真实RTL集成。
- [x] 上下文、技术约束、类似 fixtures、测试层级与 P0/P1 风险映射均已记录。
- [x] 两独立 worker 完成并聚合，状态/源路径/limitations保存在 `127-3-atdd-worker-{behavior,formal}.json`。
- [x] 预期行为断言落盘；Given/When/Then通过描述性测试名及准备/驱动/断言代码表达。多周期协议采用多断言和循环以校验同一不变量，不机械套用Web单断言限制。
- [x] 共享基础设施：输入构造器、协议monitor、握手事件scoreboard、合规CSR peer、真实CsrBlock组合、隔离RTL runner。固定seed与协议黄金常量可复现；不适用faker/HTTP/auth/data-testid。
- [x] Rustfmt `--edition 2024 --check` 四文件 exit0。最初不带edition的直接检查使用旧版style产生差异，随后按工作区edition复核成功，未改变源代码。
- [x] 初始编译 RED 日志及命令保存。没有空成功断言，也没有宣称现阶段通过任何产品测试。
- [x] observer 单独语法探针 `127-3-atdd-observer-syntax-probe.sv` 将观察端口全声明input，仅 Yosys `read_verilog -formal` exit0；源码/日志/命令保留。不是DUT替身、不是formal证明，也不是原始RTL综合证据。
- [x] 模板所需故事摘要、AC、测试路径/行数、fixtures/mocks、实现清单、RED→GREEN→REFACTOR、命令、估算与handoff齐全。
- [x] 页面/network-first/Playwright/Pact/TypeScript项目特定检查不适用；遵循mandate适用性，无额外依赖，无相关deviations。JS test.skip按用户明确要求适配为真实Rust编译RED。
- [x] CLI均已退出，无浏览器会话；独立worker临时JSON已复制到test-artifacts，保留失败证据不算遗留活动资源。
- [x] 源码快照以 `127-3-atdd-source.tar.gz` 与 `127-3-atdd-source-manifest.json` 紧凑保存并逐成员校验；未复制散落源文件。

知识片段：test-quality、data-factories、component-tdd、test-levels-framework、test-priorities-matrix、confidence-gate、evidence-integrity、test-healing-patterns、ci-burn-in及库适配mandates。复现性、独立oracle、风险分层和证据边界均用于本方案；纯Web fixture/network模式不适用。

## 风险与下一步

1. 正式执行均未开始，尤其形式归纳可能需增加**经证明**的辅助不变量；不得假设DUT不覆盖/不丢失以获得通过。peer有限响应≤7拍，未假设B/R最终ready；无整体活性宣称。
2. 随机测试逐事务排空，仅随机channel偏斜/背压/负载；并发重叠由定向测试覆盖，不称随机穷举。后续automate按审查增加有意义覆盖。
3. monitor负例验证Rust monitor，实际RTL变异测试尚未做。错误01仍为非法peer/out-of-contract；失败读0由合法peer/leaf保证。
4. 下一步按已授权流程 build 实现127.3，然后code-review、automate、clean/fmt/完整回归、单故事提交。故事保持ready-for-dev；本清单只完成ATDD，不标故事done，不关闭M2/FR196。

Story handoff：`_agile-output/implementation-artifacts/127-3-axi-lite-csr-桥.md` 的 ATDD Artifacts 已由父任务链接；若文件移动，须同步该节和本文件frontmatter。无须常规再次确认。

## 最终GREEN回链（2026-09-21）

以上RED/未执行陈述保留为ATDD时点历史。实现、独立review、automate与实际clean/fmt/justtest现已完成，见[最终验收](127-3-final-verification.md)；17最终入口，1801 workspace通过/0失败/21忽略，新增专用已另行真实运行。单故事提交由git记录核验；仅127.3关闭。
