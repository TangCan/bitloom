# FR39 / FR85 — Formal / SVA export

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

## FR39 (historical minimal)

`rhdl_formal::emit_sva(hir, &[AssertProp])` writes `*_sva.sv` with concurrent `assert property`.

`check_sva_text` is a **toy** fixture helper (string heuristics) used only in unit tests so a deliberately false property fails. It is **not** an external formal tool.

## FR85 — Option A (non-toy close; Epic 35 / Story 35.4)

**Chosen path:** real design → export SVA → documented external checker invocation.

1. **Export** from a real Counter HIR via:

   ```bash
   cargo run -p rhdl-formal --example export_fr85_counter
   # writes crates/rhdl-formal/fixtures/fr85_counter_sva.sv by default
   ```

2. **Check** with the documented product entry (not part of default `just test`):

   ```bash
   just formal-sva-check
   # or: bash scripts/formal-sva-check.sh
   ```

   The script re-exports, then invokes an **external** checker:

   - default: `verilator --lint-only --assert` when `verilator` is on `PATH`
   - optional: `sby` when selected (`BITLOOM_FORMAL_CHECKER=sby` + `BITLOOM_FORMAL_SBY_FILE=...`)

   If no checker is available, the script **exits non-zero** with a readable error (**never** silent success). Force the missing-tool path with `BITLOOM_FORMAL_FORCE_MISSING=1`.

**MUST NOT** close FR85 / claim FR39 depth done using `check_sva_text` / toy heuristics alone (NFR14 / NFR37).

## LSP (not this epic)

LSP hover/goto remains **deferred** and is **not** an Epic 35 completion criterion — see [`fr38-viz-lsp.md`](fr38-viz-lsp.md).
