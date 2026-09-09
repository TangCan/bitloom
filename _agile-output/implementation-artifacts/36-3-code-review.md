# Code review — Story 36.3 / FR93

**Verdict: Approve**

**Scope:** FR93 permanent non-goals lock in README / deferred-work + PRD addendum pointer + ATDD + NFR14 Epic 36 close.

## Findings

1. **无阻塞缺陷。** 五条永久非目标齐全；「须新 PRD 才能推翻」到位；addendum 指向 README/deferred；未把永久非目标标 done；品牌 Bitloom；未开工 Epic 37。
2. **README 将「永久非目标」与「可延期 deferred」分列**，降低把 VIP/HLS 误读成普通 backlog 的风险。
3. **ATDD** `fr93_permanent_non_goals` 九测覆盖清单、推翻门槛、addendum 指针与品牌；红→绿已验。
4. **NFR14：** Epic 36 关闭条件（含 FR93 / NFR38 / 禁止事项 / 品牌）已勾；记录 `closed`。
5. **双写清单（README + deferred）** 可接受；ATDD 锁关键句防漂移。

## AC trace

| AC | Result |
|----|--------|
| 五条永久非目标列出 | pass |
| 须新 PRD 才能推翻 | pass |
| Bitloom 品牌 + NFR14 Epic 36 关闭勾选 | pass |

## Non-blocking notes

- retrospective 保持 `optional`（符合用户指示）。
- Epic 37+ 仍 backlog，本故事未触碰。
