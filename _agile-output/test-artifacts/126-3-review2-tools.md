# Story126.3 独立审阅后工具修补

2026-09-20；仅定向验证，未更新故事状态、未提交。

- 默认test job失败上传`target/fr195/`；formal job增收`target/fr119/`隔离负例证据。
- FR119的sby版本/help、yosys版本、z3版本探针均采用`timeout --kill-after=2s 10s`；证明入口原有180秒+5秒强杀和实际FAIL/UNKNOWN非零规则保持。
- 真实负例把两个输入夹具复制到`target/fr119/fail-{pid}-{nanoseconds}`，以原子create_dir防覆盖，显式覆盖fixture环境；记录该run的wrapper.log、实际FAIL状态、真实trace，并打印证据路径。各进程互不清理对方产物。
- 既有`check_fr194_example.py`同时抽取FR194与FR195当前文档Rust代码块，在仅依赖prelude的临时crate实际编译运行；CI原入口继续强制执行两者。

命令与结果（PATH加真实sby，CARGO_PROFILE_TEST_OPT_LEVEL=1）：

1. `cargo test -p bitloom --test fr119_symbiyosys_smt_path -- --nocapture`：8项通过，`126-3-review2-fr119.log`。
2. 同时启动两次`cargo test -p bitloom --test fr119_symbiyosys_smt_path fr119_optional_real_sby_fail_fixture_when_installed -- --exact --nocapture`，均注入无效外部fixture override：各1项通过，生成不同run目录，`126-3-review2-concurrent-{1,2}.log`。
3. `python3 scripts/check_fr194_example.py`：FR194、FR195当前文档示例均编译并执行PASS，`126-3-review2-doc-examples.log`。
4. `bash -n scripts/formal-sby-check.sh`、PyYAML解析并检查CI上传路径/门禁、`git diff --check`：通过。
