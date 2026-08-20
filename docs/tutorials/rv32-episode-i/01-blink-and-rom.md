# Chapter 1 — blink / ROM 概念

一步一变：先把「指令从哪来」说清楚。

Episode I 核把 **指令存储器放在夹具外**：每拍由测试写入端口 `instr`。这等价于教学上的 ROM / 指令总线模型，便于逐步加译码而不先做 SyncReadMem 取指。

验收：阅读 `examples/rv32_core/src/lib.rs` 模块端口；运行 `cargo test -p rv32_core elaborate_ok`。
