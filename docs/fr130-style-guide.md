# FR130 — Full Style Guide pack (no Parser restore)

**Product:** Bitloom (`rhdl_firrtl` / Chisel emit). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 70 / FR130 closed** (Story **70.3**). Product path + AD-27 revise delivered in Story **70.2**.

FR122 O1–O4 close **remains valid** (NFR52). **Parser.parse not restored.**

Beyond FR122 O1–O4: Style Guide markers (`scalafmt-style`, `withClockAndReset`, per-module
`FR130 style-guide`). **Parser.parse is not restored.**

## Contract (NFR14 S1–S4)

| # | Gate | Evidence |
|---|------|----------|
| **S1** | Style Guide header | `FR130 Style Guide` + `scalafmt-style` |
| **S2** | Naming / clock discipline | `withClockAndReset` comment |
| **S3** | No Parser | `Parser.parse not required / not restored` |
| **S4** | API + per-module marker | `emit_chisel_style_guide_fr130` / `check_chisel_style_guide_fr130` |

## Forbidden closes

FR97/FR111/FR122 O1–O4 alone; docs-only; silent Parser restore.

## Non-regression (NFR52)

FR122 / FR111 / FR97 closes remain valid.

```text
cargo test -p bitloom --test fr130_style_guide
cargo test -p bitloom --test fr130_epic70_closeout
```
