# Automation summary — Story 41.2

**Mode:** Expand after implementation (FR95 in-tree HLS schedule MVP)  
**Date:** 2026-09-09

## Coverage decision

Story 41.2 ATDD `crates/bitloom/tests/fr95_in_tree_hls_schedule.rs` already guards:

- In-tree loop-unroll emits checkable schedule IR without Bambu
- IR cites FR95 / loop-unroll / trip_count / stages
- RTL stub marked `in-tree-mvp` / FR95
- Zero trip_count readable reject
- Docs distinguish FR95 in-tree vs external（外挂不得单独满足）
- External `emit_c_stub` regression still present
- Unit: `bitloom::hls::tests::in_tree_loop_unroll_without_bambu`

**No additional automate tests** — would duplicate ATDD without new risk surfaces. CLI `--in-tree` is thin wrapper over the same API covered by ATDD. E2E UI N/A.

## Harm / priority

| Risk | Level | Automation |
| ---- | ----- | ---------- |
| Doc-only / stub-Bambu steal FR95 | High | ATDD IR + docs gates |
| Empty schedule shell | High | stage_count / IR fields |
| Delete external FR35 path | Medium | `fr95_external_path_api_still_present` |

## Outcome

Automate step: **accept ATDD + unit as sufficient guardrail suite**; no new files.
