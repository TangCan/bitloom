# Story 1.5: 从 FrozenHir 发出 Yosys 友好 Verilog

Status: done

- `rhdl-vlog::emit` → Artifact `<abi_name>.v`
- wire/reg/assign/always @(posedge); no always_ff/automatic/logic contract

## Files
- crates/rhdl-vlog/**
- crates/rhdl-hir Artifact/EmittedFile
