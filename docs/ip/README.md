# Bitloom 一级 IP 索引（FR37 / FR48）

公开产品名 **Bitloom**（crates.io / CLI：`bitloom`）。与 [samitbasu/rhdl](https://github.com/samitbasu/rhdl) **无关**。

设计 crate **只依赖 [`bitloom-prelude`](../../crates/bitloom-prelude)**。五类一级 IP 与黑盒均通过：

```rust
use bitloom_prelude::ip::{
    SyncFifo, UartTx, SpiMaster, I2cMaster, Axi4LiteSlave, ExtBlackBox,
};
use bitloom_prelude::Elaboratable;
```

例化：`T::elaborate()` → `bitloom_vlog::emit` → `bitloom_sim::Sim::tick`。

## 五类 + 黑盒

| 类 | 包路径 / 类型 | Smoke 命令 | 已知限制 |
| --- | --- | --- | --- |
| **FIFO** | `bitloom_prelude::ip::SyncFifo` | `cargo test -p bitloom-prelude --lib sync_fifo` | depth-1 skid；非异步跨域 FIFO |
| **UART** | `bitloom_prelude::ip::UartTx` | `cargo test -p bitloom-prelude --lib uart_tx` | 字节保持寄存器；非波特率移位 / 全双工 |
| **SPI** | `bitloom_prelude::ip::SpiMaster` | `cargo test -p bitloom-prelude --lib spi_master` | **主**设备字节缓冲；非 CPOL/CPHA / 多 CS / 从模式 |
| **I2C** | `bitloom_prelude::ip::I2cMaster` | `cargo test -p bitloom-prelude --lib i2c_master` | **主**设备字节缓冲；非多主仲裁 / clock stretch / 从模式 |
| **AXI** | `bitloom_prelude::ip::Axi4LiteSlave` | `cargo test -p bitloom-prelude --lib axi4_lite` | **AXI4-Lite 最小从**（ADDR=8, DATA=32）；非 Full AXI / 非互联 |
| **黑盒** | `bitloom_prelude::ip::ExtBlackBox` + `vendor_blackbox_v()` | `cargo test -p bitloom-prelude --lib blackbox` | 仅端口；不内联子 HIR；vendor `.v` 旁路 |

联验夹具：`examples/ip_box`（`cargo test -p ip_box`）再导出 FIFO/UART/黑盒。

## CI / `just test`

上述 `bitloom-prelude` IP 单元测试与 `ip_box` 均在 workspace `just test`（`cargo test --workspace`）内，默认 CI 可触达。

## 例化片段

```rust
use bitloom_prelude::ip::UartTx;
use bitloom_prelude::Elaboratable;

let hir = UartTx::elaborate()?;
// emit(&hir); Sim::new(hir).tick();
```

实现源码：[`crates/bitloom-prelude/src/ip.rs`](../../crates/bitloom-prelude/src/ip.rs)。
