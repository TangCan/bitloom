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

**无生成器闭包参数**（Epic 34 / FR82 五类）；闭包定制叠加点为 **Epic 29 / FR77**（[`Crc8Lut`](#fr77--crc8lut-生成器闭包定制)）。

## Epic 29 handoff（无闭包基线 → 闭包定制叠加）

| 层 | Epic / FR | 含义 |
| --- | --- | --- |
| **IP 基线（本索引五类）** | Epic **34** / **FR82** | 五类非 stub 可综合路径；`T::elaborate()` **无** `Fn` / 生成器闭包参数 |
| **闭包定制叠加** | Epic **29** / **FR77** | [`Crc8Lut`](#fr77--crc8lut-生成器闭包定制)：elaborate-time 闭包定制 CRC 表（Story **29.3**）；叠在 Epic 27 Mem-init + 本基线之上 |
| **历史 stub** | Epic **22** / FR37+FR48 当时 AC | 端口语义 stub + smoke；**NFR37：** 规划 `done` ≠ FR82 深度 done |

**测序：** Epic 34 **先于** Story 29.3（已交付）。Epic 29 是 **closure customization overlay**，不是对五类的重复交付。  
风险：[`nfr14-risk-epic34-ip-baseline.md`](../../_agile-output/implementation-artifacts/nfr14-risk-epic34-ip-baseline.md)、[`nfr14-risk-epic29-hls-ip-closures.md`](../../_agile-output/implementation-artifacts/nfr14-risk-epic29-hls-ip-closures.md)。

## FR77 — Crc8Lut 生成器闭包定制

| | |
| --- | --- |
| **类型** | `bitloom_prelude::ip::Crc8Lut` |
| **骨架** | SyncReadMem depth-256×8；`addr` → 寄存器 `rdata`（Epic 27 `declare_sync_read_mem_with_init_fn`） |
| **无闭包默认** | `Crc8Lut::elaborate()` / `elaborate_default()` → poly **`0x07`**（CRC-8/SMBUS-style，[`DEFAULT_POLY`](../../crates/bitloom-prelude/src/ip.rs)） |
| **闭包定制** | `Crc8Lut::elaborate_with_table_fn(violations, \|i\| …)` — elaborate 期消解为普通字表（Cap-R-63）；空 `violations` = 合法 `SynthesizableClosure`（D1）；非空 → 明确诊断（如 `rhdl::E0143`/`E0144`） |
| **NFR36** | freeze / emit 无闭包 / `Fn` / `\|\|` 残留 |
| **非目标** | 不是 comb/seq 内联可综合闭包（Epic 28）；不改 FR82 五类无闭包 API |

```rust
use bitloom_prelude::ip::{Crc8Lut, crc8_table_byte};
use bitloom_prelude::Elaboratable;

// Documented default (no user Fn):
let _ = Crc8Lut::elaborate()?;

// Customize poly table at elaborate time:
let poly = 0x1du8;
let hir = Crc8Lut::elaborate_with_table_fn(&[], |i| {
    crc8_table_byte(i as u8, poly) as u64
})?;
```

ATDD：`cargo test -p bitloom --test fr77_ip_generator_closure`。

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
| **CRC LUT（FR77）** | `bitloom_prelude::ip::Crc8Lut` | `cargo test -p bitloom --test fr77_ip_generator_closure` | 闭包定制 overlay；默认 poly `0x07`；非全 CRC 引擎 / 流式 CRC |

### 黑盒 wrapper 行为与边界

- `ExtBlackBox::elaborate()` 产出**空 body**（无 reg/process）——仅端口壳。
- Bitloom **不**把 vendor 网表编进 FrozenHir；合成/仿真时与 `vendor_blackbox_v()` 或外部 `.v` 链接。
- 黑盒**不是**可综合行为模型；FR82 非 stub 证据来自五类 IP，不来自黑盒。

联验夹具：`examples/ip_box`（`cargo test -p ip_box`）。**无** AXI↔UART/FIFO 互联夹具（见 deferred-work）。

## CI / `just test`

上述 `bitloom-prelude` IP 单元测试与 `ip_box` 均在 workspace `just test`（`cargo test --workspace`）内，默认 CI 可触达。

**FR82 文档化配方（Story 34.4；亦含于 `just test`）：**

```text
cargo test -p bitloom --test fr82_ip_baseline_matrix
cargo test -p bitloom --test fr82_fifo_uart_baseline
cargo test -p bitloom --test fr82_spi_i2c_axi_baseline
```

- 矩阵：`fr82_ip_baseline_matrix` — 五类 elaborate→emit 汇总 + 无闭包扫描 + Epic 29 handoff / NFR37 文档断言  
- 深度夹具：`fr82_fifo_uart_baseline`、`fr82_spi_i2c_axi_baseline`

**FR77 文档化配方（Story 29.3；亦含于 `just test`）：**

```text
cargo test -p bitloom --test fr77_ip_generator_closure
```

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
