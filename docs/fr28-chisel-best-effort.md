# FR28 — best-effort FIRRTL / FrozenHir → Chisel Scala

Target: **Chisel 7.14.0** (paired with firtool-1.155.0).

```rust
let art = rhdl_firrtl::emit_chisel(&frozen)?;
```

- Success: `.scala` with `class … extends Module` for the flat subset (ports, wires, regs, comb/seq assigns).
- Failure: structured `ChiselGenError` (`rhdl::E0901` mem, `rhdl::E0902` instance).

**Not** a round-trip. Interop contract remains **FrozenHir ↔ FIRRTL 6.0.0 text** (`emit` / `import`).
