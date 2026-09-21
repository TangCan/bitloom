---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-identify-targets', 'step-03-generate-tests', 'step-03c-aggregate', 'step-04-validate-and-summarize']
lastStep: 'step-04-validate-and-summarize'
lastSaved: '2026-09-21'
storyId: '127.4'
status: complete
---

# Story127.4 自动化验收扩展

BMad-integrated / Rust Cargo backend。配置已加载，技能activation hooks为空；按用户七步连续授权执行，未重复请求常规确认。两worker实际并行完成，模式auto→subagent，无容量fallback；没有测量并行加速，不宣称百分比。

## Step02 覆盖计划

已按story AC1–8与ATDD历史/最终测试对照：API/52端口/全地址、owner/miss、共同reset、16seed真实七模块、17cover/6mutant、软件黄金/CI和支持限制均有现存测试，避免再次复制同类定向场景。AC8是主流程最终门禁，不由本步骤自称完成。

| 目标 | 层级/优先级 | 新覆盖及理由 | 责任 |
|---|---|---|---|
| standalone长延迟跨后端一致性 | native+实际RTL / P0 | 既有全地址是eval-only，组合真实leaf固定下一周期；新增固定seed有限随机合法producer/responder轨迹，以独立单槽port oracle逐周期比较两native和实际emitted单模块RTL。包含4owner、miss、请求等待、长响应等待/背压、reset取消/恢复及无效leaf payload，不新增generated层级承诺 | A，新fr196_csr_decoder_automate.rs |
| WO副作用观察器敏感性 | 实际七模块RTL / P0 | 已补candidate/mask正确值断言但尚无实际损坏这8条输出的负例。原始control通过，分别破坏UART TX/GPIO SET/CLEAR/IRQ TEST的candidate与write_mask，必须触发明确WO payload断言，工具错误不算反例 | B-backend，新fr196_csr_decoder_oracle.rs |
| 已有proof/cover、支持拒绝、C/Markdown、17cover/6mutant与8monitor路径 | 保留 / P0-P1 | 已有重验且源码未变，不为增加数量重复实现；新增测试后聚合运行受影响目标；最终全workspace及专用证据按七步保存 | root |

A为有限确定性cycle replay而非统计完整性证明；预算4明确seed×128已完成事务，各类命中实际计数，reset取消另外计数。使用单decoder允许自定义合法finite/long responder，对所有输出只在合同有意义时比较，不能把未选payload要求归零。B针对实际emit RTL单处输出突变，用当前独立scoreboard接受真实AXI请求；不只测试字符串替换或预设golden。缺工具强失败，原control和所有mutant分别记录。

无HTTP/API endpoints、数据库或消息服务；Provider Endpoint Map=N/A。无frontend/mobile，不启浏览器。已批准范围只扩现有FR196验收，不改产品RTL或新增公共API。知识的双层重叠原则允许native和RTL比较后端一致性，与现有组合协议测试目的不同。


## 生成与实际验证

新增`crates/bitloom/tests/fr196_csr_decoder_automate.rs`及`fr196_csr_decoder_oracle.rs`，8个P0入口，0新shared fixture，复用common/scoreboard。worker结构化原始源码见`127-4-automate-worker-{api,backend}.json`，生成时逐字核对落盘；随后仅rustfmt调整格式，最终源码随归档保存。

实际命令：`cargo test -p bitloom --test fr196_csr_decoder_automate --test fr196_csr_decoder_oracle -- --nocapture`，环境见`127-4-automate-tests.json`。exit0，6.550秒，8 passed / 0 failed / 0 ignored；命令前两个新增文件rustfmt exit0。CI普通workspace自动发现新target，既有Icarus必需配置和失败上传目录直接复用。未宣称远程CI已运行。

