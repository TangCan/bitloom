# 126.4 ATDD 不可变历史快照

`126-4-atdd-vector-probe.rs` 是产品 API 尚不存在时从当时验收测试提取的独立向量自检快照，退出0只证明当时72组向量的覆盖断言。它不是当前CI入口，也不是后续修补向量的通过证据；不随着产品或测试修改而重写。当前入口为 crates/bitloom/tests/fr195_param_sync_fifo.rs，后续通过证据另行归档。

原始生成测试快照保留在126-4-atdd-api-tests.json，红测与独立probe日志均保持原样。SHA256：

- `126-4-atdd-vector-probe.rs`：`22c8e6c02df880cbb93b4f400b62da357bf11e81e0ec1af40c30daf89f1859c1`
- `126-4-atdd-api-tests.json`：`adda38202c477b94cf5dda4631fd44c09b2377060600a5b145714dadcda10b02`
- `126-4-atdd-vector-probe.log`：`1c9cdc720c993b3b90222fc211c5d8b805e38a44cbea295ece86507bc45534a6`
