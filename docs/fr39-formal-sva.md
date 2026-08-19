# FR39 — Formal / SVA export

`rhdl_formal::emit_sva(hir, &[AssertProp])` writes `*_sva.sv` with concurrent `assert property`.

`check_sva_text` is a tiny fixture checker used in tests so a deliberately false property fails.
