# FR163 — Unlisted-protocol handwritten FL (beyond UartTx)

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 95 / FR163 closed** (Story **95.3**). Implementation Story **95.2**.

Phase 16 **FR135** `UartTxFunctional` **remains closed and valid** (NFR68). This FR adds
**at least one protocol beyond `UartTx`** with handwritten FL ≡ tick.

## Selected IP (NFR14)

| Item | Contract |
|------|----------|
| IP | `bitloom_prelude::ip::UartRx` |
| FL | `bitloom_sim::UartRxFunctional` |
| API | `IpDualModelMatrix::verify_uart_rx_handwritten` |
| Stimulus | `uart_rx_dual_stimulus` |
| Ports | `rd_data`, `rd_valid`, `rx_busy` |

## Forbidden closes

FR135 `UartTx` alone；FR126 Gpio alone；GeneratedFunctional alone；SyncFifo / FR103 alone；docs-only.

## Non-regression (NFR68)

FR103 / FR112 / FR126 / **FR135 UartTx** closes remain valid. FR135 alone ≠ FR163.

## Deferred (NFR71)

`SpiMaster` / `I2cMaster` / `Axi4LiteSlave` handwritten FL — **not** this epic；require a new contract.

## Recipe

```text
cargo test -p bitloom --test fr163_unlisted_protocol_handwritten_fl
```

## Cross-links

| Doc | Role |
|-----|------|
| [`fr135-more-ip-handwritten-fl.md`](fr135-more-ip-handwritten-fl.md) | FR135 UartTx (still closed; ≠ FR163 alone) |
| [`fr126-more-ip-handwritten-fl.md`](fr126-more-ip-handwritten-fl.md) | FR126 Gpio handwritten |
| NFR14 | `_agile-output/implementation-artifacts/nfr14-risk-epic95-unlisted-protocol-handwritten-fl-fr163.md` |
