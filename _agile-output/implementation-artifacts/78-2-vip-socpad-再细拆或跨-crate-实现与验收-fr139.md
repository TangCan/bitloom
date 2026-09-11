---
title: '78.2 VIP/SocPad 再细拆或跨 crate 实现与验收（FR139）'
type: 'feature'
created: '2026-09-11'
status: 'done'
route: 'oneshot'
baseline_commit: '0a6215a'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic78-vip-socpad-cross-crate.md'
  - '{project-root}/_agile-output/implementation-artifacts/78-1-epic-78-nfr14-风险记录.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/crates/bitloom-prelude/src/ip/'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR139 要求超出 FR131 协议模块拆分的 VIP/SocPad 再细拆（或可选跨 crate）；单体 `ip/gpio.rs` 同时承载 `Gpio`/`GpioVip`/`GpioSocPad`，阻碍更深演进且易被 FR131 alone 冒充关闭。

**Approach:** 按 NFR14 **C1** 将 `ip/gpio.rs` 再细拆为 `ip/gpio/{mod,base,vip,socpad}.rs`，`mod.rs` re-export；**不勾选 C2**（保留 prelude 内细拆，AD-6 不变）；**C3** 公开路径 `bitloom_prelude::ip::{Gpio,GpioVip,GpioSocPad,…}` 稳定；**C4** FR98/108/120/128/131 回归 + ATDD。文档/deferred 全量收口 → Story 78.3。

## Boundaries & Constraints

**Always:** C1 可检查切分；C3 稳定导出；C4 回归绿；品牌 Bitloom；设计 crate 只依赖 bitloom-prelude。

**Ask First:** 若产品强制跨 crate（C2）改写 AD-6 — 须修订 NFR14 + 迁移说明 + 脊柱。

**Never:** 以 FR131 P1–P4 alone 关闭 FR139；silent 断导出；未合同跨 crate；docs-only；勾选 Epic 78 关闭（→ 78.3）。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| C1 落地 | `ip/gpio/` 含 base/vip/socpad + re-export | ATDD 绿；`Gpio*` elaborate OK | 编译/测试失败直至修复 |
| C2 未选 | 无新 IP crate | ATDD 断言无跨 crate | 禁止 silent 加 crate |
| 回归 | FR98/108/120/128/131 测试 | 全绿 | 回滚或修复 |

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip/gpio/{mod,base,vip,socpad}.rs` — C1 细拆
- `crates/bitloom-prelude/src/ip/mod.rs` — 仍 `mod gpio` + `pub use gpio::*`；FR139 注释
- `crates/bitloom/tests/fr139_vip_socpad_split.rs` — ATDD
- `crates/bitloom/tests/fr131_ip_protocol_split.rs` — 接受 `gpio/` 目录形态
- `_agile-output/implementation-artifacts/nfr14-risk-epic78-vip-socpad-cross-crate.md` — C1–C4 合同

## Story

As a 维护者,
I want 按风险记录完成再细拆或跨 crate 搬迁,
So that IP 面可更深演进且回归与公开路径可检查。

## Acceptance Criteria

1. Given Story 78.1, when 落地拆分/搬迁 + ATDD/既有 IP 测试绿；若跨 crate 则同步迁移说明与依赖边界（NFR58）, then 切分与风险记录一致；FR98/FR108/FR120/FR128/FR131 回归不破（NFR56）
2. And 公开路径稳定或提供迁移文档；公开品牌 Bitloom

## Tasks / Subtasks

- [x] T1: C1 拆 `ip/gpio/{mod,base,vip,socpad}.rs` + re-export（AC: 1–2）
- [x] T2: ATDD `fr139_vip_socpad_split.rs`；更新 FR131 路径断言（AC: 1）
- [x] T3: C2 不选；C3 路径稳定；C4 回归（AC: 1–2）
- [x] T4: code-review Approve；automation-summary；sprint 78-2 done

## Dev Notes

- ≠ FR131 alone；≠ FR128/120 alone。
- 默认 prelude 内细拆；跨 crate 须 C2 合同（本故事未选）。
- 78.3：docs/deferred / Epic 78 关闭勾选。

## Testing

- `cargo test -p bitloom --test fr139_vip_socpad_split`
- `cargo test -p bitloom --test fr131_ip_protocol_split`
- `cargo test -p bitloom --tests -- fr108_gpio fr120_gpio fr128_soc`
- `cargo clean && cargo fmt --all && just test`

## Dev Agent Record

### Completion Notes List

- C1：`ip/gpio.rs` → `ip/gpio/{mod,base,vip,socpad}.rs`；公开路径稳定
- C2 未选；AD-6 不变
- ATDD + FR131 兼容目录形态；code-review Approve

### File List

- `crates/bitloom-prelude/src/ip/gpio/mod.rs`
- `crates/bitloom-prelude/src/ip/gpio/base.rs`
- `crates/bitloom-prelude/src/ip/gpio/vip.rs`
- `crates/bitloom-prelude/src/ip/gpio/socpad.rs`
- `crates/bitloom-prelude/src/ip/mod.rs`
- `crates/bitloom/tests/fr139_vip_socpad_split.rs`
- `crates/bitloom/tests/fr131_ip_protocol_split.rs`
- `_agile-output/implementation-artifacts/78-2-vip-socpad-再细拆或跨-crate-实现与验收-fr139.md`
- `_agile-output/implementation-artifacts/78-2-code-review.md`
- `_agile-output/implementation-artifacts/78-2-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
