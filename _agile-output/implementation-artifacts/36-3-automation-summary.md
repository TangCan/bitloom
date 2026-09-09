# Automation summary — Story 36.3 / FR93

**Decision:** ATDD `fr93_permanent_non_goals` already covers AC 1–3 as durable docs guardrails. No additional automate suite.

## Coverage

| Risk | Severity | Coverage |
|------|----------|----------|
| FR93 五条清单缺失 / 回退 | High | ATDD five list tests + lock heading |
| 「须新 PRD」推翻门槛消失 | High | ATDD `fr93_requires_new_prd_to_overturn` + addendum |
| addendum 仅 inventory、无公开指针 | Medium | ATDD addendum pointer |
| 品牌回退 / 误改 rhdl 发布名 | Medium | ATDD brand + review |
| 误开工 Epic 37 | Medium | Review + story Never |

## Command

```bash
cargo test -p bitloom --test fr93_permanent_non_goals
```
