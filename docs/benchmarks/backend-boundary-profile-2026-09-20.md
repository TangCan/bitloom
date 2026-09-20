# 32级寄存流水线剖析 — 2026-09-20

本轮真实测得：该较大工作量中 compiled 没有稳定吞吐优势；两种引擎每拍都产生约千次分配请求。Callgrind 的调用归因确认字符串克隆、分配释放和字符串键 BTree 查找是实际成本来源。本轮保留语义修复后的实现，未优化 tick 热路径；没有候选实现的前后收益证据，不把热点占比当作预计加速比。

测量快照位于最终审阅修复之前；随后只调整 freeze 驱动规范化、构造期拓扑调度与后端生成。本文的原始二进制和成对样本保持一致，不将它们冒称为最终提交的重新测量；tick 热路径未因该审阅修改。

## 工作量与方法

`crates/bitloom-sim/benches/baseline.rs` 新增 `pipeline32`：32个32位流水寄存器，另有源计数器；每级两条 add/XOR 组合节点，反向声明；每拍先稳定组合值再同时推进所有级。独立参考使用 `[u32; 32]` 和上一拍快照计算，不调用模拟器、HIR evaluator 或其辅助函数。每次输出累积滚动 checksum，所有预热/样本都断言参考值。

原生计时使用 release 优化+调试符号构建，CPU8固定，2000拍预热后运行5对各20000拍，交替 interpreter/compiled 先后。每样本重新构造、复位；展开/构造/输入设置/复位/打印不计时，`tick`、输出读取和checksum计入。保留默认coverage记账，不开波形。共享主机、不独占核心或固定频率；环境及完整样本见 [environment.txt](backend-boundary-2026-09-20/environment.txt)、[native-cpu8.csv](backend-boundary-2026-09-20/native-cpu8.csv)。

| 引擎 | min ns/拍 | median ns/拍 | max ns/拍 | checksum |
|---|---:|---:|---:|---|
| interpreter | 107514.24 | 110055.19 | 117470.24 | `b8567df545dc686d` |
| compiled | 109296.12 | 110389.61 | 114523.09 | `b8567df545dc686d` |

中位数差约0.30%，样本范围明显重叠，不能宣称 compiled 在此设计加速。该实现的组合前后两次计算与旧基线语义不同；本报告不拿旧的小夹具结果作为性能回归或优化收益对照。

## 分配：独立构建、仅测量区启用

`--features bench-alloc` 只在 benchmark 二进制安装计数 allocator。正常计时构建没有该 allocator，也没有每次分配上的计数开关读取。计数构建在构造、复位完成后才启用，在测量循环及输出读取/checksum完成后关闭；不计构造、预热、打印、参考模型、销毁。全局计数适用于本单线程夹具。

每引擎3个1000拍样本的请求数量/字节完全一致，checksum均为 `5c22e11962c54992`。分配量是 `alloc`/`alloc_zeroed`/`realloc` 请求次数与请求字节之和，包含重复分配，不是保留内存、RSS或峰值。`realloc` 按新的请求大小计；不把计数模式耗时当成本机吞吐。

| 引擎 | 请求/1000拍 | 请求字节/1000拍 | 平均请求/拍 | 平均请求字节/拍 |
|---|---:|---:|---:|---:|
| interpreter | 1,108,447 | 48,158,154 | 1108.447 | 48158.154 |
| compiled | 1,175,447 | 51,604,154 | 1175.447 | 51604.154 |

原始计数：[allocations.csv](backend-boundary-2026-09-20/allocations.csv)。compiled 并没有减少该夹具的分配量；其顺序/组合调度克隆依然存在。

## CPU 调用归因：实际 Callgrind

Linux `perf_event_paranoid=4` 拒绝 perf 事件访问，保留了[实际失败日志](backend-boundary-2026-09-20/perf-unavailable.log)。改用真实用户态 **Valgrind/Callgrind3.22.0**，每引擎独立进程、1000拍×3样本、预热0；使用与原生计时相同的带符号release二进制。不是stub，也不是从源码猜测热点。

