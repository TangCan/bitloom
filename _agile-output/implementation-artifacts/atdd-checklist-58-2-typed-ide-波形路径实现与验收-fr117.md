# ATDD Checklist — Story 58.2 Typed IDE 波形路径实现与验收（FR117）

**Date:** 2026-09-10  
**Mode:** Red-phase scaffold then green with product path  
**Stack:** backend / CLI / docs（Rust `#[test]` + `cargo bitloom wave`）  
**Generation:** AI（对齐 Epic 47.2 / 56.2）

## Acceptance mapping

| AC | Red test | Green when |
| ---- | -------- | ---------- |
| AC1 子集 B 产品路径 + typed ≠ I1–I3 | `fr117_typed_ide_wave` | `typed-wave.html` + `wave.typed.json` + 类型元数据标记；docs 合同 |
| AC2 Bitloom + NFR48 不回归 | 同测试 | VCD / interactive.html / coverage 路径仍可用；品牌 Bitloom |
| AC3 A deferred；不关 58.3 | 同测试 | docs/NFR14 声明 Tywaves deferred；sprint 58-3 backlog |

## Test file

`crates/bitloom/tests/fr117_typed_ide_wave.rs`

## Red confirmation

Expected before Story 58.2 build: `cargo test -p bitloom --test fr117_typed_ide_wave` **FAIL**（缺 docs/fr117、缺 typed 产物）。

## Notes

- 完成面 = 自研 typed IDE 波形（子集 B）；≠ Tywaves 集成。
- 负向：缺 typed 元数据不得 silent 宣称 FR117。
- 不在本故事勾选 Epic 58 / FR117 关闭（→58.3）。
