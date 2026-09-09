# ATDD Checklist — Story 45.4 一级 IP 双模型齐全 + Epic 45 收口（FR103）

| Acceptance | Test | Failure mode |
|------------|------|--------------|
| FR103 docs: five IP classes + completion surface | `fr103_docs_completion_surface_and_five_ip_classes` | missing class / completion |
| Adapter templates alone ≠ FR103 | `fr103_docs_adapters_supporting_not_sufficient` | template fake green |
| TLM product still Epic 46 | `fr103_docs_tlm_product_still_epic46` | TLM claimed closed |
| SyncFifo handwritten FL ≡ settle+tick | `fr103_sync_fifo_dual_model_pass` | FL mismatch |
| Deliberate SyncFifo mismatch Fail readable | `fr103_sync_fifo_deliberate_mismatch_fails_readable` | silent Fail |
| UART/SPI/I2C/AXI GeneratedFunctional ≡ tick | `fr103_uart_spi_i2c_axi_generated_fl_rst_compare` | gen FL fail |
| Product API exported | `fr103_product_api_exported` | missing IpDualModelMatrix |
| NFR14 Epic 45 close checkboxes | `fr103_nfr14_epic45_close_conditions_checked` | unchecked close |
| README/deferred Epic 45 closed | `fr103_deferred_readme_epic45_closed` | permanent non-goal lock |
| Sprint epic-45 done; 46–47 backlog | `fr103_sprint_epic45_done_epic46_plus_backlog` | premature 46+ |
| docs/ip + fr103 cross-links | `fr103_docs_ip_and_fr103_cross_links` | missing link |

```text
cargo test -p bitloom --test fr103_ip_dual_model
cargo test -p bitloom --test fr103_epic45_closeout
```
