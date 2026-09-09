# NFR14 风险记录 — Epic 43 VIP / 全协议一级 IP（FR98）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 12 **NFR40 / NFR43**；交付 **FR98**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic42-idiomatic-chisel.md`；`nfr14-risk-epic38-uarttx-deepen.md`；`nfr14-risk-epic34-ip-baseline.md`；`nfr14-risk-first-class-ip.md`。  
> **前置：** Epic 40 **closed**（FR94；Path B 字面绿）；FR82 五类基线（Epic 34）；FR89 UartTx 可编程波特率加深（Epic 38）— **≠** FR98。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 43 后续故事 **43.2–43.5** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR98 / Epic 43；NFR14、NFR40、NFR43；扩大 FR82/FR89；对照 FR37/FR48 |
| 记录日期 | 2026-09-09 |
| 状态 | closed — Story 43.5（FR98 / Epic 43 关闭；GPIO 可选未纳入） |

### (a) 上游约束

- **Epic 40 / FR94 已关闭：** Path B 字面绿合同已开闸；本 epic 实现 FR93#4 → **FR98**（VIP / 全协议一级 IP）。
- **FR82 基线（Epic 34）：** UART/SPI/I2C/FIFO/AXI 各有可综合最小子集（见 `docs/ip/README.md`）；**明确非目标**含 VIP / 全协议 / Full AXI。本 epic **扩大**该合同，不得把基线烟测冒充 FR98。
- **FR89（Epic 38）：** 仅 **UartTx** 可编程波特率**子集**加深（分支 A）；RX / 全双工 / VIP **仍非目标**。**不得**用 FR89 关闭话术冒充 FR98。
- **默认交付范围（ASSUMPTION · epics.md）：** UART、SPI、I2C、AXI **均**须达到本记录钉死的「全协议/近 VIP」必选条。**GPIO 可选**（见下表）；未纳入关闭必选则不得口头宣称 GPIO VIP。
- **裁剪合同：** 若产品砍掉任一类默认交付，**必须**在本记录写明显式裁剪合同（类名、理由、替代关闭条件）并经 Correct Course / PRD 对齐；否则不得宣称 FR98 全绿。
- **每类交付证据：** 清单项可 elaborate → emit → tick（或文档等价）+ **ATDD**；无 ATDD **不得**宣称 VIP / 近 VIP / FR98 该类完成。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。
- **NFR40：** VIP / 全协议 IP 为多年/高维护字面条；不得用短期「改文档重贴标签」或单类加深假绿交差。

### (b) 粗工期带

- **预计：** Epic 43 整体约 **4–12 人周**（43.1 本风险记录 ≤0.25 人周；43.2 UART 1–3 人周；43.3 SPI 1–2.5 人周；43.4 I2C 1–2.5 人周；43.5 AXI（+可选 GPIO）+ FR98 收口 1–3.5 人周）。置信度：**低–中**（近 VIP 谓词细节由本记录钉死；实现复杂度与 VIP 对拍深度 NFR40）。
- **假设：** 不回滚 FR82/FR89 回归；不静默把 Full AXI 互联或外部商业 VIP 对拍列为隐含关闭条件（除非本记录另钉）；不并行冒充 Epic 44–47 关闭。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **43.2–43.5** 标 `ready` 或开工实现。
- **不得只加深一类（例如仅 UART）就宣称 FR98 全绿**，除非本记录已写明**显式裁剪合同**（砍掉的类与替代关闭条件）。
- **不得无 ATDD 宣称 VIP / 近 VIP / 全协议**（文档口号、营销话术、或仅 FR82/FR89 已绿均不足）。
- 不得把 **FR89 UartTx 波特率子集**或 **FR82 最小子集**重标为 FR98 已交付。
- 不得静默把 AXI 验收偷换成 Full AXI 完整互联，或反过来用「仍只有单寄存器玩具」冒充近 VIP Lite。
- 不得把可选 **GPIO** 未交付说成 FR98 失败，或把未钉死的 GPIO 说成已关闭必选。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR40**（多年维护 / 字面绿诚实）共同责任人；**NFR43** 共同注意人
- 备份 / 升级路径：裁剪默认四类交付面、改近 VIP 必选条、或引入商业 VIP 对拍须升级至产品 / Correct Course；与 FR82/FR89 话术冲突升级至 AD-28 维护者。

---

### 「全协议 / 近 VIP」必选验收条（本记录钉死 · 供 43.2–43.5）

> 相对 FR82/FR89 **已知限制**抬升。每条须可测（elaborate/emit/tick + ATDD）。未列特性默认**非目标**，须在实现故事文档「明确非目标」；静默扩大未列模式禁止。

#### UART（Story 43.2）

| # | 必选条 | 相对基线 |
| --- | --- | --- |
| U1 | **TX + RX** 均可达；至少一路全双工或文档化的半双工切换合同 | FR82/FR89 仅 TX |
| U2 | **可编程波特率**驱动 TX 与 RX 位时序（可复用/扩展 FR89 `baud_div`） | FR89 仅 TX |
| U3 | 帧格式至少 **8N1**；起止位正确；文档写明已交付帧面 | 基线已有 TX 8N1；须扩到 RX |
| U4 | 至少一夹具：elaborate → emit → tick（或等价）覆盖 TX 与 RX 路径 | 基线仅 TX smoke |
| U5 | `docs/ip/`（或等价）写明已交付协议面 vs 明确非目标（如小数分频、流控、IrDA 等未列项） | 诚实边界 |

#### SPI（Story 43.3）

