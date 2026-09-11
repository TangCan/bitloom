# ATDD Checklist — Story 77.2 / FR138

| AC / predicate | Test | Notes |
|----------------|------|-------|
| P1 API/workflow/pairing | `fr138_p1_api_workflow_pairing` | `BitloomFirrtlParser.parse` ≡ `Parser.parse`；工作流；Chisel 7.14.0 + firtool-1.155.0 |
| P2 AD-27 + Correct Course | `fr138_p2_ad27_revised_and_correct_course` | AD-27 允许 FR138；Phase 16 Correct Course 痕迹 |
| P3 missing | `fr138_p3_force_missing_nonzero_readable` | `BITLOOM_PARSER_FORCE_MISSING=1` → 非零 + 可读 |
| P3 mismatch | `fr138_p3_version_mismatch_nonzero` | stub 非 1.155.0 → 非零 + 可读 |
| P4 / alone | `fr138_p4_not_satisfied_by_fr130_or_prior_alone` | ≠ FR130/122/111/97 alone；≠ docs-only |
| Docs / brand / path | `fr138_docs_and_just_path` | docs/fr138；`just parser-restore-check`；Bitloom |
| Script fail-closed | `fr138_script_never_silent_skip` | FORCE_MISSING / exit 1；实际 `-parse-only` |

```bash
cargo test -p bitloom --test fr138_parser_restore_ad27
```

Expected before Story 77.2 build: **FAIL**（缺 script / docs / AD-27 FR138 修订 / just 目标）。
