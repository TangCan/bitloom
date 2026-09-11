# Automation Summary — Story 85.4

## ATDD

- `cargo test -p bitloom --test fr152_bitloom_lsp_publish_policy`
- Relocated: `cargo test -p bitloom-lsp --test fr99_bitloom_lsp_full_elaborate|fr113_*|fr118_*`

## Publish packaging

- `cargo publish -p bitloom --dry-run` succeeds without `bitloom-lsp` on registry
