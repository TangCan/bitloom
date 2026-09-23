# PULP FIFO 外部试点（Story130.3）

本试点使用 `common_cells` v1.40.0 的 `fifo_v3`，仅冻结 `DATA_WIDTH=32`、`DEPTH=8`、`FALL_THROUGH=0`。Bitloom 与 `samitbasu/rhdl` 无关。长期 owner 为 Richard；源码版本与 Bitloom wrapper 版本分别记录。当前状态以 [支持矩阵](phase24-support-matrix.md) 为准；Story130.3/FR200 已按该单一范围验收关闭，证据见 Epic130 closeout。

## 真实绑定

`cargo bitloom external-ip binding --manifest <manifest> --lock <lock> --cache <cache> --out <binding.json>` 在校验完整来源闭包、许可证、工具与真实 FIFO 端口/默认参数后生成绑定产物。产物含 `wrapper.verilog` 与 SHA256，内容来自现有 `ElaborateSession` / `FrozenHir` / Verilog 后端生成的父模块。

CLI 链接边界仅剔除经严格检查的端口-only `fifo_v3` 声明，由锁定 RTL 提供真实行为；父模块保留后端原样输出。实例中数值参数用于审计，当前后端不发射 Verilog 参数覆盖，所以会对真实上游默认值进行完全相等检查，不支持任意参数集。第三方源码不嵌入 HIR、不创建第二 IR；设计 crate 仍只依赖 `bitloom-prelude`。

`cargo bitloom external-ip behavior --manifest <manifest> --lock <lock> --cache <cache>` 消费相同生成父模块，通过独立队列模型校验实际 RTL。测试必须经过父模块实例，不能只直接测试上游 FIFO。无 native/generated 层级行为模型，此路径明确 unsupported，不以零输出替代。

## 重放与工具

使用仓库 `scripts/phase24-external-ip-pilot.py --work <不存在的新目录>`，先在空缓存获取完整闭包，再复制产物进入只读输入/缓存挂载，写入独立 scratch。隔离环境须证明原 checkout、原缓存与外网不可访问，依次运行 compile、binding 和 behavior。普通 workspace 测试不隐含依赖开发者临时目录；需真实工具与下载的验收门禁显式执行，其 ignored 状态不计通过。

工具锁包括 Bitloom 可执行文件及实际 HDL 编译/仿真工具身份。修改实现后二进制 hash 变化必须显式重新生成工具绑定的 lock 和证据；验证入口不自动重锁。精确支持范围、上游测试、负测和当前结果见 `_agile-output/test-artifacts/130-3-final-verification.md`，历史阶段日志不能替代最终验收。

## 独立验收入口

准备 Git、Yosys、Icarus (`iverilog`/`vvp`)、bubblewrap、Z3、C++ 编译器及 make 后，运行：

```sh
python3 scripts/provision-phase24-verilator.py --root /tmp/bitloom-fr200-verilator
BITLOOM_VERILATOR_ROOT=/tmp/bitloom-fr200-verilator just fr200-external-ip-pilot-check
```

Verilator v5.052 是执行上游 SystemVerilog 测试的本地测试工具，安装目录与完整 commit、可执行文件和运行时 hash 均单独取证，不改变产品 Rust/firtool/Chisel 工具钉。上游 `fifo_tb.sv` 文件保持原文；正式门禁把其中六个原 `fifo_inst_tb` 用例放入独立进程，每组仍执行 100000 次检查。薄调度 harness 保留原参数、时钟、复位与队列检查器，仅在该组 done 后退出。每组独立 seed=1303，因此随机轨迹与原顶层交错运行不同；这不声称逐刺激一致。其 DATA_WIDTH=8 与本试点 DATA_WIDTH=32 分别验收。上游自带 `VERILATOR` 条件排除了 fall-through SVA，仍启用队列断言；这不是形式证明或完整 SVA 验收。

来源重放默认仅编译冻结 RTL；显式 `scripts/phase24-external-ip-replay.py replay --compile --pilot ...` 才增加真实父模块绑定和行为测试。CI 配置中的 `fr200-external-ip-pilot` 作业执行相同的联网准备、普通/优化负测、独立证据消费和上游套件，任一步失败就失败。配置已接入不等同于远程 CI 已运行；当前实际执行状态见最终验收记录。

## 升级与弃用

升级需修改来源意图、重新生成完整不可变锁、审阅每个仓库许可证与 NOTICE、运行上游和独立组合 RTL 测试及所有失败关闭场景。不得自动追浮动分支、换核或复用旧 PASS；弃用须有 owner 决策和替代证据。支持级别逐级验证，不把 CI 配置存在等同于 CI 实际执行。
