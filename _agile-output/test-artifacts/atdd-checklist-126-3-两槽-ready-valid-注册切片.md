---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-generation-mode', 'step-03-test-strategy', 'step-04-generate-tests', 'step-04c-aggregate', 'step-05-validate-and-complete']
lastStep: 'step-05-validate-and-complete'
lastSaved: '2026-09-20'
storyId: '126.3'
storyKey: '126-3-两槽-ready-valid-注册切片'
storyFile: '_agile-output/implementation-artifacts/126-3-两槽-ready-valid-注册切片.md'
atddChecklistPath: '_agile-output/test-artifacts/atdd-checklist-126-3-两槽-ready-valid-注册切片.md'
generatedTestFiles:
  - crates/bitloom/tests/fr195_rv_reg_slice.rs
  - crates/bitloom/tests/fr195_rv_reg_slice_formal.rs
inputDocuments:
  - _bmad/tea/config.yaml
  - _agile-output/implementation-artifacts/126-3-两槽-ready-valid-注册切片.md
  - _agile-output/implementation-artifacts/epic-126-nfr14.md
  - docs/ip/phase24-contract.md
  - docs/ip/module-composition.md
  - _agile-output/test-artifacts/126-3-formal-tool-probe.md
  - crates/bitloom/Cargo.toml
  - crates/bitloom/tests/fr194_module_composition.rs
  - crates/bitloom/tests/fr193_axi_protocol.rs
---
# Story126.3 ATDD 验收清单

## Step 1 — 前置与适用性

Richard，本次按已授权的 Create 模式连续执行。Story ready-for-dev，126.1风险门禁与126.2模块定义前置done；Cargo集成测试/Sim/Icarus框架已存在。栈判定 backend（Rust硬件生成器）；检测依据 Cargo.toml 与 Rust tests。实际工具探针已通过，与组件验收严格区分。

读取核心知识 data-factories、component-tdd、test-quality、test-healing-patterns，后端 test-levels-framework、test-priorities-matrix、ci-burn-in。采用独立纯队列模型、固定种子/边界工厂、明确时钟推进、无隐式跳过、超时保留日志。

配置的 Playwright/Pact flags 均为true，但已读取两者mandate：本故事无JS/TS runner、HTTP或独立部署consumer/provider边界，适用性门不成立，不引入浏览器、JS或Pact。Pact MCP仅检查工具列表，pact_mcp_reachable=false，无broker请求，无推断provider状态。mobile不适用。无需用户重复确认既有七步授权。

## Step 2 — 生成方式

采用AI生成，从已批准AC与正式硬件合同推导验收；backend不使用录制。红阶段允许缺RvRegSlice API导致编译失败，必须保留可执行的真实行为断言，不能以工具故障作为产品红证据。

## Step 3 — 验收策略

| AC | 优先级/层级 | 场景与断言 |
|---|---|---|
|1|P0 Rust API集成|默认32位、1/8/32/64位；elaborate与同名helper HIR完全相同；参数登记WIDTH；同名同宽复用、异宽冲突；0/65/MAX无panic Diagnostics|
|2–4|P0 native两引擎|独立VecDeque黄金模型；沿前/沿后检查，取消epoch守恒，空/1/满/同时push-pop/满释放/长背压/reset/flush/重叠|
|3/5|P0 RTL集成|实际emit→Icarus→vvp，沿前所有输入变化后输出保持旧状态，沿后队列预期；四宽度三种子，高64位；双实例独立流|
|5|P1 边界|native层级明确unsupported，不伪造平铺模拟|
|6|P0 formal|实际生成RTL，WIDTH1/8；独立ghost队列安全断言，prove归纳与cover分开；无组合输入路径结构检查；缺工具/超时/FAIL/UNKNOWN非零|
|7|P1 交付证据|红日志、固定种子、版本、中文文档与FR142显式API登记；后续build/审阅/automate/全回归独立完成|

跨引擎/RTL复用的是独立合同向量，不复制实现公式。原生单模块与RTL层级各验证不同风险。首次红测源于缺API，不把框架/工具故障当行为失败；绝不添加ignore或空断言。形式验证只证明安全性，有限排空不宣称无限活性。

## Step 4 — 执行模式与红测生成

配置requestedMode=auto、probeEnabled=true。实际协作工具支持spawn_agent；无独立agent-team运行时入口，因此resolvedMode=subagent。API/native/RTL worker独立生成，编排代理处理后端适用性判断与formal专项。受可用槽位约束只启动一个有实际工作的子代理；无UI的4B在本地记录N/A及有效JSON，不创建空浏览器测试。并行收益未测量，不宣称模板的50%。

已激活用户当前故事任务。Rust测试没有JavaScript `test.skip()`；API/native/RTL保留活动测试以取得用户要求的真实红证据。formal采用明确ignore原因的独立工具门禁，必须以`--ignored`显式运行；其缺API仍在编译时失败。默认workspace中ignored不等于证明通过；正式验收和formal CI必须另外记录prove与cover。

形式测试 `crates/bitloom/tests/fr195_rv_reg_slice_formal.rs`：2个P0实例（WIDTH1/8），每个保存生成design.v、独立ghost队列harness.sv、SBY配置、工具版本/命令/结构JSON/prove与cover日志。Yosys追踪输出组合锥，只有同步edge FF截断追踪，reset/flush/ready/data/valid全部输入均纳入检查。安全断言不假设下游ready公平；只约束合法生产者保持。cover实际要求满后排空及占用1同时push/pop。

