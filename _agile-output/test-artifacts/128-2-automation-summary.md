---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-identify-targets', 'step-03-generate-tests', 'step-03c-aggregate', 'step-04-validate-and-summarize']
lastStep: 'step-04-validate-and-summarize'
lastSaved: '2026-09-21'
inputDocuments:
  - _agile-output/implementation-artifacts/128-2-timer.md
  - _agile-output/implementation-artifacts/spec-128-2-timer.md
  - _agile-output/implementation-artifacts/epic-128-nfr14.md
  - _agile-output/test-artifacts/atdd-checklist-128-2-timer.md
  - _agile-output/test-artifacts/128-2-code-fix-evidence.md
  - _bmad/tea/config.yaml
---

# Story128.2 自动化覆盖扩展

## 前置与适用性

Create / BMad-Integrated；backend Rust/Cargo现有test harness与真实Icarus/SBY/JVM齐全。范围为Timer及新增emitter修复；无HTTP服务、数据库、浏览器或Pact消费者/提供者边界。工具清单单次probe为空，pact_mcp_reachable=false，fallback=none；无broker调用，不重试。JS工具旗标true但mandate语言/runner相关性门不成立，不引入Playwright/Pact依赖或虚构endpoint。

已读取story/spec/NFR14/ATDD与全套fr197_timer测试，Cargo/CI/runner。核心test-levels、priorities、data-factories、ci-burn-in、test-quality及mandate/pact-mcp片段本会话此前全文已读，13份文件当前SHA均一致，见128-2-automate-knowledge-sha.json；selective-testing本步全文读取。采用现有确定性独立Trace工厂、P0/P1名称及相关选择执行，最后按用户要求单独完整clean回归。

## AC覆盖与补强计划

|合同|现有覆盖|本步动作|
|---|---|---|
|AC1/2公开API、单session/地址/诊断|7 API active与两个pair定义图、同源C/MD/原文例子|复核已有覆盖，避免重复生成API测试|
|AC3–6计数、mask、事件、响应、reset|3seed共19831帧/每后端；formal安全+10cover+3故障反例|新增P0直接RTL黄金检错控制：错误有效mask门控与背压响应损坏两类真实DUT变异，必须被现有独立Trace/checker捕获|
|AC7适用后端/来源/持续CI|单Timer及两pair图三后端、原始综合、身份优化模式负例、numeric矩阵|复用已执行证据；不重新制造同风险测试或声称native层级支持|

新增目标是直接RTL scoreboard的负控制，区别于已有formal observer变异；原始控制必须PASS，变异必须实际编译成功、仿真出现明确字段断言失败，timeout/编译失败不算检出。保留原始日志和VCD，短确定性定向Trace即可，不重跑全随机三seed证明同一负控制。固定宽度/接口/描述/API不变。

Worker A：HTTP API不存在，核实公共入口已有覆盖并按schema返回零新增，不制造无关测试。Worker B-backend：仅为fr197_timer.rs生成上述P0测试完整文件JSON，不运行、不落产品文件。主代理aggregate写入/测试/归档。

执行模式：requested auto，probe true；collaboration支持subagent，无独立agent-team API，resolved subagent。两worker分工独立，保持schema与输出路径；不臆测并行加速比例。

## 生成与聚合

两worker成功：API零新增（已有7 active + 1专用backend覆盖），backend新增1个P0 active integration测试，复用既有Trace/Input/testbench，不新建fixture。主代理逐字审阅追加内容并验证原文件完整保留后落盘；三组RTL共用相同tb，明确检查原始PASS/两个指定周期字段FATAL。生成JSON另存128-2-automate-worker-*；未运行测试，不虚构性能加速。下一步验证。

## 验证与完成

新增P0测试实际1 passed/0 failed/0 ignored；编译1.31秒、测试0.16秒，命令/UTC见128-2-automate-validation.json。原始RTL exit0/PASS；两变异均编译exit0，仿真exit1，分别命中指定cycle/phase的match_event与rdata FATAL。三份tb完全相同，所有波形与退出码归档128-2-automate-raw.tar.gz及manifest，clean后仍可审计。未改产品逻辑或公共接口。

清单复核：Cargo框架、AC映射、P0命名、隔离目录/相同黄金、确定性显式时钟、工具timeout、真实故障拒绝、旧测试保留均满足；无重试掩盖失败，无未执行算PASS。硬件边界采用固定向量/Trace工厂，faker/HTTP/browser/auth/DB/TypeScript项N/A；仿真#1是时钟采样时序，不是墙钟等待。为复用私有黄金，追加现有测试文件；文件较长为已知维护代价，不另复制参考模型。保留诊断产物用于审计而不是立即清除，全量clean后由归档持有。不虚构line coverage百分比、随机稳定性或并行加速。没有浏览器session、Pact调用、新依赖或额外API测试。

运行新增测试：`cargo test --locked -p bitloom --test fr197_timer p0_timer_direct_scoreboard_rejects_mask_and_stalled_data_mutations -- --exact --nocapture`，PATH需Icarus/vvp/GNU timeout。常规workspace/CI自动包含该active测试，无需新增CI分支。现有专用ignored后端/formal仍按runner/CI明确调用。

Playwright Utils deviations: None（Rust/Cargo范围不适用）。Pact.js Utils deviations: None（未生成contract artifacts、无消费者/提供者边界）。没有未接入的RECOMMENDED JS utility。

第五步完成。下一步按用户要求实际cargo clean、cargo fmt --all、just test，再检查最终证据与单故事提交；完整回归尚未开始，不将本步当作故事done。
