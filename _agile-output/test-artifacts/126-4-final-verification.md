# Story126.4 最终验证记录（完成）

基线 `b248dd48a948a303de9aa6aebf656a5c356b52f2`。目标为 ParamSyncFifo 与 Epic126/M1；七步均完成，本记录随Story126.4单独提交，Epic126/M1关闭。

1. create-story：故事126.4已创建，完整AC及开发上下文。
2. ATDD：两个入口真实缺API红测exit101；不可变历史probe说明见126-4-atdd-snapshot.md。
3. build：新寄存器FIFO、完整参数身份、CI、prelude-only文档；三路内审及修补完成，主代理复跑见126-4-build-review-regression.log。
4. 独立code-review：四层完成，8项适用修补、2项有据拒绝、无延期；完整裁定见126-4-independent-review.md。最终31项API/native/RTL通过；源码快照、最新RTL归档见126-4-code-review-source-snapshot.json。
5. automate：完成；两个独立worker确认无需新增重复测试，31+6实际通过；732个归档文件哈希核验通过，见automation-126-4.md。
6. clean/fmt/just test：实际退出0；cargo clean移除29392文件/9.4GiB，fmt通过，460个结果块汇总1763 passed、0 failed、14 ignored。原始日志126-4-clean-workspace-regression.log及.exit/results JSON。
7. 单故事提交及M1关闭：全部前置通过，随本记录所在Story126.4提交完成；不写自引用hash。

专用工具结果：W1×D1/2/3真实SBY basecase+归纳prove及全部cover PASS；原始RTL综合1×1/8×3/64×16通过，cells为7/66/2104。首轮D2/D3 UNKNOWN保留，不混作通过。故障注入实际RTL输出错误退出1；忽略flush的形式副本状态FAIL、有反例。证据归档、命令与校验和均在本目录，后续clean不删除。

最终行为矩阵按深度每组450/459/462/468/480/522拍，仍为24配置×3固定seed；取消控制reset-only/flush-only/重叠×空/部分/满逐配置断言命中，DEPTH1部分占用不适用。独立队列守恒、在途共享reset后隔离恢复、旧FIFO已知RAM兼容均验证。工具缺失/UNKNOWN/超时严格失败。

边界：无PPA/BRAM承诺；native/generated层级仍拒绝；实例参数在elaboration专门化，空Instance.params；126.2遗留名称合法性/参数覆盖见deferred-work，不宣称账本清空。FR189/Epic122保持deferred，NFR91保持。M1完成不代表Phase24全部完成，不推送、不发布。

执行环境：`PATH=/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin:$PATH CARGO_PROFILE_TEST_OPT_LEVEL=1 bash -c 'cargo clean && cargo fmt --all && just test'`。14 ignored中8个为专用形式测试（126.3两项、126.4六项，已分别实际运行），其余6个既有doc-test未执行。未宣称本地workspace覆盖全部远端CI/JVM/商店门禁。

关闭同步后附加检查：`python3 scripts/check_phase24_gate.py`、`cargo fmt --all -- --check`、`python3 scripts/check_fr194_example.py`均退出0；最后一项三个完整设计程序真实编译执行，日志126-4-final-doc-examples.log。源码/文档白空格与归档校验在提交前核对。