| 固定seed | 周期 | 完成 | 取消 | 请求停顿周期 | 叶等待周期 | 响应停顿周期 |
|---|---:|---:|---:|---:|---:|---:|
|12740001|7053|128|9|551|4599|1187|
|a55a1234|7102|128|9|550|4699|1137|
|deadbeef|7061|128|9|553|4593|1199|
|5eedcafef00d|6913|128|9|607|4367|1223|

每seed完成分布UART/GPIO/Timer/IRQ/miss为26/26/26/25/25，取消为2/2/2/2/1；逐周期同时验证Interpreter、Compiled、实际单模块Icarus RTL。读/写均实际覆盖00/10/11，01计数0。输入生成与独立slot oracle分离，只比较合同有意义的payload；inactive值可任意。有限确定性回放不是统计完整性证明，也不扩展native/generated层级支持。

四个WO原版control各完成真实AXI→bridge→decoder→leaf事务，独立字面量核对非零mask/candidate。八个candidate/write_mask实际RTL单处突变全部compile成功、vvp exit1且目标地址WO payload fatal；不是工具错误当反例。完整计数和路径见`127-4-automate-results.json`。

生成审阅曾发现有效leaf response可生成保留码01/失败读非零，执行前已按合同修正生成器并增加实际错误码/读写计数，未改产品或oracle迎合结果。实际首次运行全部通过，无重试掩盖失败、fixme或skip。产物`127-4-automate-artifacts.tar.gz`共2302成员、7,870,820字节，SHA256 `8886080c5da150d8cba6889bc9cdee4695e0c98a7c9e27cff0c206a06f130efd`；manifest逐成员回读成功，保留已有工具尝试与新trace，排除可重建simulation二进制。

## 适用性与质量核对

- Framework/目录/CI：Cargo manifests、现有测试和CI已读；无适用浏览器/mobile/HTTP/provider边界，不新增JS依赖或虚假API。
- 知识：mandate三篇优先适用性门，test-levels-framework、test-priorities-matrix、data-factories、selective-testing、ci-burn-in、test-quality、pact-mcp、playwright-cli、confidence-gate、evidence-integrity已完整读取；合并输出截断处补读。fixture-architecture的Playwright机制不适用，复用现有Rust fixture。
- Confidence 8：依据固定ATDD接口、common真实工具链及scoreboard独立WO断言。运行前未知编译/行为现已实测解决；远程CI、无限随机空间未实测，明确不作承诺。
- checklist逐项按Rust适用性检查：8 P0、单目的行为/多信号一致性、独立seed/PID目录、合法producer、严格oracle、有限外部工具timeout。硬件tick不是墙钟盲等，golden硬件地址/数据字面量是独立要求。程序化分支生成硬件刺激并非浏览器条件跳过断言。
- 保存命令、退出码、耗时、日志SHA、工具trace与源码；无浏览器/服务进程需清理；临时worker JSON已归入故事目录。生成JSON保留原始快照，不伪装为格式化后文件。
- 文件长度例外：格式化后回放target为411行（另一个187行）；其中大部分是同一可重放端口oracle与RTL trace适配，不拆成跨target共享状态以凑行数；4个测试入口很短。保持一个独立验证职责，作为测试质量建议的明确取舍。
- 未新增重复框架/共享helper、认证或网络stub；实际RTL必须真实工具，不用mock代替。没有变更产品RTL/API。专用formal/synthesis此前最新源码重验4项通过，17cover/6反例及独立归档保持有效。

## Playwright Utils deviations

None。Rust Cargo runner不适用Playwright utility；无待接auth/HAR/webhook/burn-in配置。工具清单一次探测无Pact MCP，pact_mcp_reachable=false是能力不可用，不代表broker连通性失败，未重复探测/请求凭据。

## Pact.js Utils deviations

None。无consumer-provider边界，无Pact artifacts；配置flag不强制创造服务契约。

下一步为用户七步第六步：真实`cargo clean && cargo fmt --all && just test`，随后第七步单故事提交并按完整结果关闭M2。此automation完成不独立关闭story/Epic127。
