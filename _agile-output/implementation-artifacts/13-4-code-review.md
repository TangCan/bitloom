# Code review — Story 13.4

**Verdict:** approve

## Findings
- `cargo bitloom new` writes a crate with only `bitloom-prelude` at CLI version; no dependency on CLI package `bitloom`.
- Template mirrors `counter_ports`: `#[module]` + `rhdl_elaborate`.
- `--help` documents scaffolding / bitloom-prelude.

## Must not regress
- Generated `[dependencies]` must stay prelude-only
- Elaborate entry name remains `rhdl_elaborate` until a documented migration
