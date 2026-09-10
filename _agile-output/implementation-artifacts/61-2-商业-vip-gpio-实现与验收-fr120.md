---
title: '61.2 商业 VIP GPIO 实现与验收（FR120）'
type: 'feature'
created: '2026-09-10'
status: 'done'
route: 'oneshot'
baseline_commit: '5ed8911'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic61-commercial-vip-gpio.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-61-context.md'
  - '{project-root}/_agile-output/implementation-artifacts/61-1-epic-61-nfr14-风险记录.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR120 需要超出 FR108 P1–P4 的商业 VIP GPIO 产品面（风险记录 C1–C4），且不得破坏 FR98/FR108。

**Approach:** 在 `bitloom-prelude` `ip.rs` 新增 `GpioVip`（保留 `Gpio` 近 VIP）：C1 上升沿 IRQ、C2 开漏+`pad_oe`、C3 原子 set/clear、C4 ATDD；docs/ip 命名 FR120；回归夹具覆盖四类 IP + `Gpio`。拆分 `ip.rs` 非关闭条件（仍同文件）。

</frozen-after-approval>

## Implementation Notes

- `GpioVip`：elaborate→emit→tick；同拍优先级 wr→set→clr；`irq_out` = (pending∧en)≠0。
- ATDD：`fr120_gpio_commercial_vip`；放宽 `fr108` docs 守卫以允许命名 FR120 商业面。
- sprint：`61-2: done`；epic-61 仍 in-progress。

## Review Triage Log

- `ip.rs` 体积继续增长 — **accept**（NFR14：拆分非关闭条件）。
- FR108 文档口号守卫 — **patch**（允许 FR120/GpioVip 语境下的商业 VIP 叙述）。

## Dev Agent Record

### Completion Notes List

- `GpioVip` C1–C3 + ATDD C4；FR98/FR108 回归绿；docs/ip 更新

### File List

- `crates/bitloom-prelude/src/ip.rs`
- `crates/bitloom/tests/fr120_gpio_commercial_vip.rs`
- `crates/bitloom/tests/fr108_gpio_near_vip.rs`
- `docs/ip/README.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic61-commercial-vip-gpio.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `_agile-output/implementation-artifacts/61-2-*.md`
