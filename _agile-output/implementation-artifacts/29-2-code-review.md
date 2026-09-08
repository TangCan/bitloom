# Code Review: Story 29.2 HLS 数据流闭包

**Verdict:** Approve

**Scope:** `bitloom::hls` dissolve API + CLI `--dataflow` + `fr76_hls_dataflow_closure` ATDD + FR76 docs

## Findings

1. **Accepted (by design):** MVP uses `dissolve_dataflow_transform(..., || HlsDataflowOp::…)` (descriptor pattern like `CombInline`); no rustc body analysis of arbitrary `#[hls]` Fn bodies this story.
2. **Accepted (AD-25):** Scheduling remains external Bambu / CI stub; Bitloom only emits dissolved C.
3. **Accepted (D1):** `HlsFree` + violation tokens (`CapturingOrStateful` / `WrongPathForSynthesizable`); synthesizable leg still documented as `SynthesizableClosure`.
4. **Accepted (NFR36):** Dissolved C / stub `.v` spot-checked for closure/`Fn`/`||` residue; header wording avoids false-positive tokens.
5. **Accepted (deferred):** 29.4 viz sweep; real-Bambu quality for `map_xor`; richer op set.

## AC Trace

| AC | Result |
| ---- | ------ |
| Documented HLS dataflow closures; D1 check; dissolve before schedule/lower (FR76) | pass (`dissolve_dataflow_transform` + docs) |
| Algorithm fixture → synthesizable RTL or documented intermediate | pass (stub `.v` + emit-only `.c`) |
| No closure residue before backends (NFR36) | pass (ATDD spot-check) |
| Missing backend errors clearly (no silent success) | pass (CLI ATDD) |

## Verification

- `cargo test -p bitloom --test fr76_hls_dataflow_closure`
- `cargo test -p bitloom --test hls_smoke --test hls_supported_docs`
- `cargo test -p rhdl-hls --lib`

**Approve** — mark `29-2-hls-数据流闭包: done`.