Callgrind记录 `Ir`（动态指令计数及调用关系），不是perf硬件采样、CPU周期或原生墙钟。记录范围包括进程启动、构造、参考和全部样本；约99.83%的指令落在 `Sim::tick` 的含被调函数归因中，外围开销很小。两个进程可与其他验证并行；仪器化墙钟不用于吞吐比较。内联函数和多个调用层的 inclusive 数字互相重叠，**不能相加**；部分libc符号只有地址，未擅自命名其函数。

| 指标/调用（inclusive） | interpreter | compiled |
|---|---:|---:|
| 总 Ir | 2,591,478,490 | 2,612,210,521 |
| `String::clone` | 27.49% | 29.08% |
| `malloc` | 27.25% | 29.50% |
| `free` | 13.08% | 13.74% |
| `BTreeMap<String,u64>::insert` | 13.86% | 13.73% |
| `PortValues::get` | 12.34% | 12.19% |

保留[interpreter inclusive](backend-boundary-2026-09-20/callgrind-interpreter-inclusive.txt)、[self](backend-boundary-2026-09-20/callgrind-interpreter-self.txt)、[compiled inclusive](backend-boundary-2026-09-20/callgrind-compiled-inclusive.txt)、[self](backend-boundary-2026-09-20/callgrind-compiled-self.txt)，以及可重新注释的原始 [interpreter](backend-boundary-2026-09-20/callgrind.interpreter.gz) / [compiled](backend-boundary-2026-09-20/callgrind.compiled.gz) gzip 文件。两份运行日志也保留独立checksum。文本日志及注释报告仅清理行尾空白；原始 Callgrind gzip 数据未改写。

证据支持下一次以减少热路径 String/AssignExpr 克隆和分配作为局部实验；当前没有对优化后的实现计时，因此不宣称加速、不进行大规模存储布局重构。任何后续优化都必须保留本轮边沿、拓扑、存储器和实际RTL矩阵。

## 重跑

```bash
# 原生吞吐；下面工作量/预热可调整，samples至少3。
CARGO_PROFILE_BENCH_DEBUG=1 cargo bench -p bitloom-sim --bench baseline --no-run
BITLOOM_BENCH_WORKLOAD=pipeline32 BITLOOM_BENCH_CYCLES=20000 \
  BITLOOM_BENCH_SAMPLES=5 BITLOOM_BENCH_WARMUP=2000 \
  taskset -c 8 target/release/deps/baseline-<本次编译打印的hash>

# 分配独立构建；只能用该次cargo打印的二进制，不与原生构建混用。
cargo bench -p bitloom-sim --bench baseline --features bench-alloc --no-run
BITLOOM_BENCH_WORKLOAD=pipeline32 BITLOOM_BENCH_CYCLES=1000 \
  BITLOOM_BENCH_SAMPLES=3 BITLOOM_BENCH_WARMUP=0 \
  target/release/deps/baseline-<计数构建hash>

# CPU归因使用第一个（不含bench-alloc）二进制；分别运行两种engine。
BITLOOM_BENCH_WORKLOAD=pipeline32 BITLOOM_BENCH_ENGINE=interpreter \
  BITLOOM_BENCH_CYCLES=1000 BITLOOM_BENCH_SAMPLES=3 BITLOOM_BENCH_WARMUP=0 \
  valgrind --tool=callgrind --callgrind-out-file=callgrind.interpreter \
  target/release/deps/baseline-<原生构建hash>
callgrind_annotate --auto=no --inclusive=yes callgrind.interpreter
# 把engine和输出名改成compiled后重复。
```

本环境未全局安装Valgrind，实际使用 `/tmp/bitloom-profile-tools/extracted/usr/bin/valgrind` 并设置 `VALGRIND_LIB=/tmp/bitloom-profile-tools/extracted/usr/libexec/valgrind`。可在正常安装的Valgrind3.22.0下运行上面命令。CPU8只是本主机可用核心之一，重跑需选可用CPU并记录环境。默认原有counter/arithmetic/memory工作量仍保留；`BITLOOM_BENCH_WORKLOAD=all` 跑四组，`BITLOOM_BENCH_ENGINE=both|interpreter|compiled` 选择引擎。
