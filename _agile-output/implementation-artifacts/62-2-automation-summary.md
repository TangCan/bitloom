# Automation Summary — Story 62.2

## ATDD

- `cargo test -p bitloom --test fr121_handshake_default`
- AD-25 revise lock; docs contract; ready/valid emit; dissolve/AD-18; capturing + zero-channel failures; FR95/FR110 NFR48

## Product path

- Library: `schedule_handshake_default`, `schedule_handshake_from_transform`, `meets_fr121_handshake`
- CLI: `cargo bitloom hls --handshake [--channels N]`
- Docs: `docs/fr121-handshake-default.md`; cross-links in fr35/fr110

## Out of scope

- Epic 62 closeout checkboxes / deferred ledger stamps → Story 62.3
