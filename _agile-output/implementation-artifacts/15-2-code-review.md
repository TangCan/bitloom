# Code review: Story 15.2

## Summary

PASS. Teaching core ticks ADDI/ADD/BEQ with golden PortValues; bit-ops unblocked decode.

## Notes

- Fixed teaching BEQ offset (+8) documented in `SUBSET.md`.
- Edge-commit timing: tests hold each instr across compute/commit ticks.
- NFR24 satisfied (prelude-only design deps).
