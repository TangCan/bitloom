# Bitloom 1.2.0 发布候选验证（2026-09-23）

本轮准备目标：Phase24及缓存修复的1.2.0候选，未正式上传、推送或创建标签。基线提交为`0638f0b131bb56a3a910b7cb23c3ad43c9b75cea`；版本与内部依赖统一为1.2.0，sim恢复workspace继承，两个仿真生成器的HIR/builder最低依赖同步。无工具钉/API语义变化。

## 已完成检查

|检查|结果与证据|
|---|---|
|十包身份/版本/内部依赖、格式、差异|[readiness.json](bitloom-1-2-0-release/readiness.json)；内部rhdl-*仍publish=false|
|SemVer minor|prelude/sim/firrtl三包；[最终日志](bitloom-1-2-0-release/semver-final.log)，CLI新增动词与macro契约另按FR142及Phase24关闭记录核对|
|十个候选归档与构建|[审查后打包日志](bitloom-1-2-0-release/package-reviewed.log)，未使用no-verify|
|真实联合发布dry-run|[publish-dry-run.log](bitloom-1-2-0-release/publish-dry-run.log)，十包均因dry-run中止上传，退出0；不是实际发布|
|候选CLI/LSP安装|[CLI](bitloom-1-2-0-release/install-cli-reviewed.log) / [LSP](bitloom-1-2-0-release/install-lsp-reviewed.log)，安装到临时目录，未覆盖用户安装|
|仓库外消费|[汇总](bitloom-1-2-0-release/standalone-summary.json)：new/build生成Verilog、gen-func/gen-cycle编译运行、Phase24 CSR文档完整示例编译运行|
|生成依赖无仓库路径|[功能清单](bitloom-1-2-0-release/generated-func-Cargo.toml.txt) / [周期清单](bitloom-1-2-0-release/generated-cycle-Cargo.toml.txt)：registry 1.2.0；消费metadata确认Bitloom依赖全部来自候选解包目录|
|LSP实际协议|[initialize/shutdown结果](bitloom-1-2-0-release/lsp-smoke.json)，serverInfo.version=1.2.0，正常退出|
|只读核对registry|[结果](bitloom-1-2-0-release/registry-readonly.json)，查询时十包均无1.2.0；正式上传前须再核对|
|审查后sim库回归|[日志](bitloom-1-2-0-release/sim-reviewed-test.log)，38 passed / 0 failed / 0 ignored|
|独立审查|[四项裁定与复核](bitloom-1-2-0-release/review.md)，均修正，无新增延期事项|

仓库外验证将十个`.crate`解包到独立临时目录，使用临时CARGO_HOME中的patch配置，复用第三方registry缓存。没有使用源码仓库path依赖，但仍属于**本地候选**消费，不冒充从公开registry安装。CLI没有`--version`选项：首次探测失败保留在日志，随后用安装清单核验；LSP按真实协议核验。验证脚本曾混合stderr导致metadata JSON解析失败，已改为分流并完整重跑；这些脚本问题不算产品通过记录。最后汇总绑定审查后重打包的候选。

## 回归与范围

首次`CARGO_PROFILE_TEST_OPT_LEVEL=1 cargo test --offline --workspace`通过临时Cargo配置运行，在FR149单包publish dry-run尝试HTTP时失败并停止，见[原日志](bitloom-1-2-0-release/workspace-test.log)。不将其称为全工作区绿。

候选回归使用相同命令，仅显式过滤以下五个联网发布测试：`fr149_bitloom_firrtl_dry_run_publish`、`fr150_bitloom_viz_dry_run_publish`、`fr151_bitloom_cli_dry_run_publish`、`fr152_lsp_policy_a_documented_and_dry_run`、`fr155_bitloom_lsp_dry_run_still_green`。真实十包联合dry-run另已成功；正式依赖上架后的逐包dry-run仍需执行。候选回归退出0，共487个结果块：**1898 passed / 0 failed / 56 ignored / 5 filtered**，见[汇总](bitloom-1-2-0-release/regression-summary.json)与[原日志](bitloom-1-2-0-release/workspace-candidate-test.log)。不将ignored/filtered计为PASS，也不宣称未过滤的全工作区通过。审查中将同版本表达式改为实际依赖最低版本，输出仍为1.2.0；最终源码另外重打包、重装、重跑三包SemVer及全部38项sim库测试。

## 复现与归档

- 发布包明确限定：macro、hir、builder、vlog、sim、prelude、firrtl、viz、bitloom、lsp；用十次`-p`参数运行`cargo package --allow-dirty --offline`及`cargo publish --dry-run --locked --allow-dirty`。`--allow-dirty`仅因为候选源码当时尚未提交，没有跳过编译验证；正式上传要求审阅后的干净提交。
- 运行`python3 _agile-output/test-artifacts/bitloom-1-2-0-release/prepare-sandbox.py`，脚本将临时目录写入`/tmp/bitloom-release-sandbox-path`。用该目录的cargo-home配置分别`cargo install --path <sandbox>/sources/bitloom-1.2.0`及`bitloom-lsp-1.2.0`，指定`--root <sandbox>/install --debug --offline --target-dir <sandbox>/build`。
- 随后执行`standalone.py <sandbox>`、`lsp-smoke.py <sandbox>`与`readiness.py`。测试环境额外指定`CARGO_PROFILE_TEST_OPT_LEVEL=1`；没有放宽assert或改变产品编译配置。
- [包SHA256](bitloom-1-2-0-release/package-sha256.json)、[二进制SHA256](bitloom-1-2-0-release/binary-sha256.json)、[工具版本](bitloom-1-2-0-release/tool-versions.json)、[锁文件](bitloom-1-2-0-release/workspace-Cargo.lock.txt)均留档。锁摘要见readiness.json。正式发行标签/发布日期会改变归档元数据，重新打包时更新摘要并重验，不声称本轮dirty归档就是最终上传字节。

正式发布操作见[1.2.0发布清单](../../docs/bitloom-1-2-0-release.md)。未运行新的JVM/外部CIRCT/物理门禁，本轮不改变它们的实现或工具钉；既有Phase24证据仍以历史源绑定范围为准。FR189/Epic122 deferred、NFR91、层级native/generated unsupported与物理验证边界保留。

收尾锁文件核对：回归临时Cargo配置添加了十个`patch.unused`记录，所有package版本/source/checksum与归档完全相同；已恢复不带临时patch标记的归档锁，`cargo metadata --offline --locked`及readiness检查再次通过，见[核对记录](bitloom-1-2-0-release/lock-reconciliation.json)。没有重新解析或升级依赖。

日志归档说明：Cargo输出的tab缩进与末尾空行会触发git空白检查；受影响的`.log`仅规范化显示空白，同名`.log.gz`保存规范化前的逐字原始输出。命令、退出结果与测试计数没有修改，两种文件均纳入证据摘要。
