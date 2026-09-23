# Bitloom 1.2.0 实际发布验证（2026-09-23）

用户明确“授权”十个包正式上传后执行。**十包全部发布成功**，未push、未创建GitHub Release或正式标签。发布源码提交为`e33d12fd747e576a542e24f18be1e3cfea4760d7`，候选提交为`967ad859e4613b48f8c800ab54ef01996d9a4ae0`。根锁文件从候选归档恢复，逐包使用`--locked`，没有修改产品源码或工具钉。

## 上传与真实性

- 发布前只读确认十个1.2.0版本均不存在：[preflight.json](bitloom-1-2-0-published/preflight.json)。
- 顺序为macro → hir → builder → vlog → sim → prelude → firrtl → viz → bitloom → lsp。每包先`cargo publish -p <name> --dry-run --locked`，再`cargo publish -p <name> --locked`，均退出0并等待索引可用：[逐包状态](bitloom-1-2-0-published/journal.json)。同目录每包都有dry-run与publish原始日志。
- [Registry核对](bitloom-1-2-0-published/registry-confirmation.json)：十包版本均为1.2.0、未yank；实际下载归档SHA256等于crates.io API的checksum；发布包Git元数据均指向上述干净源码提交。
- 首次直接比较本地target/package归档摘要与registry摘要未通过：本地仍含候选的VCS元数据，且家族依赖锁摘要随正式归档更新。这不是源码或版本差异。随后逐文件核对：源码/清单逐字相同，依赖版本与来源完全相同；差异仅限`.cargo_vcs_info.json`及Cargo.lock中Bitloom家族包的checksum。所有新checksum都逐项对上实际registry下载结果。`journal.json`中的archive_sha256是当时读取的**本地**归档摘要；实际发布摘要以registry-confirmation.json为准，不混为上传字节摘要。

## 真实安装与使用

使用新的临时CARGO_HOME，没有patch或path覆盖；只复用第三方registry缓存，不覆盖用户已安装二进制。实际执行默认release配置：

```bash
cargo install bitloom --version 1.2.0 --locked --root <临时安装目录>
cargo install bitloom-lsp --version 1.2.0 --locked --root <临时安装目录>
```

[CLI安装](bitloom-1-2-0-published/install-cli-registry.log)、[LSP安装](bitloom-1-2-0-published/install-lsp-registry.log)均通过。[仓库外汇总](bitloom-1-2-0-published/standalone-summary.json)及[执行日志](bitloom-1-2-0-published/standalone.log)覆盖：new/build生成Verilog、gen-func/gen-cycle的registry 1.2.0清单与编译运行、Phase24 CSR文档完整示例运行。消费metadata确认Bitloom依赖来源全部为crates.io，非本地候选目录。CLI版本通过cargo install --list核实；[LSP initialize/shutdown](bitloom-1-2-0-published/lsp-smoke.json)报告1.2.0并正常退出。

候选阶段过滤的五个联网发布测试已补跑通过：[5 passed / 0 failed](bitloom-1-2-0-published/publish-tests-summary.json)，[完整日志](bitloom-1-2-0-published/publish-tests.log)。本轮没有再次运行完整工作区；候选阶段的1898 passed / 0 failed / 56 ignored / 5 filtered与本次补跑分别记录，不能合写成一次未过滤回归。

## 留存与边界

[evidence-sha256.json](bitloom-1-2-0-published/evidence-sha256.json)覆盖本轮记录。为通过Git空白检查，受影响的可读log仅规范化tab/末尾空白，同名log.gz保存逐字原始输出。脚本用于记录执行方式，不应直接重复运行上传脚本。

公开产品仍为Bitloom；设计crate仅依赖bitloom-prelude。FR189/Epic122仍deferred、NFR91保留；没有新增物理签核、板卡或native/generated层级支持宣称。未发布任何rhdl-*内部包或examples，未推送Git或创建远端发布。
