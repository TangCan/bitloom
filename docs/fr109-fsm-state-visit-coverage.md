# FR109 — FSM / state-visit simulation coverage (C3)

**Product:** Bitloom (`cargo bitloom` / `bitloom-sim`). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 51 / FR109 closed** (Story **51.3**). Recorder + fixture delivered in Story **51.2**. FR105 Mux v2 MVP remains closed (NFR44). Tywaves typed IDE → deferred（NFR47）；**LCOV + 树内 GUI → FR114 / Epic 56 已关闭**（Story 56.3）。

## Contract (NFR14 M1–M4)

| # | Item | Product evidence |
|---|------|------------------|
| **M1** | FSM / state-visit metric | Registered label set; visit hit/miss |
| **M2** | Report dialect | `# bitloom-sim coverage v3` + `# FR109 C3 FSM/state-visit`; `state_hit` / `state_miss` |
| **M3** | Fixture | ≥1 runnable sim-path fixture (ATDD) |
| **M4** | Readable miss | Unvisited states appear as `state_miss` |

**FR105 Mux v2 alone ≠ FR109. FR34 toggle alone ≠ FR109. Docs-only ≠ FR109.**

## Report dialect (v3 when FSM registered)

```
# bitloom-sim coverage v3
# FR109 C3 FSM/state-visit
hit …
miss …
branch_hit …
branch_miss …
state_hit fsm:demo:Idle
state_miss fsm:demo:Done
```

Without FSM registration, the report stays **`# bitloom-sim coverage v2`** (FR105 Mux path unchanged).

## API

```rust
sim.register_fsm_states("demo", ["Idle", "Busy", "Done"]);
// … tick / drive …
sim.sample_state_visit("demo", "Idle");
let report = sim.coverage_report();
```

Parsers: `parse_state_report` (alongside `parse_report` / `parse_branch_report`).

**FR157:** labels may be obtained automatically from `#[bitloom::fsm]` / `FsmLabels` — see [`fr157-auto-fsm-labels.md`](fr157-auto-fsm-labels.md). FR109 visit semantics unchanged.

## Cross-links

- FR105 Mux v2 (still closed): [`fr105-sim-coverage-ext.md`](fr105-sim-coverage-ext.md)
- FR34 toggle: [`fr34-sim-coverage.md`](fr34-sim-coverage.md)
- FR157 auto labels: [`fr157-auto-fsm-labels.md`](fr157-auto-fsm-labels.md)
- NFR14: `_agile-output/implementation-artifacts/nfr14-risk-epic51-fsm-state-visit-coverage.md`

## Non-goals (NFR47)

- Tywaves / LCOV GUI → **FR114 / Epic 56**
- Claiming Mux v2 report is C3
- Commercial coverage product co-sim

```text
cargo test -p bitloom --test fr109_fsm_state_visit_coverage
cargo test -p bitloom --test fr109_epic51_closeout
cargo test -p bitloom --test fr105_sim_coverage_ext
```
