# Epic 86 Context: 发版后诚实面与 SemVer 跟进

**FRs:** FR153  
**NFRs:** NFR14, NFR64, NFR65, NFR67  
**Depends on:** Epic 85 closed (`bitloom-firrtl` / `bitloom-viz` / `bitloom` 1.0.0 on crates.io).

## Goal

Update SemVer assume-published / remove 1.0.0 special-case so `just semver-check` defaults to minor; refresh `docs/fr146-*` / README / Release honesty for CLI published; close Epic 86 / Phase 18 story list. NFR59 stays deferred (NFR67). Do not silently expand FR142.

## Stories

| Story | Deliverable |
| --- | --- |
| **86.1** | NFR14 risk record (gate for 86.2–86.3) |
| **86.2** | SemVer default + docs/Release honesty (FR153) |
| **86.3** | Closeout: Epic 86 / Phase 18 pointers; NFR59 still deferred |

## Soft order

86.1 → 86.2 → 86.3. No ready on 86.2–86.3 until 86.1 done.
