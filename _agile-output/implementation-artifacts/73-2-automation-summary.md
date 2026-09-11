# Automation Summary — Story 73.2

ATDD `fr134_tywaves_gui_ide` already covers G1–G4 predicates; no extra suite expansion required beyond regression.

- `cargo test -p bitloom --test fr134_tywaves_gui_ide`
- `cargo test -p bitloom --test fr125_upstream_tywaves`
- `cargo test -p bitloom --test fr117_typed_ide_wave`
- `cargo clean && cargo fmt --all && just test`
