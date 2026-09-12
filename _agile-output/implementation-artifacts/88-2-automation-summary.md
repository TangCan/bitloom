# Automation Summary — Story 88.2

## ATDD

- `cargo test -p bitloom --test fr152_bitloom_lsp_publish_policy`

Locks: policy (a) selected; `publish=true`; honesty (b) superseded; (a)≠ deepen; NFR72 defer install claim; FR151 CLI dry-run still green; lsp dry-run green.

## Manual / packaging

- `cargo publish -p bitloom-lsp --dry-run --allow-dirty` — success (abort upload due to dry run)
