# Epic 22 Context: 一级 IP 库

<!-- Compiled from planning artifacts. Edit freely. Regenerate with compile-epic-context if planning docs change. -->

## Goal

设计者可经官方 IP（树内或组织发布包）例化 UART / SPI / I2C / FIFO / AXI4-Lite 最小从，并保留黑盒路径；每类至少有 elaborate → emit → tick smoke，且设计侧只依赖 `bitloom-prelude`（及文档化的官方 IP 包名）。

## Stories

- Story 22.1: NFR14 风险记录（一级 IP）
- Story 22.2: 树内 FIFO + UART + 黑盒（FR37）
- Story 22.3: SPI IP smoke（FR48）
- Story 22.4: I2C IP smoke（FR48）
- Story 22.5: AXI4-Lite 最小从接口（FR48）
- Story 22.6: IP 索引与例化文档（FR48 收口）

## Requirements & Constraints

- FR37：树内至少 FIFO + UART 可 elaborate/emit/tick；另至少一黑盒 wrapper（不透明、不内联子 HIR）。
- FR48：五类一级 IP 均可依赖例化；各至少一 smoke；AXI 类 = AXI4-Lite 最小从（Open Q7 已关闭）；保留黑盒。
- 设计 crate 只依赖 `bitloom-prelude`（及文档化官方 IP 包）；禁止静默把五类缩成「仅 FIFO」；禁止把 AXI 扩成 Full AXI 冒充达标。
- 开工前须有本 epic 的 NFR14 风险记录（含树内 vs 组织发布治理偏好）；缺记录不得将 22.2–22.6 标 ready。
- 品牌 Bitloom；与 `samitbasu/rhdl` 无关。

## Technical Decisions

- AD-28 / NFR14：P3 风险门禁；并行多项须记维护叠加。
- AXI 达标定义锁定 AXI4-Lite 最小从接口（非 Full AXI / 非完整互联）。
- IP 可为树内 stub 或官方包；新鲜 IP 宜稳定后再深绑进核心 prelude 表面；黑盒路径须保留。
- 最小可综合 stub 即可，但须真实 Bitloom 模块（可 elaborate），不得空壳跳过 elaborate。

## Cross-Story Dependencies

- 依赖 Epic 19（NFR14 模板；若 IP 用 Bundle 则依赖 FR51）。可与 Epic 20/21/23/24 并行。
- 22.1 门禁 → 22.2–22.6；22.2 立 FIFO/UART/黑盒模式 → 22.3–22.5 复用；22.6 在五类齐后收口文档与索引。
