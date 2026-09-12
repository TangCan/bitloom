# Automation Summary — Story 109.2

## ATDD

- `cargo test -p bitloom --test fr176_deeper_parser_chisel_ecosystem`

Locks: docs contract; emit/check; FR165-alone fails FR176; just/script/CI; FORCE_MISSING; AD-27 revise.

## Product gate

```bash
just chisel-ecosystem-deepen-check
BITLOOM_ECOSYSTEM_FORCE_MISSING=1 just chisel-ecosystem-deepen-check   # expect non-zero
```

CI job `chisel-ecosystem-deepen` mirrors the just recipe.
