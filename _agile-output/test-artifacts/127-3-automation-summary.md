---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-identify-targets', 'step-03-generate-tests', 'step-03c-aggregate', 'step-04-validate-and-summarize']
lastStep: 'step-04-validate-and-summarize'
lastSaved: '2026-09-21'
storyId: '127.3'
inputDocuments:
  - _agile-output/implementation-artifacts/127-3-axi-lite-csr-桥.md
  - _agile-output/implementation-artifacts/spec-127-3-axi-lite-csr-bridge.md
  - _agile-output/test-artifacts/127-3-independent-review.md
  - _agile-output/test-artifacts/127-3-independent-fix-evidence.md
  - _agile-output/test-artifacts/atdd-checklist-127-3-axi-lite-csr-桥.md
  - _bmad/tea/config.yaml
---
# Story127.3 Automation 扩展

Richard，本轮按已授权第5步Create运行，仅该桥故事；现有Cargo/native/真实RTL/SBY测试框架完备，detected_stack=backend，BMad-Integrated。activation由根代理完成并移交，prepend/append/persistent facts为空。产品API/RTL无改动需求；不clean/commit/push或关闭story。

已加载story/spec/ATDD/独立review与修补实测记录，检查现有测试和fixtures；知识使用test-levels-framework、test-priorities-matrix、data-factories、selective-testing、ci-burn-in、test-quality、library/Playwright/Pact mandates。前轮同上下文已完整读取且未变更的data-factories/test-quality继续沿用。JS工具profile按mandate范围不适用Rust runner，不引入Web依赖。Pact MCP仅工具列表探测一次，pact_mcp_reachable=false；无broker调用/无Pact消费者服务边界，不生成合同服务测试。无浏览器或UI探索。

## AC与现有覆盖审计

| AC | 已有覆盖 | 本轮选择 |
|---|---|---|
|1|固定端口/共享定义/双实例/诊断/层级明确拒绝|不重复|
|2|raw全部偏斜/16WSTRB/8PROT，独立并发生产者，真实配对|不重复大矩阵|
|3|锁定、轮转、受阻与有界offer义务；formal及RTL变异|不重复|
|4|00/10/11，B/R独立，延迟CSR完成；formal已放开失败写rdata|补合法失败写非零无关rdata的真实可达cover见证，避免环境修复没有可达证据|
|5|8种reset竞争沿/真实leaf共同reset；原始结构FF时钟核对|不重复|
|6|真实RW/RO/WO/W1C、提交快照、反值字节、事件碰撞|真实SV producer monitor新增但未有其自身负例，补敏感性|
|7|16seed×1000及并发seed，proof/cover/三mutant，8拍尾段|选择验证监测器负例；不重跑不相关大矩阵来堆数量|
|8|兼容/例/API/CI已存在|最终clean/fmt/workspace归根代理第6步|

## 选择与优先级

- 127.3-FORMAL-006 / P0：只增加cover见证，CSR写错误2/3带非零rdata消费并正确B消费；用现有port-only observer，不增加安全假设，见证flags仅使用端口；共享formal源码保留原有已证明的对应关系assertion，未将其作为assumption。
- 127.3-RTL-013 / P0：提取真实组合所用SV hold monitor为同源片段，实际Icarus验证AW/W/AR/CSRrsp撤valid与改payload（含接受沿），合法保持/接受/reset正例；必须识别精确fatal，compile/tool失败不能算kill。

证据等级：既有13普通/3专用是前轮实际结果；本轮新增结果未运行前均为pending。其他AC已完整实测，不制造重复测试；随机不是穷举、B/R安全不等于公平活性。

## Step03 execution context

requested=auto，capability_probe=true；会话有spawn_agent，独立agent-team API缺失，因此resolved=subagent。A/formal与B-backend/RTL-monitor同时派发，分别只修改formal文件、integration/tools及同目录支持文件，无重叠编辑。UI/mobile workers不适用。性能收益未测量，不填40–70%估计。工具版本实际exit0已存127-3-automate-tool-versions.json。

## Step03C 聚合

两路success=true，JSON有效且全部内容与落盘源逐字相等。共新增1个普通Rust入口（P0），扩展1个既有formal入口的2条P0 cover；不能把两个cover算作两个Rust测试。新monitor入口含24负例+8正例。全桥现有17入口：14普通、3专用ignored。生成1份共享SV monitor和1个精确fatal runner helper，复用其余fixtures；无新HTTP mocks、随机factory或JS合并fixture。错误路由见证最终按P0数据完整性归类。

按父任务明确授权，worker在生成后已执行本身影响范围的真实检查；聚合阶段只核对记录，不重新生成代码/运行测试。下一步统一复核实际执行证据与checklist。

## 文件和实际执行

| 文件 | 改动 |
|---|---|
|`crates/bitloom/tests/fr196_axi_lite_csr_formal.rs`|两条事务绑定的非零写错误数据cover；原有assume/assert表达式与独立review快照完全相同|
|`crates/bitloom/tests/fr196_axi_lite_csr/integration.rs`|共享monitor注入原有真实组合；新增一个32场景自检入口|
|`crates/bitloom/tests/fr196_axi_lite_csr/producer_monitor.sv`|从原有tick提取的同源四通道hold监测器，含接受沿/reset|
|`crates/bitloom/tests/fr196_axi_lite_csr/tools.rs`|小型纯monitor runner，预期负例必须vvp exit1、一个精确FATAL且无PASS标记；工具/编译/超时不会被算作kill|

