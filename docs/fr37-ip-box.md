# FR37 — IP box（起步）+ FR48 索引 + FR82 基线

完整五类与黑盒索引见 **[docs/ip/README.md](ip/README.md)**（经 `bitloom_prelude::ip`）。

- In-tree / prelude：`SyncFifo`（depth-4 + full/empty）、`UartTx`（8N1 bit-bang）、`SpiMaster`（Mode-0-ish byte shifter）、`I2cMaster`（START+8data+STOP）、`Axi4LiteSlave`（单寄存器握手）、`ExtBlackBox`（opaque）；**FR77 overlay：** `Crc8Lut`（默认 poly `0x07` / `elaborate_with_table_fn`）
- 演示 crate：`examples/ip_box`（仅依赖 `bitloom-prelude`）
- FR82：五类均为非 stub 可综合路径（文档最小子集）；五类 API **无**生成器闭包
- Epic 29 / FR77：闭包定制为 **overlay**（`Crc8Lut`；Story 29.3），叠在 Epic 27 Mem-init + FR82 基线之上；相对 Epic 22 stub 见 **NFR37** / [`docs/ip/README.md`](ip/README.md)
