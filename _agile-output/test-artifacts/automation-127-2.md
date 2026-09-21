---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-identify-targets', 'step-03-generate-tests', 'step-03c-aggregate', 'step-04-validate-and-summarize']
lastStep: 'step-04-validate-and-summarize'
lastSaved: '2026-09-21'
inputDocuments:
  - AGENTS.md
  - _bmad/tea/config.yaml
  - _agile-output/implementation-artifacts/127-2-csr-描述-rtl-与软件地址产物.md
  - _agile-output/implementation-artifacts/spec-127-2-csr-product.md
  - _agile-output/test-artifacts/127-2-independent-review.md
  - _agile-output/test-artifacts/127-2-independent-fix-evidence.md
---
# Story127.2 自动化扩展

Create / BMad-Integrated。Rust Cargo backend-only硬件库，既有Rust integration tests、Icarus/vvp、SBY/Yosys/Z3脚手架可用。resolver prepend/append/persistent_facts/on_complete均空。遵循用户七步顺序；本步骤不改变story/sprint状态或提交。

配置utils flags=true，但无JS/TS Playwright runner或微服务consumer/provider边界，故Playwright/Pact/UI均不适用，不生成浏览器测试；Pact工具列表探测不可达，pact_mcp_reachable=false，fallback=none（无需broker）。

Confidence: 9。依据fr196_csr.rs已有独立producer monitor和多field黄金、fr196_csr_formal.rs端口observer；独立审查R4/R11解释实际回归风险。Unknowns：真实故障检出结果须执行后记录。

覆盖计划：P0硬件API仿真层补真实vvp producer withdrawal/payload违规负例和正例控制；P0 RTL oracle层补多field漏位/输出损坏突变；P0 backend integration层补真实SBY坏RTL的FAIL与反例，原RTL作为PASS控制。既有配置/codec行为已充分定向覆盖，不重复镜像实现。两worker分别独占functional/formal测试文件，manager合并验证与证据。

执行模式：requested=auto，probe=true；runtime collaboration spawn可用，独立agent-team机制无，resolved=subagent。UI/mobile workers不适用。硬件端口API映射worker A，形式工具集成映射worker B-backend。

聚合：两worker success=true，JSON完整content与live文件逐字节一致后落盘；增加3个P0测试（API仿真2，backend形式集成1）。run_rtl_source/automation_tb为文件内专属helper；无额外跨文件fixture。完整worker源码JSON仅暂存/tmp，最终源码tar保存不可变文件；持久worker JSON省去重复content。未测量并行加速比。

## 实施与覆盖结果

只修改 `crates/bitloom/tests/fr196_csr.rs` / `fr196_csr_formal.rs`，未修改产品实现、公开API、合同或CI。新增3个P0测试：

| 入口 | AC/风险 | 实际结果 |
|---|---|---|
| `p0_real_rtl_producer_monitor_faults_and_legal_cancellation_for_each_prefix` | AC3/7，R4：monitor可否抓住真实违约 | 无前缀/a_/b_分别撤回valid、改变write/addr/wdata/wstrb；15次vvp exit1，具体producer FATAL，无PASS标记。各前缀reset取消、保持至接受共6正控制exit0。 |
| `p0_real_rtl_multifield_missing_high_bit_mutant_is_caught_by_independent_oracle` | AC4/7，R11：多field数据完整性 | freshly emitted原版写/读黄金PASS；仅把control_value生成输出的bit31屏蔽，vvp exit1，`multifield-golden control_value expected=800000f1 got=000000f1`。 |
| `p0_csr_formal_rejects_lost_event_output_with_counterexample` | AC4/7：安全proof能否检出可达破坏 | 同一生成RTL原版prove PASS；仅将events_value公开输出改为0，observer与输入假设不变；SBY exit2、status `FAIL 2 1`，basecase step3失败命名性质 `csr_events_value_matches_reference`，真实非空VCD/Verilog/smtc/yw反例。 |

仿真新增23次实际执行（7正控制、16故障检出）；形式新增1正证明与1故障检出。测试数指Rust入口而非循环展开，不将23次仿真当23项测试。多field输出损坏仅证明该类故障可被oracle发现，不声称完整mutation score。

`run_rtl_source`复用原有process-group timeout，先要求版本探测和编译成功，再返回实际vvp status/log；既有run_rtl仍要求exit0和PASS。新负例断言exit1、准确fatal、无成功标记，不catch任意panic冒充通过。形式负例要求exit2、FAIL、指定basecase assertion与trace，ERROR/UNKNOWN/缺工具/超时均失败；新增formal保持带原因的ignore，专用CI现有全target `--ignored` 自动纳入。正常workspace不要求SBY。

## 定向命令与结果

完整argv、实际exit、耗时、环境见 [commands](127-2-automation-commands.json)，版本见 [environment](127-2-automation-environment.log)。PATH前置`/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin`，`CARGO_PROFILE_TEST_OPT_LEVEL=1`、`PYTHONDONTWRITEBYTECODE=1`。Rust/Cargo1.97.1、Icarus/vvp12.0、Yosys0.33、SBY yosys-0.47、Z3 4.8.12；没有工具pin升级。