最终三个定向cargo命令均exit0：

```bash
export PATH=/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin:$PATH
export CARGO_PROFILE_TEST_OPT_LEVEL=1 BITLOOM_REQUIRE_RTL=1 PYTHONDONTWRITEBYTECODE=1
cargo test -p bitloom --test fr196_axi_lite_csr p0_sv_producer_monitor -- --nocapture
cargo test -p bitloom --test fr196_axi_lite_csr p0_same_session_bridge_real_csr -- --nocapture
cargo test -p bitloom --test fr196_axi_lite_csr_formal p0_port_oracle_safety_induction_and_nonvacuous_covers -- --ignored --nocapture
```

- SV selftest 2.6235s：24次预期 `$fatal(1)` 精确诊断、8次合法保持/接受/reset通过，按1个Rust入口计数；这些是monitor自身敏感性，不是假称32次产品变异。
- 真实bridge+CsrBlock 0.2426s：177提交、176消费、B110/R64；差额是已明确的reset取消，无新副作用重复。
- Formal 27.2782s：prove8完整基例/归纳PASS，cover64共7项PASS。新trace0/1分别error2/3、非零rdata1，step3提交→step4 CSR消费→step5正确B消费，无reset夹断；`127-3-automate-formal-witnesses.json`记录完整见证。
- 初轮monitor测试也通过，但其纯monitor正例曾使用error01；随后将全部载荷初始化改为合法2、变异为3并复跑，不混称初轮是完整peer合法刺激。初轮日志/执行JSON原字节与33工具目录全部保留；最终33目录另列，总66目录。未发生非预期失败，没有重试掩盖失败。
- manager独立核对最终33目录的编译/运行exit及精确FATAL计数、formal原始status和7项cover，以及产品文件与独立review源码快照相同；`127-3-automate-manager-validation.json`保存结果。

## 技能checklist验证

- [x] 框架、故事8AC、ATDD与现有覆盖均识别；范围是两个具体缺口，未复制原有大矩阵。
- [x] 两独立worker按auto→subagent分工，JSON成功且生成内容等于实际文件；共享源/helper齐备。
- [x] 测试命名含P0，准备→驱动→可失败断言清晰，固定合法协议值可复现；逐周期检查和多负例循环是硬件协议需要，不套用Web单断言禁循环限制。
- [x] Rust类型真实编译、限定文件rustfmt edition2024和diff空白检查通过；SV实际Icarus运行，不以字符串存在替代敏感性。
- [x] 每场景/PID独立目录，工具进程有timeout且全部结束；无浏览器/DB/网络资源。工具目录保留为审计证据，清理由主代理归档验收后统一执行。
- [x] 所有新目标实际验证；无弱化断言、跳过或未修复测试。auto_validate=true；auto_heal默认false，未启用Web修复循环；合法数值fixture改进明示且保留初轮证据。
- [x] 命令、必要环境、优先级和新测试写法在本summary供维护者直接使用；已有Cargo目标/CI专用formal入口会自动包含补强，无新增package.json/scripts/README复制品。
- [x] 同上下文已读data-factories/test-quality与本技能副本SHA256相同，知识继续有效；fixture纯函数/共享片段原则已应用。coverage百分比未计算，不编造数字。
- [x] 无10轮burn-in宣称；本轮两个确定性monitor版本实际运行、最终真实RTL与formal一次完成。现有16seed×1000与并发矩阵证据属于前轮，完整workspace由根任务最终再跑。

## Playwright Utils deviations

None。Rust/SystemVerilog非Playwright runner且无对应包，mandate适用性明确排除；无需要补接的auth/HAR/Webhook依赖。

## Pact.js Utils deviations

None。无独立部署的服务消费者/提供者边界，未生成Pact产物；硬件CSR端口合同依据已批准故事和实际源码。

## 限制与交接

本轮cover证明存在合法轨迹，不增加无条件活性承诺；安全仍无CSR响应时限、B/R公平性假设。纯monitorprobe并不验证完整DUT，其同源片段同时重跑真实组合形成独立证据。此前BMC mutant/综合未因仅新增cover而重复运行，原验收保持，不能将本轮说成所有门禁重跑。

下一步按用户明确七步顺序交根代理执行实际clean/fmt/just test、必要专用门禁与单故事commit；不插入额外审批。story/sprint/goal不由本任务改写，M2/FR196整体未关闭，未推送/发布。

## 持久归档

- `_agile-output/test-artifacts/127-3-automate-source-snapshot.tar.gz`：7成员，30,572字节；包SHA及每成员bytes/SHA256见同名manifest，全部从tar回读核对。
- `_agile-output/test-artifacts/127-3-automate-tool-artifacts.tar.gz`：543成员，320,521字节；包SHA及每成员bytes/SHA256见同名manifest，全部从tar回读核对。

源码快照含当前根代理已更新的桥文档、未改产品和全部桥测试支持源。工具包保留初轮+最终monitor各33目录、proof/cover及7条VCD；仅省略可重建Icarus可执行文件，不删失败日志。原有127-3历史证据未覆盖。
