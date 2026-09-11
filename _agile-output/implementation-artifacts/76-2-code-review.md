# Code Review — Story 76.2 / FR137

**Verdict: Approve**

**Summary:** External CIRCT **compile-gate MVP** lands E1–E4 beyond FR129 C1–C4: pinned firtool-1.155.0 / AD-9 ensure (≠ PATH random; ≠ CIRCT HEAD), required CI job `circt-external` + `just circt-external-check` (no `continue-on-error`), missing/mismatch → non-zero readable failure, ATDD green. NFR58 synced (spine AD-9 / Phase 16, docs/fr137, README, fr129 cross-link). Sim not selected (documented). Epic closeout correctly deferred to 76.3.

## Adversarial checks

1. Silent skip like `firtool-smoke`? — **pass** (FORCE_MISSING / version mismatch fail-closed).
2. Docs-only / FR129 alone close? — **pass** (script compiles `.fir`; docs ban alone; ATDD E4).
3. FR127 false-positive from comment text? — **fixed** (avoid literal `continue-on-error` in adjacent CI comments).
4. Fixture firtool-incompatible? — **fixed** (FIRRTL 4.0.0 + `connect` + `public module`).
5. Design crate CIRCT dep? — **pass** (CLI/CI/scripts only).
6. Epic 76 closeout premature? — **pass** (NFR14 close boxes unchecked; 76.3 backlog).

## Residual / NFR59

- Sim gate not in this epic (documented).
- macos/windows/linux-aarch64 CI assets still NFR11/NFR59.
