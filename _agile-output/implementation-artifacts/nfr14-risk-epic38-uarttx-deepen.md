# NFR14 风险记录 — Epic 38 一级 IP 显式加深（FR89 / UartTx）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 11 **NFR39**（禁止静默扩大子集）。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic37-interop-hls.md`；`nfr14-risk-epic34-ip-baseline.md`。  
> **前置：** Epic 34 / **FR82** UartTx 基线已交付（8N1 bit-bang，**baud = clk**）；见 `docs/ip/README.md`、`bitloom_prelude::ip::UartTx`。  
> **加深目标（ASSUMPTION）：** `bitloom_prelude::ip::UartTx`（非 SyncFifo）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 38 后续故事 **38.2–38.3** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR89 / Epic 38；NFR14、NFR39；对照 FR82 基线 |
| 记录日期 | 2026-09-09 |
| 状态 | closed — Story 38.3；分支 A 加深已交付；Epic 38 关闭条件已勾选 |

### 加深分支选择（Story 38.1 钉死 · 必读）

| 选项 | 含义 | 本 epic 决策 |
| --- | --- | --- |
| **(A) 可编程波特率子集** | 在 FR82 TX 8N1 之上增加可配置分频/波特计数（仍 8N1 TX；非全特性） | **选用** |
| **(B) 最小 RX 子集** | 增加最小 UART RX 路径 | **不选用**（本 epic **不得**声称已交付） |

**选定：分支 A — 可编程波特率子集。**

**选型理由（可行性）：** A、B 在现有 prelude HIR/`ElaborateSession` 表面均可实现；A 沿用既有 TX 移位/帧状态机，仅叠加分频器，合同面更小、与 FR82「TX 已有、RX 明确非目标」文档对照更清晰；B 引入采样/边沿/帧同步，范围更大且易被误读为全双工。按产品偏好（二者皆可行时优先 A）钉死 A。

### (a) 上游约束

- **FR89：** 在 FR82 基线之上，对**至少一类**一级 IP 交付**显式加深子集**（可 elaborate → emit → tick + 文档边界）；允许树外示例；**禁止**静默宣称全协议 / VIP / Full AXI。
- **加深类钉死：** `bitloom_prelude::ip::UartTx`（Epic ASSUMPTION；非 `SyncFifo`）。
- **FR82 基线边界（对照，不得吞掉）：** 当前 UartTx = **8N1 bit-bang，baud = clk**；文档已知限制含：非可编程波特率 / RX / 全双工（`docs/ip/README.md`；`ip.rs` non-goals）。Epic 38 **仅**把「可编程波特率」从非目标提升为本 epic 选定加深子集；**不**自动解锁 RX / 全双工 / VIP。
- **分支 A 交付语义（供 38.2）：** 可编程波特率（分频或等价计数）驱动 TX 位时序；仍为 **8N1 TX**；须至少一夹具 elaborate → emit `.v` → tick（或文档等价）。具体端口/参数形状由 38.2 实现并文档化。
- **分支 B 明确非交付：** 最小 RX **不**在本 epic 范围；不得在文档/测试标题中暗示 RX 已交付。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。
- **NFR39：** 禁止静默扩大子集；未选中的分支与 VIP/全协议不得冒充完成。
- **与 deferred-work：** 「全协议 / VIP / 可编程 baud 全家桶」等 standing 项：本 epic **仅**合同化可编程波特率**子集**；全协议仍须新合同；38.3 须交叉引用收口。

### (b) 粗工期带

- **预计：** Epic 38 整体约 **0.75–2 人周**（38.1 本风险记录 ≤0.25 人周；38.2 可编程 baud 实现 + 文档边界 0.5–1.25 人周；38.3 ATDD/边界收口 + 可选树外示例 0.25–0.75 人周）。
- **置信度 / 假设：** 中；假设沿用既有 `ElaborateSession` / emit / `Sim::tick`，分频宽度与复位语义保持最小可测。若要求多波特率表、小数分频、或同时交付 RX，工期显著上修且须改本记录 / PRD。Epic 39 工期**不**计入本记录。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **38.2–38.3** 标 `ready` 或开工实现。
- **不得声称全双工 / 可编程全特性 / VIP**（或等价完成话术）作为本 epic 关闭条件。
- **不得静默扩大到 SPI / I2C / AXI**（或 SyncFifo 深度全家桶）并宣称 FR89 已覆盖多类加深。
- **不得把 FR82 baud=clk 基线冒充「已含可编程波特率」**而不交付分支 A 加深语义。
- **不得声称分支 B（最小 RX）已交付**（本记录已钉死 A）。
- 不得在未改 PRD / 本记录的前提下把 FR89 静默砍回「仅文档、无 elaborate/emit/tick」。
- 不得引入生成器闭包定制 API 冒充本 epic 加深（闭包仍属 Epic 29 / FR77 轨道）。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR39** 子集边界共同责任人
- 备份 / 升级路径：加深分支改选（A→B）或扩大到 VIP/多类 IP 须升级至产品 / PM 并改本记录与 PRD；与 FR82 基线话术冲突升级至 AD-28 维护者。

