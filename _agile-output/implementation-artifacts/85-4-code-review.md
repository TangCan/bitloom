# Code Review — Story 85.4

**Verdict:** Approve

**Summary:** FR152(b) documented: `bitloom-lsp` stays `publish=false`; path dep removed from `bitloom`; lib ATDD relocated to `bitloom-lsp/tests/`. ATDD proves `cargo publish -p bitloom --dry-run` succeeds without requiring lsp on crates.io. Option (a) deferred to a new contract.

## Triage

No high/medium patches.
