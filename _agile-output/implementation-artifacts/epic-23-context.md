# Epic 23 Context: 内置层次与时序可视化

<!-- Compiled from planning artifacts. Edit freely. Regenerate with compile-epic-context if planning docs change. -->

## Goal

设计者通过产品 CLI/文档入口获得模块层次图与时序图（或等价可浏览视图），不以「自行打开 GTKWave」为唯一完成路径；复用既有 FrozenHir→HTML / VCD 能力并产品化。

## Stories

- Story 23.1: NFR14 风险记录（可视化）
- Story 23.2: 层次视图产品入口（FR38 / FR49）
- Story 23.3: 时序图 / 波形产品入口（FR38 / FR49）
- Story 23.4: 统一文档与 UJ-6 可视化半程
- Story 23.5: FR38/FR49 联验收口

## Requirements & Constraints

- FR38 / FR49：产品入口产出层次视图 + 时序/波形（或等价交互）视图；不得以「用户自行开 GTKWave」作为唯一完成定义。
- FR40：`visualize` / `doc` / `wave`（名称可调）须有 `--help` 与 smoke。
- 允许基于 VCD/FST 渲染/转码，但必须有产品命令或文档入口产出可查看工件。
- 完整 LSP hover/goto 仍可延期，不阻塞本 epic。
- 开工前须有本 epic 的 NFR14 风险记录；缺记录不得将 23.2–23.5 标 ready。
- 品牌 Bitloom；与 `samitbasu/rhdl` 无关。

## Technical Decisions

- AD-28 / NFR14：P3 风险门禁；禁止静默删掉 `wave`/`visualize` 入口而不改 PRD。
- AD-24 / FR31：默认波形仍为 VCD；FST 可选（gtkwave `vcd2fst`）；关闭 FST 时 VCD 路径必须仍可用。
- 既有 `rhdl-viz::to_html`（Story 10.4）与 `bitloom-sim` VCD dump 可复用并产品化；CLI 落在 `cargo bitloom`。
- 设计 crate 仍只依赖 `bitloom-prelude`；可视化生成器属工具链 / CLI。

## Cross-Story Dependencies

- 依赖既有 VCD/FST 与 10.4 HIR HTML；可与 Epic 20–22/24 并行。
- 23.1 门禁 → 23.2–23.5；23.2 层次入口 → 23.3 时序入口；23.4 文档；23.5 同一夹具联验收口。
