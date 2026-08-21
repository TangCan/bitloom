# Code Review: Story 19.2 修订 AD-5（允许生成 Rust 功能模拟器）

**Reviewer:** adversarial pass (unattended)  
**Date:** 2026-08-21  
**Verdict:** accept

## Findings

### Low — accepted

1. **Revised 原先缺显式 FR47 / 推翻表：** Rule 已写 FR47 与「不承诺 SystemC TLM-2.0」，但 Revised 仅写「PRD ①C / Epic 19.2」。  
   **Fix:** Revised 补齐 **PRD FR47** 与 §0 **推翻表**，并写明历史「禁止 HIR→TLM」不再阻断 FR47。

2. **AGENTS 无旧禁但仍可漂移：** Brand lock 未正向指向 AD-5/FR47。  
   **Fix:** Brand lock 增加一句：允许生成 Rust functional-sim；不承诺 SystemC TLM-2.0。

### Low — fixed during review

3. **ATDD Rule 旧禁断言过宽：** 初版用「不得」+「降低」+「TLM」组合，误伤 Rule 中「不得进入 HIR」与「不要求从 HIR 降低 SystemC TLM-2.0」。  
   **Fix:** 改为显式禁止旧禁短语列表（`禁止从 HIR 生成/降低 TLM`、`禁止一切 HIR→功能模拟器生成` 等）。


- 未实现 FR47 CLI/API 生成器（Epic 21）。
- 未改仿真运行时 / 另立 HIR。
- `docs/fr29-*.md` / 历史 epic 文仍写「无 HIR→TLM」——属既有阶段一叙事；现行合同以脊柱 AD-5 + PRD 推翻表为准，本故事不批量改写史档。

## AC checklist

| AC | Status |
| --- | --- |
| AD-5 允许生成 Rust 功能模拟器 crate | pass |
| 不强制 / 不承诺 SystemC TLM-2.0 | pass |
| 周期精确仅 FrozenHir `tick` | pass |
| Revised 引用 PRD FR47 与推翻表 | pass |
| 旧「禁止 HIR→TLM」不阻断 FR47 | pass |
| AGENTS 无旧禁 + 正向指针 | pass |
| 未实现 FR47 生成器 | pass |
| ATDD `ad5_rust_functional_sim_allowed` | pass (targeted) |

## Automate

- Strengthened ATDD: AGENTS must assert SystemC TLM-2.0 **not contracted** (not merely mention TLM).
- Deferred (not this story): historical fr29/epics FR14 wording; bitloom-sim comment narrative; managed bmad:context refresh.

## Disposition

引用补齐 + Brand lock 指针 + ATDD 门禁（含 review 收紧）→ mark story done。
