# Code review — 47-3 仿真覆盖率扩展-fr105-收口

**Story:** 47-3-仿真覆盖率扩展-fr105-收口  
**Baseline:** `6cf5fec`  
**Verdict:** **Approve**

## Findings

1. **FR105 C2+R* 落地。** `bitloom-sim` 在 `Mux` 求值路径记录 `mux:<sel>:t/f`；报告头 `# bitloom-sim coverage v2`；保留 FR34 `hit`/`miss`；夹具同时展示 toggle 与 branch hit/miss。
2. **C3 诚实裁剪。** `docs/fr105-sim-coverage-ext.md` 显式 MVP crop FSM；非静默省略。
3. **收口完整。** NFR14 Epic 47 关闭清单全勾 + closed；README/deferred/AGENTS/doc-19/fr34/fr104 交叉；sprint `47-3: done` + `epic-47: done`；无 epic-48。
4. **未越界。** 设计 crate 仍 prelude-only；FR104 interactive 路径保留；未开工新 epic。

## AC trace

| AC | Result |
|----|--------|
| C1+C2(+C3 crop)+R1–R2+夹具；docs/deferred；FR104/105 可勾；NFR14 Epic 47 关闭 | pass |
| 字面绿剩余门 = 各 epic 实现关闭 | pass |
| `47-3`/`epic-47` done；不开工 48+；FR34/FR104 回归 | pass |

## Notes

- Prior `fr104` sprint 守卫已放宽，避免与 Epic 关闭冲突。
- FR34 单元测试已接受 v2 头；`parse_report` 仍只解析 toggle 行。
