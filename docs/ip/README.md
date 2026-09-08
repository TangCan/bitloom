# Bitloom 一级 IP 索引（FR37 / FR48 / FR82）

公开产品名 **Bitloom**（crates.io / CLI：`bitloom`）。与 [samitbasu/rhdl](https://github.com/samitbasu/rhdl) **无关**。

设计 crate **只依赖 [`bitloom-prelude`](../../crates/bitloom-prelude)**。五类一级 IP 与黑盒均通过：

```rust
use bitloom_prelude::ip::{
    SyncFifo, UartTx, SpiMaster, I2cMaster, Axi4LiteSlave, ExtBlackBox,
};
use bitloom_prelude::Elaboratable;
```

例化：`T::elaborate()` → `bitloom_vlog::emit` → `bitloom_sim::Sim::{settle,tick}`。  
同周期输入门控（如 `wr_en && !full`、AXI `*valid && *ready`）须在 `set_inputs` 后调用 `Sim::settle()` 再 `tick()`。

**无生成器闭包参数**（Epic 34 / FR82）；闭包定制叠加点为 **Epic 29**。

## Epic 34 / FR82 深度（相对 Epic 22 stub）

| 类 | Epic 22 stub | FR82（本索引当前） |
| --- | --- | --- |
| **FIFO** | depth-1 skid，无 full/empty | **非 stub：** depth-4 sync FIFO，`wr_en`/`rd_en`/`full`/`empty` |
| **UART** | 字节保持，无串行帧 | **非 stub：** 8N1 bit-bang（baud=`clk`），`tx` + busy 门控写 |
| **SPI** | 端口语义 stub | **非 stub（文档最小子集）：** Mode-0-ish MSB byte shifter，`cs_n`/`sclk`/`mosi` |
| **I2C** | 端口语义 stub | **非 stub（文档最小子集）：** START + 8 data + STOP bit-bang |
| **AXI** | 握手镜像 stub | **非 stub（文档最小子集）：** AXI4-Lite **最小从**单寄存器 write/read 握手玩具 |
| **黑盒** | 仅端口 | **保留：** 不内联 vendor HIR；见下节 |

**决策表（NFR37）：** Story 34.3 **未**书面降级任一类——SPI / I2C / AXI 均以合同化最小子集交付；全协议 / VIP / Full AXI **不**在本 epic，见下表「已知限制」。不得静默声称「五类 VIP 级完成」。

## 五类 + 黑盒

| 类 | 包路径 / 类型 | Smoke 命令 | 已知限制（基线边界） |
| --- | --- | --- | --- |
| **FIFO** | `bitloom_prelude::ip::SyncFifo` | `cargo test -p bitloom-prelude --lib sync_fifo` | depth=4、宽=8；非异步/CDC FIFO（[`SyncFIFO`](../../crates/bitloom-prelude) 为 bridge 标记） |
| **UART** | `bitloom_prelude::ip::UartTx` | `cargo test -p bitloom-prelude --lib uart_tx` | 8N1、1 bit/clk；非可编程波特率 / RX / 全双工 |
| **SPI** | `bitloom_prelude::ip::SpiMaster` | `cargo test -p bitloom-prelude --lib spi_master` | Mode-0-ish、MSB-first、1 bit/clk；非其它 CPOL/CPHA / 多 CS / DMA / slave |
| **I2C** | `bitloom_prelude::ip::I2cMaster` | `cargo test -p bitloom-prelude --lib i2c_master` | START+8data+STOP 教学玩具（SCL 恒高）；非 ACK 驱动 / 伸展 / 多主 / 10-bit / slave |
| **AXI** | `bitloom_prelude::ip::Axi4LiteSlave` | `cargo test -p bitloom-prelude --lib axi4_lite` | AXI4-Lite **最小从**单寄存器握手（ADDR=8, DATA=32）；忽略 addr/wstrb；非 Full AXI / 互联 / VIP |
| **黑盒** | `bitloom_prelude::ip::ExtBlackBox` + `vendor_blackbox_v()` | `cargo test -p bitloom-prelude --lib blackbox` | 仅端口；不内联子 HIR；vendor `.v` 旁路 |

### 黑盒 wrapper 行为与边界

- `ExtBlackBox::elaborate()` 产出**空 body**（无 reg/process）——仅端口壳。
- Bitloom **不**把 vendor 网表编进 FrozenHir；合成/仿真时与 `vendor_blackbox_v()` 或外部 `.v` 链接。
- 黑盒**不是**可综合行为模型；FR82 非 stub 证据来自五类 IP，不来自黑盒。

联验夹具：`examples/ip_box`（`cargo test -p ip_box`）。**无** AXI↔UART/FIFO 互联夹具（见 deferred-work）。

## CI / `just test`

上述 `bitloom-prelude` IP 单元测试与 `ip_box` 均在 workspace `just test`（`cargo test --workspace`）内，默认 CI 可触达。  
FR82 ATDD：`cargo test -p bitloom --test fr82_fifo_uart_baseline`、`cargo test -p bitloom --test fr82_spi_i2c_axi_baseline`。

## 例化片段

```rust
use bitloom_prelude::ip::SpiMaster;
use bitloom_prelude::Elaboratable;
use bitloom_sim::Sim;
use bitloom_hir::PortValues;

let hir = SpiMaster::elaborate()?;
let mut sim = Sim::new(hir);
let mut pv = PortValues::default();
pv.set("rst", 0);
pv.set("start", 1);
pv.set("tx_data", 0xA5);
pv.set("miso", 0);
sim.set_inputs(pv);
sim.settle();
sim.tick();
```

实现源码：[`crates/bitloom-prelude/src/ip.rs`](../../crates/bitloom-prelude/src/ip.rs)。  
风险记录：[`nfr14-risk-epic34-ip-baseline.md`](../../_agile-output/implementation-artifacts/nfr14-risk-epic34-ip-baseline.md)。
