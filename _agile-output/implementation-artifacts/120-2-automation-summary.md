# Automation summary — Story 120.2 / FR187

- ATDD: `cargo test -p bitloom --test fr187_handshake_lower_deepen` (7 ok)
- Regression: `fr180_handshake_dialect_deepen` still green
- Full gate: `cargo clean && cargo fmt --all && just test`
- No new CI job required (product path covered by ATDD + existing HLS CLI); FORCE_MISSING via env
