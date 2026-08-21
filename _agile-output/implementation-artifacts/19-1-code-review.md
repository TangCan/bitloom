# Code Review: Story 19.1 NFR14 风险记录模板

**Reviewer:** adversarial pass (unattended)  
**Date:** 2026-08-21  
**Verdict:** accept with minor strengthen

## Findings

### Medium — fixed

1. **ATDD 断言过松：** `AD-28` 双重 `contains` 无意义；`NFR14`+`crates.io`/`FCFS` 组合可被正文偶然命中而不保证消歧表存在。  
   **Fix:** 加强测试：要求同时出现 `NFR14-crates` 与「本门禁」语义关键词；显式断言 `AD-28` 与 `PRD`/`NFR14` 共现；断言四字段标题形态 `(a)`/`(b)`/`(c)`/`(d)` 或等价章节标题。

### Low — accepted

2. **路径说明：** 用户管道写 `crates/rhdl-rs/tests/`，仓库已更名 `bitloom`；故事 Dev Notes 已注明，测试落在 `crates/bitloom/tests/` — 正确。
3. **模板未单独 README：** 使用说明同文件顶部 — 符合 AC「可同文件或 sibling」。

### Out of scope (not defects)

- 未实现 FR47 生成器 / 未填写 Epic 20–24 具体风险记录（留给 20.1+）。

## AC checklist

| AC | Status |
| --- | --- |
| 模板含 (a)(b)(c)(d) | pass |
| 缺记录不得 ready FR46–49 / FR50 | pass |
| NFR14 vs NFR14-crates | pass |
| 并行 / Chipyard 维护风险 | pass |
| 引用 AD-28 / PRD NFR14 | pass |
| ATDD 缺文件失败 | pass (red observed) |

## Disposition

Strengthen ATDD → re-run targeted test → mark story done.
