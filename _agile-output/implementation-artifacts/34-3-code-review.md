# Code Review: Story 34.3 SPI / I2C / AXI 基线（或文档最小子集）

**Verdict:** Approve

**Scope:** `bitloom_prelude::ip::{SpiMaster,I2cMaster,Axi4LiteSlave}` FR82 deepen + `fr82_spi_i2c_axi_baseline` ATDD + `docs/ip` / language-surface / deferred-work

## Findings

1. **非 stub 证据充分。** SpiMaster 为 Mode-0-ish MSB byte shifter（`cs_n`/`sclk`/`mosi` + busy 门控）；I2cMaster 为 START+8data+STOP；Axi4LiteSlave 为单寄存器 write→bvalid / read→rvalid 握手。均有 elaborate→emit `.v`→tick 夹具（prelude 单测 + ATDD）。
2. **无静默降级（NFR37）。** `docs/ip/README.md` 决策表写明三类均以合同化最小子集交付；全协议 / VIP / Full AXI 显式列为非目标，而非静默声称五类 VIP 完成。
3. **无生成器闭包 API。** `Elaboratable::elaborate()` 无 `Fn`；ATDD 扫描 Spi/I2c/Axi 实现段。闭包留给 Epic 29。
4. **轻量建议（不挡合入）：** sim `Shl` 不截断位宽——本故事已对 SPI/I2C `& 0xFF`；未来可在 sim 按 signal width 掩码（非本故事）。

## AC Trace

| AC | Result |
| ---- | ------ |
| SPI / I2C / AXI4-Lite（或文档最小子集）elaborate→emit→tick；每类夹具或决策表 | pass（三类均交付 + 夹具；决策表钉死子集边界） |
| 未交付类型须书面降级（NFR37） | pass（无降级类；限制表诚实） |
| 无闭包定制参数 | pass |

## Decision

**Approve** — 可标 done；sprint `34-3-spi-i2c-axi-基线-或文档最小子集: done`。
