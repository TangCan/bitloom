# Story 1.4: freeze 拦住多驱动与不可综合构造

Status: done

## Met

- Multi-drive across processes → E0140
- `reject_unsynthesizable` → E0141 structured Diagnostic
- No panic; codes `rhdl::E0xxx`

## Files

- crates/rhdl-hir/src/lib.rs
- crates/rhdl-builder/src/lib.rs
