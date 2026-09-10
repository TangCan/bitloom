# Automation Summary — Story 63.1

## ATDD

- `cargo test -p bitloom --test nfr14_risk_epic63_official_style_chisel`
- Locks NFR14 a–d, O1–O4 beyond D1+D3, API `emit_chisel_idiomatic_fr122` / `check_idiomatic_chisel_fr122`, default forbid Parser, ban FR97/FR111/mechanical/docs-only, owners, gate 63.2–63.3

## Regression guards patched

- `fr117_epic58_closeout` / `fr118_epic59_closeout` / `fr119_epic60_closeout` / `fr120_epic61_closeout` / `fr121_epic62_closeout`: allow `epic-63` backlog|in-progress|done after 63.1

## Out of scope

- No official-style emit/check product path; no AD-27 Rule body revise; no FR122 closeout docs