---

### FR82 基线 vs Epic 38 加深（对照）

| 维度 | FR82 / Epic 34（基线） | Epic 38 / FR89（本记录） |
| --- | --- | --- |
| 目标 IP | 五类最小非 stub | **仅 UartTx** 显式加深 |
| UartTx 时序 | 8N1，**baud = clk** | **分支 A：** 可编程波特率子集（仍 8N1 TX） |
| RX / 全双工 | 明确非目标 | **仍非目标**（分支 B 未选） |
| VIP / 全协议 | 非目标 | **仍非目标**（NFR39） |
| SPI/I2C/AXI/FIFO | 各有基线 | **不**因本 epic 加深 |

### 风险主题对照（本 epic）

| 主题 | 风险 | 缓解（本 epic 故事） |
| --- | --- | --- |
| FR82 边界被 VIP 叙事吞掉 | 文档/营销把加深写成全协议 UART | 38.1 钉死 A + 禁止项；38.2/38.3 文档「明确非目标」 |
| 未选分支冒充交付 | RX 或其它 IP 被顺手勾 done | 分支表 + ATDD 断言选 A 非 B；NFR39 |
| 静默扩类 | SPI/I2C/AXI「顺便加深」 | 禁止清单；FR89 一类即可关闭 |

### Epic 38 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **38.1** | 本 NFR14 风险记录 + ATDD；钉死分支 **A** | **本故事** |
| **38.2** | UartTx 分支 A 加深实现（prelude → HIR → emit）+ 文档边界 | Gate：须本记录后才可 ready |
| **38.3** | ATDD/边界收口；可选树外示例；勾选关闭条件 | Gate：须本记录后才可 ready |

### 并行 / 维护叠加（Chipyard 式）

- 与 Epic 39（IDE / 多视图）并行时：不得把宿主工作流故事依赖未就绪的 UART 加深 API；IP 加深与 IDE 合同分离。
- 与 prelude 发布表面叠加：加深 API 稳定前避免深绑发布叙事；树内夹具须可演示。
- `docs/ip/README.md` 与 deferred-work「全协议仍须新合同」须同一叙事；禁止「代码加深、文档仍写死 baud=clk 且无 FR89 边界」。

### 引用

- AD-28 — 风险门禁（NFR14）
- PRD NFR14 / FR82 / FR89；NFR39（禁止静默扩大子集）
- `docs/ip/README.md` — FR82 UartTx 基线与已知限制
- `nfr14-risk-epic34-ip-baseline.md` — FR82 五类基线门禁（已关闭）
- deferred-work — 全协议 / VIP / 可编程 baud 全家桶仍须新合同（本 epic 仅子集 A）
- 体例：`nfr14-risk-epic37-interop-hls.md`
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 38 关闭条件（Story 38.3 勾选）

- [x] **FR89：** UartTx **分支 A（可编程波特率）** 可 elaborate → emit → tick（或文档等价）— Story 38.2（`baud_div`；夹具 `fr89_uarttx_programmable_baud`）
- [x] **文档边界：** `docs/ip/README.md`（或等价）写明本 epic 交付子集与明确非目标（全协议/VIP/全双工/RX 等）— Story 38.2–38.3
- [x] **未选分支：** 不得声称分支 B（最小 RX）已交付 — NFR39
- [x] **ATDD / 收口：** 加深路径自动化稳定；deferred-work「全协议仍须新合同」交叉引用 — Story 38.3（`fr89_epic38_boundary_closeout`）
- [x] **禁止事项未触发：** 无 VIP/全协议宣称；无静默扩到 SPI/I2C/AXI；无提前标 38.2–38.3 ready（对本记录而言）
- [x] **品牌 / 依赖：** 仍为 Bitloom；设计 crate 只依赖 `bitloom-prelude`

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 38 故事 38.2–38.3 标 `ready`。**  
**加深分支已钉死为 A（可编程波特率）；Epic 38 关闭条件已由 Story 38.3 勾选（status: closed — Story 38.3）。**
