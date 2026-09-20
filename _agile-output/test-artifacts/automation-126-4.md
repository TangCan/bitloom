---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-identify-targets', 'step-03-generate-tests', 'step-03c-aggregate', 'step-04-validate-and-summarize']
lastStep: 'step-04-validate-and-summarize'
lastSaved: '2026-09-20'
storyId: '126.4'
mode: 'Create'
inputDocuments: ['AGENTS.md', '_bmad/tea/config.yaml', '_agile-output/implementation-artifacts/126-4-参数-fifo-与-m1-关闭.md', '_agile-output/implementation-artifacts/spec-126-4-param-sync-fifo.md', '_agile-output/implementation-artifacts/epic-126-context.md', '_agile-output/test-artifacts/atdd-checklist-126-4-参数-fifo-与-m1-关闭.md', '_agile-output/test-artifacts/126-4-independent-review.md', '_agile-output/test-artifacts/126-4-code-review-source-snapshot.json', 'Cargo.toml', 'crates/bitloom/tests/fr195_param_sync_fifo.rs', 'crates/bitloom/tests/fr195_param_sync_fifo_formal.rs']
---
# Story126.4 自动化覆盖核验

## Activation / 步骤1

完整调用 bmad-testarch-automate，Create，BMad-Integrated。resolver workflow成功：prepend/append/persistent_facts均空；on_complete为空。Richard，communication_language=Chinese。用户七步连续执行授权涵盖输入确认。输出按本故事隔离为automation-126-4.md，保留其他故事automation-summary。

Rust Cargo backend；Cargo.toml及crates/bitloom/tests提供现有框架。该故事无浏览器、HTTP、数据库、移动或独立服务边界。已读knowledge index及library-integration、Playwright/Pact mandates、pact-mcp、test-levels-framework、test-priorities-matrix、data-factories、selective-testing、ci-burn-in、test-quality。采用独立模型、确定种子、隔离产物、风险分层、严格工具退出及证据保全。Playwright/Pact开关true，但JS runner/package作用域不成立，按mandate scope discipline不引入JS依赖及附属框架。API profile接口非HTTP，相关认证/端点/selector知识不适用。一次工具列表probe：pact_mcp_reachable=false，无broker调用；不存在provider map需求。

产品及测试在前阶段已完成，独立review已关闭适用发现；本阶段只核查真正覆盖缺口、实际运行、归档当前源码绑定的证据，不冒充ATDD首红或再次完成独立review。

## 步骤2：覆盖计划

|合同|优先级/层次|已有覆盖与本次动作|
|---|---|---|
|AC1/2/5|P0 Rust API|默认32×4、精确端口、共享定义复用、完整WIDTH/DEPTH冲突、非法参数及恢复；核查负例与源快照|
|AC3/4/6|P0 native + 真实RTL|24配置×3固定seed，独立VecDeque，空/满/中间传输、背压、取消控制×占用矩阵、epoch守恒及bit63；重新执行并审计72组唯一命中|
|AC7|P0 RTL集成/P1 native拒绝|三实例共享reset、独立flush、两同参数复用+专门化；重跑层级与明确拒绝|
|AC8|P0 形式/综合|W1 D1/2/3归纳安全+非空cover，W1D1/W8D3/W64D16原始RTL综合；显式--ignored重跑并保全proof与cells|
|AC9|P0兼容|31项target内旧FIFO真实RTL；前阶段旧baseline/dual-model/prelude通过，最终workspace由第6步执行|
|AC10|人工证据门禁|记录本阶段，不提前更改M1/FR195状态，最终clean回归和提交由主代理执行|

检视ATDD与四路独立review后未发现新的高价值覆盖缺口；不复制已有测试或增加非合同的深度矩阵来凑数。待worker独立核查。测试分层分别检验Rust语义、emit/RTL执行、无界小配置归纳及可综合性，不把层次结果互相替代。

## 步骤3 / 3C：独立worker与聚合

能力检查：tea_execution_mode=auto，tea_capability_probe=true；supports subagent=true / agent-team=false，resolved=subagent。API与backend两worker完整执行对应步骤文件；浏览器/mobile不适用。唯一temp后缀126-4-2026-09-20，两个worker JSON与aggregate summary保存；worker JSON另归档到本故事证据目录。两个success=true，均未发现新的重要合同覆盖缺口；新增测试0、新fixture0、修改测试文件0。已有31 API/native/RTL + 6 formal/synthesis项保持。现有trace工厂、observer和PID隔离目录足够，无需新增基础设施。未测量并行加速，不采用模板的40–70%宣称。

API worker确认AC1–7/9映射充分；backend worker确认AC8真实RTL独立observer、仅端口协议/初始reset假设、原始RTL综合、严格PASS和timeout均存在。当前源码SHA256逐项等于code-review快照。Playwright Utils deviations与Pact.js Utils deviations均空（不在适用作用域），无provider端点、认证或网络fixture。

## 步骤4：实际验证与checklist

