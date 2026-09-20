# FR194 / Story126.2 验证记录

2026-09-20，Rust 1.97.1、Icarus Verilog / vvp 12.0 stable。新增模块组合入口与冻结前诊断的证据；不作为FIFO、注册切片、原生层级执行、综合、PPA或formal交付证据。

## 实际 RTL

```sh
PATH=/tmp/bitloom-maintenance-tools/bin:$PATH BITLOOM_REQUIRE_RTL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1 cargo test -p bitloom --test fr194_module_composition -- --nocapture
```

结果：5 passed / 0 failed / 0 ignored。测试 `rtl_same_and_different_parameters_preserve_instance_state_for_both_module_orders` 分别编译执行顶层先定义和子模块先定义电路；每个电路包含两个8位与一个16位寄存实例，内部局部名字相同。每种顺序192拍，独立数据与enable、单时钟同步reset；种子 `0x19480816`。

`rtl_two_gpio_banks_keep_masked_writes_and_pad_reads_independent` 对两个共享同一定义的GPIO实例运行160拍，分别驱动写使能、mask、data、dir及pad输入，覆盖零mask、全mask、部分mask与运行中reset；种子 `0x19469100`。参考模型逐位计算状态和读回，不调用DUT构造函数计算预期。

审阅补充顶层→wrapper→leaf两层实例场景（另192拍，相同种子与独立期望）。总计4次实际 RTL 编译与执行、736拍、4,723项边沿前后输出比较。复位测试同时检查上升沿前仍保留原状态，避免异步复位错误蒙混通过。工具探针、编译和执行均有60秒上限，超时后终止并回收进程、保留日志；新增超时用例实际验证该失败路径。测试始终要求实际工具存在；缺失、编译失败、vvp非零退出或缺成功标记均失败。生成的完整输入/预期保存在 `target/fr194/<场景>-<进程号>/tb.sv`，同目录有design.v、commands.log、compile.log、run.log；失败时保留。复现命令不要求此进程号固定。

另外两项测试核验旧Gpio入口与共享定义得到相同电路及原端口序列，并核验Interpreter、Compiled、GeneratedFunctional继续拒绝层级电路。原始日志中的预期panic属于该拒绝测试，不是RTL失败；见[RTL测试日志](evidence/fr194-rtl.log)。

## 仅 prelude 的文档示例

通过已接入CI的 `python3 scripts/check_fr194_example.py` 从[模块组合说明](module-composition.md)提取完整 `src/main.rs` 到临时独立Cargo工程，唯一设计依赖为本地 `bitloom-prelude`，执行 `cargo run --offline --manifest-path <临时工程>/Cargo.toml`，退出0。未直接依赖CLI、HIR、builder或模拟器。编译与执行记录见[示例日志](evidence/fr194-prelude-example.log)。这证明仓库内接口可用，不代表已发布crate具有新API。

## 定向诊断与 API 兼容

`cargo test -p bitloom-builder -p bitloom-hir`：初轮71 tests passed，0失败；另有3个原有doc示例ignored。初轮包含10个新增builder测试、12个新增HIR测试；审阅再新增2个builder和1个HIR测试，针对诊断去重、完整保留、冲突/环定位。覆盖参数规范化与身份冲突、回调错误保留、活动模块生命周期、正反方向/类型连接、未知或重复连接、实例/过程多驱动、显式顶层与唯一根回退、不可达实例环，以及4,000层无环实例图的非递归遍历。

`bash scripts/semver-check.sh`：prelude、sim、firrtl各196项通过、58项不适用跳过。该工具检查可识别的API破坏，不能代替行为兼容回归或完整发布决策；[原始日志](evidence/fr194-semver.log)。`cargo fmt --all -- --check` 和 `python3 scripts/check_phase24_gate.py` 均通过。

## 完整回归与最终修补验证

完整 `cargo test --workspace --no-fail-fast`（同上PATH、RTL及test优化环境）退出0：456个测试目标结果，1,718 passed / 0 failed / 6原有ignored / 0 filtered，包含FR193两个长矩阵。该轮执行于审阅修补前；之后对仅涉及诊断去重/定位及测试基础设施的修补，重新运行builder、HIR、prelude、FIRRTL、Verilog五包：114 passed / 0 failed / 6原有ignored，并重新运行FR194五项集成和文档示例，全部通过。不将重复回归数量相加作为测试总数。

命令、计数和原始日志SHA256见[验证清单](evidence/fr194-verification.json)；[最终五包日志](evidence/fr194-final-packages.log)。三层独立审阅的当前变更问题均已修补；实例/局部网名合法性与Instance.params忽略两个既有缺口登记延后，本故事不宣称解决。