| 命令 | 结果 |
|---|---|
| `cargo test -p bitloom --test fr196_csr --test fr196_csr_config -- --nocapture` | exit0；12行为+9配置PASS，0失败/ignored；23.422s；[log](127-2-automation-functional.log) |
| `cargo test -p bitloom --test fr196_csr_formal -- --ignored --nocapture` | exit0；4测试实际PASS；10.71s测试/11.731s命令；[log](127-2-automation-formal.log) |
| `cargo test -p bitloom-prelude csr` | exit0；3私有codec PASS；[log](127-2-automation-codec.log) |
| `rustfmt --edition 2024 --check crates/bitloom/tests/fr196_csr.rs crates/bitloom/tests/fr196_csr_formal.rs` | exit0，仅检查本轮两文件 |
| `git diff --check` | exit0 |

总28个定向Rust测试实际PASS，新增3项。原增强Probe safety depth16、step14归纳成功；cover depth24共5条全部到达（3条step2、2条step4）。Probe及五种singleton原始RTL综合成功，singleton只综合，不冒充安全proof。初始reset与producer保持输入假设保留，无rsp_ready公平性、无DUT输出正确性假设。没有新增非预期失败，没有修复或healing迭代；故意违约/突变的真实失败是负例结果，不能写成工具全exit0。

## 归档与完整性

- [源码tar](127-2-automation-source-snapshot.tar.gz)：19 members、60,296 bytes，SHA256 `5ed8a126274bbc5184426c5674fe1bb6bd8bc2fba9d2a817810c9c40bc571570`。涵盖CSR实现/测试、相关文档/CI、Cargo锁文件与固定工具pin、命令记录runner。成员摘要见[source members](127-2-automation-source-members.sha256)，逐成员从tar回读核验见[source verification](127-2-automation-source-verification.log)。不建立普通.rs/.py历史副本目录。
- [工具tar](127-2-automation-tool-artifacts.tar.gz)：44目录、397文件、2,173,885 bytes，SHA256 `691c771cc387144cffccdd426daff489bd5d53d41f2248baa12caae8ea7b1754`。仿真PID1115963、C消费者PID1117485、形式PID1119505；保存原RTL/突变RTL/TB/monitor、命令/版本/exit、C头/消费者、proof/cover/status/反例及原始综合JSON。仅省略可重建simulation二进制、.o及重复SBY model/src。工具成员摘要与回读核验独立保存。
- [API worker](127-2-automation-api-worker.json) / [backend worker](127-2-automation-backend-worker.json)保留结构化生成结果，完整源码保存在源码tar避免普通文本副本干扰检索；聚合摘要见[summary](127-2-automation-summary.json)。
- [本轮清单](127-2-automation-checklist.md) / [总SHA256](127-2-automation-sha256.txt)；所有历史build/review文件与旧manifest保持。

## 知识与清单适配

采用test-levels-framework/test-priorities-matrix选择P0硬件API与工具集成层；data-factories采用既有有效配置工厂和具名独立黄金（合同mask/地址必须固定，不能随机替换）；fixture-architecture采用纯helper与独立label/PID目录；selective-testing按两个相关target；ci-burn-in用于识别非确定性风险，没有发现重试/竞态，不为次数重复执行已通过测试。test-quality/evidence-integrity要求实际可失败断言、准确exit、原版控制与反例；confidence-gate证据如前。

Playwright Utils deviations: None（Rust Cargo runner不在mandate作用域）；Pact.js Utils deviations: None（无consumer/provider边界，无Pact产物）。已读取mandates/API-only资料用于相关性核验；没有推荐但未接入的auth/HAR/webhook依赖。无浏览器会话。默认auto_validate=true、auto_heal_failures=false、max_healing_iterations=3；本次无失败修补，不启用浏览器healing。

新test主体均短于1000行，沿用原hardware test文件聚合布局（整个文件超过1000行，非新增巨型单test）；真实工具产物按要求持久保留，不作fixture即时删除，其他测试不读取旧目录。Verilog #1为确定性模拟时间步进，非墙钟hard-wait；Rustmatch/if用于固定表驱动正负预期，不取决于运行时可选状态。每个新test检查单一风险，多条断言共同区分真实故障与工具错误。

## 局限与交接

形式结论限增强Probe；singleton仅结构综合。事件输出置零突变不等同穷尽W1C优先级所有坏实现；多field高位输出突变不等同所有字段/所有配置完备验证。Generated Rust standalone仍未验；无PPA、板级、无条件活性或全CI通过宣称。桥/decoder/外设产品不在此故事。

当前步骤已完成，下一步按用户顺序由主代理执行clean/fmt/旧回归与`just test`全workspace，然后单故事commit；本代理未运行clean/完整workspace/旧回归，未改story/sprint/spec/goal状态，未git add/commit/push/publish。最终resolver exit0，`workflow.on_complete`为空；[hook记录](127-2-automation-on-complete.log)，没有追加动作。
