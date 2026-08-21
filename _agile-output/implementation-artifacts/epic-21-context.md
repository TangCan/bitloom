# Epic 21 Context: 双视图模拟器生成

<!-- Compiled from planning artifacts. Edit freely. Regenerate with compile-epic-context if planning docs change. -->

## Goal

让设计者对双视图模块**生成** Rust 功能模拟器与周期精确模拟器工件，经桥接/对照运行，并与产品等价检查联验；故意不一致则 fail。手写 bridge/abstraction/both 路径保持可用，生成路径建立其上。

## Stories

- Story 21.1: NFR14 风险记录（双模拟器生成）
- Story 21.2: 手写 bridge / abstraction / both 回归（FR29）
- Story 21.3: 生成 Rust 功能模拟器工件（FR47 腿 1）
- Story 21.4: 生成周期精确模拟器工件 + 桥接对照（FR47 腿 2）
- Story 21.5: 双视图等价接入生成路径（FR30）

## Requirements & Constraints

- FR47：CLI/API **生成**功能模拟器工件（**Rust crate**；Open Q6 已关闭；不强制 SystemC）与周期精确模拟器工件；经桥接或对照运行；与 FR30 联验；故意破坏等价则 fail。
- FR29：手写 `#[bridge]` / `#[abstraction]` / mixed `both` 与 `PortValues` 对照仍须可用；**不再**禁止生成功能模拟器（生成见 FR47）；生成不取代手写标注能力。
- FR30：双视图等价为产品能力；P3 收口须接入 FR47 生成路径（一致 pass、故意不一致 fail）。
- 设计 crate 仍只依赖 `bitloom-prelude`；生成器属工具链 crate。
- 开工前须有本 epic 的 NFR14 风险记录（字段 a–d）；缺记录不得将 21.2–21.5 标 ready。
- 禁止静默把生成路径改回「仅手写对照」而不改 PRD；禁止宣称 SystemC TLM 已交付。
- SM-7 反指标：仅手写 functional、无生成路径 = 未完成。

## Technical Decisions

- AD-5：周期精确只从 FrozenHir `tick`；功能视图可为手写 `#[functional_model]` **或**工具链生成的 Rust 功能模拟器 crate；不承诺 SystemC TLM-2.0。
- AD-17：对照只比较 `PortValues`（HIR 拥有）。
- AD-28 / NFR14：P3 风险门禁；并行多项须记 Chipyard 式维护叠加。
- 工业 TLM↔RTL 自动等价极少；FR47/FR30 字面完成成本高（addendum 已知工程现实）。
- 功能模拟器形态默认生成 Rust crate（PRD Open Q6 已关闭）；周期精确沿既有 tick/cdylib。

## Cross-Story Dependencies

- 依赖 Epic 19（AD-5 修订 + NFR14 模板）；可与 Epic 20/22/23/24 并行（各需 NFR14）。
- 21.1 风险记录 → 门禁后方可 ready 21.2–21.5。
- 21.2 手写回归稳定 → 21.3/21.4 生成路径；21.3→21.4→21.5（FR30 接生成产物）。
