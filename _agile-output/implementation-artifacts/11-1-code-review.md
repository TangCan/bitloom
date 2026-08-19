# Code review — Story 11.1 (AD-2 → bitloom)

**Verdict:** Approve with notes.

| Check | Result |
|-------|--------|
| AC: AD-2 publish `bitloom` | Pass |
| Forbid `rhdl` / `rhdl-bits` | Pass |
| AGENTS aligned | Pass (pre-locked Brand lock) |
| Scope creep into Cargo rename | None (deferred to 11.2) |
| Guardrail test | Pass (`ad2_publish_identity`) |

**Notes for 11.2:** Structural seed still lists `crates/rhdl-rs/`; package `name` / bin still `rhdl-rs` / `cargo-rhdl` — expected until 11.2.
