---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-generation-mode', 'step-03-test-strategy', 'step-04-generate-tests', 'step-04c-aggregate', 'step-05-validate-and-complete']
lastStep: 'step-05-validate-and-complete'
lastSaved: '2026-09-21'
storyId: '128.4'
storyKey: '128-4-gpio-csr-wrapper'
storyFile: '_agile-output/implementation-artifacts/128-4-gpio-csr-wrapper.md'
atddChecklistPath: '_agile-output/test-artifacts/atdd-checklist-128-4-gpio-csr-wrapper.md'
generatedTestFiles: ["crates/bitloom/tests/fr197_gpio_api.rs", "crates/bitloom/tests/fr197_gpio.rs"]
inputDocuments:
  - '_agile-output/implementation-artifacts/128-4-gpio-csr-wrapper.md'
  - '_agile-output/implementation-artifacts/epic-128-nfr14.md'
  - 'docs/ip/phase24-contract.md'
  - '_agile-output/test-artifacts/128-4-csr-recon.md'
  - '_agile-output/test-artifacts/128-4-atdd-knowledge-sha.json'
  - '_bmad/tea/config.yaml'
pact_mcp_reachable: false
fallback_source: none
---
# Story 128.4 ATDD

Step01：已全文读取故事、正式合同、风险门及旧GPIO/CSR/IRQ实现；Rust/Cargo backend，现有integration tests及三后端工具可用。前置128.3/126.2/128.1 done。技能resolver hooks/persistent为空；既有用户连续七步授权覆盖例行确认。13篇此前完整读入的知识逐篇SHA相同，见128-4-atdd-knowledge-sha.json。Playwright/Pact标志虽true，但Rust硬件测试不在JS runner或服务边界范围，无需引入UI/Pact依赖。单次工具列表probe为空，不调用broker，不推断provider状态。

Step02：AI generation模式；根据已批准硬件合同编写Rust与实际RTL测试，无浏览器录制。

Step03 API冻结：新增无配置unit类型`ip::GpioCsr`，`registers()->CsrBlock`，`define_module(&mut ElaborateSession, impl Into<String>)->Result<String,Diagnostics>`，`Elaboratable`顶层名GpioCsr；CSR私有模块建议BitloomGpioCsrRegisters。16端口：clk Clock、rst Reset，输入req_valid1/write1/addr16/wdata32/wstrb4/rsp_ready1/pad_in32；输出req_ready1/rsp_valid1/rdata32/error2/pad_out32/pad_oe32/raw_event1。pad_out=OUT & DIR、pad_oe=DIR；数据非物理三态。raw_event=(!rst) & reduction_or(sync2 & ~history & ~DIR)。DIR/OUT仍同步复位，不声称rst拉高未到沿就清状态输出。

描述block名GpioCsr；dir/out/in/set/clear/rise_event分别0/4/8/12/16/20，访问Rw/Rw/Ro/Wo/Wo/W1c；owner Leaf/External/External/None/None/Leaf；各单字段bits mask0xffffffff reset0，event仅rise_event为rise_bits；所有reject=false。全32有效，无ID。标准CSR连接与唯一OUT守恒保持故事正文。

|AC|层次/优先级|独立验收|
|---|---|---|
|1|API P0/P1|16精确端口、静态描述黄金、复用/诊断、native/generated明确拒绝|
|2,3,5|实际RTL P0|手写逐拍32位参考，全部16WSTRB×5可写寄存器、空/混合SET和全1/混合CLEAR、错误/响应账本|
|3,4,6|实际RTL P0|sync两级和history逐拍、沿前DIR、初始高/bit31/方向/clear竞争/背压/reset|
|1,4|组合RTL P0/P1|双实例共享/重命名定义隔离，真实GPIO→IRQ4（其余四路0），sticky不重触发|
|5,6,7|API与持续门禁 P1|旧Gpio/FL、软件产物确定性、原文示例/C头，三后端执行与formal/synth分别列证据|

RED先以缺GpioCsr API编译失败核验，不能据此声称行为断言已执行。行为随机扩充但必达定向必须有计数；formal与合成在build补真实专用测试，不以mock代替。

Step04编排：config auto/probe=true；现有spawn_agent/send_message可并行独立worker，按agent-team语义执行；两个worker分别API/组合和实际RTL行为，仅返回full-source JSON，主代理聚合后落盘。Rust #[ignore]对应RED scaffold，尚未以skip计PASS。

Step04C：两个worker成功，12个Rust入口全部ignored RED（API10、行为2），主代理全文读取约1740行源码。API/组合与逐拍行为覆盖AC1–7；formal/原始综合在build增加。fixture为Rust内置参考与真实RTL驱动，没有UI/Pact mocks、faker或新依赖。原始worker JSON已保存本目录。Playwright Utils deviations: None（不适用）；Pact.js Utils deviations: None（不适用）。软件产物固定docs/ip/gpio-csr-registers.{h,md}。

Step05完成：按checklist核对Rust适用项，UI selectors/HTTP/Pact/faker/浏览器清理为N/A；12个scaffold原始JSON留存，当前任务10个活动入口已激活，2个专用JVM入口仍ignored但build必须实际执行。两目标实际cargo test都报E0432缺GpioCsr，exit101，见128-4-atdd-red.{json,log}；仅接口RED，不声称模型/RTL已跑。旧GPIO兼容测试自身验证既有行为，其所在新API测试目标当前无法编译，不能误说旧GPIO产品原本失败。

实现清单：T2按冻结描述/端口实现唯一OUT/同步链；T3实际direct全向量、三组合及专用后端，加入独立formal/cover/原始synth及真实DUT变异；T4原文示例/C头/登记/CI与证据runner；T5独立审查/automate/clean+fmt+just test/单提交。预估仍故事2–3有效人日，不作墙钟承诺。先保持现有黄金，失败区分产品与夹具，修复均保留日志。

命令：`cargo test --locked -p bitloom --test fr197_gpio --test fr197_gpio_api -- --nocapture`；专用分别精确运行`p1_gpio_firrtl_chisel_same_independent_vectors`与`p1_composition_firrtl_chisel_shared_renamed_actual_gpio_irq`并带`--ignored --exact --nocapture`。主代理下一步bmad-build，提交仍延至第七步。
