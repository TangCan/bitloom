# Phase24 结项补验（2026-09-23）

本记录属于独立结项维护提交，保留原22个单故事提交。产品基线为`4bdd680e1a6db7e9ca77e6d4fcd7caa4b825d2b1`；本次仅修129.3验收链/CI和结项资料，未修改`crates/*/src`产品实现、公开API、包版本或产品工具钉。

## 补审与实际主/隔离重放

四层独立审查及后续独立复核见[裁定](phase24-closeout-review.md)。20条初始候选归为12个修复组，追加3条复核问题均已处理；未以修改原规格替代修复。

实际命令：`python3 _agile-output/test-artifacts/129-3-build-runner.py --evidence _agile-output/test-artifacts/phase24-closeout-core.json`。该命令与`just fr198-fr201-core-check`使用相同runner，本轮额外覆盖自定义输出路径。最终完整运行退出0，**275.947秒**；隔离子流程153.715秒，使用独立checkout、空Cargo target、独立后端/formal目录，共享既有registry/Maven工具缓存。核心原始索引[完整JSON](phase24-closeout-core.json)已明确晋升为`129-3-latest-results.json`；[路径检查](phase24-closeout-verification/custom-output-check.json)证实在晋升前默认证据未被自定义路径重放改写。

|拓扑|后端|每处运行随机事务|主运行断言|主运行后端耗时ms|
|---|---|---:|---:|---:|
|AXI|direct|16000|62416|7726|
|AXI|FIRRTL|16000|62416|6185|
|AXI|Chisel|16000|62416|32295|
|direct CSR|direct|16000|60406|5114|
|direct CSR|FIRRTL|16000|60406|4903|
|direct CSR|Chisel|16000|60406|32230|

主与隔离均完整执行上述六路；每路均保留原16个seed与原始独立SV oracle、非空VCD及未经formal插桩的Yosys综合。合计两处各96000随机事务，不把相同seed重放当额外独立随机覆盖。后端耗时包含各自工具步骤，完整runner时间另含构建/形式/兼容/归档/校验。

形式在两处实际执行：IRQ连接归纳prove depth4、cover depth2；reset/response BMC depth8；新增request/reset BMC depth8及cover depth8，5个非空且hash绑定的见证；IRQ接线、请求保持、reset取消三种受控mutant均真实断言失败并产生counterexample VCD。四个外设叶在系统formal中blackbox，假设复位后清响应且受阻保持响应；请求取消检查到新epoch输入之前。不是完整AXI/UART证明或活性保证。

三库SemVer真实执行PASS，各196 checks pass、58 skip；skip不算pass。普通与`-O`各8项验收器自动化在主/隔离均执行，包含坏aggregate/归档篡改/缺工具/超时/自定义输出/干净checkout空补丁路径/拓扑命名碰撞；缺工具负控走真实preflight并逐项要求exit4及精确诊断。失败/超时不伪装为形式反例。

## 原始资料与失败保留

- `129-3-historical-results.json`保存旧schema的历史索引；旧target原件已不存在的事实不倒填，当前新重放提供补充证据。
- 最终主运行：`129-3-raw/1790137443475049031-2153715/`（432个manifest文件）。
- 最终隔离运行：`129-3-raw/1790137570516560772-2173635/`（430个manifest文件），子索引位置由完整JSON的`isolated_replay.evidence`给出。
- 第一次完整运行134.318秒，因聚合错误将CSR/direct选入AXI/direct，独立consumer以`backends/direct: topology`拒绝，退出1。完整失败日志和当轮资料保留在`129-3-raw/1790137246619365268-2123239/`；不计PASS。修复精确拓扑选择后，8项测试和完整门禁重新通过。
- Rust补测早期sbt临时路径过长失败已由短runtime目录修复；最终执行源码与归档逐一匹配，[7项输入绑定](phase24-closeout-verification/final-source-binding.json)保存fmt后的实际SHA。
- 工具安装与闭包身份、原始准备日志在[工具记录](phase24-closeout-tools/provision.json)及同目录identities.json。本机原有Cargo/Maven缓存；不宣称全新机器安装速度。

## clean、格式与工作区回归

实际`cargo clean`退出0（6.329秒）、`cargo fmt --all`退出0（1.268秒）。之后独立ATDD consumer在普通和`-O`模式均退出0，证明完整原始资料在clean后仍存在并通过hash/性质/命令/路径校验。原始命令及状态见[commands.json](phase24-closeout-verification/commands.json)。

最终工作区回归已完成，实际退出0：**487个结果块、1894 passed / 0 failed / 56 ignored**，耗时1021.046秒。仅统计本次完整运行，不合并取消运行；ignored不计PASS，相关核心专用门禁另有上述实际执行证据。计数与日志SHA见[workspace-summary.json](phase24-closeout-verification/workspace-summary.json)。第二次实际clean/fmt分别3.873秒/1.267秒，均退出0。

## 结项边界

本轮未执行远端GitHub Actions；CI配置已要求完整本地同源门，并在成功/失败时上传证据，不能将配置存在当作远端已通过。FR189/Epic122/122.2/122.3保持deferred/未交付，NFR91及既有明确遗留保留。未push/publish，不承诺native/generated层级、任意外部参数、全协议证明、PPA/时序或物理签核。

### 回归执行配置记录

默认profile的`just test`在220.213秒处由本轮显式TERM取消，退出143；当时未见断言失败，但未完成，**不计PASS**。历史130.3默认profile完整回归耗时1476.434秒。为缩短等待，本轮再次实际clean/fmt后运行`env CARGO_PROFILE_TEST_OPT_LEVEL=1 just test`；仅进程环境指定test编译优化，不修改Cargo配置，不关闭debug assertions，不改变测试选择、随机预算或普通ignored策略。最终结果仅取该完整运行，不能合并取消运行的计数或据此作受控加速比声明。

### 状态同步后的最终核验

Phase24 gate、普通/`-O`证据consumer及`cargo fmt --all -- --check`均退出0；59个新增本地文档链接全部存在，六Epic/22故事均done，Phase24 complete，Epic122及两项延期故事仍deferred。产品源码、Cargo版本/锁文件及Rust工具钉无差异；见[最终一致性记录](phase24-closeout-verification/final-consistency.json)。
