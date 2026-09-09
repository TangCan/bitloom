# FR38 / FR49 — built-in hierarchy visualization

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

## Hierarchy entry (Story 23.2)

```bash
cargo bitloom visualize --input crates/rhdl-firrtl/fixtures/external_hierarchy.fir --out-dir target/viz
# alias:
cargo bitloom doc --input crates/rhdl-firrtl/fixtures/external_hierarchy.fir --out-dir target/viz
```

Writes `hierarchy.html` with:

- Modules and ports
- Instance hierarchy list
- Mermaid flowchart of the instance tree

Open the HTML in a browser. This is the product hierarchy path for FR38/FR49 — not a library-only dump.

## LSP / FR91 Path B (explicit defer)

**FR91 Path B (Story 39.3):** Self-hosted Bitloom language-server (shallow diagnostic / symbol LSP MVP **or** full hover/goto) remains **explicitly deferred**. There is **no** Bitloom `language-server` binary in this repo.

**MUST NOT** claim Bitloom LSP / language-server **delivered**. Hierarchy / timing HTML does **not** claim LSP done and does **not** count as FR91 completion (HTML ≠ LSP). Host **rust-analyzer** (FR90) is the Wave D IDE path — **not** a substitute for FR91 self-hosted LSP.

Reinforced in `_agile-output/implementation-artifacts/deferred-work.md` and NFR14 Epic 39 (`nfr14-risk-epic39-ide-multiview.md` — FR91 close checkbox).

**FR90 host path:** Bitloom design crates use **rust-analyzer** (host IDE) for completion / goto / rustc diagnostics — see [`fr90-host-ide-rust-analyzer.md`](fr90-host-ide-rust-analyzer.md). That host workflow is **not** a Bitloom language-server and does **not** close FR91 (Path B).

**Epic 35:** LSP hover/goto is **not** an Epic 35 completion criterion (still deferred). Story 35.4 only re-documents this boundary; it does **not** deliver a language-server.

## Wave / timing

See [`fr38-wave.md`](fr38-wave.md) — `cargo bitloom wave` emits `timing.html` + `wave.vcd` (not GTKWave-only).
