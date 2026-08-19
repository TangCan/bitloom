# Addendum — RHDL 阶段二 PRD

技术 HOW、选项矩阵与调研绑定。不替代 `prd.md` 中的 FR。

## 机制选项（FR23 CDC）— 已由 AD-22 关闭

**决议（2026-08-19）：** Clash 式 phantom 域；合法跨越仅 `DoubleFlop` / `SyncFIFO`。见架构脊柱 AD-22。

| 选项 | 优点 | 缺点 | 状态 |
|------|------|------|------|
| Clash 式 phantom domain | rustc 静态拦住误跨域 | API 噪声；学习成本 | **ADOPTED (AD-22)** |
| Spinal 式 ClockDomain + 编译检查 | 生产差异化强 | 实现复杂 | 未采纳 |
| Chisel 式库级 FIFO + 文档纪律 | 实现快 | 无语言强制 | 未采纳 |

## Mem 语义锚（FR26 / AD-21）

- **语言表面：** 暴露 CHIRRTL 友好名（`Mem` / `SyncReadMem`；文档对应 `cmem`/`smem` 语义）。**已决议 2026-08-19。**
- **降级与互转：** 锚定 FIRRTL 规范 `mem` / `firrtl.mem`；CHIRRTL 方言文本不是 FrozenHir↔`.fir` 合同。
- 双口跨时钟仅经命名 CDC FIFO（与 FR23 / AD-22 衔接）。

## NFR3 实现草图

- 资产：`firtool-1.155.0` / `firrtl-bin-linux-x64.tar.gz` + `.sha256`（调研核验 [22]）。
- 缓存目录；`RHDL_FIRTOOL_PATH`；勿默认信任 PATH。
- 注意上游已有 1.156.0；升级策略见 NFR12。

## HLS（FR35）

- 发射 C/LLVM/XLS IR → 调用 Bambu/XLS；无自研 scheduler（调研 [17][18][19]）。
- CIRCT HLS 非生产路径参考，不作依赖。

## 表面加厚顺序（FR22 与 P1）

调研建议：IR+comb/seq/ops+emit+sim 可写性 → Mem → 多时钟 → FST → HLS。rust-hdl 过薄表面导致重写的教训进入 FR22 动机（调研 [15][16]）。

## Rejected for this PRD

- 把 later-product 继续作为「无 FR ID 的保留清单」唯一真相——与本次升格意图冲突。
- 把 HLS 或 FST 提到 P0——与证据排序冲突。

## Phase-3 identity supersession (2026-08-19)

阶段二 Vision / FR21 / SM-4 中的 crates.io 发布名 **`rhdl-rs`** 已被阶段三 **FR41** 取代：公开产品名 **Bitloom**，发布名 **`bitloom`**。详见命名研究与 Epic 11；本 addendum 不改阶段二 FR 编号，但实现与文档须按 Bitloom 执行。