| # | 必选条 | 相对基线 |
| --- | --- | --- |
| S1 | **可配置 CPOL/CPHA**（四模式或文档钉死的 ≥2 模式子集，须覆盖与 FR82 Mode-0-ish 对照） | 基线单一 Mode-0-ish |
| S2 | **Master** 多字节/字传输 + `cs_n` 帧边界正确 | 基线单字节 shifter |
| S3 | 至少一夹具 elaborate → emit → tick + ATDD | 基线最小 smoke |
| S4 | 文档写明已交付模式/位序 vs 非目标（DMA、多 CS 阵列、slave 等——若未列则非目标） | 诚实边界 |

#### I2C（Story 43.4）

| # | 必选条 | 相对基线 |
| --- | --- | --- |
| I1 | **ACK/NACK 驱动**的 Master 写（及文档要求的读）路径 | 基线教学玩具无真实 ACK |
| I2 | **START / 7-bit 地址 / 数据 / STOP**；SCL 为真实时钟边沿（非恒高玩具） | 基线 SCL 恒高等 |
| I3 | 至少一夹具 elaborate → emit → tick + ATDD | 基线最小 smoke |
| I4 | 文档写明已交付 vs 非目标（clock stretch、多主、10-bit、slave 等——若未列则非目标） | 诚实边界 |

#### AXI（Story 43.5）

| # | 必选条 | 相对基线 |
| --- | --- | --- |
| A1 | **AXI4-Lite 近 VIP**（默认）或本记录另钉的 AXI 子集：正确 **AW/W/B** 与 **AR/R** 握手；**addr 译码**与 **wstrb** 语义不可再「忽略」交差 | FR82 单寄存器忽略 addr/wstrb |
| A2 | 至少 **多寄存器**或文档等价的可寻址从窗口（非单玩具寄存器） | 基线单寄存器 |
| A3 | 至少一夹具 elaborate → emit → tick + ATDD | 基线最小 smoke |
| A4 | 文档写明 Lite vs Full；**不得**无合同宣称 Full AXI / 互联完成 | 诚实边界 |

#### GPIO（可选 · 默认不纳入 FR98 关闭必选）

| # | 条 | 本记录决策 |
| --- | --- | --- |
| G0 | 双向 GPIO + 方向控制 + elaborate/emit/tick + ATDD | **可选**；Story 43.5 **仅当**产品决定纳入时实现 |
| G1 | 未实现时 | **不**构成 FR98 关闭失败；**不得**口头宣称 GPIO VIP 已交付 |

**显式裁剪合同（当前）：无。** 默认四类 UART/SPI/I2C/AXI 全交付；GPIO 可选。若日后裁剪，在本表下方追加裁剪段并改状态说明。

### FR82 / FR89 vs FR98（摘要）

| 路径 | 角色 | 可否单独关闭 FR98 |
| --- | --- | --- |
| FR82 五类最小子集 | 基线可综合 smoke | **否** |
| FR89 UartTx 波特率加深 | 单类子集 | **否** |
| 本记录近 VIP 四类 + ATDD | FR98 完成面 | **是**（须 43.2–43.5 全满足或显式裁剪） |

### Epic 43 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **43.1** | 本 NFR14 风险记录 + ATDD | **本故事** |
| **43.2** | UART 全协议 / 近 VIP（U1–U5） | Gate：须本记录后才可 ready |
| **43.3** | SPI 全协议 / 近 VIP（S1–S4） | Gate：须本记录后才可 ready |
| **43.4** | I2C 全协议 / 近 VIP（I1–I4） | Gate：须本记录后才可 ready |
| **43.5** | AXI（A1–A4）+ 可选 GPIO + FR98 收口 | Gate：须本记录后才可 ready |

### 并行 / 维护叠加（Chipyard 式 · NFR40 / NFR43）

- 可与 Epic 44–47 **并行规划**，但各自须独立 NFR14；本 epic 不得冒充其他字面条关闭。
- `docs/ip/README.md` 与 deferred-work「VIP 级全协议 IP → FR98」须同一叙事；禁止「代码加深一类、文档仍写 VIP 非目标且宣称 FR98」。
- prelude IP 面 churn：Epic 45 亦可能触达一级 IP——保持合同边界清晰，避免双重关闭口径。

### 引用

- AD-28 — 风险门禁（NFR14）
- PRD NFR14 / FR98；NFR40 / NFR43；扩大 FR82 / FR89
- 对照：FR37 / FR48；`docs/ip/README.md`
- 前置：Epic 40 / FR94；Epic 34 / FR82；Epic 38 / FR89
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 43 关闭条件（Story 43.5 勾选）

- [x] **UART：** U1–U5 + ATDD — Story 43.2
- [x] **SPI：** S1–S4 + ATDD — Story 43.3
- [x] **I2C：** I1–I4 + ATDD — Story 43.4
- [x] **AXI：** A1–A4 + ATDD；（GPIO G0 **未纳入** — G1 可选未交付）— Story 43.5
- [x] **文档 / deferred：** 不再把「VIP 级全协议 IP」列为永久非目标；`docs/ip/` 边界诚实
- [x] **禁止事项未触发：** 无单类加深冒充全绿（除非显式裁剪）；无无 ATDD 宣称 VIP
- [x] **品牌 / 依赖：** 仍为 Bitloom；设计 crate 只依赖 `bitloom-prelude`

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 43 故事 43.2–43.5 标 `ready`。**  
**FR98 完成面 = 本记录钉死的 UART/SPI/I2C/AXI 近 VIP 必选条 + ATDD；不得以 FR82/FR89 或单类加深单独关闭 FR98（除非显式裁剪合同）。**
