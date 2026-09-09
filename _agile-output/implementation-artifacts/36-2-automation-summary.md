# Automation summary — Story 36.2 / FR87

**Decision:** ATDD `fr87_doc19_contract_green` already covers AC 1–5 as durable docs guardrails. No additional automate suite.

## Coverage

| Risk | Severity | Coverage |
|------|----------|----------|
| P5–P7 合同绿条款缺失 / 回退 | High | ATDD p5/p6/p7 |
| 字面未交付勾选全绿话术回潮 | High | ATDD forbid + NFR38 cite |
| README / deferred 断链合同绿 | Medium | ATDD readme+deferred |
| 误扩 FR93 正文进本故事 | Medium | Review + story Never |

## Command

```bash
cargo test -p bitloom --test fr87_doc19_contract_green
```
