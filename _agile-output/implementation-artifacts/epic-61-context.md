# Epic 61 Context: 商业 VIP GPIO 全家桶

<!-- Compiled from planning artifacts. Edit freely. Regenerate with compile-epic-context if planning docs change. -->

## Goal

IP 集成者获得超出 FR108 近 VIP（P1–P4）的商业 VIP / 全协议 GPIO 完成面；协议与断言深度由本 epic NFR14 钉死。公开品牌 Bitloom；不以 FR98 四类近 VIP或 FR108 alone 关闭 FR120。

## Stories

- Story 61.1: Epic 61 NFR14 风险记录
- Story 61.2: 商业 VIP GPIO 实现与验收（FR120）
- Story 61.3: FR120 收口与文档指针

## Requirements & Constraints

- **FR120：** 商业 VIP GPIO 完成面必须超出 P1–P4；风险记录钉死相对增量的协议/模式、断言深度、elaborate/emit/tick + ATDD 义务。
- **禁止关闭口径：** 不得仅 FR108 P1–P4；不得仅 FR98 四类；不得口头「商业 VIP」无夹具；不得 docs-only。
- **NFR48：** 不得改写 FR98 / FR108「已关闭」；UART/SPI/I2C/AXI 与 `Gpio` 近 VIP 回归不破。
- **NFR51：** 未列入风险记录清单的协议（如全 SoC pad 环、商业对拍记分板、debounce/驱动强度等）保持 deferred，不得 silent 宣称全家桶外延。
- **依赖：** 硬依赖 Epic 57 已关闭；设计 crate 只依赖 `bitloom-prelude`。

## Technical Decisions

- 主触碰面：`bitloom-prelude` `ip.rs`（体积风险）；**评估拆分**，拆分本身不是 FR120 关闭条件。
- FR108 `Gpio`（P1–P4）保留为近 VIP 基线；商业加深以风险记录钉死的增量面交付（可同文件扩展或并列类型）。
- 品牌：Bitloom / `bitloom-*`。

## Cross-Story Dependencies

- 61.1 门禁：无有效 NFR14 ⇒ 61.2–61.3 不得标 ready。
- 61.2 实现 + ATDD 覆盖风险记录清单；61.3 文档/deferred/IP README 收口并勾选 Epic 61 关闭。
