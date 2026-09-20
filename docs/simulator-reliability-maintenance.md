# 仿真可靠性维护 — 2026-09-20

本次维护保留既有 Bitloom 公开 API、Rust 1.97.1、唯一 FrozenHir 与 firtool 1.159.0 产品钉。不发布包，不重开 sprint，不宣称 FR189 交付或 NFR91 清空。

## 复现与修复

基线 `b1d2adb594edf20ba601e822ecd0b7b720883560` 上，新增位向量回归首先复现一位 `0 - 1` 错误：原生仿真返回 `18446744073709551615`，独立一位参考值为 `1`。该提交的隔离副本还复现 `1 << 64 == 1`、`1 << (1 << 32) == 1`（均应为零），以及负数 64 位算术右移零位时 panic。

原生解释器与 compiled schedule 现在按声明位宽截断输入、中间网、寄存器更新、待读数据与存储器写入；进程内及独立生成的功能模型采用相同规则。逻辑移位的大位移归零，不再对 64 取模；算术右移的大位移按符号填充，零位移不会触发移位 64 位的 panic。手写 mixed-model 八位计数器也在 256 处回绕。

真实 RTL 执行另暴露后端问题：Verilog 符号扩展现在支持一位标量源；FIRRTL 使用公开 top、现代 connect/regreset 语法和具体同步复位端口，导入时保留 HIR 复位类型。左移限制动态移位操作数位宽，并显式使超宽移位归零；有符号输入通过无符号转换保持逻辑移位语义。所测算术设计已可由既有产品钉编译并执行。异宽回归进一步覆盖 `UInt32 << UInt1`、`UInt8 << UInt8 → UInt16`、窄目标等组合：左移的归零界限以目标位宽为准，切片不超移位端口声明位宽；signed 逻辑结果扩宽先零扩展再转换目标类型，左右移均可通过规范编码导入往返。

仿真值仍存储于 `u64`。接受的 UInt128 直通在原生、compiled、进程内和真实编译的生成功能 crate 中保留低 64 位；所有宽度掩码对 `width >= 64` 直接保留该低字，避免 Rust 移位溢出。这是既有低字行为的兼容性修复，不是任意精度仿真。原轮维护未覆盖 Chisel，其 signed Shr 差异当时单独登记；后续本日维护已通过真实 Scala/JVM→RTL 矩阵修复，见[后端边界证据](backend-boundary-evidence-2026-09-20.md)。

原 `emit_cycle_crate_compiles` 仅检查字符串；现在真正调用独立 Cargo 测试，逐拍检查 310 拍计数器（含回绕及第二次复位）。另外分别编译 cycle 与 functional crate，检查 40 拍同步存储器，与小型参考模型、原生 Sim 及进程内模型同时对照。向 16 位端口送入未预先截断的原始值，通过溢出的 16 位和写入 8 位存储器，再分别读入 4 位和 16 位寄存器，验证输入、写入、窄读截断及写使能抑制、复位、读延迟。该测试先暴露 `RegD <- MemRead` 缺少 builder 生成分支，现已补齐。每次调用使用唯一目录和独立 Cargo target，失败时保留产物供检查。

## 回归与命令

`crates/bitloom/tests/simulator_bit_vectors.rs` 覆盖 1、8、32、64 位、正负数，以及 0、width−1、width、63、64、65、2³²、最大可表示移位值（先按声明的移位输入位宽约束）。原生解释器、compiled schedule、进程内功能模型、真实编译的独立功能 Rust、Verilog、经 firtool 降级的 FIRRTL，均对照独立位向量参考值。涵盖溢出/下溢、后续运算前的中间截断、位逻辑、相等与有符号/无符号比较、零/符号扩展。另检查超宽原始输入、复位 2/3 和 Bool 使能 2/3 的一位截断，并比较返回的输入端口值；Bool 使能 2/3 的同组向量也实际编译执行独立 FunctionalSim。

```bash
cargo test -p bitloom-sim
cargo run -q -p bitloom -- firtool ensure
# 使用上条命令打印的可执行文件所在目录：
export RHDL_FIRTOOL_PATH=/path/to/firtool/1.159.0/bin
# PATH 须含 iverilog、vvp。
BITLOOM_REQUIRE_RTL=1 BITLOOM_REQUIRE_FIRRTL=1 \
  cargo test -p bitloom --test simulator_bit_vectors -- --nocapture
cargo test --workspace
cargo fmt --all -- --check
bash scripts/semver-check.sh
bash scripts/test-semver-check.sh
```

严格环境变量使缺工具返回失败。普通本地运行明确报告未执行的外部路径；CI 的 Rust job 必须运行直接 RTL，对应 CIRCT job 必须运行完整 FIRRTL 对照。为兼容 Icarus，firtool 禁用随机初始化和自动局部变量；testbench 在检查前显式复位寄存器。

SemVer 脚本现在覆盖 `bitloom-prelude`、`bitloom-sim`、`bitloom-firrtl`。失败传播回归用 shell 替身检查包括 `--release-type` 的完整参数，分别验证未设覆盖时的 minor 默认值和显式 major 覆盖，检查包清单，并分别注入每个包失败和缺工具失败；它补充、不能替代真实注册表基准门禁。

完整工作区检查还发现两处历史版本断言硬编码 1.0.0，以及一个扫描任意 1.1.0 依赖而误报 workspace 版本的测试。现通过 Cargo metadata 检查当前包与依赖，并以 CHANGELOG 保留历史发布事实。真实 CLI 发布预演另暴露 `bitloom-viz ^1.1.1` 尚无注册表候选；该 crate 源码自既有 1.1.0 发布以来未变，故仅将 workspace 的兼容依赖下限恢复为 `1.1.0`，本地路径包仍为 1.1.1，不降低检查、不执行发布。

PIPE.md 及相关教程说明既有 next-register 统一提交语义。README 将历史 1.0/1.1.0 发布证据与本地 manifest 版本分开，并为历史 live-tip 观测补上日期。

可复现工作量、完整样本、环境、校验和与优化决定见[release 性能基线](benchmarks/simulator-baseline-2026-09-20.md)。

最终完整回归还复现 FR158 两个测试并发修改 PATH 的竞态；两段测试 override 现用同一个互斥锁隔离，保留缺工具与假工具断言。

最终验证：严格跨后端 integration 4 项、sim 38 项、FIRRTL 单元 19 项通过；带真实 firtool/Icarus 的完整 workspace 测试、三库真实 SemVer、参数/失败传播、fmt 与差异空白检查均通过。完整 workspace 保留 6 个既有忽略 doctest，不代表独立 JVM/formal/SystemC 门禁全部执行。下一步见[项目分析](project-next-steps-2026-09-20.md)。

后续边沿修复：当前 `tick` 自动拓扑settle后采样，同时提交寄存器/存储器；内部碰撞read-before-write，SyncReadMem读阶段与目的寄存器分开。原轮测试中写在先即读新值、采上拍组合值的预期已由独立硬件参考更正；详情及RV32/FIFO调用方迁移见[本日后续记录](backend-boundary-evidence-2026-09-20.md)。
