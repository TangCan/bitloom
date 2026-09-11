# FR139 — VIP/SocPad further split (or cross-crate)

**Product:** Bitloom (`bitloom-prelude::ip`). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 78 / FR139 closed** (Story **78.3**). Product path delivered in Story **78.2** (**C1** prelude-internal split; **C2** not selected).

Phase 16 **规划故事已齐（Epic 72–78）**；实现关闭态：**Epic 72**（闸门 FR133）与 **Epic 78**（本 FR）**已关闭**；**Epic 73–77** 仍须各自实现关闭。本拆分/搬迁已关闭；未列入更深 IP 布局仍属 **NFR59**。终局宣称须对应 **FR133–139** 关闭后方可勾选（**FR140**）。

## Contract (NFR14 C1–C4)

| # | Gate | Evidence |
|---|------|----------|
| **C1** | VIP/SocPad further split | `ip/gpio/{mod,base,vip,socpad}.rs` + re-export（超出 FR131 协议层 `ip/gpio`） |
| **C2** | Cross-crate (optional) | **Not selected** — remains inside `bitloom-prelude`；**AD-6** intact |
| **C3** | Stable public API | `bitloom_prelude::ip::{Gpio,GpioVip,GpioSocPad,…}` unchanged |
| **C4** | Regression | FR98 four-class + FR108 + FR120 + FR128 + FR131 elaborate/tests green |

## Forbidden closes

FR131 P1–P4 alone；FR128/FR120 alone；docs-only；silent export rename；uncontracted AD-6 break.

## Non-regression (NFR56)

FR98 / FR108 / FR120 / FR128 / FR131 closes remain valid. Design crates still depend only on `bitloom-prelude`.
Further IP layout not listed here remains **NFR59** (≠ FR139 C1 alone).

```text
cargo test -p bitloom --test fr139_vip_socpad_split
cargo test -p bitloom --test fr139_epic78_closeout
```
