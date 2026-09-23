# Bitloom 1.2.0 发布记录

状态（2026-09-23）：**十个Bitloom包均已正式发布到crates.io，版本1.2.0**。逐包dry-run/上传、registry校验和、真实安装与仓库外使用验收通过。整理后的主线已于2026-09-23推送（`f450afb`）；`v1.2.0`正式标签已推送，[GitHub Release](https://github.com/TangCan/bitloom/releases/tag/v1.2.0)已发布；后续维护提交的远端CI 19/19通过。Bitloom与samitbasu/rhdl无关。

本版本为Phase24与后续缓存修复的SemVer minor增量；接口依据[公开API表面](public-api-1-0-surface.md)，交付范围见[Phase24总验收](ip/phase24-closeout.md)，新增功能与修复见[CHANGELOG](../CHANGELOG.md)。历史1.1.x已发布版本不改写。

## 实际发布结果

发布源码提交：`e33d12fd747e576a542e24f18be1e3cfea4760d7`；候选准备提交：`967ad859e4613b48f8c800ab54ef01996d9a4ae0`。用户明确授权后按依赖顺序逐包发布，各包等待索引可用才继续，没有重传或yank。

|包|版本|结果|
|---|---|---|
|[bitloom-macro](https://crates.io/crates/bitloom-macro/1.2.0)|1.2.0|已发布、索引可用|
|[bitloom-hir](https://crates.io/crates/bitloom-hir/1.2.0)|1.2.0|已发布、索引可用|
|[bitloom-builder](https://crates.io/crates/bitloom-builder/1.2.0)|1.2.0|已发布、索引可用|
|[bitloom-vlog](https://crates.io/crates/bitloom-vlog/1.2.0)|1.2.0|已发布、索引可用|
|[bitloom-sim](https://crates.io/crates/bitloom-sim/1.2.0)|1.2.0|已发布、索引可用|
|[bitloom-prelude](https://crates.io/crates/bitloom-prelude/1.2.0)|1.2.0|已发布、索引可用|
|[bitloom-firrtl](https://crates.io/crates/bitloom-firrtl/1.2.0)|1.2.0|已发布、索引可用|
|[bitloom-viz](https://crates.io/crates/bitloom-viz/1.2.0)|1.2.0|已发布、索引可用|
|[bitloom](https://crates.io/crates/bitloom/1.2.0)|1.2.0|已发布、索引可用|
|[bitloom-lsp](https://crates.io/crates/bitloom-lsp/1.2.0)|1.2.0|已发布、索引可用|

[实际发布与安装验证](../_agile-output/test-artifacts/bitloom-1-2-0-published-verification.md)包含逐包日志与下载校验和。此前五个单包联网发布测试现已补跑：5 passed / 0 failed；不再属于待执行项。候选回归的56项ignored仍维持原验证范围，未因发布而计作PASS。

安装：

```bash
cargo install bitloom --version 1.2.0 --locked
cargo install bitloom-lsp --version 1.2.0 --locked
```

## 版本与发布范围

十个发布包统一为1.2.0，内部发布依赖最低版本同步到1.2.0；sim使用workspace版本。设计crate仍只依赖bitloom-prelude。以下顺序同时用于发布前dry-run和实际上传：

1. bitloom-macro
2. bitloom-hir
3. bitloom-builder
4. bitloom-vlog
5. bitloom-sim
6. bitloom-prelude
7. bitloom-firrtl
8. bitloom-viz
9. bitloom
10. bitloom-lsp

不得发布rhdl-*内部包或examples。工作区示例继承版本变化不表示发布它们。Rust 1.97.1、firtool/Chisel等工具钉保持。

## 候选验证

使用Cargo多包`package`验证尚未上传的家族依赖，另在仓库外将候选归档解包，以临时Cargo配置覆盖crates.io的十个家族包，安装CLI/LSP并演练new/build及功能/周期仿真生成。这是本地候选包验收，不是已从crates.io安装的证明。

根Cargo.lock被仓库忽略；本轮另行归档验证用锁文件及SHA256。正式发布从干净候选提交恢复该锁文件，再运行`--locked`检查，不能静默重新解析依赖：

```bash
cp _agile-output/test-artifacts/bitloom-1-2-0-release/workspace-Cargo.lock.txt Cargo.lock
sha256sum Cargo.lock
# 对照readiness.json中的workspace_lock_sha256，再执行逐包发布检查。
```

运行结果、候选包SHA256与具体命令记录在[发布候选验证](../_agile-output/test-artifacts/bitloom-1-2-0-release-verification.md)。本轮十包联合`cargo publish --dry-run --locked --allow-dirty`已实际通过，使用Cargo原生未发布依赖闭包检查；日志中的Uploading行均紧随“aborting upload due to dry run”，没有上传。单独执行非叶子包的dry-run仍可能受尚未上架的1.2.0依赖影响；正式上传时仍逐包检查。

## 正式发布操作记录（1–5已完成）

1. 审阅本候选提交、验证记录、包清单与已知限制，明确授权正式上传及是否推送/创建GitHub Release。
2. 核对crates.io各包1.2.0仍未占用以及发布凭据/权限。将CHANGELOG候选段落转为实际发布日期；若改变包内容，重新打包并更新摘要。
3. 按上面的依赖顺序，每包先`cargo publish -p <包名> --dry-run --locked`，成功后再`cargo publish -p <包名> --locked`；等待该包索引可用后继续下一个。只上传通过验证的源码提交；部分成功时记录已上传包，不重传同版本。
4. 所有包完成后，在不带临时patch配置的新环境执行`cargo install bitloom --version 1.2.0 --locked`与`cargo install bitloom-lsp --version 1.2.0 --locked`，复核new/build、gen-func/gen-cycle生成物的registry依赖及`cargo run`。CLI无`--version`选项，使用`cargo install --list`核实安装版本；LSP通过initialize响应的serverInfo.version核实。
5. 记录真实上传结果，再按授权创建带说明的`v1.2.0`标签、推送与发布公告。现有release-plz工作流执行release-pr，不能把它当作已上传。

## 保留边界

FR189/Epic122仍deferred、未交付，NFR91保持。新增层级组合的native/generated仿真仍unsupported；功能/周期生成验收使用其已支持的叶节点设计。外部IP只承诺已验收试点及明确支持矩阵，不宣称任意IP包管理。没有板卡、物理签核；最初`f450afb`未生成GitHub Actions记录，后续维护提交的远端CI已实际通过（见下方记录）；本地软件包发布准备不替代这些验证。

## 本地历史归档整理（2026-09-23）

原始验收归档移至仓库外，恢复步骤与提交映射见 [证据归档说明](evidence/README.md)。本文上传源码 SHA 保留真实发布时的值；整理历史不改变已发布包。整理阶段未恢复推送；随后主线与正式标签已推送，见下方收尾记录。

## 推送后的发布收尾

归档整理后的 `main` 已正常快进推送至 `f450afb0551c8664c08dd50a398129e71e7d6785`，备份分支未推送。CI工作流已启用，但该提交没有生成运行记录；增加 `workflow_dispatch` 入口以执行真实远端验证，现有门禁保持不变。正式 `v1.2.0` 标签已指向 `2cda375fbd5b8f1f33c6c3eb6abd93b73eda9e83`，它对应真实上传源码 `e33d12fd747e576a542e24f18be1e3cfea4760d7`，映射已逐提交验证。

### 远端 CI 跟进

- 首次手动运行 [35860205600](https://github.com/TangCan/bitloom/actions/runs/35860205600) 发现 FR164 旧层级夹具、FR200 runner 隔离及 FR201 审计旧 SHA 问题。不得将其宣称为通过。
- [35861060548](https://github.com/TangCan/bitloom/actions/runs/35861060548)：FR164 修复通过；Ubuntu22 namespace 探针通过，但 Icarus11 无法编译试点 RTL。
- 修复提交 `40ed90c` / `07f11b2` 保留仿真边界、真实隔离及原历史审计内容，追加 Icarus12 源码钉死配置。[35861844189](https://github.com/TangCan/bitloom/actions/runs/35861844189) 后续因旧Yosys解析限制失败，并取消已被新一轮替代的剩余任务。
- 上述修改属于上传之后的门禁维护，未重传包、未移动标签，不宣称这些后续修改已包含在 crates.io 1.2.0 中。

原始验收归档仍只有本地外部备份；异地备份位置待用户提供，未上传到公共 Release。

### 最终结果（2026-09-23）

- 维护提交 `b24652cbb43d51d992efead3acfdb53ceeaae088` 的 [CI 35864105067](https://github.com/TangCan/bitloom/actions/runs/35864105067) **19/19 job success**。工作区 487 个结果块、**1905 passed / 0 failed / 56 ignored**；ignored 不计 PASS。文档示例、数值后端、形式、核心集成、SemVer 与外部 IP 专用 job 均通过。
- 试点保留完整 namespace/宿主文件拒绝、工具身份、正常及优化 Python 负向验证；未修改的上游 FIFO 套件六组各 100000 次，合计 **600000** 次检查通过。分进程执行与原顶层不宣称逐刺激轨迹等价，原有限验收边界保持。
- 补齐的 CI 环境项包括 test job 的 Yosys/just、试点 runner 的 Icarus12/Yosys0.33，以及 APT mirror transport。历史审计使用提交映射和已完成的固定历史区间；真实版本/API 越界反例仍拒绝。
- [正式 Release](https://github.com/TangCan/bitloom/releases/tag/v1.2.0) 于 `2026-09-23T13:28:40Z` 发布，非 draft、非 prerelease。标签仍指向实际上传源码的整理后等价提交，不移动标签、不重传包。上述 CI 结果针对维护提交，不能冒充原标签的全绿 CI。
- 结构化结果、任务链接与证据摘要见 [最终验证](evidence/release-ci-verification.json)。原始远端日志及上游结果保存在仓库外证据库，未重新塞入 Git 历史。异地备份仍待存储位置。
