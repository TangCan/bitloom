---
title: '仿真一致性、生成产物回归、兼容性与性能基线维护'
type: bugfix
created: '2026-09-20'
status: done
route: dispatch
baseline_commit: b1d2adb594edf20ba601e822ecd0b7b720883560
review_loop_iteration: 0
context:
  - AGENTS.md
  - _agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md
---

<frozen-after-approval reason="用户已明确授权完整四项工作，按指定顺序执行">

## Intent

修正基础位运算跨后端差异，为生成 cycle crate 建立真实编译和逐拍执行回归，补齐 FIRRTL 兼容性门禁并纠正文档，最后建立可重复性能基线、据实决定后续优化。四项均须完成，不能以部分测试通过代替整个目标。

## Boundaries & Constraints

保持 Bitloom / prelude-only、唯一 FrozenHir、Rust 1.97.1 和所有既有稳定公开 API。以维护已交付能力为范围，不升级 firtool、不宣称 FR189 交付、不清空 NFR91、不发布或推送。本次不新增 epic，不改变 sprint 完成状态。先复现后修复；对未执行或缺依赖的外部门禁如实说明并补齐必要验证。四项按顺序实施；用户已授权全部范围，无需再次确认范围。

## I/O & Edge-Case Matrix

| 场景 | 输入 | 期望 |
|---|---|---|
| 逻辑移位 | 合法位宽，移位 0、width-1、width、63、64、65、最大可表示值 | RTL 位向量语义，大移位归零而非模 64；输出截断 |
| 算术右移 | 正负数，移位 0、width、64、65 | 符号填充；无 Rust 越界 panic |
| 基础运算 | 加减溢出、窄寄存器、逻辑运算、比较与扩展 | 所有已支持仿真路径按目标位宽一致；中间结果也截断 |
| cycle 生成 | 计数器与带写使能存储器、复位、多拍输入 | 生成独立 crate 实际 cargo 编译运行；每拍端口等于预期及原生 Sim |
| 兼容性门禁 | prelude、sim、firrtl | 三库均受检查；任一失败返回失败 |
| 基线 | 小计数器、算术数据通路、存储器设计 | 多次 release 测量、固定工作量、校验和、环境与复现命令记录 |

</frozen-after-approval>

## Code Map

- `crates/bitloom-sim/src/lib.rs`：Sim eval/seq/comb，当前 Shl/Shr 与 SAR 使用模 64；写回缺目标位宽截断，核实并修复。
- `crates/bitloom-sim/src/generate.rs`：进程内 GeneratedFunctional 与独立 Rust 模板，须一起修复，不能只修一条路径。
- `crates/bitloom-sim/src/cycle.rs`：生成 HIR builder 与 cycle wrapper；现 `emit_cycle_crate_compiles` 仅查文本；resolve_dep 支持树内路径依赖。
- `crates/bitloom-vlog/src/lib.rs` / `crates/rhdl-firrtl/src/lib.rs`：RTL/FIRRTL 对照；后端生产依赖不能互相增加，跨后端测试可放 CLI integration tests。
- `scripts/semver-check.sh`：PACKAGES 缺 bitloom-firrtl；CI 已调用脚本。
- `examples/rv32_pipe/PIPE.md`：就地更新说明已过时，现 Sim 使用 next_regs 提交。
- `README.md` / `Cargo.toml` / `crates/bitloom-sim/Cargo.toml`：历史 1.1.0 宣称与不同本地版本须区分，不把本地版本等同于已发布。
- `crates/bitloom-sim/src/engine.rs`：compiled schedule；benchmark 必须记录存储器路径回退解释器，不能误称加速。

## Tasks & Acceptance

**Execution:**
- [x] 新增基础运算跨后端回归，证明旧版失败，再修复 native/compiled/functional/emitted functional 位宽与移位语义；尽可能执行真实 RTL 对照。
- [x] `cycle.rs` 增加独立 crate 真正 cargo test 和多拍存储器/复位/写使能回归，避免共享 target 锁及临时目录碰撞。
- [x] `semver-check.sh` 纳入 FIRRTL，验证真实门禁与失败传播；纠正 PIPE.md 和 README 版本说法，保留历史结项事实。
- [x] 增加可重复 release benchmark 与基线报告，记录多次统计、环境、工作量、校验和、限制，并基于测量决定是否优化或拆分。

**Acceptance Criteria:**
- Given 边界输入矩阵，when 执行各支持路径，then 与独立位向量预期及实际 RTL 一致，原有回归不倒退。
- Given 生成 cycle crate，when 在独立目录编译执行，then 每拍输出和 native/reference 对照通过，非字符串检查冒充编译。
- Given 三库稳定表面，when 执行兼容性脚本，then 所有目标均检查且错误传播；文档不声称未验证发布状态。
- Given 相同 benchmark 命令，when 多次执行，then 输出可比较的 release 基线并给出有证据的优化决策。

## Implementation Notes

用户的连续目标明确要求全部实施；无需新增范围审批。实施与审阅分离，保持四项执行顺序。可按发现添加必要回归，不能扩大为未授权产品功能。

## Spec Change Log

## Review Triage Log

