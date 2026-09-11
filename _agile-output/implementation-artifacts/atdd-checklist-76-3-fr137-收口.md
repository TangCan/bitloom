# ATDD Checklist — Story 76.3 / FR137 closeout

| AC / predicate | Test | Notes |
|----------------|------|-------|
| NFR14 关闭勾选 | `fr137_closeout` | 五条 `- [x]` + status **closed** |
| docs/fr137 closed | `fr137_closeout` | `Epic 76 / FR137 closed` + Epic 77 仍须 + sim Not selected |
| README Epic 76 | `fr137_closeout` | `Epic 76 已关闭`；禁 `Epic 76–77` / `76–77` |
| AGENTS Epic 76 | `fr137_closeout` | FR137 / Epic 76 closed；Epic 77 still open |
| Epic 77 仍开 | `fr137_closeout` | 不得假装 Epic 77 已关 |
| deferred FR137 | `fr137_closeout` | FR137 已关闭 + NFR59 更广 CIRCT/MLIR |
| sprint epic-76/76-3 | `fr137_closeout` | both `done` |
| epics frontmatter | `fr137_closeout` | `phase16Epic76Status: complete` |
| FR129 隔离 | `fr137_closeout` | C1–C4 仍有效；fr129 指针 FR137 closed |
| NFR59 / FR140 | `fr137_closeout` | 更广 CIRCT/MLIR lower + 宣称纪律 |

```bash
cargo test -p bitloom --test fr137_epic76_closeout
```

Expected after Story 76.3 build: **PASS**.
