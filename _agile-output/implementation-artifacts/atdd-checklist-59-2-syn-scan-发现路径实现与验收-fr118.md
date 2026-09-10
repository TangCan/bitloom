# ATDD Checklist — Story 59.2 Syn-scan 发现路径实现与验收（FR118）

**Date:** 2026-09-10  
**Mode:** Red-phase then green with product path  
**Stack:** Rust integration tests via `bitloom` → `bitloom-lsp`  
**Generation:** AI（对齐 Epic 55.2）

## Acceptance mapping

| AC | Red test | Green when |
| ---- | -------- | ---------- |
| AC1 syn-scan + 无 metadata 正例 / 失败可读 | `fr118_syn_scan_design_root_discovery` | discover + elaborate Pass/Fail |
| AC2 FR99 / FR113 回归 | 同文件 DesignFixture + fr113_meta_ok | called_finish 绿 |
| AC3 shallow ≠ finish | `fr118_shallow_syn_scan_does_not_finish` | called_finish=false |

## Test file

`crates/bitloom/tests/fr118_syn_scan_design_root_discovery.rs`

## Notes

- Fixtures under `crates/bitloom-lsp/fixtures/fr118_syn_{ok,fail}`.
- Closeout / epic-59 done → Story 59.3.
