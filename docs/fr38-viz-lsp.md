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

## LSP / FR91 Path B / Epic 44 (FR99)

**FR91 Path B (Story 39.3 — historical close):** Epic 39 closed FR91 by **explicit defer** of a self-hosted Bitloom language-server as that epic’s completion path. That Path B defer is **historical only** and is **not** a Phase 12 / FR99 completion bar. Hierarchy / timing HTML does **not** count as LSP (HTML ≠ LSP). Host **rust-analyzer** (FR90) is the Wave D Rust IDE path — **not** a substitute for Bitloom hardware-semantics LSP.

**Epic 44 / FR99 (Phase 12) — closed (Story 44.4):** Stories **44.2–44.3** delivered installable **`bitloom-lsp`** with **didSave → full-design elaborate**, `publishDiagnostics`, and document symbols / goto — see [`fr99-bitloom-lsp.md`](fr99-bitloom-lsp.md). Epic 44 / FR99 closeout and Path B completion-narrative revocation are **done**. HTML still ≠ LSP.

**MUST NOT** claim: (1) HTML visualization is LSP; (2) rust-analyzer alone completes FR99; (3) FR91 Path B defer still satisfies Phase 12 LSP literal-green.

Reinforced in `_agile-output/implementation-artifacts/deferred-work.md`, NFR14 Epic 39 (`nfr14-risk-epic39-ide-multiview.md` — FR91 Path B checkbox remains historically ticked), and NFR14 Epic 44 (`nfr14-risk-epic44-full-elaborate-lsp.md` — **closed**).

**FR90 host path:** Bitloom design crates use **rust-analyzer** (host IDE) for completion / goto / rustc diagnostics — see [`fr90-host-ide-rust-analyzer.md`](fr90-host-ide-rust-analyzer.md). That host workflow remains available and is **not** a substitute for FR99 `bitloom-lsp`.

**Epic 35:** LSP hover/goto was **not** an Epic 35 completion criterion. Story 35.4 only re-documented that boundary.

## Wave / timing

See [`fr38-wave.md`](fr38-wave.md) — `cargo bitloom wave` emits `timing.html` + `wave.vcd` (not GTKWave-only).
