---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-generation-mode', 'step-03-test-strategy', 'step-04-generate-tests', 'step-04c-aggregate', 'step-05-validate-and-complete']
lastStep: 'step-05-validate-and-complete'
status: complete
lastSaved: '2026-09-21'
storyId: '128.2'
storyKey: '128-2-timer'
storyFile: '_agile-output/implementation-artifacts/128-2-timer.md'
atddChecklistPath: '_agile-output/test-artifacts/atdd-checklist-128-2-timer.md'
generatedTestFiles:
  - crates/bitloom/tests/fr197_timer.rs
  - crates/bitloom/tests/fr197_timer_api.rs
inputDocuments:
  - _agile-output/implementation-artifacts/128-2-timer.md
  - _agile-output/implementation-artifacts/epic-128-nfr14.md
  - docs/ip/phase24-contract.md
  - _bmad/tea/config.yaml
  - Cargo.toml
  - crates/bitloom/Cargo.toml
  - crates/bitloom-prelude/src/ip/csr/rtl.rs
  - crates/bitloom/tests/fr196_csr.rs
---

# Story128.2 ATDD

Step01：Create模式；用户全故事七步授权已覆盖本故事，128.1/127.2 done，128.2 ready-for-dev。已全文读取故事/验证、正式合同、CSR接口/RTL及测试模式，核对NFR14矩阵。Rust/Cargo backend，无浏览器/mobile/HTTP边界；既有Cargo integration tests可用。customization hooks/persistent为空。TEA两utils=true但Rust runner不适用，Pact无consumer/provider边界不生成产物；Pact工具清单一次检查为空，pact_mcp_reachable=false，无broker调用/推断states。知识索引及mandate已读，其余13篇沿用本会话已完整读入且逐字核对automate副本，SHA见128-2-atdd-knowledge-sha.json。没有假端点、JS依赖或浏览器healing。

Step02：backend选择AI generation，由明确合同生成独立Rust/SV参考。原计划仅字符串文档验证不适合Timer硬件；本轮实际编译RED，随后真实RTL GREEN。不重复请求用户确认。

## 固定接口（ATDD/build输入）

新增 `bitloom_prelude::ip::Timer` 固定32位单时钟类型；`Timer::define_module(&mut ElaborateSession, impl Into<String>) -> Result<String, Diagnostics>`；`Elaboratable for Timer` 的 `Timer::elaborate()`；`Timer::registers() -> CsrBlock` 提供同源本地寄存器描述与已有C/Markdown生成方法。无需TimerConfig。端口恰为clk Clock、rst Reset，输入req_valid1/write1/addr16/wdata32/wstrb4/rsp_ready1，输出req_ready1/rsp_valid1/rdata32/error2/match_event1。match_event为沿前组合事件，同上升沿采样，reset与有效配置写门控；不是sticky EVENT。描述名字Timer，寄存器ctrl/count/compare/EVENT（EVENT大写避免SV关键字），字段bits，CTRL/COUNT External RW、COMPARE Leaf RW、EVENT Leaf W1c；event输入名match_bits私有组合连接。共享定义私有子模块名不是新增稳定API。

## Step03策略

|AC|P/层级|验收|
|---|---|---|
|1|P0 Rust结构+真实多实例RTL|固定端口、共享定义复用/冲突、唯一session、实例隔离；native/generated层级明确拒绝|
|2|P0实际RTL/P1描述产物|独立硬编码0/4/8/c、WSTRB16种、reserved mask、高位/洞/未对齐、错误读0；C/Markdown同源并核对黄金|
|3–5|P0实际RTL独立scoreboard|periodic/one-shot/COMPARE0/bit31/wrap/越过、同值和0写抑制、零有效mask不停、W1C set胜clear、raw事件与sticky分离|
|6|P0实际RTL账本|read-before-update、长背压/持续valid/消费无refill、运行中及碰撞reset取消，提交=消费+取消+在途|
|7|P1后端/专用formal+综合/人工|build执行direct/Chisel/FIRRTL同向量、prove/cover独立、原始产品综合；FR142/minor/兼容/完整回归后才关闭|

原始new API缺失应触发E0432编译RED；它只证明缺少入口，不能宣称所有行为已失败。新Rust验收保持active以观测RED，遵循项目既有ATDD方式；不使用ignore假造绿色。专用耗时formal如采用ignore需明确另跑并保留真实结果，不计默认通过。无重复UI/E2E测试。confidence=9：合同与实际CSR模块提供接口语义；证明/后端结果待build实测。

Step04：requestedMode=auto/capability_probe=true，运行时具备subagent但无agent-team接口，resolvedMode=subagent；时间标签1282-20260921。A生成真实RTL行为红测，B生成结构/组合边界红测（backend替代无关browser E2E）；输出临时JSON后聚合，尚未运行。无加速比例声明。

## Step04/04c 聚合

A/B均成功；根全文审阅两个源码，full-source JSON与写盘逐字一致，并将JSON归档。共8个active Rust入口（6 P0/2 P1）：A 1入口×3独立seed定向+2400随机cycle；B7入口含真实双Timer共享定义/隔离。不是8个浏览器/API endpoint测试；E2E=0。Rust active RED为上述项目适配，不引入skip/空断言；缺API预期compile RED尚待Step05运行。软件参考只从总线与raw事件观察，地址黄金独立；工具runner有超时且非零必失败，留RTL/tb/VCD/日志。现阶段测试源码定义覆盖不等于已验证行为。主代理纠正define_module返回String以保持现有生成器惯例，未改产品。

Playwright Utils deviations: None（Rust runner不适用）。Pact.js Utils deviations: None（无consumer/provider边界）。Fixture为Rust输入/状态参考与临时RTL目录，工具日志保留到target后build归档；无需JS merged-fixtures/faker/selectors/mock endpoints。接口端口13项为真实硬件合同，literal地址/位掩码不得随机替代。实施次序T1固定接口/红测→T2唯一owner实现→T3真实各后端/证明/综合→T4文档/兼容，直到第七步才commit。

## Step05 实际RED与交接

2026-09-21T08:06:27Z执行`cargo test --locked -p bitloom --test fr197_timer --test fr197_timer_api --no-run`，实际exit101、0.284s；两个目标各有E0432 missing Timer，原始log/json已归档。未实现任何产品代码，未伪造功能失败；8入口的行为断言须build GREEN阶段实际执行。B文件随后仅rustfmt格式调整，worker JSON保持原始生成快照。Checklist按Rust/backend适配核对：7AC映射、8入口/手写黄金、工具失败与超时、独立状态/seed、故事链接及配置齐备，无浏览器会话。已完成红测与handoff，下一步用户指定bmad-build。T1–T4、2–3有效人日估算见故事；不提前commit、不声称FR197完成。
