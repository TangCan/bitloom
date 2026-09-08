# Code Review: Story 29.4 闭包透明抽检与文档

**Verdict:** Approve

## Findings

1. **Accepted (by design):** Single ATDD matrix `fr76_fr77_nfr36_transparency_matrix.rs` consolidates FR76 dissolve C + stub RTL and FR77 `Crc8Lut` viz HTML + Verilog + FIRRTL NFR36/Cap-R-64 spot-checks; deep goldens remain in sibling `fr76_hls_dataflow_closure` / `fr77_ip_generator_closure` (presence asserted).
2. **Accepted (docs):** README / `docs/fr35-hls.md` / `docs/ip/README.md` / language-surface include HLS+IP examples, D1 constraint classes (`HlsFree` vs `SynthesizableClosure`), and explicit FR47「sim generators」disambiguation.
3. **Accepted (epic close):** `nfr14-risk-epic29-hls-ip-closures.md` Epic 29 关闭条件 all `[x]`; sprint `29-4` + `epic-29` marked `done`.
4. **Accepted (deferred):** real-Bambu quality / streaming CRC / automatic rustc body analysis remain out of Epic 29 close scope.

## AC Trace

| AC | Result |
| ---- | ------ |
| Spot-check viz + Verilog/FIRRTL (and HLS C/RTL) no closure semantics (Cap-R-64 / NFR36) | pass |
| Automated/documented check passes | pass (`cargo test -p bitloom --test fr76_fr77_nfr36_transparency_matrix`) |
| User docs: HLS + IP examples, constraint class, FR47 消歧 | pass |
| NFR14 ticks Epic 29 close conditions | pass |

## Verification

- `cargo test -p bitloom --test fr76_fr77_nfr36_transparency_matrix`
- Sibling deep fixtures remain: `fr76_hls_dataflow_closure`, `fr77_ip_generator_closure`

**Accept**
