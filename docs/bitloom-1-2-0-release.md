# Bitloom 1.2.0 发布记录

状态（2026-09-23）：**十个Bitloom包均已正式发布到crates.io，版本1.2.0**。逐包dry-run/上传、registry校验和、真实安装与仓库外使用验收通过。未执行Git推送、创建GitHub Release或正式标签。Bitloom与samitbasu/rhdl无关。

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

## 正式发布操作记录（1–4已完成；5仅完成结果归档）

1. 审阅本候选提交、验证记录、包清单与已知限制，明确授权正式上传及是否推送/创建GitHub Release。
2. 核对crates.io各包1.2.0仍未占用以及发布凭据/权限。将CHANGELOG候选段落转为实际发布日期；若改变包内容，重新打包并更新摘要。
3. 按上面的依赖顺序，每包先`cargo publish -p <包名> --dry-run --locked`，成功后再`cargo publish -p <包名> --locked`；等待该包索引可用后继续下一个。只上传通过验证的源码提交；部分成功时记录已上传包，不重传同版本。
4. 所有包完成后，在不带临时patch配置的新环境执行`cargo install bitloom --version 1.2.0 --locked`与`cargo install bitloom-lsp --version 1.2.0 --locked`，复核new/build、gen-func/gen-cycle生成物的registry依赖及`cargo run`。CLI无`--version`选项，使用`cargo install --list`核实安装版本；LSP通过initialize响应的serverInfo.version核实。
5. 记录真实上传结果，再按授权创建带说明的`v1.2.0`标签、推送与发布公告。现有release-plz工作流执行release-pr，不能把它当作已上传。

## 保留边界

FR189/Epic122仍deferred、未交付，NFR91保持。新增层级组合的native/generated仿真仍unsupported；功能/周期生成验收使用其已支持的叶节点设计。外部IP只承诺已验收试点及明确支持矩阵，不宣称任意IP包管理。没有板卡、物理签核或新版远端CI结果；本地软件包发布准备不替代这些验证。
