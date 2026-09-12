# Automation Summary — Story 103.2

- ATDD: `crates/bitloom/tests/fr170_chisel_head_parser.rs`
- Product: `scripts/parser-head-migration-check.sh` + FIRRTL 6.0.0 fixture
- Scala: `fr170_bitloom_firrtl_parser_mainline.scala` (`parseUpdateMainline`)
- AD-27 revise 2026-09-12; Just + CI `parser-head-migration`
- Docs: `docs/fr170-chisel-head-parser.md`
- Regression: `cargo fmt --all && just test`
