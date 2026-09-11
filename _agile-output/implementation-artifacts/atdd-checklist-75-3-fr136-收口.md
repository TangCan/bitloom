# ATDD Checklist — Story 75.3 / FR136 closeout

| AC / predicate | Test | Notes |
|----------------|------|-------|
| NFR14 关闭勾选 | `fr136_closeout` | `- [x] **75.2 / FR136` + status closed |
| docs/fr136 closed | `fr136_closeout` | `closed` / `已关闭` + Epic 76–77 仍须 |
| README Epic 75 | `fr136_closeout` | `Epic 75 已关闭`；禁 `Epic 75–77 仍须` |
| AGENTS Epic 75 | `fr136_closeout` | FR136 / Epic 75 closed；76–77 still open |
| 76–77 仍开 | `fr136_closeout` | 不得假装 76–77 已关 |
| deferred FR136 | `fr136_closeout` | FR136 已关闭 + NFR59 更广 pad/外设 |
| sprint epic-75/75-3 | `fr136_closeout` | both `done` |
| epics frontmatter | `fr136_closeout` | `phase16Epic75Status: complete` |
| FR128 隔离 | `fr136_closeout` | GpioSocPad 仍有效；fr128 指针 FR136 closed |
| NFR59 / FR140 | `fr136_closeout` | 更广 pad/外设 + 宣称纪律 |

```bash
cargo test -p bitloom --test fr136_epic75_closeout
```

Expected after Story 75.3 build: **PASS**. Expected before build (red phase): **FAIL**.
