# Epic 62 Context: Handshake 默认可综合

<!-- Compiled from planning artifacts. Edit freely. Regenerate with compile-epic-context if planning docs change. -->

## Goal

设计者可将 Handshake（或等价动态数据流）作为文档化的默认可综合 HLS/RTL 语义产品路径；必须修订 AD-25（推翻现行「禁止 Handshake 为默认可综合」）。不以 FR95/96 MVP 或 FR110 Q1+Q2 alone 关闭 FR121。公开品牌 Bitloom。

## Stories

- Story 62.1: Epic 62 NFR14 风险记录
- Story 62.2: AD-25 修订 + Handshake 默认可综合路径（FR121）
- Story 62.3: FR121 收口与文档指针

## Requirements & Constraints

- **FR121：** Handshake / 动态数据流进入默认可综合完成面；范围、与 dissolve/AD-18 关系、发射/验收谓词、失败语义由本 epic NFR14 钉死。
- **禁止关闭口径：** 不得仅 loop-unroll / in-tree-mvp stub；不得仅 FR110 Q1+Q2；不得 docs-only；不得未修订 AD-25 即宣称默认语义。
- **NFR48：** FR95/96 / FR110「已关闭」仍有效，不得改写为失败；回归不破。
- **NFR50：** 触及 AD-25 须先修订脊柱再标实现 story ready。
- **NFR51：** 未列入风险记录的全优化套件等保持 deferred。
- **依赖：** 硬依赖 Epic 57 已关闭；设计 crate 只依赖 `bitloom-prelude`。

## Technical Decisions

- **AD-25 必须修订：** 允许 Handshake/动态 DF 为文档化默认可综合语义（FR121）；实现 epic 内引用修订后 AD（NFR50）。
- 闭包数据流变换仍须遵守 AD-18 冻前溶解；捕获闭包不得进入 tick。
- 产品路径落在树内 HLS / 文档开关面（与 FR95/96/FR110 共存，不回滚既有路径）。
- 品牌：Bitloom / `bitloom-*`。

## Cross-Story Dependencies

- 62.1 门禁：无有效 NFR14 ⇒ 62.2–62.3 不得标 ready。
- 62.2 修订 AD-25 + 产品路径 + ATDD；FR95/96/FR110 回归不破。
- 62.3 文档/deferred/HLS 文档收口并勾选 Epic 62 关闭。
