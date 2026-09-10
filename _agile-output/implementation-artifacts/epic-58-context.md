# Epic 58 Context: Tywaves 级 typed IDE 波形

<!-- Compiled from planning artifacts. Edit freely. Regenerate with compile-epic-context if planning docs change. -->

## Goal

调试 / 验证工程师获得超出 FR104 `interactive.html` I1–I3 与 FR114 LCOV/`coverage.html` 的 **Tywaves 级（或风险记录钉死的自研等价）typed IDE 波形**一等产品路径。关闭后可按 FR123 宣称该加深面；Phase 12/13 波形与覆盖率关闭证据仍有效。

## Stories

- Story 58.1: Epic 58 NFR14 风险记录
- Story 58.2: Typed IDE 波形路径实现与验收（FR117）
- Story 58.3: FR117 收口与文档指针

## Requirements & Constraints

- **FR117：** 须交付 Tywaves 一等集成或自研等价 typed IDE 波形；禁止仅以 interactive.html I1–I3、静态 VCD/GTKWave、或 FR114 LCOV alone、或 docs-only 关闭。
- **NFR14 / NFR49：** 无 Epic 58 专用风险记录则 58.2–58.3 不得 ready。
- **NFR48：** 不得改写 FR104/105/FR114「已关闭」为失败；默认 VCD / interactive.html / LCOV 路径须仍可用。
- **NFR51：** 未选加深子集保持 deferred，不得 silent 宣称。
- 公开品牌 **Bitloom**；设计 crate 只依赖 `bitloom-prelude`。
- 无独立 UX 合同；交互验收以产品文档/夹具 + ATDD（或文档化手动清单）为准。

## Technical Decisions

- 风险记录须至少钉死一类：（A）Tywaves 一等集成，或（B）自研等价 typed IDE 波形；写清工具/版本/夹具与验收谓词。
- 波形/可视化实现形状不在脊柱钉死 crate 切分；本 epic 属 sim/可观测性加深，不依赖 syn-scan / SBY / GPIO。
- 硬依赖 Epic 57（FR116）已关闭。

## UX & Interaction Patterns

- 无 UX-DR；typed IDE 波形交互细则由本 epic NFR14 与产品文档钉死。

## Cross-Story Dependencies

- **58.1** 开门禁；钉死 A/B 子集后 **58.2** 实现 + 夹具/ATDD；**58.3** 文档/deferred 收口并勾选 Epic 58 / FR117。
- 不依赖 Epic 59–63；可与之并行但各需自有 NFR14。
