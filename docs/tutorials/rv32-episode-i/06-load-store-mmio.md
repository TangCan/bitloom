# Chapter 6 — Load/Store 与 MMIO

引入：`LW`/`SW`、DMEM(16)、LED MMIO 地址 `0x100` → `led_out`。

验收：

```bash
cargo test -p rv32_core tick_sw_mmio_led_golden
cargo run -p bitloom -- build --package rv32_core --manifest-dir . --out-dir /tmp/rv32-out
```

主路径是 Bitloom `cargo` / `tick` / `build`，不以 Make/SBT 为必装。
