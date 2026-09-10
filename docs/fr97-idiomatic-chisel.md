# FR97 — Idiomatic / 可维护 Chisel Scala

**完成面（AD-27 修订 / Epic 42 已关闭）：** FrozenHir / `.fir` 路径经 Bitloom 自生成 **idiomatic / 可维护** Chisel Scala，验收超出「钉死栈下能编译即可」。

**公开品牌：** Bitloom（crates.io / CLI：`bitloom`）。设计 crate 只依赖 `bitloom-prelude`。

## 与机械 FR28 / FR46 的区分

| 路径 | API | 合同 | 可否单独关闭 FR97 |
|------|-----|------|-------------------|
| **机械可编译** | `emit_chisel` | FR28 / FR46：钉死栈编译 + 端口/层次谓词；允许机械风格 | **否** |
| **Idiomatic / 可维护** | `emit_chisel_idiomatic` + `check_idiomatic_chisel` | **FR97**：命名 + 结构 + 可读性（本页钉死） | **是**（须可验收） |

机械路径文档见 [`fr28-chisel-compilable.md`](fr28-chisel-compilable.md)（诚实句：可编译 ≠ idiomatic）。**不得**仅改文案把机械 emit 标成 FR97。

**Phase 13 可维护加深（FR111）：** 多模块一致性 + 加严标记 → [`fr111-idiomatic-chisel-depth.md`](fr111-idiomatic-chisel-depth.md)（`emit_chisel_idiomatic_fr111` / `check_idiomatic_chisel_fr111`）。FR97 MVP alone ≠ FR111。

**Phase 14 官方风格全家桶（FR122）：** 超出 FR111 D1+D3 → [`fr122-official-style-chisel.md`](fr122-official-style-chisel.md)（`emit_chisel_idiomatic_fr122` / `check_idiomatic_chisel_fr122`）。FR97/FR111 alone ≠ FR122。

## Idiomatic 验收谓词（钉死）

对齐 NFR14 Epic 42 风险记录；**未**采纳 Chisel 官方 Style Guide 全文——不得口头宣称「符合官方风格」。

1. **命名：** `class {Module}`、端口 `val {port}`、实例 `val {inst} = Module(new {Child})` 与 FrozenHir 公开名一致；`clk`/`rst` → Chisel `clock`/`reset` 为稳定映射。
2. **结构：** `extends Module` + `IO(new Bundle {…})`；层次用 `Module(new …)` 与按方向连线。
3. **可读性：** 头注释含 `FR97` / idiomatic；正文分节（`// --- IO ---`、`registers`、`wires`、`instances`、`logic`、`memories`）；禁止单行巨型扁平堆作为通过标准。

自动化：`check_idiomatic_chisel(scala, &hir)` — 失败码 `rhdl::E0904`，英文 + 中文可读；**机械 `emit_chisel` 产出必须失败**（显式降级：继续用机械 API，产物不得标 FR97）。

**作用域 / 空电路（2026-09-10 硬化）：** 端口命名与 `IO(new Bundle)` 按**各模块 class 块**校验（非整文件子串）；`hir` 无模块 → **拒绝**（无空电路豁免；正常 elaborate 在 freeze 已拦空电路）。

## API

```rust
use rhdl_firrtl::{emit_chisel, emit_chisel_idiomatic, check_idiomatic_chisel};

// 机械（FR28）— 显式降级面
let mechanical = emit_chisel(&frozen)?;

// FR97 idiomatic
let art = emit_chisel_idiomatic(&frozen)?;
check_idiomatic_chisel(&art.files[0].contents, &frozen)?;
```

**钉死版本对（AD-9 / NFR12）：** Chisel **7.14.0** ↔ firtool **1.155.0**（与 FR28 相同）。

## 官方现实（非完成定义）

CIRCT 时代 Chisel **不**提供「`.fir` → idiomatic Scala Circuit」官方产品路径；`Parser.parse` 已废弃（chipsalliance/chisel#4899）。**不**要求恢复 Parser。FR97 = Bitloom 工具链自生成合同（AD-3 交换边界仍为 `.fir` + firtool）。

## 范围说明（Epic 42 已关闭）

- **42.1：** NFR14 风险记录（idiomatic 验收条 + 禁止事项）。
- **42.2：** idiomatic 发射 + 验收断言 + 文档区分机械 vs FR97。
- **42.3：** README / deferred 收口、NFR14 Epic 42 关闭勾选、机械回归确认 — **已完成**（本页为现行完成面）。
