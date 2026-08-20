# Code review — Story 13.1

**Verdict:** approve with notes

## Findings
- Docs-only change; Cargo package names still `rhdl-*` until 13.2 — intentional per story guardrails.
- AD-6 mermaid now uses `bitloom-*` labels while workspace dirs remain `rhdl-*`; comment clarifies — good.
- ATDD covers AD-2/AD-6/`AGENTS.md`; does not assert package.toml rename (correct for 13.1).
- AD-19 still references `rhdl_prelude::` Rust path — deferred to 13.2; no functional break.

## Must not regress
- CLI publish identity remains `bitloom`
- Forbid `rhdl` / `rhdl-bits` / `rhdl-rs`
- AD-14 host shim unchanged
