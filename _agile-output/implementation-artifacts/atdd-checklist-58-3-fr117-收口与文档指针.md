# ATDD checklist — Story 58.3 / FR117 收口与文档指针

**Date:** 2026-09-10  
**Mode:** Closeout docs + NFR14 checkboxes + sprint + ATDD string gates  
**Stack:** docs / deferred / NFR14 / sprint / Rust `#[test]`  
**Generation:** AI（对齐 Story 54.3 / 56.3）

## Acceptance mapping

| AC | Red test | Green when |
| ---- | -------- | ---------- |
| NFR14 Epic 58 关闭勾选 | `fr117_nfr14_epic58_close_conditions_checked` | 全部 `[x]` + status closed / Story 58.3 |
| docs / README / deferred 诚实面 | `fr117_docs_readme_deferred_closed` | FR117/Epic 58 已关闭；A deferred；交叉链 |
| NFR48 FR104/FR114 仍有效 | `fr117_fr104_fr114_mvp_still_valid` | epic-47 / epic-56 done；docs 仍 closed |
| sprint 收口 | `fr117_sprint_epic58_done` | `58-3: done`；`epic-58: done`；`58-2: done` |
| NFR51 Tywaves A deferred | `fr117_tywaves_a_remains_deferred` | docs/NFR14/deferred 不得 silent 宣称 A |

## Test files

- `crates/bitloom/tests/fr117_epic58_closeout.rs` — **NEW** closeout ATDD
- `crates/bitloom/tests/fr117_typed_ide_wave.rs` — sprint 守卫接受 `58-3`/`epic-58` done

## Verification

```bash
cargo test -p bitloom --test fr117_epic58_closeout
cargo test -p bitloom --test fr117_typed_ide_wave
```

## Notes

- 收口 = 文档 + NFR14 勾选 + ATDD + sprint；**不**改 `rhdl-viz` 运行时。
- 完成面 = 自研 typed IDE 波形（子集 B）；Tywaves A 保持 deferred（NFR51）。
- FR104 / FR114 关闭证据仍有效（NFR48）。