| 来源 | 判定 | 证据与处理 |
|---|---|---|
| blind-1：>64 位截断 | high / patch | builder 接受 UInt128；新增通用 mask_width 调用会执行 1u64 << 128。保留 u64 低字语义，将 >=64 视为无需截断，补真实生成模型测试；不宣称任意精度支持。 |
| blind-2：窄移位量 | high / patch | firtool 1.159.0 实测 UInt32 << UInt1 的 bits(shift,4,0) 报 high 超出输入宽度；修正切片并补异宽 RTL 对照。 |
| blind-3：宽目标左移 | high / patch | builder 的 assign_shl 不限制目标同宽；新 guard 以源宽 8 提前清零，但 16 位目标应保留 1<<8=256。界限改用目标宽度。 |
| blind-4：signed Shr 导入 | high / patch | parse_expr 拒绝包含括号的 dshr 参数；asUInt 新包装变为 Ref，lookup 返回零。补对称 canonical 解析与往返断言。 |
| blind-5：Chisel signed Shr | medium / defer | chisel.rs 既有 Shr 使用 SInt >>，是算术移位；本次未修改该后端。登记独立 Chisel 数值一致性跟进，当前对拍证据明确只涵盖 Rust、Verilog、FIRRTL。 |
| blind-6：生成使能测试缺口 | medium / patch | 生成模板使能改动未被现有 Bool2/3 测试调用；扩展现有夹具为实际独立 FunctionalSim 编译执行。 |
| blind-7：存储器截断测试缺口 | medium / patch | 新 cycle 夹具事先 &0xff，无法区分写入和 pending 截断是否存在；builder 支持宽写入与窄读寄存器，补原始超宽输入及独立预期，覆盖 cycle/functional 生成。 |
| blind-8：样本引擎顺序 | low / patch | harness 按引擎整批运行，共享主机漂移会混入比较；交替采样后重测，保留旧样本并标明限制。 |
| blind-9：偶数样本中位数 | low / patch | samples>=3 允许偶数，现取上中位；改为两个中间值平均。默认五样本历史值不受影响。 |
| blind-10：SemVer 参数替身 | low / patch | 替身仅检查前三参数，无法抓住 --release-type 丢失；直接断言完整参数，并覆盖未设置 override 的默认路径。 |
| edge-1：窄移位量 | high / patch | 与 blind-2 同根因；firtool 实际报切片越界，统一修正。 |
| edge-2：宽目标左移 | high / patch | 与 blind-3 同根因；源宽 guard 与目标截断语义冲突，统一修正。 |
| edge-3：signed Shr 导入 | high / patch | 与 blind-4 同根因；新 asUInt 包装缺少 parser 对称分支，统一修正。 |
| verification-1：生成使能测试 | medium / patch | reviewer 已查验全部生成入口；native/in-process 的 Bool2/3 测试不能检测模板恢复为未截断读法，按其建议补独立编译。 |

补丁实施时，异宽 signed 目标进一步要求先对逻辑结果零扩展再 asSInt，避免 FIRRTL 连接时符号扩展；与 blind-3 的异宽目标修复一并处理，并由真实 RTL 矩阵验证。

最终完整回归暴露 FR158 两个既有测试并发覆盖进程 PATH（fake genhtml 在第二次查找时变为不可见）；为这两段 override 增加同一个测试内互斥锁，保持缺工具/假工具断言不变。独立 verification reviewer 再查确认：两段 override 与恢复均在锁内；同测试二进制其余断言不依赖 PATH。

## Verification

- 针对新增边界测试先红后绿，记录输出。
- `cargo test -p bitloom-sim`、相关 CLI integration tests、`cargo test --workspace`、`cargo fmt --all -- --check`。
- `bash scripts/semver-check.sh`；必要时使用有明确基准版本的本地 rustdoc 校验补充诊断，不隐藏失败。
- 新 benchmark 的 release 命令多次运行，保存测量报告。

### 最终执行结果（2026-09-20）

- 基线红灯：一位下溢、64/2³² 大移位与 SAR64 零移位 panic；新 cycle 存储器夹具揭示生成分支缺失。修复后均有回归覆盖。
- `cargo test -p bitloom-sim`：38 项通过；`cargo test -p bitloom-firrtl --lib`：19 项通过。
- `BITLOOM_REQUIRE_RTL=1 BITLOOM_REQUIRE_FIRRTL=1 cargo test -p bitloom --test simulator_bit_vectors`：4 项通过；同宽与异宽夹具实际运行 Verilog、firtool1.159.0/Icarus，独立 Rust 产物实际编译执行。
- 最终 `cargo test --workspace`：exit 0；运行时启用上述两个严格标志，并配置真实 firtool/Icarus。包含 CLI/库发布预演；6 个既有 ignored doctest 保持原状。该命令不等于所有独立 JVM/formal/SystemC 外部门禁。
- 最终三库 `bash scripts/semver-check.sh`：exit 0，注册表基准均通过；完整参数/默认值/失败传播替身通过。
- `cargo fmt --all -- --check`、`git diff --check`：通过。
- 性能：三轮共 90 个正式样本（其中最终交替顺序 30 个），校验和全部通过；另 24 个短样本验证偶数中位数与交替顺序，不作为性能比较。
- 三层审阅 14 条逐项判定；本批问题均修复。Chisel signed Shr 既有缺陷登记 deferred，未将 Chisel 纳入本次数值一致性宣称。最后发现的 FR158 PATH 竞态已修复并独立复查。
- 决定：暂不优化或拆分；优先处理 Chisel 数值一致性，再扩展支持边界测试与代表性工作量剖析。分析见 `docs/project-next-steps-2026-09-20.md`。

日志保留在本机 `/tmp/bitloom-maintenance-final-workspace.log`、`/tmp/bitloom-maintenance-final-semver.log`、`/tmp/bitloom-review-{sim,firrtl,vectors}.log`；可复现 benchmark 原始样本随 `docs/benchmarks/` 提交。
