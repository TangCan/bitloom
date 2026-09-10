# Bitloom 一级 IP 索引（FR37 / FR48 / FR82）

公开产品名 **Bitloom**（crates.io / CLI：`bitloom`）。与 [samitbasu/rhdl](https://github.com/samitbasu/rhdl) **无关**。

设计 crate **只依赖 [`bitloom-prelude`](../../crates/bitloom-prelude)**。五类一级 IP 与黑盒均通过：

```rust
use bitloom_prelude::ip::{
    SyncFifo, UartTx, UartRx, SpiMaster, I2cMaster, Axi4LiteSlave, Gpio, ExtBlackBox,
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
| **NFR36 / Cap-R-64** | freeze / emit / **viz** 无闭包 / `Fn` / `\|\|` 残留（Story **29.4** 抽检） |
| **约束类（D1）** | 可综合腿 = **`SynthesizableClosure`**（FR74）；**不是** HLS `HlsFree` |
| **消歧** | ≠ **FR47** sim generators（双视图 crate）；≠ FR76 外挂 HLS 数据流闭包 |
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
| **UART** | 字节保持，无串行帧 | **非 stub：** 8N1 bit-bang；FR82 时 `baud_div=0`（1 bit/clk）；**FR89 / Epic 38：** 可编程 `baud_div`（TX）；**FR98 / Epic 43.2：** [`UartTx`](#五类--黑盒)+[`UartRx`](#五类--黑盒) 近 VIP |
| **SPI** | 端口语义 stub | **非 stub（文档最小子集）：** Mode-0-ish MSB byte shifter，`cs_n`/`sclk`/`mosi`；**FR98 / Epic 43.3：** 近 VIP 加深（CPOL/CPHA / 多字节） |
| **I2C** | 端口语义 stub | **非 stub（文档最小子集）：** START + 8 data + STOP bit-bang；**FR98 / Epic 43.4：** 近 VIP 加深（ACK/NACK / 7-bit addr / 半周期 SCL） |
| **AXI** | 握手镜像 stub | **非 stub（文档最小子集）：** AXI4-Lite 单寄存器握手玩具；**FR98 / Epic 43.5：** 近 VIP 加深（多寄存器 / addr / wstrb） |
| **黑盒** | 仅端口 | **保留：** 不内联 vendor HIR；见下节 |

**决策表（NFR37）：** Story 34.3 **未**书面降级任一类——SPI / I2C / AXI 均以合同化最小子集交付。**Epic 43 / FR98** 已将 UART/SPI/I2C/AXI4-Lite 加深至近 VIP（见上表）；**Epic 50 / FR108 已关闭**（Story 50.3）交付 GPIO 近 VIP（方向 / 读写 / 掩码）。**Epic 61 / FR120 已关闭**（Story 61.3）交付商业 VIP GPIO（`GpioVip`：IRQ / 开漏+OE / 原子 set-clear；超出 P1–P4）。Full AXI / 互联仍为明确非目标。不得静默声称「Full AXI 互联完成」；不得以 FR108 alone / FR98 alone 冒充 FR120。

## 五类 + GPIO + 黑盒

| 类 | 包路径 / 类型 | Smoke 命令 | 已知限制（基线边界） |
| --- | --- | --- | --- |
| **FIFO** | `bitloom_prelude::ip::SyncFifo` | `cargo test -p bitloom-prelude --lib sync_fifo` | depth=4、宽=8；**单时钟** FR82 IP；非跨域（语言级 CDC 见 [`SyncFIFO`](../fr79-syncfifo-cdc.md) / FR79） |
| **UART TX** | `bitloom_prelude::ip::UartTx` | `cargo test -p bitloom-prelude --lib uart_tx`；`cargo test -p bitloom --test fr89_uarttx_programmable_baud` | **FR82+FR89：** 8N1 TX，`baud_div`（clk/bit−1；0≡baud=`clk`） |
| **UART RX** | `bitloom_prelude::ip::UartRx` | `cargo test -p bitloom-prelude --lib uart_rx`；`cargo test -p bitloom --test fr98_uart_near_vip` | **FR98 / Epic 43.2 近 VIP：** 8N1 RX，同 `baud_div` 语义；`rd_valid` 一周期脉冲；**全双工合同** = 同设计例化 `UartTx`+`UartRx`（独立 `tx`/`rx` 线）。**明确非目标：** 片上半双工切换、parity、流控、IrDA、小数分频、FIFO'd UART、商业 VIP 对拍 |
| **SPI** | `bitloom_prelude::ip::SpiMaster` | `cargo test -p bitloom-prelude --lib spi_master`；`cargo test -p bitloom --test fr98_spi_near_vip` | **FR98 / Epic 43.3 近 VIP：** 可配置 **CPOL/CPHA（四模式）**；Master **多字节**（`byte_count`，`0`≡1）；半周期 `sclk`（idle=`cpol`）；MSB-first；`rx_data`/`rx_valid`；整帧 `cs_n` 有效。对照 FR82 Mode-0-ish。**明确非目标：** DMA、多 CS 阵列、slave、LSB-first、非 8×N 字长、商业 VIP 对拍 |
| **I2C** | `bitloom_prelude::ip::I2cMaster` | `cargo test -p bitloom-prelude --lib i2c_master`；`cargo test -p bitloom --test fr98_i2c_near_vip` | **FR98 / Epic 43.4 近 VIP：** ACK/NACK 驱动 Master **写** + **读**；START / 7-bit `addr` + `rw` / 数据 / STOP；半周期 `scl`（idle 高）；`rx_data`/`rx_valid`；`ack_error` 于 NACK。对照 FR82 SCL 恒高玩具。**明确非目标：** clock stretch、多主、10-bit、slave、SMBus PEC、商业 VIP 对拍 |
| **AXI** | `bitloom_prelude::ip::Axi4LiteSlave` | `cargo test -p bitloom-prelude --lib axi4_lite`；`cargo test -p bitloom --test fr98_axi_near_vip` | **FR98 / Epic 43.5 近 VIP：** AXI4-Lite 从窗口字址 `0x00/0x04/0x08/0x0C`（ADDR=8, DATA=32）；正确 **AW/W/B** 与 **AR/R** 握手；**addr 译码** + **wstrb** 字节合并；未映射写忽略 / 读 0。对照 FR82 单寄存器忽略 addr/wstrb 玩具。**明确非目标：** Full AXI（burst/ID/QoS）、互联、多从阵列、商业 VIP 对拍。**GPIO：** FR98 关闭时为**可选**（G0/G1；未纳入不构成 FR98 失败）；**FR108 / Epic 50** 升格交付近 VIP（见下行） |
| **GPIO** | `bitloom_prelude::ip::Gpio` / `GpioVip` | `cargo test -p bitloom-prelude --lib gpio`；`cargo test -p bitloom --test fr108_gpio_near_vip`；`cargo test -p bitloom --test fr108_epic50_closeout`；`cargo test -p bitloom --test fr120_gpio_commercial_vip`；`cargo test -p bitloom --test fr120_epic61_closeout` | **FR108 / Epic 50 已关闭**（Story 50.3）：8-bit bank 近 VIP — `dir`（1=out）；`wr_en`/`wr_data`/`wr_mask`；`pad_in`→`pad_out`/`rd_data`。**FR120 / Epic 61 已关闭**（Story 61.3）：[`GpioVip`](#五类--gpio--黑盒) 超出 P1–P4 — **C1** 上升沿 IRQ（`irq_en`/`irq_clear`/`irq_status`/`irq_out`）；**C2** 开漏+`pad_oe`；**C3** 原子 `set_*`/`clr_*`；**C4** ATDD。见 [`fr120-commercial-vip-gpio.md`](../fr120-commercial-vip-gpio.md)。**FR98 交叉链：** FR98 关闭时 GPIO 为**可选**；FR108 为 Phase 13 近 VIP；FR120 为 Phase 14 商业 VIP 完成面（NFR48：FR98/FR108 关闭仍有效）。**明确非目标（NFR51 / 未列入 FR120）：** 全 SoC pad 环、商业对拍记分板、debounce/驱动强度、电平·降沿·双沿 IRQ 全家桶、模拟 |
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

**FR76+FR77 透明抽检（Story 29.4 / Cap-R-64 / NFR36；亦含于 `just test`）：**

```text
cargo test -p bitloom --test fr76_fr77_nfr36_transparency_matrix
```

矩阵覆盖：同一 `Crc8Lut` 闭包消解后 **viz HTML + Verilog + FIRRTL** 无闭包语义；FR76 dissolve C / stub RTL 同行。

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

## FR103 — 功能 + 周期双模型齐全（Epic 45）

五类一级 IP（FIFO/UART/SPI/I2C/AXI）具备可运行功能视图与周期精确模型，并与刺激/等价路径联验。完成面：[`docs/fr103-ip-dual-model.md`](../fr103-ip-dual-model.md)（`IpDualModelMatrix`）。**Epic 45 / FR103 已关闭**（Story 45.4）。SystemC TLM 产品见 Epic 46 / FR101（**已关闭** — LT-only MVP）。
