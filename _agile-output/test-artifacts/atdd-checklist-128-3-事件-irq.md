---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-generation-mode', 'step-03-test-strategy', 'step-04-generate-tests', 'step-04c-aggregate', 'step-05-validate-and-complete']
lastStep: step-05-validate-and-complete
status: complete
lastSaved: '2026-09-21'
storyId: '128.3'
storyKey: '128-3-事件-irq'
storyFile: '_agile-output/implementation-artifacts/128-3-事件-irq.md'
atddChecklistPath: '_agile-output/test-artifacts/atdd-checklist-128-3-事件-irq.md'
generatedTestFiles:
  - crates/bitloom/tests/fr197_irq.rs
  - crates/bitloom/tests/fr197_irq_api.rs
inputDocuments:
  - _agile-output/implementation-artifacts/128-3-事件-irq.md
  - _agile-output/test-artifacts/128-3-create-story-validation.md
  - _agile-output/implementation-artifacts/epic-128-nfr14.md
  - docs/ip/phase24-contract.md
  - _bmad/tea/config.yaml
  - crates/bitloom-prelude/src/ip/csr/rtl.rs
  - crates/bitloom/tests/fr197_timer.rs
  - crates/bitloom/tests/fr197_timer_api.rs
---

# Story128.3 ATDD

Step01：Create；已完整读取故事与验证，前置done、ready-for-dev；Rust/Cargo backend，既有integration框架和工具可用。resolver hooks/persistent为空。Playwright/Pact utils标志true，但本项目Rust硬件验收不属于JS runner或consumer/provider服务边界，不引入Pact/UI产物。工具列表单次probe为空，pact_mcp_reachable=false、fallback_source=none，不调用broker。13篇知识沿用本会话完整读入内容并逐篇SHA相同，记录128-3-atdd-knowledge-sha.json；mandate和Pact降级路径已复核。无需重批既定范围。

Step02：backend采用AI generation，独立Rust参考与实际SV仿真；无浏览器recording。

## 固定接口

`bitloom_prelude::ip::Irq` 无配置固定五路类型；`Irq::registers()->CsrBlock`、`Irq::define_module(&mut ElaborateSession, impl Into<String>)->Result<String,Diagnostics>`、`Elaboratable for Irq`。standalone顶名Irq，private CSR名BitloomIrqCsr（非新增稳定API）。14端口：clk Clock、rst Reset，输入req_valid1/write1/addr16/wdata32/wstrb4/rsp_ready1/raw_events5；输出req_ready1/rsp_valid1/rdata32/error2/irq1。raw_events为沿前硬件事件，同上升沿采样，不加edge detector。

描述名Irq，pending(0,W1c,Leaf,event_bits)、enable(4,Rw,Leaf)、test(8,Wo,None)、raw(12,Ro,External)。四寄存器各五字段 timer/uart_rx/uart_tx/uart_error/gpio，mask分别1/2/4/8/16，reset0，与所在寄存器access相同；无reject。软件头/Markdown docs/ip/irq-registers.{h,md}。内部event_bits是硬件脉冲与commit门控TEST组合，不属于公共针脚。

## Step03 验收映射

|AC|优先级/层|独立验收|
|---|---|---|
|1|P0结构/实际组合|14端口/唯一session/复用冲突/方向位宽/同定义与不同名双实例；native层级拒绝|
|2/4/5|P0实际RTL|五路独立pending/enable/TEST、32组合、连续事件、mask不丢、W1C set胜clear、TEST提交门控、RAW只硬件、快照/无refill|
|3|P0总线/P1软件描述|全部16WSTRB、保留位、权限优先、洞/未对齐/高别名、C/Markdown手写黄金|
|6|P0实际组合|reset取消账本、真实Timer→IRQ raw与sticky分离、新match重置pending|
|7|P1专用/人工|三后端同向量、formal prove/cover/负控制、原始综合、FR142/minor/prelude-only/CI/完整回归|

先生成失败scaffold，Step05激活当前测试观测真实E0432 RED；编译缺API不是行为失败证据。耗时专用入口可ignore但必须build独立运行，不能记默认PASS。用户七步覆盖任务授权，commit只第七步。

Step04调度：requestedMode=auto, probeEnabled=true，运行时有subagent、无agent-team接口，resolvedMode=subagent。timestamp=1283-20260921T0930。A行为RTL参考，B公开API/实例/Timer集成（backend替代不适用浏览器E2E），各JSON聚合；不宣称并行加速比例。

## Step04/04c 聚合

两worker成功，full-source JSON归档；主代理全文读行为241行与API/组合源码。共10个Rust入口（7 P0/3 P1），scaffold全部ignore；1个P1是实际JVM/FIRRTL专用门禁，另外9个Step05激活。每seed32×32源/mask组合、240组WSTRB、逐源碰撞与RAW、3200随机cycle；组合包括同名双实例、重命名双实例与真实Timer。源码覆盖不等于实测，尚无行为PASS。Rust fixture/参考/RTL工具runner自包含，无UI/HTTP mocks或faker；硬件地址黄金刻意手写、随机仅扩充。Playwright Utils deviations: None（不适用）；Pact.js Utils deviations: None（无边界）。

T1接口/红测→T2唯一owner实现→T3三后端/形式/综合→T4文档/兼容/全回归；估算2–3有效人日。RED执行命令 cargo test --locked -p bitloom --test fr197_irq --test fr197_irq_api --no-run。全部七步完成才commit。

## Step05 实际RED与验证

激活9个当前Rust测试，保留1个专用后端ignore；实际cargo --no-run exit101，两个目标分别E0432缺少Irq，原始log/JSON已保存（128-3-atdd-red.*）。无其它编译错误，尚未验证行为。此为接口RED，不将9个未执行行为称为失败或通过。

按checklist的Rust/backend适用项审阅：7AC映射、10入口、固定接口与独立地址/状态参考、合法受阻producer、显式超时/非零失败、账户守恒、唯一工件目录、原文软件产物核对、story手动交接已链接。JS DOM/HTTP/faker/merged-fixtures不适用；无browser会话。构建阶段需执行全部三后端、形式、综合和专用ignore入口并留证。ATDD完成，下一bmad-build；未提交、未实现产品、未关闭128.3。
