# Automation Summary — Story 115.2

## ATDD

- `cargo test -p bitloom --test fr182_unpaired_firtool_product_pin`

Locks: docs; AD-9 FR182 unpaired exception; CLI/FIRTOOL_TARGET 1.159.0; gate scripts; FR179 path/channel; FR173/174/179 close honesty; live `firtool info`/`ensure`.

## Expand

CI firtool-ensure / circt-external* / parser-* jobs resolve AD-9 **1.159.0**. FR179 job still uses floating-head cache channel.
