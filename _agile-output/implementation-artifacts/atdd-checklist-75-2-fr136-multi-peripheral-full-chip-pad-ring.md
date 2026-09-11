# ATDD Checklist — Story 75.2 / FR136

| AC / predicate | Test | Notes |
|----------------|------|-------|
| R1 multi-peripheral | `fr136_r1_chip_pad_ring_elaborate_ports` | `ChipPadRing` elaboratable；GPIO + UART pad 端口 |
| R2 full-chip shape | `fr136_r2_full_chip_ring_shape` | ≥3 bank 或宽≥24 + `bank_pin_index`；≠ 双 bank 16 |
| R3 scoreboard Pass | `fr136_r3_ring_scoreboard_pass` | 期望 vs 观测 pad 序列（GPIO `pad_out` 与/或 UART `uart_tx`） |
| R3 Fail | `fr136_r3_deliberate_wrong_model_fails` | 故意错期望 → Fail |
| R4 / alone | `fr136_r4_not_satisfied_by_fr128_or_fr120_or_fr108_alone` | ≠ FR128/120/108 alone；≠ docs-only |
| Docs / brand | `fr136_docs_contract` | docs/fr136 + Bitloom + R1–R4 |
| NFR56 / deps | `fr136_design_crate_prelude_only` | prelude Cargo.toml 无 bitloom-sim；回归路径仍 prelude |
| FR139 intact | `fr136_fr139_gpio_split_intact` | `ip/gpio/{base,vip,socpad}` 仍在 |

```bash
cargo test -p bitloom --test fr136_multi_peripheral_full_chip_pad_ring
```

Expected before Story 75.2 build: **FAIL**（缺 `ChipPadRing` / 环形状 / 记分板 / docs）。
