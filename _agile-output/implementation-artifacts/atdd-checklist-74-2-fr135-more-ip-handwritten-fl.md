# ATDD Checklist — Story 74.2 / FR135

| AC / predicate | Test | Notes |
|----------------|------|-------|
| F1 API | `fr135_exports_uart_tx_functional` | `UartTxFunctional` + `verify_uart_tx_handwritten` in `bitloom-sim` |
| F2 Pass | `fr135_uart_tx_handwritten_pass` | FL ≡ settle+tick on pinned ports |
| F2 Fail | `fr135_deliberate_mismatch_fails` | 故意错模型 → !Pass |
| F3 / alone | `fr135_not_satisfied_by_gpio_or_generated_alone` | ≠ Gpio/FR126 alone；≠ GeneratedFunctional alone；≠ SyncFifo alone |
| Docs / brand | `fr135_docs_contract` | docs/fr135 + Bitloom + F1–F3 边界 |
| NFR56 / deps | `fr135_design_crate_prelude_only` | prelude Cargo.toml 无 bitloom-sim |

```bash
cargo test -p bitloom --test fr135_more_ip_handwritten_fl
```

Expected before Story 74.2 build: **FAIL**（缺 `UartTxFunctional` / verify API / docs）。
