# Automation Summary — Story 97.2

## Commands

```bash
cargo test -p bitloom --test fr165_deeper_chisel_parser_ecosystem
just chisel-style-lint-check
BITLOOM_STYLE_LINT_FORCE_MISSING=1 just chisel-style-lint-check   # expect non-zero
cargo fmt --all && just test
```

Locks: L1–L5 Style Guide/linter deepen; FR130 alone fails; force-missing; HEAD Parser deferred (NFR71).
