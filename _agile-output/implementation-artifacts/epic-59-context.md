# Epic 59 Context: 无 metadata 全树 syn-scan

<!-- Compiled from planning artifacts. Edit freely. Regenerate with compile-epic-context if planning docs change. -->

## Goal

硬件设计者在无 `[package.metadata.bitloom] design_roots` 时仍能经 workspace `#[bitloom::top]` syn-scan（或等价）发现设计根并进入 elaborate 路径。关闭后可按 FR123 宣称该加深面；Phase 12 FR99 与 Phase 13 FR113 关闭证据仍有效。

## Stories

- Story 59.1: Epic 59 NFR14 风险记录
- Story 59.2: Syn-scan 发现路径实现与验收（FR118）
- Story 59.3: FR118 收口与文档指针

## Requirements & Constraints

- **FR118：** 须交付无 metadata 全树 `#[bitloom::top]` syn-scan（或等价）；禁止仅以 FR99 DesignFixture、仅 FR113 metadata `design_roots`、shallow 伪装 finish、或 docs-only 关闭。
- **NFR14 / NFR49：** 无 Epic 59 专用风险记录则 59.2–59.3 不得 ready。
- **NFR48：** 不得改写 FR99 / FR113「已关闭」为失败；DesignFixture 与 Cargo-graph+metadata 路径须仍可用。
- **NFR51：** 未列入本 epic 的加深保持 deferred，不得 silent 宣称。
- 公开品牌 **Bitloom**；设计 crate 只依赖 `bitloom-prelude`。
- 硬依赖 Epic 57（FR116）已关闭；扩大 FR113 metadata 完成面。

## Technical Decisions

- 风险记录须钉死：workspace 扫描范围、`#[bitloom::top]` 识别策略、无 metadata 根解析/失败语义、与 Cargo-graph+metadata 共存规则。
- 实现形状在 `bitloom-lsp` 发现层（见 `docs/fr113-lsp-design-root-discovery.md` 模式）；根 id 可与 FR113 注册表文档等价衔接，但发现源不得仅靠 metadata。
- Gate：Story 59.1 NFR14 未完成前，59.2–59.3 不得标 ready。

## UX & Interaction Patterns

- 无独立 UX 合同；验收以产品文档/夹具 + ATDD 为准。

## Cross-Story Dependencies

- **59.1** 开门禁；钉死策略后 **59.2** 实现 + 夹具/ATDD；**59.3** 文档/deferred 收口并勾选 Epic 59 / FR118。
- 不依赖 Epic 60–63；可与之并行但各需自有 NFR14。
