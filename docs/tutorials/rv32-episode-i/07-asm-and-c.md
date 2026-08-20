# Chapter 7 — 手写 asm 与 GNU C（简化）

一步一变：用仓库内编码助手拼指令字，再对照外部 `riscv64-unknown-elf-as`（可选，非必装）。

```rust
use rv32_core::{enc_addi, enc_sw};
let prog = [enc_addi(1, 0, 0x100), enc_addi(2, 0, 0xA5), enc_sw(1, 2, 0)];
```

C 侧：写极简函数生成同样的立即数常量即可；完整 newlib/链接脚本超出 Episode I。

验收：`cargo test -p rv32_core subset_minimal_filter_program`。
