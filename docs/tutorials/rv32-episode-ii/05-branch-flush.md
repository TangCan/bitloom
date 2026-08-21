# Chapter 5 — 分支 flush

一步一变：控制冒险 — **predict-not-taken**；taken 时 flush 错误路径并 redirect PC。

## 引入

默认每拍 PC+4。EX 判定 `BEQ` taken 时：

- 冲刷 IF/ID 与 ID/EX（NOP / bubble）
- 下一拍 PC = 分支目标
- 错误路径上会写 RF 的指令**不得**提交

比较数可经 EX 转发（紧邻生产者 → BEQ）。

## 验收

```bash
cargo test -p rv32_pipe tick_beq_taken_flush_wrong_path_not_committed
```

类型：`tick`。

至此 Episode II **流水主线**（FR64）完成；可选 CSR/trap 见 [Ch.06](./06-csr-m-trap.md)（`examples/rv32_priv`，NFR32）。
