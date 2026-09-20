# Bitloom 后端边界与验证证据 — 2026-09-20

本轮维护在 FrozenHir 及现有公开 API 上修复数值/边沿语义，未发布或推送；Bitloom 与 `samitbasu/rhdl` 无关。历史 FR 关闭记录仍成立，FR189 未交付、NFR91 未清空。

## 数值与边界结果

真实旧生成器先经 Scala 编译，再在 JVM 展开时因 `UInt64 → SInt64` 符号扩展连接失败。已成功降级的旧异宽夹具又由 Icarus 复现 `SInt8 ff << 0 → SInt16` 得到错误符号扩展，以及 `ff >> 1 → SInt16` 得到 `ffff` 而不是 `007f`。本轮修复共用 Chisel 表达式生成器：逻辑移位先使用无符号位型，显式零扩展并按目标类型转换；左移限制推断位宽，同时用完整移位量检查溢出；扩展和寄存器保留目标类型。新增 `SInt8 ff >> 1 = 7f`、`SAR = ff`、有符号寄存器、UInt 符号扩展目标和 SInt 零扩展目标。

严格入口 `bash scripts/chisel-numeric-check.sh` / `just chisel-numeric-check` 必须具备 Java、sbt、firtool 和 Icarus，缺失直接失败。同一独立参考矩阵导出所有 Scala，单次 JVM 编译，使用 `emitSystemVerilogFile` 及其完整 `filelist.f`，保留所有模块和验证 include，再实际执行 RTL。CI 有独立 `chisel-numeric` job，失败保留源码、向量、工具输出产物。

Chisel 仍为 **7.15.0**，firtool 仍为 **1.159.0**（既有 unpaired product pin）。真实 Chisel POM 依赖 Scala **2.13.18**；旧脚本 2.13.16 在 sbt 的 SIP-51 检查失败。本轮仅将数值及旧 FR28 JVM 脚本改成 2.13.18，没有关闭检查或改 Chisel/firtool 产品钉；sbt **1.10.11**，Java **17.0.20**，Icarus **12.0**。

边界集包括固定种子 `0x91b320d4` 的24节点反向声明 DAG、`q.d=q+input`、双寄存器交换、非碰撞异步/同步存储器、混合 enabled/plain 寄存器及下游寄存器、两种模块排列且父子同名局部 `q`。内存夹具还覆盖用户符号与后端生成的读阶段临时名冲突，所有用户声明都参与避让。共13项 integration（既有4项、新增9项）中，生成的 functional/cycle Rust crate 均实际编译运行；支持层级的 Verilog、FIRRTL、Chisel 实际编译并执行父子流水线。另检查无可识别顶层的递归实例输入返回诊断。

真实层级编译额外发现并修复：freeze 用第一模块充当顶层导致父模块被丢弃；FIRRTL 将父网连接到 child output 作为错误 sink。现在优先电路名指定的模块，否则推断未被实例化的根；无法找到根返回 `E0002`。不支持的 `Sim`/`GeneratedFunctional` 构造明确 panic，生成 crate 入口返回错误；CLI wave/coverage 返回可读 unsupported，不能产生貌似有效的首模块波形。已有 waveform 正向夹具改为单模块计数器，层级可视化仍保留层级夹具。

审阅补充覆盖了同一 process 的最后赋值优先级（含重复内存读及内存/非内存驱动覆盖）、4,096节点反向组合链、有符号计数回绕，以及保持时钟低电平时断言异步复位。freeze 统一消除已覆盖的 Net/RegD 驱动，保留全部 MemWrite；组合排序使用缓存依赖及就绪集合。异步复位的无边沿行为只由直接 Verilog/Chisel RTL 证明，native/generated Rust 仍是周期模型，FIRRTL 异步元数据保留不在本轮证明内。

审阅另登记4项既有缺口：显式顶层的递归实例图、多个候选根的歧义、fallback 顶层特殊 IO 校验，以及 Chisel SAR 使用 SInt 移位量。这些没有纳入已通过能力，见[deferred ledger](../_agile-output/implementation-artifacts/deferred-work.md)。

## 唯一边沿与内存约定

- 每拍先按组合依赖拓扑计算稳定输入，再求所有顺序 RHS，最后同时提交寄存器/存储器写入并重算组合输出；调用方不再需要手动 `settle()` 才能获得当前输入反馈。
- `Mem` 的 `RegD <- MemRead` 在边沿采样旧 bank。内部同址碰撞固定为 read-before-write，两种语句次序一致；**FIRRTL 的碰撞结果仍 undefined，不宣称跨后端碰撞等价**。
- `SyncReadMem` 每边沿采样一个读阶段；目的寄存器在当前边沿按自身 reset/enable 接收上一读阶段。下游寄存器采样边沿前的 q。reset 清目的寄存器，不清 bank、不禁止读阶段采样；写入按显式 we 且非 reset 才提交。直接 Verilog 补齐读阶段，Chisel 将 read 放在 q 的 enable 条件外。
- 跨 RTL 内存测试先写遍 bank 并等待流水填满，避免把未初始化 X 当成零。包含中途 reset、enable 抑制及 always-enabled 对照。同步读阶段一拍加目的寄存器一拍，不应把旧 write-first 的两次观察误称为统一硬件语义。
- FIRRTL **内存**文本生成仍有未连接 readwriter/注释式 write 限制；本轮内存行为证明仅覆盖 native、generated Rust、直接 Verilog 和 Chisel。FIRRTL 数值/DAG/层级通过不能替代 FIRRTL 内存执行。

