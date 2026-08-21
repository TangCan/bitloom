# FR46 — Chisel / `.fir` → Bitloom（反向腿）

**验收合同（AD-27 / FR46）：** `.fir`（及文档化 Chisel 工作流产出）→ FrozenHir → emit / tick；公开端口名/宽/向与实例图满足与正向对称的往返谓词。

**钉死版本对：** Chisel **7.14.0** ↔ firtool **1.155.0**（与 FR28 相同）。

## CIRCT 交换边界

本工具链**不**解析 Chisel Scala 源码，也**不**恢复已删除的 Scala `Parser.parse`。产品路径：

1. Chisel 设计在钉死栈下编译 / `emit` → FIRRTL 文本（经 firtool），或直接提供兼容子集的 `.fir`
2. `rhdl_firrtl::import(.fir)` → `FrozenHir`
3. 同一后端：`emit`（FIRRTL）/ `bitloom_vlog::emit` / `emit_chisel` / `bitloom-sim` `tick`

```rust
let hir = rhdl_firrtl::import(fir_text)?;
rhdl_firrtl::ports_roundtrip_ok(&original, &hir)?;
rhdl_firrtl::instance_graph_roundtrip_ok(&original, &hir)?;
let _ = bitloom_vlog::emit(&hir);
```

## 夹具

- **导出再导入：** `emit` → `import` → `ports_roundtrip_ok` / `instance_graph_roundtrip_ok`
- **外部 `.fir`：** [`crates/rhdl-firrtl/fixtures/external_hierarchy.fir`](../crates/rhdl-firrtl/fixtures/external_hierarchy.fir) — 使用 firtool 常见的 `y <= u0.y` 输出连线

## CLI 产品入口（Story 20.5）

```bash
cargo bitloom import --input design.fir --out-dir out
# optional: also re-emit FIRRTL text
cargo bitloom import --input design.fir --out-dir out --also-fir
# optional: also emit Chisel Scala (FR28 `emit_chisel`)
cargo bitloom import --input design.fir --out-dir out --also-chisel
```

混合夹具（一侧 Bitloom elaborate、一侧外部 `.fir`，同一 `bitloom_vlog::emit`）：[`examples/chisel_mixed`](../examples/chisel_mixed)。

## 与 FR28 / AD-3

- **AD-3：** FrozenHir ↔ FIRRTL 6.0.0 文本契约不变。
- **FR28：** FrozenHir/`.fir` → 可编译 Chisel Scala（正向）。
- **FR46 反向：** 本文档路径；CLI 产品入口见 Story 20.5（`cargo bitloom import`）。

## NFR10

HIR→源码再生仍仅调试，**不得**冒充本 FR46 反向腿。
