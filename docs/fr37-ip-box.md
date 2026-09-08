# FR37 — IP box（起步）+ FR48 索引 + FR82 基线

完整五类与黑盒索引见 **[docs/ip/README.md](ip/README.md)**（经 `bitloom_prelude::ip`）。

- In-tree / prelude：`SyncFifo`（depth-4 + full/empty）、`UartTx`（8N1 bit-bang）、`ExtBlackBox`（opaque）
- 演示 crate：`examples/ip_box`（仅依赖 `bitloom-prelude`）
- FR48 另含：`SpiMaster`、`I2cMaster`、`Axi4LiteSlave`（34.3 前仍为 stub）
- FR82：FIFO/UART 为非 stub 可综合路径；API **无**生成器闭包（Epic 29）
