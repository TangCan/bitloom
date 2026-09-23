# Story130.3 验收记录（2026-09-23，完成）

Story130.2 已在独立提交 4152610 完成 FR199 修复、clean/fmt/regression 和来源门禁。Story130.3 的真实绑定、独立行为、上游测试、隔离、独立审查与自动化均已通过；单一外部试点支持等级提升至 maintained。

## 已执行的真实验收

- 三仓库完整来源闭包与冻结参数保持：`fifo_v3`，DATA_WIDTH=32、DEPTH=8、FALL_THROUGH=0。
- wrapper 来自既有共享 ElaborateSession、FrozenHir 与 Verilog 后端；CLI 只移除精确验证的 opaque 外部声明，保留父模块原样。行为测试经过 `BitloomExternalFifo`，不是直接绕过父模块实例化上游。
- 独立队列模型实际执行 104 个采样步骤，覆盖空/满、同时 push/pop、数据顺序、有数据异步复位及 flush；这不是 formal assertion 计数。
- 实际 Icarus/vvp 分别拒绝空父模块与零输出父模块。工具锁包括驱动、真实编译 helper 与 VPI 运行时 hash。
- 新鲜空缓存 pilot 后，normal/-O 各 22 个精确诊断负测通过；探针改为独立 host-only 临时目录后，再次逐项复跑通过。完整记录在 `130-3-review-verification/{normal,optimized}/`，首次 fetch 记录保留。
- 2 项普通 hermetic integration 测试通过；4 项真实工具 integration 测试另行显式激活并通过。普通运行中的 ignored 不计 PASS。
- 10 项 Rust unit 通过；真实模拟器 mutant 单测另行显式激活，1 项通过。原始日志见 `130-3-automation-{rust-unit,simulator-mutants}.log`。
- 独立 evidence consumer 在 normal/-O 下各通过 1 个真实基线 + 18 个拒绝变异；包括零检查、假成功、缺失工具/源/wrapper 身份、宿主根挂载与缓存变化。
- 将原 checkout 和原 cache 实际只读挂载后，source-only 重放通过；`readonly-input-replay.json` 保存 exact command、stdout、退出码和耗时。

## 最终门禁

1. **完整回归已通过：** `130-3-regression/commands.json` 与原始日志记录实际 `cargo clean`、`cargo fmt --all`、`just test`，三条命令均退出 0。487 个结果块，1893 passed / 0 failed / 55 ignored；ignored 不计 PASS。workspace 启动后同步的 FR199 直接 compile-contract 单测另行复跑：11 passed / 0 failed / 1 ignored，见 `inherited-fr199-units.log`，不重复加入 workspace 汇总。其末尾空行导致一次 fmt check 失败，已格式化并复查通过；失败与修复命令均保留在 `final-checks.json`。生产 CLI SHA256 仍为 c40e36535ba32de8c285161c508aa06d1581809762dd493bcb9a2097acffd0c4，与新鲜 pilot 的冻结可执行文件完全一致。
2. **上游套件已通过：** `130-3-upstream-parallel.json` 的六组各 100000 次、总计 600000 次检查全部通过，13 条编译/仿真/版本命令均退出 0，墙钟约 664.6 秒。原 `fifo_inst_tb` 驱动、队列 oracle、断言和源码 hash 不变；薄调度 harness 仅按原时钟/复位/参数连接单组实例并等待 done 后退出。独立审查见 `130-3-upstream-parallel-review.md`。每组 seed=1303；分进程随机序列及完成后停止时点不同于原顶层交错运行，未声称逐刺激轨迹相同。首轮单进程套件 900 秒超时归档，1000 次诊断不计验收；冗余单进程重试的取消/未完成结果另存，不计 PASS。
3. **canonical 与提交：** 最终 lock SHA256=49b22842deda1d09bb86baaa5c8c3474f013a52c7d587f624f827705b5141d6e；实际构建 CLI、210 来源文件、隔离 binding 输出均逐项比对。更新 canonical 后，独立 consumer normal/-O 各11通过，见 `130-3-final-canonical-consumer-{normal,optimized}.log`。FR199旧锁和原始证据保存在 `130-2-final-regression/accepted-snapshot/`，不混同两次工具快照。原86步/JSON-only阶段不作为最终证据。

## 边界

上游 Verilator 路径使用原测试的 DATA_WIDTH=8，与 Bitloom 固定 32 位独立模型分别验收。原 upstream `VERILATOR` 条件排除 fall-through SVA，队列断言仍启用；不声称形式证明。固定单时钟试点不支持任意参数、多核、复杂协议、native/generated 层级模型或物理签核。新增 CLI 表面逐项登记 FR142/SemVer minor；无包版本/产品工具钉变化，无 push/publish。

四层审查23条发现已逐项修复并由最终独立复核确认；Story130.3/Epic130随本独立提交关闭。FR189/Epic122 deferred、NFR91 与其他历史边界不由本记录关闭。
