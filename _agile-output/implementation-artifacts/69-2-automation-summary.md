# Automation Summary — Story 69.2

## ATDD

- `cargo test -p bitloom --test fr129_circt_handshake`
- AD-25 revise lock; docs contract; dialect+multiclock emit; dissolve/AD-18; single-clock/zero-elastic/capturing failures; FR121 alone ≠ FR129

## Product path

- Library: `schedule_circt_handshake`, `schedule_circt_handshake_from_transform`, `meets_fr129_circt_handshake`
- CLI: `cargo bitloom hls --circt-handshake [--channels N] [--clock-domains N] [--elastic-depth N]`
- Docs: `docs/fr129-circt-handshake.md`; cross-links in fr35/fr121

## Out of scope

- Epic 69 closeout checkboxes / deferred ledger stamps → Story 69.3
