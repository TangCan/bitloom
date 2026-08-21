# FR28 — FrozenHir / FIRRTL → 可编译 Chisel Scala

**验收合同（AD-27）：** 在钉死栈下可编译 + 公开端口名/宽/向与实例层次往返谓词。允许机械风格；**不以**「结构化尽力失败」交差。

**钉死版本对（AD-9 / NFR12）：** Chisel **7.14.0** ↔ firtool **1.155.0**。

```rust
let art = rhdl_firrtl::emit_chisel(&frozen)?;
// 或 .fir → import → emit_chisel
```

**CLI：** `cargo bitloom import --input design.fir --out-dir out --also-chisel` 在写 `.v` 的同时经 `emit_chisel` 写出 `.scala`。

## 产出

- 成功：`.scala`（`class … extends Module`），含层次时 `Module(new Child)` 与按方向连线（跳过 `clk`/`rst`；Chisel `Module` 隐式 clock/reset）。
- 子集外：`MemDecl` → 结构化失败 `rhdl::E0901`（不得冒充 FR28 已覆盖 mem）。

## CI / 本机

- **默认 CI：** Rust 语法检查 + 端口/层次谓词测试（`cargo test -p rhdl-firrtl`）即可绿；**不**要求 CI 安装完整 Chisel JVM。
- **可选真编译：** 本机 Java ≥ 17 + coursier/sbt 时，可运行 [`scripts/chisel-fr28-compile.sh`](../scripts/chisel-fr28-compile.sh)；不满足则干净跳过（不得静默把 FR28 降为尽力失败）。

## 与 AD-3

FIRRTL 文本契约仍是 **FrozenHir ↔ FIRRTL 6.0.0**（`emit` / `import`）。Chisel Scala 是独立产品路径（AD-27），不替代文本契约。
