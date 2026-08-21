# Code Review: Story 19.3 修订 AD-20（允许 Bundle/Vec）

**Reviewer:** adversarial pass (unattended)  
**Date:** 2026-08-21  
**Verdict:** accept

## Findings

### Low — fixed during review

1. **ATDD 缺口：** 未锁「不得 silently / 无检查」、Revised「PRD FR51」+「本 FR 非目标」、Rule「不再作为阻断」。  
   **Fix:** 收紧 `ad20_bundle_vec_allowed` 断言。

2. **language-surface Comb/seq：** Stage-2 条仍像 AD-20=仅 FR22。  
   **Fix:** 加一句指向 Composite / FR51。

### Deferred

- AD-12 阶段一 ground「无 Bundle/Vector」无交叉引用 AD-20 可选扩展——读者或误读为硬禁；合同以 AD-20 为准。
- `epics.md` Story 19.3 Given 仍用「不得进入」现在时；Epic 17 文仍写「禁止未立项 Bundle/Vec」。
- Capability map「阶段二表面加厚 + Bundle/Vec」同列 FR22/FR51——叙事粗，非合同冲突。

### Rejected

- 非可综合路径 Bundle 规则、FR51 其它复合类型枚举、ATDD 锁 HIR 节点选型——超出本故事「架构修订 + 门禁」意图。
- AGENTS 须写 elaborate→emit→tick 全链——AC 未要求；正向指针已够。

## AC checklist

| AC | Status |
| --- | --- |
| AD-20 允许文档化 Bundle/Vec 可综合路径 | pass |
| 位宽/方向 emit 前失败 | pass |
| 引用 PRD FR51 | pass |
| 明确 FR22「本 FR 非目标」边界 | pass |
| 未实现完整语言（留给 19.4） | pass |
| ATDD `ad20_bundle_vec_allowed` | pass (targeted) |

## Automate

- Strengthened ATDD: silently/无检查、Revised PRD+非目标、Rule 不再作为阻断；language-surface Comb/seq → Composite 指针。
- Deferred: AD-12 交叉叙事、epics 史态措辞、Capability map 合并行。

## Disposition

引用/边界补齐 + Brand lock + language-surface + ATDD 门禁（含 review 收紧）→ mark story done。