首次formal红命令：

```sh
env PATH=/tmp/bitloom-1263-sby-installed/bin:/tmp/bitloom-maintenance-tools/bin:$PATH cargo test -p bitloom --test fr195_rv_reg_slice_formal -- --ignored --nocapture
```

2026-09-20实际退出101，唯一error为`E0432 unresolved import bitloom_prelude::ip::RvRegSlice`。日志：`_agile-output/test-artifacts/126-3-atdd-formal-red.log`。这是缺产品API的首红；尚未进入RTL生成或形式工具，不能称组件形式证明已通过。真实工具探针见既有126-3-formal-tool-probe.md，不以该探针替代组件验证。

## 后续 GREEN 实现清单

- [ ] 用既有builder/HIR实现`RvRegSlice<const WIDTH:u32=32>`与共享`define_module`定义体，完整登记WIDTH；非法宽度返回Diagnostics。
- [ ] 两个寄存数据槽、寄存占用、同步reset > flush > 握手；状态输出隔离全部输入组合路径。
- [ ] 运行API/native/RTL目标，保持黄金队列独立；修实现缺陷，测试错误须明确解释，不放宽合同换绿。
- [ ] 运行专用formal `--ignored`目标；WIDTH1/8分别归纳prove与cover为PASS，保留结构追踪与见证。接入formal CI的显式执行命令，不能仅靠workspace默认测试。
- [ ] 文档逐符号追加FR142和中文使用说明；证明适用范围仅安全性，工具版本/种子/实际覆盖均记录。
- [ ] 继续用户七步的build→独立code-review→automate→clean/fmt/全回归→单故事commit；不由本ATDD任务修改sprint状态或提交。

计划估算沿用Epic126 NFR14：本故事2–3有效人日，非代理墙钟或完成承诺。Refactor只在相关验收为绿之后执行，保留两槽合同和历史SyncFifo行为。

## 工厂、夹具、模拟与界面要求

Rust帧/固定种子流工厂与独立队列在测试目标内；每次运行新建Sim与RTL工件目录，无跨测试共享状态。形式夹具由实际HIR生成design.v，独立harness只读公开端口，不窥探产品内部寄存名。日志保留是故障取证要求，非未清理数据库状态。无数据库、HTTP mock、认证夹具、data-testid、UI组件或浏览器会话，以上模板项均N/A，不创建Faker或Playwright依赖。

## Step 4C — 聚合结果

已验证两个worker JSON：API成功、E2E适用性N/A成功；归档在`126-3-atdd-api-tests.json`、`126-3-atdd-e2e-tests.json`，汇总`126-3-atdd-summary.json`。不保留仅依赖/tmp的交接资料。

| 文件 | 数量 | 覆盖 |
|---|---:|---|
|`crates/bitloom/tests/fr195_rv_reg_slice.rs`|8 tests|4个宽度×3个种子×2个native引擎+RTL；共享定义/default32/参数登记/端口；非法宽度；双实例真实RTL；native层级拒绝|
|`crates/bitloom/tests/fr195_rv_reg_slice_formal.rs`|2 tests|宽1与8的真实RTL归纳安全证明、cover与无输入组合路径结构检查|

固定种子：`0x12631950a551`、`0xdeadbeef8012`、`0x73592401ffff`。每条流420拍（最后20拍排空）；双实例第二路错位11拍，使用不同种子。黄金向量自检12组合实际命中38拍背压、44–45拍连续同时收发，accepted=245–249、delivered=238–242、cancelled=7，所有epoch守恒；见`126-3-atdd-vector-probe.log`。该自检只证明测试输入质量，不算产品功能pass。实际DUT须逐拍匹配队列预期，才能确认同样握手和覆盖。

八个API/仿真测试均为active，没有ignore或空断言。两个formal测试有明确专用工具ignore原因，在组件验收时显式`--ignored`运行。两个红目标都因缺产品API而未进入行为阶段。没有mock产品或替代RTL。

## Step 5 — 最终验证与交接

- [x] AC1–7已映射；前置、栈与框架满足；UI/HTTP/JS/Pact模板项已明确N/A。
- [x] 源文件已rustfmt；真实Rust集成测试命令可达编译器；无占位assert或工具缺失跳过。
- [x] 元数据/文件路径/红日志/知识适用性/固定种子/后续任务完整，story回链已登记。
- [x] API/native/RTL红测实际退出101；formal显式入口红测实际退出101；均唯一error E0432（`RvRegSlice`缺失）。不把向量自检或工具探针称作组件pass。
- [x] 产物归档至test-artifacts；未打开浏览器，因此无待关闭会话。
- [ ] GREEN尚未实现；实际组件native、RTL、formal结果与文档/CI仍待build及后续步骤。

真实API红命令（日志`126-3-atdd-api-red.log`）：

```sh
env PATH=/tmp/bitloom-maintenance-tools/bin:$PATH cargo test -p bitloom --test fr195_rv_reg_slice -- --nocapture
```

下一步是用户七步中的 **bmad-build**，依次实现上述GREEN清单，再执行code-review/automate/clean+fmt+全回归/单故事提交。本ATDD不改产品、不更新sprint-status、不提交，也不声称FR195或M1完成。首红只有缺API，因此GREEN阶段必须确认不存在被缺API遮蔽的测试/API接线错误；发现测试缺陷可修正，但须保留独立合同断言。
