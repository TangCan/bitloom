# Story129.3 build evidence

日期：2026-09-22。状态：build 实现与定向门通过；最终七步关闭仍需 code review、automate、clean/fmt/workspace regression 和单故事提交。本记录不提前关闭 Epic129。

## 三后端完整系统

三条路线复用 `fr198_peripheral_system::system(true)` 的同一 `FrozenHir` 和同一独立 `axi_tb.sv`。每条路线实际执行 16 个固定 seed × 1000 随机事务，另有 62,416 个运行时断言和定向事务；不是 emit/compile-only。

|路线|随机事务|断言|VCD bytes|原始综合 cells|RTL SHA256|
|---|---:|---:|---:|---:|---|
|direct Verilog|16,000|62,416|437,952|3,520|`62f4cc537f2002af3a16f470c02345169288415663c79da08e68fd57f84a179e`|
|FIRRTL→firtool|16,000|62,416|189,987|3,587|`084ce25e51504d36d24d75b7bffbd11f005be7e52d9964630ebf48a26f859569`|
|Chisel→JVM→firtool|16,000|62,416|191,212|3,587|`1f7a701d094328645b2d8cec526e804e67b7e96080aaf3da847660d3ff46feb3`|

direct CSR 无 bridge 第二组合另行实际执行通过。三路线都对未插桩 RTL 执行 Yosys hierarchy/check/synth/stat；cells 不是 PPA、时序或物理签核。

## Formal 与兼容

- SBY `prove` depth 4：五源 IRQ 聚合/连接不变量 PASS。
- SBY `response_reset_bmc` depth 8：从初始 reset 起检查 B/R/CSR 响应取消与受阻 valid/payload 保持，PASS。
- SBY `cover` depth 2：IRQ 事件可达，PASS。
- 受控 DUT mutation `i_raw_events = ~irq_events`：SBY exit 2，`EXPECTED_FAIL`，产生 counterexample。
- 四个外设 leaf 在系统 formal 中 blackbox；assumption 为 reset 后 `rsp_valid=0`，受阻时保持 `rsp_valid/rdata/error`。这不是全 AXI 或全 UART 证明。
- `scripts/test-semver-check.sh` 与真实 `scripts/semver-check.sh` 均 exit 0，覆盖 `bitloom-prelude`、`bitloom-sim`、`bitloom-firrtl`。公开表面与 crate 版本无 diff。
- 八项缺工具负控制均 exit 4；缺 Icarus/VVP/Yosys/firtool/Java/sbt/sby/solver 不可变成 PASS。

机器可读索引：`129-3-latest-results.json`。原始运行日志位于 `target/fr201-runner/`，后端源、命令、工具身份、VCD 和 evidence 位于 `target/fr201-core/`，formal 原始状态/trace 位于 `target/fr201-core-formal/`。

## 隔离重放

`129-3-build-runner.py` 默认调用 `129-3-isolated-replay.sh`；后者从 `HEAD + current diff + untracked files` 建立 detached worktree，使用独立 `CARGO_TARGET_DIR`、backend root 和 formal root。隔离 runner 明确输出 PASS，并只把 `isolated_replay.exit_code=0`、`used_main_target=false`、`used_hidden_tmp_rtl=false` 合并回主 evidence，避免保留已清理 `/tmp` 的伪原始路径。本机外层工具随后报告 `/proc`/Yosys history 沙箱审计并返回 1；该外层提示不改变脚本内部全部命令和合并 evidence 的 exit 0，最终回归将再次执行源工作区门。

首次隔离构建会解析 Cargo registry；冷 JVM 需要获取 Scala/Chisel Maven 依赖。CI 使用 Rust1.97.1、firtool1.159.0、Chisel7.15.0、Scala2.13.18、sbt1.10.11，并显式安装 Icarus、Yosys、Z3、SBY 和 cargo-semver-checks。
