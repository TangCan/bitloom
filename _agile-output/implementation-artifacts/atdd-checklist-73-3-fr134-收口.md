# ATDD Checklist — Story 73.3 / FR134 closeout

| AC / predicate | Test | Notes |
|----------------|------|-------|
| NFR14 关闭勾选 | `fr134_closeout` | `- [x] **73.2 / FR134` + status closed |
| docs/fr134 closed | `fr134_closeout` | `closed` / `已关闭` |
| README Epic 73 | `fr134_closeout` | `Epic 73 已关闭` + Phase 16 指针 |
| 74–77 仍开 | `fr134_closeout` | 不得假装 74–77 已关 |
| deferred FR134 | `fr134_closeout` | FR134 已关闭 + NFR59 更深 GUI/IDE |
| sprint epic-73/73-3 | `fr134_closeout` | both `done` |
| epics frontmatter | `fr134_closeout` | `phase16Epic73Status: complete` |
| FR125 隔离 | `fr134_closeout` | T1–T4 仍有效；alone ≠ FR134 |
| NFR59 / FR140 | `fr134_closeout` | 更深 GUI/IDE + 宣称纪律 |

```bash
cargo test -p bitloom --test fr134_epic73_closeout
```

Expected before Story 73.3 build: **FAIL**（docs/NFR14/deferred/README/sprint/epics 未收口）。
