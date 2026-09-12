# FR127 — Forced real `sby` in default CI

**Product:** Bitloom. **Status:** **Epic 67 / FR127 closed** (Story **67.3**).

Beyond FR119 local-optional `just formal-sby-check` / docs alone.

## CI shape (NFR14 S1–S4)

| Item | Contract |
|------|----------|
| Job | `.github/workflows/ci.yml` → `formal-sby` |
| Install | `bash scripts/ci-install-sby.sh` (yosys + z3 + sby) |
| Run | `bash scripts/formal-sby-check.sh` (same as `just formal-sby-check`) |
| Failure | No `continue-on-error`; missing tools → non-zero |

**FR161** deepens install hygiene (pins / drift tracking) — see
[`fr161-formal-sby-image-hygiene.md`](fr161-formal-sby-image-hygiene.md).
FR127 alone ≠ FR161.

## Forbidden closes

Docs-only; silent skip; rewriting FR119 closed as failed.

## Non-regression (NFR52)

FR119 local path remains valid.

ATDD: `cargo test -p bitloom --test fr127_forced_sby_ci`
