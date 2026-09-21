---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-generation-mode', 'step-03-test-strategy', 'step-04-generate-tests', 'step-04a-subagent-api-failing', 'step-04b-subagent-e2e-failing', 'step-04c-aggregate', 'step-05-validate-and-complete']
lastStep: 'step-05-validate-and-complete'
lastSaved: '2026-09-21'
workflowType: 'testarch-atdd'
storyId: '127.2'
storyKey: '127-2-csr-描述-rtl-与软件地址产物'
storyFile: '_agile-output/implementation-artifacts/127-2-csr-描述-rtl-与软件地址产物.md'
atddChecklistPath: '_agile-output/test-artifacts/atdd-checklist-127-2-csr-描述-rtl-与软件地址产物.md'
generatedTestFiles:
  - crates/bitloom/tests/fr196_csr.rs
  - crates/bitloom/tests/fr196_csr_config.rs
  - crates/bitloom/tests/fr196_csr_formal.rs
inputDocuments:
  - AGENTS.md
  - _bmad/tea/config.yaml
  - _agile-output/implementation-artifacts/127-2-csr-描述-rtl-与软件地址产物.md
  - _agile-output/implementation-artifacts/epic-127-nfr14.md
  - docs/ip/phase24-contract.md
  - _agile-output/test-artifacts/127-2-atdd-api-contract.md
---

# Story127.2 CSR ATDD 清单

Richard；2026-09-21；baseline `77ea01392e5be74b9f2622009ad146edb2468ea9`。静态 Rust 配置同源生成 CSR leaf、地址 Markdown/C header；仅 FR196 描述/叶节点子集，不包含桥、全局译码或外设产品。预计4–6有效人日沿用故事，非代理墙钟承诺。

## 预检与执行模式

读取 activation SKILL/resolver/config；prepend/append/persistent_facts 全空。Create 为明确任务，既有七步授权覆盖重复确认。Rust backend；Cargo 和现成 integration 框架存在，127.1/126.2 done，故事 ready-for-dev、8条 AC 完整。参考 fr194_module_composition、fr195_param_sync_fifo 和 formal 工具 runner、builder/prelude 现有模式。

AI generation；无浏览器/HTTP/consumer-provider 边界，Playwright/Pact/TS fixture/data-testid/mock服务为 N/A，未添加 JS 依赖。已读两个 mandate 的相关性门槛。Pact MCP 工具列表单次探测无 SmartBear 工具，pact_mcp_reachable=false；此处没有 provider states，不请求 broker。测试知识采用 data-factories 的配置工厂/具名黄金常量、component-tdd 的先红后绿、test-quality/evidence-integrity 的真实可失败断言和隔离、test-healing 的根因分析、test-levels/priorities/ci-burn-in 的P0定向优先。网络/auth/recurse utils 无适用操作，不引入虚构服务。

能力探测：collaboration spawn/followup 可用，auto→subagent；A新worker，B因线程上限复用已完成127.1的worker，二者按固定契约独立生成。用户要求真实编译RED，优先于skill通用`test.skip()`模板：配置/行为全部 active；formal 两个测试遵循现有 FR195 CI 专用 `--ignored` 门禁布局（默认workspace job只装Icarus，不装SBY），其编译仍 active。ignore行不是证据，专用命令真实运行必需；绝不将缺工具当通过。

## AC 映射与测试层级

| AC | 优先级/层级 | 验收责任 |
|---|---|---|
| 1 | P0 Rust配置API | 合法边界、地址/mask/reset/权限/owner/event/name/generated collision全部拒绝 |
| 2 | P0 Rust API | 正规化与完整身份、E0244 poison、进入helper前失败可恢复；私有codec模块单测交build |
| 3 | P0 native+RTL | 沿前唯一提交、响应快照/背压/reset、计数守恒 |
| 4 | P0 native+RTL | 16WSTRB、权限/洞/未对齐/高位、W1C独立自然事件和set优先 |
| 5 | P0 native+层级RTL | RO动态/WO低8、reject、候选值无环、external RW唯一owner与同沿peer |
| 6 | P1 Rust+C11 | 独立黄金常量、三类确定性、单/双头严格编译 |
| 7 | P0真实工具 | Interpreter/Compiled、Icarus/vvp、共享session peer；独立CSR SBY prove/cover与原始RTL综合 |
| 8 | P1人工+回归 | prelude-only例、中文docs、FR142/SemVer minor、旧接口与七步闭合由build及后续阶段完成 |

## 固定 API / 数据与 fixture

唯一签名和端口依据：[API契约](127-2-atdd-api-contract.md)。配置为 host 数据，offset u32/mask u64可表达非法值；RW支持leaf/external，RO external、WO None、W1C leaf。候选值不依赖reject/commit；commit为沿前组合成功脉冲。C头local offset独立于RTL模块名。工厂构造完整有效Probe bank；所有关键地址/mask和期望手写，不调用DUT merge/decode。每测试独立session/模拟器；外部工具临时目录隔离并保留诊断产物，不污染产品文件。

## GREEN 实施清单（未实施）

- [ ] T2 实现契约5种公开类型/5个方法、各字段、完整校验与规范排序；private codec往返/schema/重复额外缺键/缺索引/count/枚举/byte>255与非法重建单元负测。
- [ ] T2 非捕获共享body，完整参数不摘要；同session复用、E0244及poison、预校验无半模块。
- [ ] T3 leaf时序/16WSTRB/事件/动态reject/external-owner端口；不新增HIR/native层级。
- [ ] T4 Markdown/C头与prelude-only示例、中文docs、FR142逐符号与SemVer minor。
- [ ] T5 执行全部native/RTL/层级peer与随机计数；修测试只能凭独立证明oracle错误。
- [ ] T5 在 `.github/workflows/ci.yml` 既有 `formal-sby` job 添加FR196专用 `--ignored` 命令与失败产物，保持默认workspace环境边界；ATDD不改CI。
- [ ] T5 独立formal prove与cover均真实PASS；原始RTL综合check，无observer污染；缺工具恢复固定环境并重跑。
- [ ] T6 code-review→automate→clean/fmt/just test→单故事commit，主代理状态更新；FR196/M2/Phase24不得关闭。

