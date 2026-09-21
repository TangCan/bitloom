# Story127.3 automate: actual SV producer monitor sensitivity

One new P0 native test (`p0_sv_producer_monitor_rejects_each_hold_violation_and_accepts_release`) executes 32 actual Icarus simulations. The existing monitor declarations and body were moved verbatim into `crates/bitloom/tests/fr196_axi_lite_csr/producer_monitor.sv`, wrapped in a task. Both the original real bridge+CsrBlock tick and the probe call that exact shared source before the rising edge. The original integration stimulus sequence is unchanged.

24 negative cases cover AW/W/AR/CSR response × still stalled/eventual accepting edge × valid withdrawal/first payload field/second payload field. Every negative requires simulation exit 1 and precisely one matching `FATAL:` diagnostic line; reaching the success marker fails. Tool version, compile, timeout or process errors cannot satisfy this check. Eight positive cases cover stable acceptance followed by legal withdrawal and reset cancellation followed by a fresh accepted offer, for all four channels. Probe payload uses legal error encodings 2→3. Cases print counts and elapsed time in future native runs.

These are monitor-only sensitivity probes, not product-DUT mutation evidence, protocol-wide verification, or additional formal proof. No browser, Pact or Playwright runner applies to this Rust/RTL backend. No additional shared fixture provisioning is needed; the local testbench factory uses fresh per-case source and the existing per-process tool directories.

Final targeted execution, using the prescribed PATH and environment:

- `cargo test -p bitloom --test fr196_axi_lite_csr p0_sv_producer_monitor -- --nocapture`: exit 0, one native entry passed, 24 exact-fatal negatives and eight positives; wall 2.6235s.
- `cargo test -p bitloom --test fr196_axi_lite_csr p0_same_session_bridge_real_csr -- --nocapture`: exit 0, one existing native integration passed; wall 0.2426s.
- Scoped rustfmt check and diff whitespace check: exit 0.

Initial runs also passed; manager review then changed probe numeric values from 1→2 to legal CSR errors 2→3 and both requested filters reran. Initial log/JSON bytes are preserved under `127-3-automate-monitor-initial-*`, with original filenames retained for their JSON references. Final evidence is `127-3-automate-monitor-final-executions.json`, including command/start UTC/wall/exit, actual versions, source SHA256, and all 66 initial+final target artifact directories for manager archiving (33 per run). The final subset is also named separately. No unexpected failures occurred.

Backend schema output: `/tmp/tea-automate-backend-tests-1273.json`. Only integration.rs, tools.rs, new shared producer_monitor.sv and this worker’s evidence were edited. No product changes, story/sprint edits, clean, commit or push. Manager owns aggregate archive and full regression.
