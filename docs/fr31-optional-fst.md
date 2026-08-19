# FR31 — optional FST waveforms (AD-24)

Default dump is still **VCD** (`Sim::enable_vcd`). FST is opt-in.

RHDL does **not** ship an FST writer. Conversion is:

1. Native `tick` writes a sibling `.vcd`
2. `Sim::finish_waves` runs **gtkwave `vcd2fst`** (or `RHDL_VCD2FST`)

```bash
# typical gtkwave install
vcd2fst wave.vcd wave.fst
```

```rust
sim.enable_fst("wave.fst")?; // requires vcd2fst on PATH or RHDL_VCD2FST
// ... ticks ...
sim.finish_waves()?;
```

**Verilator `--trace-fst`** is for *Verilated* C++ models, not `rhdl-sim` tick. Use it when co-simulating emitted Verilog.

Closing FST leaves VCD dump unchanged (`enable_vcd` only).
