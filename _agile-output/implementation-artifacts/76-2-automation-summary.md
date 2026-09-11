# Automation Summary — Story 76.2 / FR137

## Guardrail tests

- `cargo test -p bitloom --test fr137_external_circt_compile_sim_gate`
- Locks E1–E4: firtool-1.155.0 channel / AD-9, CI `circt-external` + `just circt-external-check` without continue-on-error, FORCE_MISSING + version-mismatch non-zero readable failure, ≠ FR129/121/110/95/96 alone, ≠ docs-only, compile-gate MVP docs

## Local / CI paths

- `just circt-external-check` → `scripts/circt-external-check.sh`
- CI required job `circt-external`: `cargo run -p bitloom -- firtool ensure` + gate script
- Fixture: `crates/rhdl-firrtl/fixtures/fr137_external_circt_gate.fir`

## Out of scope (→ 76.3)

- Epic 76 / FR137 deferred-work + NFR14 closeout checkboxes
- Do not mark 76.3 ready; do not start Story 76.3
