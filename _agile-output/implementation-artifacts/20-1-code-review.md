# Code Review: Story 20.1 NFR14 风险记录（Chisel 双向）

**Reviewer:** adversarial pass (unattended; nested review subagents skipped per parent constraint)  
**Date:** 2026-08-21  
**Verdict:** accept with ATDD strengthen

## Findings

### Medium — fixed

1. **ATDD 对 FR46 选项 A/B/C 断言过松：** 仅匹配「选项」或「拟选」可被旁白命中。  
   **Fix:** 要求正文同时出现 A/B/C 形态（`选项 A`/`A.` 等），并保留拟选 / `[ASSUMPTION]`。

2. **ATDD 未锁「不得…而不改 PRD」语义：** 仅查 `尽力失败` 不足以覆盖 AC 的静默降级句。  
   **Fix:** 断言 FR28 +（结构化）尽力失败 + 不得/禁止，并要求出现 `PRD`/`prd`。

### Low — accepted

3. **负责人写 Richard：** 与仓库近期故事一致；无组织 RACI 表可对照。
4. **工期带为粗估：** 符合 NFR14「粗工期带」；非缺陷。

### Out of scope (not defects)

- 未实现 FR28/FR46 生成器（留给 20.3+）。
- 未修订 AD-27 / NFR9 推翻句（留给 20.2）。

## AC checklist

| AC | Status |
| --- | --- |
| (a) 上游约束含 Chisel/firtool 钉死 + #4899 / 无 Scala Parser | pass |
| (b) 粗工期带 | pass |
| (c) 禁止静默降级（含 FR28 尽力失败） | pass |
| (d) 负责人 | pass |
| 引用 FR46 选项 A/B/C 拟选方向 | pass（A + ASSUMPTION） |
| 无记录则 20.2–20.5 不得 ready | pass |
| ATDD 缺文件失败 | pass（路径断言） |

## Disposition

Strengthen ATDD → re-run targeted test → mark story done.

## testarch-automate

- 新增 `crates/bitloom/tests/nfr14_risk_chisel_bidirectional.rs`（文档门禁；非 N/A）。
- 审查后收紧选项 A/B/C 与 FR28 静默降级断言；无单独 e2e 框架扩展（本故事为 artifacts 门禁）。
