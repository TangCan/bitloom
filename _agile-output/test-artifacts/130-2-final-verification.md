# Story130.2 独立来源快照最终验证（2026-09-23）

主代理最终复核通过，Story/sprint 随本次 Story130.2 独立提交置 done；仅关闭 FR199，不声称 FR200、Epic130 或 Phase24 关闭。

本轮在独立 `/tmp/bitloom-130-2-closeout` 快照执行，未使用根工作区的 Story130.3 回归替代。完整命令、UTC、耗时、退出码与日志位置见 `130-2-final-regression/commands.json`；终端退出结果见 `terminal-results.json`。

| 实际执行 | 结果 |
|---|---|
| `cargo clean`、`cargo fmt --all` | 各 exit0 |
| `just test`（`cargo test --workspace`） | exit0；485结果块，1889 passed / 0 failed / 50 ignored；1475.84秒 |
| `cargo build -p bitloom` | exit0；随后固定本轮CLI身份 |
| 空缓存在线 fetch | exit0；重新取得三仓库，全部 source records 与既有锁逐字段一致，共210 tracked files |
| 独立复制 cache/input 的 source-only replay | exit0；Yosys实际parse/compile，网络与原路径访问负探针通过，原/复制cache摘要不变 |
| 原 checkout 与原cache实际只读挂载下再重放 | exit0；外层只读挂载、内层隔离probe/编译及cache integrity原始记录均归档 |
| `fr199_external_ip_lock p0_online -- --ignored --nocapture` | 显式执行1 passed / 0 failed / 0 ignored；不是将默认ignored计PASS |
| ATDD普通 / `python -O` | 各3/3通过；fixture自有lock，canonical未被验收变异 |
| 独立证据consumer普通 / `python -O` | 各11/11通过；含错误来源/工具/隔离/只读挂载/host probe/cache integrity变异拒绝 |

canonical lock SHA256：`3c57f91816380e731dbb5882c783dc7737d9359903e9b12b0e22dbf58921d725`。CLI SHA256：`a5ac9eeabcfae5e06c15f49e88621ece7b08abac05de72b388a03c71ca89af54`。源身份与169+13+28文件分配见 `130-2-final-regression/closure-identity.json`；线上/离线原始证据为 `130-2-online-fetch.json`、`130-2-offline-replay.json` 及 `130-2-offline-replay-isolation.json`。只读原输入证明见 `130-2-final-regression/readonly-inputs.json` 与同目录 `readonly-inner-*.json`。

来源登记和锁定事实已验证，但按NFR14更强边界，外部试点支持矩阵仍全部 no / not delivered；真实绑定、行为、维护与整个Epic关闭留给Story130.3。没有修改工具产品pin、包版本、发布状态或既有FR189/NFR91边界，没有push/publish。