旧 SyncFifo 手写模型也移除 write-first 和 reset 清 bank，增加独立字面序列回归。RV32 例子移除旧单遍模拟器的额外驱动周期与 `pc_f` 补偿；异步 dmem 读值和 MEM/WB 的目的寄存器编号在同一边沿采样，保留 branch/load-use 的原独立期望。五个受影响 RV32 库定向测试通过；这不代表完整 RV32 外部 RTL 证明。

## 能力清单与本轮执行类型

| 能力 | 命令 / 工具钉 | 本轮证据类型与结果 | 限制 |
|---|---|---|---|
| Rust workspace | `cargo test --workspace`；Rust1.97.1 | 1680 passed / 0 failed / 6既有ignored；452条target结果 | 包含行为测试、源码/标记检查和替身测试；不能汇总成全部外部工具通过 |
| 位运算、DAG、反馈 | `just chisel-numeric-check` | 13 Rust integration通过；18组实际 Scala/JVM→RTL→Icarus通过 | 同宽1/8/32/64及指定异宽矩阵；不是任意精度 |
| 内存边沿 | 同上 | native、两类generated Rust、direct Verilog、Chisel实际执行通过 | FIRRTL内存未证明；碰撞不跨后端比较 |
| 层级后端 | 同上 | 两种模块次序实际 direct/FIRRTL/Chisel RTL执行通过 | native/generated/CLI仿真明确拒绝层级 |
| 旧FR28门禁 | `scripts/chisel-fr28-compile-required.sh …counter.scala` | Java17/Scala2.13.18/Chisel7.15 实际编译通过 | 固定夹具compile-only，不是RTL行为测试 |
| 性能与分配 | `cargo bench -p bitloom-sim --bench baseline`；独立`bench-alloc`构建 | 32级流水线独立checksum、多次交替计时、Callgrind3.22.0和分配请求计数 | 见独立剖析报告；仪器化耗时不是本机吞吐 |
| formal（内部） | workspace 的 bounded exhaustive / bridge tests | Rust有限输入/深度证明路径 | 不等同SymbiYosys |
| formal（外部） | `just formal-sby-check`；项目脚本钉Yosys/sby/Z3 | 本轮未执行，PATH无sby/Yosys | 不标通过，不以替身测试代替 |
| SystemC LT/AT | FR101/FR107 C++ smoke tests；声明2.3.3、实际允许2.3.x | 实际SystemC2.3.4 + g++编译执行，两目标7+5项通过 | 指定LT/AT smoke，不是完整SoC/TLM等价 |
| FR164 CIRCT sim门禁 | `just circt-external-sim-check` | 独立门禁本轮未运行；现脚本为外部编译+另一Rust夹具 | 不能称为执行该外部编译输出RTL |
| Style Guide/linter | FR165/176/181/188检查函数及workspace测试 | 标记/源码结构验收 | 未实际运行scalafmt/wartremover，不称真实lint |
| IDE/store/ChiselSim | workspace相关侧车、manifest、launch测试 | 配置与文件验收，部分替身 | 不宣称本轮运行GUI、上架store或实际ChiselSim |
| 发布预演 | workspace FR149–152调用`cargo publish --dry-run --allow-dirty` | 最终workspace通过，包含实际firrtl/viz/CLI/LSP打包预演 | 无上传，不代表依赖同时发布 |
| API兼容 | `scripts/semver-check.sh`；registry baseline三库 | 三库各196 checks通过 | 覆盖prelude/sim/firrtl公开API，不代表行为未修正 |

## 本轮执行记录

原始 JVM 红灯、实际 RTL 红灯与绿色日志位于 [evidence](benchmarks/backend-boundary-2026-09-20/)。最终 `cargo test --workspace --no-fail-fast` 返回0：1680 passed、0 failed、6个既有ignored doctest，共452条target结果；`cargo fmt --all -- --check`、三库 `scripts/semver-check.sh`（每库196检查）通过。workspace环境PATH含真实Icarus，但未设RHDL_FIRTOOL_PATH；FIRRTL/Chisel强制执行证据来自另行通过的严格门禁。三路审阅后的局部修复已完成；严格门禁最终得到13 tests/18 cases通过，全工作区及API检查随后重跑。详见[验证摘要](benchmarks/backend-boundary-2026-09-20/verification.txt)。CPU/分配环境、原始样本与分析见[剖析报告](benchmarks/backend-boundary-profile-2026-09-20.md)。
