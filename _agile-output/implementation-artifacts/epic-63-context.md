# Epic 63 Context: 官方风格 Chisel 全家桶

<!-- Compiled from planning artifacts. Edit freely. Regenerate with compile-epic-context if planning docs change. -->

## Goal

Chisel 互操作用户获得超出 FR111 D1+D3 的官方/idiomatic 风格全家桶验收面（FR122）；风险记录钉死规则集与 emit/check API；默认仍不恢复废弃 Scala `Parser.parse`（除非风险记录另开并修订 AD-27）。不以 FR97 MVP、FR111 alone、机械 emit 或 docs-only 关闭。公开品牌 Bitloom。

## Stories

- Story 63.1: Epic 63 NFR14 风险记录
- Story 63.2: AD-27 修订（若需）+ 官方风格全家桶路径（FR122）
- Story 63.3: FR122 收口与 Phase 14 故事清单指针

## Requirements & Constraints

- **FR122：** 相对 FR111 D1+D3 的官方风格规则集增量、验收谓词、`emit_*`/`check_*` API 形状由本 epic NFR14 钉死。
- **禁止关闭口径：** 不得仅 FR97；不得仅 FR111 D1+D3；不得仅机械 `emit_chisel`；不得 docs-only；默认禁止恢复 `Parser.parse`。
- **NFR48：** FR97 / FR111「已关闭」仍有效，不得改写为失败；回归不破。
- **NFR50：** 若触及 AD-27 须先修订脊柱再标实现 story ready。
- **NFR51：** 未列入风险记录的项保持 deferred。
- **依赖：** 硬依赖 Epic 57 已关闭；设计 crate 只依赖 `bitloom-prelude`。

## Technical Decisions

- **AD-27 可能修订：** 允许官方风格全家桶为合法产品完成面（FR122）；默认仍不恢复 Parser；实现 epic 内视需要修订并引用（NFR50）。
- 产品路径落在 `rhdl-firrtl` Chisel 发射面（与 FR97/FR111 共存，不回滚既有路径）。
- 品牌：Bitloom / `bitloom-*`。

## Cross-Story Dependencies

- 63.1 门禁：无有效 NFR14 ⇒ 63.2–63.3 不得标 ready。
- 63.2 视需要修订 AD-27 + 全家桶 emit/check + 多模块 ATDD；FR97/FR111 回归不破。
- 63.3 文档/deferred/README 收口并勾选 Epic 63 关闭；声明 Phase 14 规划/实现故事清单完整指针。
