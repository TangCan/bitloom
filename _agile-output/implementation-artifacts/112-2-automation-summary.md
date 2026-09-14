# Automation Summary — Story 112.2

## ATDD

- `cargo test -p bitloom --test fr179_floating_circt_git_head`

Locks: docs pin 1.159.0≠1.158.0≠1.156.0; script/just/CI; AD-9 FR179; FORCE_MISSING; live download/parse.

## Expand

CI job `circt-floating-git-head` runs the same script as `just circt-floating-git-head-check`.
