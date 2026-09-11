# ATDD Checklist — Story 76.2 / FR137

| AC / predicate | Test | Notes |
|----------------|------|-------|
| E1 tool pin | `fr137_e1_firtool_version_channel_pinned` | firtool-1.155.0 + tag/`firrtl-bin-linux-x64` / AD-9；≠ PATH 信任；≠ CIRCT HEAD |
| E2 CI + path | `fr137_e2_ci_required_job_and_just_path` | `circt-external` job + `just circt-external-check`；无 continue-on-error |
| E3 missing | `fr137_e3_force_missing_nonzero_readable` | `BITLOOM_CIRCT_FORCE_MISSING=1` → 非零 + 可读 |
| E3 mismatch | `fr137_e3_version_mismatch_nonzero` | stub 非 1.155.0 → 非零 + 可读 |
| E4 / alone | `fr137_e4_not_satisfied_by_fr129_or_prior_alone` | ≠ FR129/121/110/95/96 alone；≠ docs-only |
| Docs / brand / MVP | `fr137_docs_compile_gate_mvp_contract` | docs/fr137；选定编译门禁；Bitloom；NFR58 指针 |
| Script fail-closed | `fr137_script_never_silent_skip` | ≠ firtool-smoke `exit 0` skip 模式 |

```bash
cargo test -p bitloom --test fr137_external_circt_compile_sim_gate
```

Expected before Story 76.2 build: **FAIL**（缺 script / CI job / docs / just 目标）。
