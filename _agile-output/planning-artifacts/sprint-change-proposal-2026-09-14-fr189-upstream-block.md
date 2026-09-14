# Sprint Change Proposal — 2026-09-14 — FR189 / Epic 122 upstream block

**Status:** proposed (awaiting product yes/no)  
**Trigger:** Story **122.2** ready-for-dev; NFR14 forbids silent stay on **firtool-1.159.0**; GitHub `llvm/circt` releases + `circt-live-tip-check` both resolve tip = **firtool-1.159.0** only (2026-09-14).

## Problem

FR189 requires AD-9 **default product pin** strictly **>1.159.0**. No such release exists. Implementing 122.2 now would either invent a fake tag or silent-Ok on FR182 — both forbidden.

## Options (pick one)

| ID | Action | Effect |
|----|--------|--------|
| **A (recommended)** | **Park** Epic 122 (`122-2` stays ready-for-dev / blocked-upstream); continue soft-parallel **Epic 123 → 124**; FR191 honesty **lists FR189 unclosed** | No fake pin; loop continues |
| **B** | **Wait** — pause Phase 23 loop until `firtool-*` >1.159.0 publishes, then resume 122.2 | Strict serial; backlog stalls |
| **C** | **Revise FR189 MVP** via Correct Course approve — e.g. close on “bump readiness / refuse-1.159.0-as-FR189 gate” only, concrete pin → **NFR91** until upstream ships | Changes FR189 acceptance; needs explicit yes |

## Recommended default

**A** — park Epic 122; proceed Epic 123 (FR190) then 124 with honesty that FR189 remains open.

## Non-negotiables

- Must not claim FR189 closed on **1.159.0** alone  
- Must not treat FR186 live tip as product pin  
- `git push` is not an FR  

Reply **`A`**, **`B`**, or **`C`** (or `yes` = A).
