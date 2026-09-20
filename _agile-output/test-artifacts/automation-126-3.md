---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-identify-targets', 'step-03-generate-tests', 'step-03c-aggregate', 'step-04-validate-and-summarize']
lastStep: 'step-04-validate-and-summarize'
lastSaved: '2026-09-20'
inputDocuments:
  - AGENTS.md
  - _bmad/tea/config.yaml
  - _agile-output/implementation-artifacts/126-3-两槽-ready-valid-注册切片.md
  - _agile-output/implementation-artifacts/spec-126-3-rv-reg-slice.md
  - _agile-output/test-artifacts/atdd-checklist-126-3-两槽-ready-valid-注册切片.md
  - _agile-output/test-artifacts/126-3-build-evidence.md
  - _agile-output/test-artifacts/126-3-review2-tools.md
---

# Story126.3 自动化扩充

## 激活与步骤1：前置及适用性

Richard，本轮采用 Create、BMad-Integrated。通过 resolver 获取 workflow：prepend/append/persistent_facts 均为空，on_complete 为空。按用户七步授权执行第五步，不提交、不清理、不更新故事状态。

Cargo.toml 和 crates/bitloom/Cargo.toml 为现成 Rust 集成测试框架；检测 backend（Rust 硬件生成器），已有两套 fr195 测试。UI、HTTP endpoint、数据库、微服务、移动端及Pact消费/提供方边界均不属于本故事。

知识：已读 tea-index、library-integration-mandate、playwright-utils-mandate、pactjs-utils-mandate、pact-mcp，以及 test-levels-framework、test-priorities-matrix、data-factories、selective-testing、ci-burn-in、test-quality、evidence-integrity。采用独立模型、明确断言、可复现数据、按风险选层、严格错误与证据隔离原则。Playwright/Pact 的 true 标志不能把 Rust runner 变成 JS/TS runner；不加载无关实现模板、不生成浏览器/HTTP/Pact脚手架。工具列表只探测一次：pact_mcp_reachable=false，fallback_source=none，无broker调用，无虚构provider state。

## 步骤2：覆盖计划

| AC/风险 | 现有证据 | 本轮动作 |
|---|---|---|
| AC1 API、宽度1..64、身份 | 默认宽度、非法0/65/MAX、共享定义、异宽共存 | 审计，避免复制 |
| AC2/3/4 顺序守恒、取消、背压、注册隔离 | 四宽×三种子×两个native引擎+真实RTL；独立VecDeque；端口前后沿探测 | 保留，不扩随机数量 |
| AC5 双实例 | 独立种子和错位11拍的真实RTL | 保留 |
| AC6 安全归纳、非空性 | WIDTH1/8 prove与满后空/同时收发cover、结构负例 | P1补满槽reset/flush后重新接收的两个cover，不添加内部假设 |
| 工具严格失败 | FR119实际FAIL/trace与缺工具；runner timeout/status检查 | 独立审计，必要时补薄弱点 |
| AC7 | 七步流程最终证据 | 本轮只完成automate，clean全回归与commit由主代理继续 |

测试层为Rust API集成/真实RTL/形式工具，新增cover专注取消恢复可达性，避免重复行为矩阵和产品代码镜像。

## 步骤3：生成与聚合

能力探测：具备subagent，无独立agent-team启动接口；auto解析为subagent。无HTTP任务的API worker在协调端输出空结果（适用性审计），backend worker并行生成两个P1 cover的完整文件JSON。已检查两个success、完整schema，聚合落地一个修改文件，未新增Rust测试函数、fixture或helper。worker及汇总JSON归档126-3-automation-evidence；不把模板40–70%加速当实测。

两个cover均是正向合取：上拍满槽且取消，本拍为空、无reset/flush且有真实push。flush路径明确上拍!rst，reset路径允许flush重叠。没有添加假设或改变既有安全断言；这是非空性补强，不是发现产品缺陷。

## 步骤4：实际验证与质量清单

2026-09-20，PATH=`/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin:$PATH`，`CARGO_PROFILE_TEST_OPT_LEVEL=1`，Rust1.97.1。

```sh
cargo test -p bitloom --test fr195_rv_reg_slice --test fr195_rv_reg_slice_formal --test fr119_symbiyosys_smt_path -- --include-ignored --nocapture
rustfmt --edition 2024 --check crates/bitloom/tests/fr195_rv_reg_slice_formal.rs
git diff --check
```

实际退出均为0。联合回归20项通过：FR119 8项、FR195 native/RTL 9项、FR195 formal 3项（结构检查1、真实宽1/8各1）；无忽略项、无失败。日志：`126-3-automation-regression.log`。结构检查负例及native层级明确拒绝产生的预期panic被各测试精确处理，不是产品失败。

每宽四条cover全部由真实smtbmc命中：单槽同时收发step3，新增满槽reset恢复step5，新增满槽flush恢复step5，既有满后排空step6。新增属性对应生成design_formal.v的99与101行；SBY源范围分别97.72–99.88与99.89–101.98。每宽basecase及temporal induction同时PASS，保留原无限安全证明。新cover仅证明存在恢复路径，不声称无限活性或全状态覆盖百分比。

正式产物原路径为`target/fr195-formal/width{1,8}-3041368`；最终design.v/design_formal.v、harness、slice.sby、版本与命令、prove/cover/structure日志、status、真实VCD/YW见证均归档`126-3-automation-evidence/formal-width{1,8}`，随后cargo clean不会丢失证据。

工具审计：FR195 runner对非零、缺工具、timeout均assert失败，status严格只接收PASS；FR119保留真实FAIL+trace验证与强制缺工具负例。默认workspace的formal ignore由专用CI `--ignored`强制入口补足；此次使用`--include-ignored`实际执行。未引入控制脚手架工具替身，也不把结构JSON合成负例视为硬件证明。本轮未修改工具脚本或CI，保留现有强制入口及失败上传。

- [x] 现成Cargo框架、故事/spec/ATDD已加载，AC映射与P1新增范围明确。
- [x] 只增加两个形式属性，复用已有测试入口，无重复行为测试、新fixture或helper。
- [x] 新cover非空满槽前提，reset/flush分开且随后真实push；原assert与assume完整保留。
- [x] 固定种子、独立队列及进程目录隔离保留，真实工具时限保留。
- [x] 真实定向执行、计数核对、格式和diff检查通过；没有自动heal、放宽期望或新增ignore。
- [x] workflow步骤文件、worker JSON及统计有记录；浏览器/HTTP/JS/Pact/faker/页面selector/数据库cleanup/package.json修改均N/A。
- [x] 无浏览器会话，无需关闭；工具证据保留供审阅，不把保留日志当资源泄漏。

### Playwright Utils deviations

None。Rust runner不在mandate作用域，没有需要接线的推荐Web utility。

### Pact.js Utils deviations

None。没有消费/提供方边界，未生成合同脚手架。

## 变更与交接

代码变更仅`crates/bitloom/tests/fr195_rv_reg_slice_formal.rs`增加四行（两条P1 cover和注释）。新增本报告、`126-3-automation-regression.log`及`126-3-automation-evidence/`。既有文档中的两cover结果属于历史证据，本报告记录最新四cover结果，未回写审查为产品缺陷。

本轮本地Linux结果不是远端CI实跑声明；未执行全workspace或clean，也未提交、发布或push。没有本步骤未决产品缺陷。下一步按用户七步继续clean/fmt/全回归，然后一故事一提交；不需重复启动ATDD或test-review，不提前关闭Story126.3、FR195或M1。

完成钩子：已运行 resolver `--key workflow.on_complete`，返回空字符串，按技能跳过hook并正常结束。
