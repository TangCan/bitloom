# Automation summary — Story 38.3

**Mode:** Expand after implementation (FR89 Epic 38 closeout)  
**Date:** 2026-09-09

## Coverage decision

Story 38.3 is guarded by closeout ATDD
`crates/bitloom/tests/fr89_epic38_boundary_closeout.rs`:

- NFR14 Epic 38 close conditions all checked + status closed
- deferred-work: full protocol still needs new contract; Story 38.3 cross-ref closed
- deepen ATDD fixture `fr89_uarttx_programmable_baud` still present
- no branch-B RX delivery claims
- docs/ip keeps FR89 subset + non-goals

Deepen behavior remains covered by Story 38.2 ATDD + prelude unit tests.
Optional `examples/ip_box` demo exercises `baud_div` hold (smoke, not duplicate ATDD).

**No additional automate tests** — closeout ATDD + existing deepen ATDD are sufficient;
further tests would duplicate doc/NFR14 assertions without new risk surfaces.

## Harm / priority

| Risk | Level | Automation |
| ---- | ----- | ---------- |
| Epic closed without NFR14 checkboxes | High | closeout ATDD |
| Open deferred pointer leaves FR89 ambiguous | Med | deferred cross-ref ATDD |
| RX / VIP silently claimed at close | High | no-RX + docs ATDD |
| Deepen fixture deleted during closeout | High | fixture-presence ATDD |

## Outcome

Automate step: **accept closeout ATDD + existing FR89 deepen ATDD as sufficient**; no new files beyond the closeout suite already added in this story.