## 执行命令与证据边界

```bash
cargo test -p bitloom --test fr196_csr_config -- --nocapture
cargo test -p bitloom --test fr196_csr -- --nocapture
cargo test -p bitloom --test fr196_csr_formal -- --ignored --nocapture
```

formal 为专用实际工具测试；需要timeout/yosys/sby/z3，行为RTL需要iverilog/vvp，软件需要cc。无headless/headed/浏览器debug操作。GREEN后再refactor，保持黄金值/AC，最后由主流程回归；ATDD不clean、不commit、不改sprint。

## 聚合与红阶段结果

两worker success=true，结构化结果已存 [API worker](127-2-atdd-api-worker.json) / [行为 worker](127-2-atdd-behavior-worker.json)。4C聚合检查所有测试用固定契约、独立期望、无placeholder pass或工具缺失绕过；支持fixture已内联函数完成，未新增服务mock。

| 文件 | tests / 行数 | 当前状态 |
|---|---|---|
| `crates/bitloom/tests/fr196_csr_config.rs` | 6 / 503 | active；配置、所有emit拒绝、复用/poison、确定性、MD/C11黄金 |
| `crates/bitloom/tests/fr196_csr.rs` | 4 / 1044 | active；独立逐拍oracle、native双引擎/真实RTL叶、真实peer、层级拒绝、external保留位 |
| `crates/bitloom/tests/fr196_csr_formal.rs` | 2 / 277 | 专用--ignored；真实证明/cover及原始综合runner骨架，编译参与RED |

测试框架定义总数12；本阶段未有任何CSR产品测试执行PASS。行为文件含协议oracle、向量生成、native/Icarus适配、peer构造及四个入口；各测试拥有独立状态。formal超时180s、RTL/C11超时60s均硬失败；不以通用1.5min建议削弱proof。临时工具文件保留在按测试标签+PID隔离的target子目录，作为失败证据而非共享状态；不删除有诊断价值的产物。

最终真实命令：

```bash
cargo test -p bitloom --test fr196_csr_config --test fr196_csr --test fr196_csr_formal --no-run
```

**exit101，仅3个E0432**，分别缺少 `CsrAccess/CsrBlock/CsrField/CsrOwner/CsrRegister` 公共API。完整[最终红日志](127-2-atdd-final-red.log)；[环境](127-2-atdd-environment.log)确认baseline与Rust/Cargo1.97.1。首次config/formal红的glob enum衍生诊断保留在[初次日志](127-2-atdd-config-formal-red.log)，限定variant后[定向最终日志](127-2-atdd-config-formal-final-red.log)只剩2个E0432。测试自身 scoped rustfmt --check exit0见[格式日志](127-2-atdd-format.log)；不是最后clean/fmt/regression阶段。

独立提取的[oracle probe源](127-2-atdd-oracle-probe.rs)及[日志](127-2-atdd-oracle-probe.log)只验证向量/scoreboard本身可运行：三leaf seed和一peer seed共2310/2311/2311/2311帧，全部16WSTRB、31拍停顿、取消/接受/消费守恒断言通过；peer拒绝17、允许high候选且old-even9、成功partial83。**没有执行DUT，不能记作native或RTL通过**。peer reject采用candidate[31] && current[0]，沿前/沿后oracle与真实peer均遵守此独立策略。

## 剩余验证与人工检查

- [ ] 真实native两引擎、Icarus叶和三模块同session peer、C11单/双头全部运行；保存版本、seed、接受/提交/响应/取消计数。
- [ ] formal初始reset假设明确，无rsp_ready公平性；prove16与cover24分开报告。observer只用端口参考状态；若需归纳加强，只能增加可证明assert，不能假设DUT正确。4个cover为成功提交、错误提交、当前响应曾背压后恢复、W1C同拍清/set冲突。
- [ ] 原始 `design.v` 综合（不含observer），Yosys hierarchy/proc/check/synth/check完成，有触发器、无latch；无PPA/BRAM结论。
- [ ] private codec所有畸形键/schema/count/index/enum/byte负例由build module tests补齐，不扩公共API。
- [ ] prelude-only可编译用户示例、中文文档、FR142逐符号表面与minor说明、旧M0/M1回归。
- [ ] AC8后续七步闭合仅127.2 done，Epic127 in-progress；桥/decoder/外设未交付，FR189 deferred、NFR91保留。

RED→GREEN下一步为bmad-build，不需再次激活普通测试（已active）。专用formal必须显式--ignored实跑；编译RED不等于行为RED、bounded cover不等于proof、任何not-run均不得变成PASS。故事已追加ATDD/API回链，原8项AC/范围和状态未改。本阶段无产品代码、无sprint/goal修改、无clean/commit/push/publish。

## 完成校验

已按skill checklist核对前置、故事元数据、8项AC层级/优先级、独立黄金、两worker结果、fixture/工具隔离、真实编译红、范围诚实与下游实施任务。Browser/HTTP/auth/mock/data-testid/JS相关项均N/A；无CLI浏览器遗留进程。on_complete最终resolver返回空字符串，无追加终端动作，正常结束。
