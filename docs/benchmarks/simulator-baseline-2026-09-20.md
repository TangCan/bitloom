# Bitloom 仿真器 release 基线 — 2026-09-20

本轮 CPU 固定、交替引擎顺序运行中，compiled schedule 的每拍中位耗时在计数器夹具上约减少 12%，在算术夹具上约减少 11%；不代表任意设计的普遍收益。存储器设计的两种引擎选择均执行解释器，微小时间差不能视为存储器加速。现有证据支持保留实现，先收集剖析数据，再决定优化或模块拆分。

## 复现

```bash
cargo bench -p bitloom-sim --bench baseline
# Linux 可选：固定到一个可用逻辑 CPU。
taskset -c 8 cargo bench -p bitloom-sim --bench baseline
# 显式指定工作量（以下也是默认值）：
BITLOOM_BENCH_CYCLES=200000 BITLOOM_BENCH_SAMPLES=5 \
  cargo bench -p bitloom-sim --bench baseline
```

`benches/baseline.rs` 使用 Cargo 优化 bench profile，不引入基准框架依赖。展开设计不计时；每个样本重新构造并复位仿真器。每种设计先让两种引擎各完成 20,000 拍预热，再测量五对各 200,000 拍的样本；第 1/3/5 对先 interpreter，第二/四对先 compiled，减少随时间漂移与引擎顺序的混淆。每种引擎单独排序求中位数；偶数样本取中间两项的平均。计时包含 `Sim::tick`、读取 `out` 与滚动校验和，不含构造、展开、复位及输入设置。`black_box` 防止工作被优化掉。所有预热及正式样本的校验和均与独立 Rust 参考实现断言比较，错误将使基准失败。

工作量：

- 计数器：一个 32 位寄存器逐拍递增，一条输出连接。
- 算术：计数器后接 16 个依赖的 32 位加法/XOR/左移/右移操作；常量 `0x9e3779b9`，移位 5 位，中间值按 32 位截断。
- 存储器：256×32 异步读存储器，每拍将拍前计数器写到固定地址零，递增计数器并读取输出。这隔离了存储器执行路径开销，**不是**真实缓存局部性或带宽基准。`TickEngine::Compiled` 检测到存储器后调用解释器，输出中明确标为 `effective_engine=interpreter`。

不记录波形；保留仿真器正常记账开销。本基线测量复位后的周期吞吐，不代表启动、展开、生成的独立功能 Rust 或波形输出性能。

## 环境与证据

- 源码：基于 `b1d2adb594edf20ba601e822ecd0b7b720883560` 的维护工作树；报告随源码提交，不代表独立已发布版本。
- Rust 1.97.1（`8bab26f4f`、LLVM 22.1.6），edition 2024，默认优化 bench profile。
- Linux x86_64、内核 6.8.0-100；AMD EPYC 7532，32 核/64 逻辑 CPU，启用动态频率提升。
- 共享主机。CPU8 固定运行开始于 2026-09-20 16:01:38 +08:00，负载均值 8.53 / 9.78 / 11.15。亲和性防止迁移，但不独占核心、不固定频率。旧方法的两轮样本先跑完全部 interpreter、再跑 compiled；其中未固定 CPU 的一轮还与编译重叠。它们仅作为历史记录保留，不作为当前主要比较。
- [交替运行环境](simulator-baseline-2026-09-20-alternating-environment.txt)、[CPU8 交替顺序全部样本](simulator-baseline-2026-09-20-alternating-cpu8.txt)。

每行五个样本，单位 ns/拍，越小越快：

| 工作量 | 请求引擎 | 实际引擎 | 最小 | 中位 | 最大 |
|---|---|---|---:|---:|---:|
| 计数器 | interpreter | interpreter | 2765.83 | 2783.26 | 2818.46 |
| 计数器 | compiled | compiled | 2438.29 | 2458.76 | 2475.49 |
| 算术 | interpreter | interpreter | 13363.27 | 13539.68 | 13977.25 |
| 算术 | compiled | compiled | 12002.90 | 12081.67 | 12527.06 |
| 存储器 | interpreter | interpreter | 3096.34 | 3103.77 | 3123.90 |
| 存储器 | compiled | 回退 interpreter | 3063.95 | 3100.09 | 3133.29 |

200,000 拍下，两种请求引擎的全部样本均得到以下独立核验的校验和：

| 工作量 | 校验和 |
|---|---|
| 计数器 | `df1fa2dafed646ff` |
| 算术 | `a1b7250b42de2f26` |
| 存储器 | `0f5a029b41b5a716` |

历史旧顺序记录：[未固定 CPU 原始样本](simulator-baseline-2026-09-20-unpinned.txt)、[CPU8 原始样本](simulator-baseline-2026-09-20-cpu8.txt)、[当时环境](simulator-baseline-2026-09-20-environment.txt)。两轮均先跑完全部 interpreter 样本、再跑 compiled；不可排除时间顺序混淆。旧 CPU8 中位数分别为计数器 2755.49 / 2453.26，算术 13331.43 / 12082.45，存储器 3037.70 / 2989.80 ns/拍。旧未固定 CPU 的中位数（interpreter / compiled 请求）为计数器 2721.93 / 2435.94，算术 14356.17 / 12003.11，存储器 3072.09 / 3090.80 ns/拍。个别计数器 compiled 样本达 4052.39，算术 interpreter 达 17809.10 ns/拍，说明单次采样会误导判断。

另执行 `BITLOOM_BENCH_CYCLES=1000 BITLOOM_BENCH_SAMPLES=4 taskset -c 8 cargo bench -p bitloom-sim --bench baseline` 验证偶数样本路径（[smoke 原始输出](simulator-baseline-2026-09-20-even-smoke.txt)）。检查交替顺序与中间两项平均均通过；短工作量仅验证行为，不纳入性能比较。

## 决定

本次不追加性能优化或源码拆分。compiled schedule 在两个纯寄存器夹具上有适度收益，不能据此宣称任意设计加速。存储器两种选择执行同一解释器，约 0.1% 的中位差处于主机波动范围，不是加速证据。

下一步性能调查应先剖析 `tick` 中的分配、模块/调度克隆和字符串键查找，再原样复跑这些带校验和的工作量，并加入更具代表性的大设计。这些候选来自代码检查，尚不是实测热点；仅拆分文件不能改善所测吞吐。后续优化须保持位向量与独立生成 crate 回归，并在可比负载下报告多次前后样本。本报告提供可复现基线，不设 CI 时间阈值，也不宣称相对未测旧版的性能变化。
