# Code review — Story 36.2 / FR87

**Verdict: Approve**

**Scope:** doc-19 P5–P7 contract-green rewrite + README/deferred pointers + ATDD.

## Findings

1. **无阻塞缺陷。** P5/P6/P7 合同绿条款与 research Recommendations §1 / epics Story 36.2 对齐；禁字面全绿（FR87/NFR38）；README「状态与 deferred」与 deferred-work 交叉链到位；未开工 36.3 FR93 正文。
2. **历史愿景与合同口径分离清晰**（「非完成勾选条件」块），降低误勾风险。
3. **ATDD** `fr87_doc19_contract_green` 五测覆盖 AC 1–5；红→绿已验。
4. **NFR14：** FR87 关闭项已勾；FR93 / 整 epic 关闭仍留给 36.3（符合门禁文案）。
5. **品牌：** 公开名 Bitloom；未宣称 crates.io `rhdl`。

## AC trace

| AC | Result |
|----|--------|
| P5 外挂 HLS + 薄 IP + VCD/层次 | pass |
| P6 精选 IP 深度 + 文档站 + rust-analyzer | pass |
| P7 同刺激 + adapter 模板；禁自动等价 / SystemC TLM | pass |
| 禁字面未交付勾选全绿 | pass |
| README 指向合同绿 | pass |

## Non-blocking notes

- Gantt / 早期章节仍含历史 RHDL 字面任务名；已用文首合同绿框与 §19.7–19.9 覆盖验收口径，可接受。
- FR93 永久非目标清单全文 → Story 36.3（本故事仅指针）。
