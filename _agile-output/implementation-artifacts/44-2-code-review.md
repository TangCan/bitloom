# Code Review: Story 44.2 Bitloom LSP 服务器 MVP（FR99）

**Verdict:** Approve

**Scope:** `crates/bitloom-lsp` + `docs/fr99-bitloom-lsp.md` + fr38/fr90/README/deferred honesty + FR91/FR90 guard rewrite + `fr99_bitloom_lsp_server_mvp` ATDD

## Findings

1. **无阻塞缺陷。** 可安装/可启动 `bitloom-lsp`（stdio + `tower-lsp-server`）；`initialize` 返回 `serverInfo` + `textDocumentSync` capabilities；VS Code 接线文档齐全；夹具 `fr99_bitloom_lsp_initialize_session` 可复现会话。
2. **诚实边界正确：** 未实现按键全 elaborate（44.3）；未勾选 Epic 44 / FR99 关闭（44.4）；文档明确 rust-analyzer 单独不完成本故事；HTML ≠ LSP 保留。
3. **Epic 39 回归交接正确：** FR91/FR90 从「禁止 `bitloom-lsp` 存在」改写为「禁止未文档别名半成品；允许 Epic 44 文档齐全 MVP」；Epic 39 FR91 Path B checkbox / story done 仍成立。
4. **依赖边界：** 设计 crate 未引入 `bitloom-lsp`；LSP 为工具链面 crate（`publish = false`）。

## AC Trace

| AC | Result |
| ---- | ------ |
| `bitloom-lsp` + initialize/capabilities + 编辑器接线；夹具可复现会话 | pass |
| 品牌 Bitloom；不得声称仅 rust-analyzer 完成 | pass |
| 未做 44.3 elaborate；未勾选 FR99/Epic 44 关闭；prelude-only 设计依赖 | pass |

## Decision

**Approve** — sprint `44-2: done`；`44-3`/`44-4` backlog；`epic-44: in-progress`。
