# FR168 — SPI / I2C / AXI handwritten FL（三者皆交付）

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 101 / FR168 closed** (Story **101.3**). Product path delivered in Story **101.2** (all three).

Phase 19 **FR163** `UartRxFunctional` **remains closed and valid**（NFR73）。本 FR 在保留 FR163 / FR135 / FR126 / FR103 关闭面的前提下，为 **SPI + I2C + AXI 三者** 各交付手写 FL ≡ `Sim::settle`+`tick`。

## Selected IP（NFR14 · 三者皆须）

| Item | SPI | I2C | AXI |
|------|-----|-----|-----|
| IP | `bitloom_prelude::ip::SpiMaster` | `bitloom_prelude::ip::I2cMaster` | `bitloom_prelude::ip::Axi4LiteSlave` |
| FL | `bitloom_sim::SpiMasterFunctional` | `bitloom_sim::I2cMasterFunctional` | `bitloom_sim::Axi4LiteSlaveFunctional` |
| API | `IpDualModelMatrix::verify_spi_master_handwritten` | `verify_i2c_master_handwritten` | `verify_axi4_lite_slave_handwritten` |
| Stimulus | `spi_master_dual_stimulus` | `i2c_master_dual_stimulus` | `axi4_lite_slave_dual_stimulus` |
| Ports | `mosi_byte` / `rx_data` / `rx_valid` / `busy` / `cs_n` / `sclk` / `mosi` | `tx_byte` / `busy` / `scl` / `sda_out` / `rx_data` / `rx_valid` / `ack_error` | `s_axi_awready` / `wready` / `bresp` / `bvalid` / `arready` / `rdata` / `rresp` / `rvalid` |

Pinned stimuli: SPI Mode-0 single-byte（对齐 `fr98_spi_mode0_byte_transfer_rx`）；I2C write+ACK；AXI write+read handshake.

## Forbidden closes

**≠ FR163 `UartRx` alone**；≠ FR135 `UartTx` alone；≠ FR126 Gpio alone；**≠ GeneratedFunctional alone**；≠「至少一项」SPI 或 I2C 或 AXI；docs-only.

## Non-regression（NFR73）

FR103 / FR112 / FR126 / FR135 / **FR163** closes remain valid. FR163 alone ≠ FR168.

## Crate boundary

设计 crate → **`bitloom-prelude` only**；手写 FL / 矩阵在 **`bitloom-sim`**。不得强制设计 crate 依赖 `bitloom-sim`。

## Recipe

```text
cargo test -p bitloom --test fr168_spi_i2c_axi_handwritten_fl
cargo test -p bitloom-sim --lib
```

## Cross-links

| Doc | Role |
|-----|------|
| [`fr163-unlisted-protocol-handwritten-fl.md`](fr163-unlisted-protocol-handwritten-fl.md) | FR163 UartRx（仍关闭；≠ FR168 alone） |
| [`fr135-more-ip-handwritten-fl.md`](fr135-more-ip-handwritten-fl.md) | FR135 UartTx |
| NFR14 | `_agile-output/implementation-artifacts/nfr14-risk-epic101-spi-i2c-axi-handwritten-fl-fr168.md` |

## FR193 / Story125.3 AXI protocol correction

历史FR168只以公开的顺序write/read stimulus关闭，并未证明独立AW/W或并发读写正确。Story125.2的[红测历史](ip/phase24-axi-red-evidence.md)保留。125.3为`Axi4LiteSlaveFunctional`增加私有AW/W捕获，读快照在更新bank前形成，读响应与B阻塞独立；同步reset清捕获和响应。公开类型/字段/端口及四寄存器8-bit地址映射不变。

[FR193绿色证据](ip/phase24-axi-green-evidence.md)记录独立事务队列/字节黄金模型驱动的两个native引擎、手写FL和真实direct RTL矩阵。结果不依赖FL与DUT互相对拍，也不把旧顺序stimulus通过当作新协议矩阵证明。本次不宣称Chisel/FIRRTL矩阵或形式证明；新CSR桥和Phase24后续epic仍未交付。
