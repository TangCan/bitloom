# FR60 语言表面可行性决议（Story 15.1）

**Verdict: PASS（闸门通过）** — 在补齐最小 `Lit`/`Eq`/`Mux` 后，Episode I 所需控制与 SyncReadMem 端口可 `elaborate` 并由 `bitloom-sim` `tick` 验证。见本包测试。

## 已证明可用

| 需求 | 证据 |
|------|------|
| 同步复位 Reg | `state` / `q` 默认 sync reset；rst 后为 0 |
| SyncReadMem 端口 | `declare_sync_read_mem` + write/read；读延迟 1 周期 |
| 多状态控制 | Idle→Fetch→Exec→Idle，经 `assign_eq` + `assign_mux` + `assign_lit` |
| 设计依赖 | 仅 `bitloom-prelude`（NFR24） |

## 缺口与书面决议

1. **`begin_then`/`begin_else`/`end_if` 不进入 HIR**：只做 latch 完整性分析。数据相关选择必须用 `assign_mux`/`assign_eq`（本 story 已补）。禁止把 latch API 当成可仿真分支。
2. **顺序过程尚无条件写 mem（`we` 门控）**：不阻塞 Episode I；测试/核可用恒定写或把写地址/数据 mux 到「空操作」槽。若以后要 seq-if，另开语言故事，**不得**静默宣称已支持。
3. **完整 ALU/译码算子**（sub/and/or/xor/shift/slt…）不在本闸门范围；15.2 按需扩展 `AssignExpr`，或用既有 `Add`/`Inc`/`Mux` 组合出最小子集。
4. **微架构**：接受单周期或显式 FSM；本 spike 用 3 态 FSM 证明控制面可行。不宣称 FR56（完整教学核）已满足。

## 非目标

无 CSR/trap/MMU/Linux/流水线；与 `samitbasu/rhdl` 无关；公开名 Bitloom。
