# FR81 Mem→Chisel 合同决策 — 2026-09-09

> **权威角色：** Epic 33 / **FR81** 唯一路径裁决页；加深 FR28 / **AD-27**（可编译 Chisel Scala 产品腿）。  
> **给定：** Story 33.1 NFR14 — `_agile-output/implementation-artifacts/nfr14-risk-epic33-chisel-mem.md`。  
> **绑定：** `epics.md` Epic 33 Story 33.2；PRD addendum 回链；NFR12 / NFR37 / FR71。  
> **品牌：** Bitloom；设计 crate 只依赖 `bitloom-prelude`。  
> **状态：** **ADOPTED**（Story 33.2）— **Path A only**

---

## 1. 唯一路径（FR81）

| 项 | 裁决 |
| --- | --- |
| **选定路径** | **Path A — 支持文档化 Mem 子集**，使 `emit_chisel` 对子集内 `MemDecl` 产出可在钉死栈下编译的 Chisel Scala |
| **未选定** | Path B（永久非目标 + 替代验收）— **明确拒绝**作为 Epic 33 关闭路径 |
| **理由** | AD-27 / FR46 产品路径是 FrozenHir → **可编译** Chisel Scala；Mem 留在永久 E0901 与「可编译产品腿」张力过大。NFR14 利弊表 Path A 更贴合深度目标；范围可压到 AD-21 已有单时钟表面 |
| **NFR37** | Epic 20 / FR28「done」+ 历史全量 `MemDecl`→E0901 **≠** FR81 深度关闭；本页选定 Path A 后方进入可测关闭条件（实现 → 33.3；夹具/FR71 → 33.4） |

**互斥声明：** 本文**只**采纳 Path A。不得同时声称「永久非目标」与「支持子集」。Path B 的替代验收条文**不**作为本 epic 验收。

---

## 2. 支持的 Mem 形态（Path A 子集）

锚定 **AD-21** 语言表面与 HIR `Stmt::MemDecl`（单时钟；`sync_read` 区分形态）。子集内须映射到 Chisel 标准库等价物（机械风格可接受）：

| Bitloom / HIR | 语义 | 目标 Chisel（钉死 7.14.0） | 备注 |
| --- | --- | --- | --- |
| `Mem`（`sync_read = false`） | 异步读 / 寄存器堆风格 | `Mem`（或文档等价单口异步读） | 单时钟；单逻辑口面（AD-21 表面） |
| `SyncReadMem`（`sync_read = true`） | 同步读（读延迟 1） | `SyncReadMem` | 单时钟；与 AD-21 / language-surface 一致 |
| 可选 `init: Option<Vec<u64>>` | Elaborate-time 常量初值（FR73 已消解为字） | Chisel 侧等价 init / 字面填充 | 仅常量字；无闭包残留（NFR36） |

**共同约束（子集内）：**

- 单时钟模块包络（默认 AD-15；多时钟仅经 AD-22 CDC 原语，**不**经裸双时钟 mem）。
- 深度 / 位宽为 HIR 已钉死的 `depth` / `width`。
- 互转/FIRRTL 腿仍锚 `firrtl.mem`（AD-3 / AD-21）；本页只约束 **`emit_chisel` Scala 腿**。

---

## 3. 子集外（仍须明确失败 — 保留 E0901）

下列**不**在 Path A 支持面内；实现（33.3）**不得**删除 E0901 冒充全表面支持：

- 未封装的双时钟 / 跨时钟裸 `mem`（AD-21 / AD-22：仅经命名 CDC FIFO）。
- 超出当前 HIR `MemDecl` 字段的多口/掩码/写延迟变体（若未来扩展 HIR，须先扩合同再降级）。
- 以 CHIRRTL 方言文本冒充 FrozenHir↔`.fir` 合同（仍禁止；AD-3）。
- 任何「碰巧 emit 出未验收/不可编译 Scala」而无正例夹具的路径。

**FR71：** 既有无 Mem 黄金夹具与 `just chisel-fr28-jvm` / GHA `fr28-chisel-jvm` **不得**因本决策或后续实现变红或被削弱。

---

## 4. 版本约束（NFR12 / AD-9）

| 组件 | 钉死版本 | 说明 |
| --- | --- | --- |
| Chisel | **7.14.0** | Stack / AD-9；interop 对照 |
| CIRCT firtool | **1.155.0** | 与 Chisel 7.14.0 配对 |

- **不得**为「让 Mem 编过」私自升 firtool-1.156.0 或换未配对 Chisel。
- 升钉须等上游正式配对并更新 ARCHITECTURE-SPINE Stack / AD-9 后再改。

---

## 5. 对后续故事的关闭指针

| Story | 本决策下的期望 |
| --- | --- |
| **33.3** | 实现 Path A：子集内 `MemDecl` → 可编译 Scala；子集外稳定 **E0901**（或文档等价结构化失败）；不漂移 NFR12；不破坏 FR71 |
| **33.4** | ATDD 覆盖 Path A 正例 + 子集外失败；`just chisel-fr28-jvm` 回归；文档更新 Mem↔Chisel 边界；勾选 NFR14 Epic 33 关闭条件 |

---

## 6. 引用

- `nfr14-risk-epic33-chisel-mem.md` — 利弊表；本页选定其 **Path A** 行
- ARCHITECTURE-SPINE：**AD-27**（可编译 Scala）、**AD-21**（Mem/SyncReadMem）、**AD-9** / Stack（NFR12）
- PRD：**FR81** / FR28 / FR46；**NFR12**；**NFR37**；FR71 / NFR34
- `crates/rhdl-firrtl/src/chisel.rs` — 今日全量 `MemDecl`→`rhdl::E0901`（33.3 按本页收敛）
- `epics.md` Epic 33 Story 33.2–33.4
