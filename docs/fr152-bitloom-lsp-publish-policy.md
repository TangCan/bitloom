# FR152 — bitloom-lsp publish strategy (a)

> **Contract:** Phase 19 / **FR155** / Epic 88 / Story 88.2（升格 Phase 18 **FR152(b)** → **FR152(a)**）。  
> **Brand:** Bitloom.  
> **Honesty:** Phase 18 FR152(**b**) 关闭证据仍有效；本文件记录 **(a)** 为当前产品政策，不是把 (b) 改写为「失败」。

## Selected policy: **(a)** (Phase 19)

| Option | Meaning | Status |
| --- | --- | --- |
| (b) | Keep `bitloom-lsp` `publish = false`, and ensure it does **not** block `cargo publish -p bitloom` | **Superseded** — Phase 18 closed; see honesty above |
| **(a)** | Promote `bitloom-lsp` to `publish = true` + version on crates.io (**live** `cargo publish` → Story 88.3) | **Selected** |

## Workspace rules

- `crates/bitloom-lsp/Cargo.toml`: `publish = true`; package metadata suitable for crates.io; deps use workspace entries that carry **version** (registry) so packaging succeeds.
- `crates/bitloom/Cargo.toml`: **no** `bitloom-lsp` path dependency (unchanged from (b) — CLI install path **FR151** stays independent).
- LSP library ATDD that needs `use bitloom_lsp::…` lives under `crates/bitloom-lsp/tests/`.
- File/doc/CLI ATDD for LSP packaging claims may live under `crates/bitloom/tests/` (dry-run / policy guards).

## Publish packaging assert

- `cargo publish -p bitloom-lsp --dry-run` must succeed (Story 88.2).
- **Live** `cargo publish -p bitloom-lsp` (Story **88.3**): **Published bitloom-lsp v1.0.0** on crates.io (2026-09-12). See [`fr155-bitloom-lsp-publish.md`](fr155-bitloom-lsp-publish.md).
- `cargo publish -p bitloom` / `cargo install bitloom` (**FR151**) must remain green and must **not** require unpublished path-only lsp.

## Honesty

- **(a) = crates.io packaging + live publish** for the existing LSP binary/crate surface.
- **(a) ≠ shipping a deeper LSP product** (feature deepen remains other FR / NFR59 items if any).
- Do not use Phase 18 (b) docs alone to claim (a) without FR155 / Epic 88 close evidence (**NFR72**).
- Live publish evidence: [`fr155-bitloom-lsp-publish.md`](fr155-bitloom-lsp-publish.md).
