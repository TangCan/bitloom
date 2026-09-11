# ATDD Checklist — Story 77.3 / FR138 closeout

| AC / predicate | Test | Notes |
|----------------|------|-------|
| NFR14 关闭勾选 | `fr138_closeout` | 五条 `- [x]` + status **closed** |
| docs/fr138 closed | `fr138_closeout` | `Epic 77 / FR138 closed` + FR130 仍有效 + NFR59 |
| README Epic 77 | `fr138_closeout` | `Epic 77 已关闭`；禁「Epic 77 仍须」 |
| AGENTS Epic 77 | `fr138_closeout` | FR138 / Epic 77 closed；禁 still-open |
| Phase 16 全关 | `fr138_closeout` | Epic 72–78 规划故事关闭诚实面 |
| deferred FR138 | `fr138_closeout` | FR138 已关闭 + NFR59 更深 Parser |
| sprint epic-77/77-3 | `fr138_closeout` | both `done` |
| epics frontmatter | `fr138_closeout` | `phase16Epic77Status: complete` |
| FR130 隔离 | `fr138_closeout` | Style Guide 仍有效；fr130 指针 FR138 closed |
| NFR59 / FR140 | `fr138_closeout` | 更深 Chisel/Parser 生态 + 宣称纪律 |

```bash
cargo test -p bitloom --test fr138_epic77_closeout
```

Expected before Story 77.3 build: **FAIL** (red). After build: **PASS**.
