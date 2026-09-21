---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-identify-targets', 'step-03-generate-tests', 'step-03c-aggregate', 'step-04-validate-and-summarize']
lastStep: step-04-validate-and-summarize
lastSaved: '2026-09-21'
storyId: '128.3'
status: complete
detected_stack: backend
pact_mcp_reachable: false
inputDocuments:
  - _agile-output/implementation-artifacts/128-3-事件-irq.md
  - _agile-output/implementation-artifacts/spec-128-3-irq.md
  - _agile-output/test-artifacts/atdd-checklist-128-3-事件-irq.md
  - _agile-output/test-artifacts/128-3-code-review.md
  - _agile-output/test-artifacts/128-3-root-code-review-recheck.json
  - _agile-output/test-artifacts/128-3-automate-knowledge-sha.json
  - _bmad/tea/config.yaml
  - crates/bitloom/Cargo.toml
---

# Story128.3 自动化补强

Create / BMad-Integrated，Rust Cargo backend；已有活动与专用RTL/形式测试框架，不新增浏览器/JS设施。完整story/spec/ATDD及源码、三类测试、CI、工具harness均已读并实测。输出按故事隔离，避免覆盖历史automation-summary。

Step01：resolver前后hooks、persistent和on_complete均空。13篇此前全文读知识逐篇SHA相同，本轮补全文读selective-testing并再核实mandate作用域。Playwright库适用门要求JS/TS+Playwright runner，本项目Rust不满足；Pact无consumer/provider服务边界，不生成Pact工件或加载不适用运行器机制。Pact工具清单本轮单次探测为空，pact_mcp_reachable=false，fallback_source=none，无broker调用/重试。核心风险/独立参考/隔离/可证伪与证据规则适用，浏览器/mobile/auth/webhook机制不适用。

## Step02 风险覆盖计划

AC1/3 API、描述/诊断/软件产物、双实例与Timer组合已有活动8项及专用3图三后端；AC2/4/5/6全向量每seed1024源/掩码、320WSTRB、逐源竞争/快照/reset三seed共97,738帧，direct/FIRRTL/Chisel实际PASS。AC7独立formal证明、23cover、3formal故障注入、原始综合、12harness×普通/-O与CI、源码绑定归档已覆盖。

选定补强：P0 direct RTL scoreboard检错控制。原始控制真实通过，再对生成DUT作少量定点变异（五源映射错误、TEST字节选择/注入错误），同一个独立短向量testbench必须报期望行为断言且留下VCD；编译错误/timeout不计检出。现有formal变异验证observer，不能替代direct scoreboard的检错证据。复用当前Oracle/Trace而不复制产品next-state；不改产品/工具钉/黄金，不再重复正常接口测试或引入测试专用产品路径。API worker只读审计剩余具体缺口，允许无新增测试并输出原因。无Provider Endpoint Map：没有服务边界。

执行顺序：直接RTL新检错控制定向→root验收→实际clean/fmt/全workspace；新源码需要当前源码绑定归档，旧完整run只保留其历史有效范围。Playwright Utils deviations: None（不适用）；Pact.js Utils deviations: None（无Pact工件）。

Step03 dispatch：requestedMode=auto，probeEnabled=true；运行时subagent工具存在且本轮实际spawn成功，agent-team接口不可用，resolvedMode=subagent。timestamp=1283-20260921T1041，API只读复核与backend单个P0检错控制并行；E2E/mobile因backend跳过。输出固定/tmp/tea-automate-{api,backend}-tests-1283-20260921T1041.json。不宣称未经测量的并行加速比例。

Step03/03c：两worker成功。API全文复核无具体缺口，0新测试；backend输出完整现文件+1 P0，主代理逐diff审阅119行新增，原931行逐字节保持、基线SHA匹配。170帧：逐源读回与每源6种TEST strobes，原控制和两DUT变异使用字节相同黄金tb，要求预定cycle/after/rdata唯一FATAL、编译成功/vvp1/VCD。只有生成，尚未实际验证。无需共享fixture/HTTP/认证/网络mock，不引入不适用JS文件。worker完整JSON和统计已归档，未测量并行加速。

## Step04 实测与验收

新增1个P0实际PASS（0.35秒测试体，含编译cargo命令1.99秒）；原始170帧正常结束，GPIO映射变异cycle18 after rdata expected10/got1，TEST漏WSTRB变异cycle23 after rdata expected0/got1，三个编译均exit0、原vvp0/两变异vvp1，唯一预定FATAL且VCD有效。主代理复核所有命令及三份tb原字节一致。原始工件128-3-automate-raw.tar.gz已回读比对、SHA/成员见128-3-automate-manifest.json，完整命令见128-3-automate-validation.json/.log。

执行：`cargo test --locked -p bitloom --test fr197_irq p0_irq_direct_scoreboard_rejects_source_mapping_and_test_byte_faults -- --exact --nocapture`，PATH需iverilog/vvp/timeout。普通cargo workspace自动包含此活动测试，CI既有IRQ失败目录包含其工件。无新增ignore/skip/fixture/依赖，不改产品行为。Rust精确地址、事件位、拍号、WSTRB属于刻意固定的领域黄金，不替换为faker。唯一temp目录隔离；硬件#1推进是离散时钟而非wall-clock等待；每外部命令60秒上限。检错控制一项含原始与两变异，不能报成3个新增Rust测试。

Checklist适用项完成：框架/AC风险/优先级/不重复/独立参考/确定性/隔离/命令失败传播/真实执行/原始证据/CI默认入口/说明与完整JSON归档。JS/DOM/HTTP/认证/Pact/provider/浏览器session不适用，无浏览器残留。自动修补无需执行；没有未修复测试或虚假fixme。文件沿用现有helper以免复制黄金，未为行数偏好另造测试框架。

### Playwright Utils deviations
None（Rust非Playwright测试）。

### Pact.js Utils deviations
None（无消费者/提供方合同）。

下一步按用户第六步实际cargo clean→cargo fmt --all→just test，随后第七步单故事commit。当前automate完成不意味着Story128.3或FR197整体关闭。新源码最终证据将再绑定一次完整runner；旧run仅代表旧源码阶段，不回写历史指纹。