2026-09-20，以下命令全部exit=0；PATH=/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin:$PATH，CARGO_PROFILE_TEST_OPT_LEVEL=1。完整命令、版本输出、72矩阵命中、stdout/stderr和返回值见126-4-automation-run.log；逐命令用时见126-4-automation-commands.json。

```sh
rustc --version
cargo --version
iverilog -V
vvp -V
sby --version
yosys -V
z3 -version
timeout --version
cargo test -p bitloom --test fr195_param_sync_fifo -- --nocapture
cargo test -p bitloom --test fr195_param_sync_fifo_formal -- --ignored --nocapture
git diff --check
```

31 API/native/RTL通过，0失败/忽略，命令3.282秒；6 formal/synthesis通过，0失败/忽略，命令13.451秒。主矩阵24配置×3种子全部真实执行两native引擎与Icarus/vvp。种子0x12641950a551/0xdeadbeef8012/0x73592401ffff；额外3条trace用于层级，不能把75条日志当75个矩阵配置。80个本轮产物目录=72单配置RTL+层级+旧FIFO+3形式+3综合；不存在历史目录混入。

真实工具版本：rustc/cargo1.97.1；Icarus/vvp12.0 stable；SBY yosys-0.47；Yosys0.33(git2584903a060)；Z3 4.8.12；GNU timeout9.4。W1D1/D2/D3的prove与cover均status PASS。综合只读原始design.v：W1D1=7 cells/2FF，W8D3=66 cells/26FF，W64D16=2104 cells/1029FF；严格check -assert、cell白名单及多驱动检查通过。原始RTL、独立observer、SBY文件、求解器日志、归纳证明、cover VCD见证及综合JSON均归档；不含可重建的simulation二进制。

- [x] Rust框架、BMad输入、AC1–10映射、优先级和现有ATDD/review已核对；无需新测试或fixture。
- [x] 数据工厂固定种子、硬件合法边界为有意数据；每项配置/PID独立产物。显式工具超时、非零/缺失/UNKNOWN均失败，无静默skip。formal专用ignore说明仍真实，已显式运行全部6项。
- [x] 断言检验实际DUT，队列模型独立；先前真实RTL/formal故障注入失败仍见126-4-negative-controls.log，本阶段没有重做或冒称新增负控。
- [x] 本轮既有测试37项通过；数据完整性P0=35，P1 native层级拒绝=1，独立专门化API=1（现有名称无p前缀，按P1管理）；新增=0。未测量源代码行覆盖百分比。
- [x] 参数化辅助函数中的确定条件与VecDeque建模必要，不适用浏览器“无条件分支”规则。RTL #1为仿真事件时序，不是墙钟sleep。多断言服务同一时序合同；不按JS一断言模板拆碎测试。
- [x] 无浏览器会话/外部数据需清理；工具进程均退出。target外归档用于可审计证据，不删除。temp worker JSON与summary已复制到test-artifacts。
- [x] auto_validate=true；auto_heal_failures默认false；0失败，无healing、fixme、重试或降低断言。无新代码需要格式化，git diff --check退出0。
- [x] checklist中JS package、HTTP状态、JWT、selector、网络、Pact、移动、Playwright fixture项N/A；Cargo测试布局和现有docs/ip/param-sync-fifo.md命令适用，无README/CI修改必要。

### Playwright Utils deviations

None（Rust Cargo runner，无推荐utility未接线）。

### Pact.js Utils deviations

None（无consumer/provider边界，未生成contract artifacts）。

## 证据、范围与交接

新增文件仅本故事automation证据：automation-126-4.md，126-4-automation-run.log，126-4-automation-commands.json，126-4-automation-source-snapshot.json，126-4-automation-evidence-manifest.json，126-4-automation-rtl-proof.tar.gz，以及api/backend worker JSON与worker-summary JSON。源码snapshot与此前独立review一致；archive SHA256与732文件逐项SHA256已解包流读取核验，无数据缺失。manifest含80目录、72唯一矩阵组、6proof/cover状态、3综合统计及全部文件哈希。

本阶段区别：ATDD保留缺API首红，build/独立review保留各自历史结果；automate是在独立review修补后的同一源码上重新实跑并审计覆盖，未复制历史PASS为本轮结果。没有修改产品、测试、CI、故事、规划或状态，没有commit/cargo clean。

边界仍为正式24配置有限仿真；形式证明仅W1D1/2/3，32位累计计数为模计数，容量与槽队列等式另有精确断言；初始同步reset及合法受阻生产者为假设，未假设最终ready，未声称活性。综合仅三代表配置结构与cells，不是PPA、BRAM或所有参数证明。native层级仍明确unsupported。

下一步按用户七步第6步由主代理cargo clean→fmt→全workspace回归，再第7步真实闭合M1并单故事提交；本记录不替代这些门禁，不提前宣布M1完成。FR189 deferred/NFR91保持。

完成钩子：实际运行resolver --key workflow.on_complete，exit=0，返回空字符串；按技能跳过hook，正常完成automate。
