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

## CI / 本机（FR71 / NFR34）

- **默认 CI（Epic 25 / FR71）：** 除 Rust 谓词外，须有 **required** JVM job（见 Story 25.3）对黄金 `.scala` 跑真编译；失败则红。
- **Required 本机/脚本：** [`scripts/chisel-fr28-compile-required.sh`](../scripts/chisel-fr28-compile-required.sh)（或 `BITLOOM_REQUIRE_CHISEL_JVM=1` + [`chisel-fr28-compile.sh`](../scripts/chisel-fr28-compile.sh)）。缺 Java≥17 / sbt / 编译失败 → **非零退出**。ATDD：`bash scripts/test-chisel-fr28-required.sh`。
- **黄金夹具：** [`crates/rhdl-firrtl/testdata/fr28_golden_counter.scala`](../crates/rhdl-firrtl/testdata/fr28_golden_counter.scala)。
- **逃生舱：** `BITLOOM_CHISEL_JVM_SKIP=1` 可跳过并 exit 0——**仅**本地逃生；**默认 CI 不得设置**（NFR34）。
- **可选 legacy：** 不设 require 时，缺工具链仍可 skip=0（非合同路径）。
- **本机 `just test`：** 默认可仍仅 Rust（Story 25.2 增加 `just chisel-fr28-jvm`）。

## 与 AD-3

FIRRTL 文本契约仍是 **FrozenHir ↔ FIRRTL 6.0.0**（`emit` / `import`）。Chisel Scala 是独立产品路径（AD-27），不替代文本契约。
