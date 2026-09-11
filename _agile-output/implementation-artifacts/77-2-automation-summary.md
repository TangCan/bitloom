# Automation Summary — Story 77.2 / FR138

## Tests

- `cargo test -p bitloom --test fr138_parser_restore_ad27` — **7 passed** (P1–P4)
- Guards: AD-27 FR138 revise + `BitloomFirrtlParser.parse` ≡ `Parser.parse`, Chisel 7.14.0 / firtool-1.155.0 pairing, Correct Course Phase 16 reuse, FORCE_MISSING / version mismatch non-zero readable, ≠ FR130/122/111/97 alone / ≠ docs-only, `just parser-restore-check`
- Regression intent: FR97 / FR111 / FR122 / FR130 (NFR56); FR130 Style Guide S3 wording retained for that FR
- Full gate: `cargo clean && cargo fmt --all && just test`

## Coverage notes

- Product path = `firtool -parse-only` under AD-9 (CIRCT-era substitute for removed Scala Parser)
- Scala façade documents API name only; not pulled into design crates
- No Epic 77 closeout; do not start Story 77.3
