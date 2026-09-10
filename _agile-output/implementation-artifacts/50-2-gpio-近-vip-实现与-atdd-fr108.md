---
title: '50.2 GPIO 近 VIP 实现与 ATDD（FR108）'
type: 'feature'
created: '2026-09-10'
status: 'done'
baseline_commit: '2cb53ac'
---

## Story

As a IP 集成者,
I want GPIO 达到风险记录验收条并可 elaborate/emit/tick,
So that FR98 可选缺口被合同关闭。

## Tasks

- [x] `bitloom_prelude::ip::Gpio`（P1 direction / P2 R/W / P3 mask）
- [x] prelude unit smoke + masked/dir tick
- [x] ATDD `fr108_gpio_near_vip`（含 P4 负向 mask=0、docs 边界、NFR44 FR98 smoke）
- [x] `docs/ip/README.md` 索引 + 边界（FR98 可选 vs FR108 交付）
- [x] 评估 `ip.rs` 拆分 → 保持同文件（非关闭条件）
- [x] code-review / automate
- [x] sprint 50-2 done

## File List

- crates/bitloom-prelude/src/ip.rs
- crates/bitloom/tests/fr108_gpio_near_vip.rs
- crates/bitloom/tests/fr98_axi_near_vip.rs（G1 docs 断言兼容 FR108）
- docs/ip/README.md
- _agile-output/implementation-artifacts/nfr14-risk-epic50-gpio-near-vip.md
- _agile-output/implementation-artifacts/50-2-*.md
- _agile-output/implementation-artifacts/sprint-status.yaml
