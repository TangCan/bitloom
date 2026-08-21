# Code Review: Story 20.2 架构 AD — FIRRTL→可编译 Chisel（FR28 条）

**Reviewer:** adversarial pass (unattended; nested review subagents skipped per parent constraint)  
**Date:** 2026-08-21  
**Verdict:** accept with minor ATDD cleanup

## Findings

### Medium — fixed

1. **ATDD `agents_has_nfr9_ban` 死变量 / 逻辑含糊：** 清理为「若出现 NFR9 则必须 overturn/推翻」。

### Low — accepted

2. **AD-27 Rule 仍含「不承诺可维护 Chisel」字样：** 出现在「NFR9『…』已被推翻」引用中，属故意；ATDD 要求同句含推翻语义。
3. **`docs/fr28-chisel-best-effort.md` / README 仍写「尽力」：** 历史文档漂移；本故事范围是脊柱 AD + 门禁，留给 20.3 实现时改写产品文档。

### Out of scope (not defects)

- 未实现 FIRRTL→Chisel 生成器（留给 20.3）。
- 未删除 `rhdl-firrtl` 现有 `chisel_best_effort_*` 测试（旧尽力路径；合同升格后由 20.3 替换）。

## AC checklist

| AC | Status |
| --- | --- |
| 可编译 Chisel + 端口/层次谓词 + 机械风格 / Open Q5 | pass |
| 不要求 Parser.parse / firrtl.Parser | pass |
| NFR9 推翻关系 | pass |
| 未实现生成器 | pass |
| ATDD 绿 | pass |
| Given 20.1 风险记录存在 | pass（ATDD 断言文件） |

## Disposition

Cleanup ATDD → re-run → mark done.

## testarch-automate

- 新增 `crates/bitloom/tests/ad27_compilable_chisel.rs`（脊柱 AD 门禁；非 N/A）。
- 覆盖 FR28/FR46 引用、NFR9 推翻、Parser 边界、AGENTS 指针、20.1 风险记录前置。
