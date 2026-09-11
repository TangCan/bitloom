# Automation Summary — Story 85.2

## ATDD

- `cargo test -p bitloom --test fr149_bitloom_firrtl_publish`

Locks: package name `bitloom-firrtl`, `publish=true`, workspace path+version, CLI dep rename, FR149 doc, dry-run publish success.

## Publish

- `cargo publish -p bitloom-firrtl --dry-run` — OK
- `cargo publish -p bitloom-firrtl` — **Published bitloom-firrtl v1.0.0** to crates.io (2026-09-11)
