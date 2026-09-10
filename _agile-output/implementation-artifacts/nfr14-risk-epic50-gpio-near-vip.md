# NFR14 风险记录 — Epic 50 GPIO 一级近 VIP（FR108）

> **权威：** PRD NFR14；AD-28；Phase 13 **NFR44 / NFR45 / NFR47**；交付 **FR108**。  
> **前置：** Epic 48 **closed**（FR106）；Epic 43 **closed**（FR98 UART/SPI/I2C/AXI 近 VIP；GPIO G0 **可选未纳入**）。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **50.2–50.3** 标 `ready`。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR108 / Epic 50；NFR14、NFR44、NFR45、NFR47；对照 FR98 G0/G1 |
| 记录日期 | 2026-09-10 |
| 状态 | **closed** — Story 50.3 / Epic 50（2026-09-10） |

### (a) 上游约束

- **Epic 48 / FR106 已关闭：** Phase 13 加深合同已开闸；本 epic 将 FR98 **可选 G0** 升格为 **FR108** 必选。
- **FR98 / Epic 43（已关闭 · 隔离）：** UART/SPI/I2C/AXI4-Lite 近 VIP **仍有效**（NFR44）；**不得**改写为失败，亦不得把「四类已关」冒充 GPIO 已交付。
- **G0（历史可选）：** 双向 GPIO + 方向控制 + elaborate/emit/tick + ATDD — 现为本 epic **必选验收面**。
- **设计依赖：** 设计 crate 只依赖 `bitloom-prelude`；GPIO 实现预计落在 `bitloom-prelude` IP 面（`ip.rs` 体积风险见下）。
- **品牌：** Bitloom / `bitloom-*`。

### (b) 粗工期带

- **预计：** Epic 50 整体约 **1–4 人周**（50.1 ≤0.25；50.2 GPIO 实现 0.75–3；50.3 收口 0.25–0.75）。置信度：**中**。
- **假设：** 不要求商业 VIP 对拍 / 全协议 GPIO 全家桶（NFR47）；既有四类近 VIP 回归不破。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **50.2–50.3** 标 `ready`。
- **不得口头宣称商业 VIP GPIO / 全协议 GPIO 全家桶**（近 VIP ≠ 商业 VIP）。
- **不得无 ATDD 关闭 FR108。**
- 不得改写 FR98「已关闭」为失败（NFR44）。
- 不得静默扩大超出下方近 VIP 必选条（NFR47）。
- 不得冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR47** 共同责任人；NFR44/45 共同注意人。

---

### GPIO「近 VIP」必选条（供 50.2 / FR108）

| # | 必选条 | 说明 |
| --- | --- | --- |
| **P1** | 方向控制 | 每脚或 bank 级方向（in/out）可综合语义 |
| **P2** | 读写 | 输出写 / 输入读（或文档等价端口面）经 elaborate → emit → tick 可检查 |
| **P3** | 掩码或等价子集 | 写掩码 / 按位更新 **或** 文档钉死的等价最小子集 |
| **P4** | ATDD | elaborate/emit/tick + 负向/边界至少一类可读失败 |

**明确非目标（NFR47）：** 商业 VIP 对拍、中断控制器全家桶、全 SoC pad 环、未列入的模拟/开漏扩展。

### `ip.rs` 体积风险（非关闭条件）

- 现状：`bitloom-prelude` `ip.rs` 体积大（Phase 13 inventory ~2869 LOC 量级；Story 50.2 后继续增长）。
- **评估（50.2）：** GPIO 仍落地于同一 `ip.rs`（与 UART/SPI/I2C/AXI 一致），**未**拆分独立模块文件；拆分本身不是 FR108 关闭条件。

### Epic 50 关闭条件（Story 50.3 勾选）

- [x] **50.2 / FR108：** P1–P4 + ATDD
- [x] **文档 / deferred / FR98 交叉链**
- [x] **禁止事项未触发**
- [x] **品牌 / 依赖：** Bitloom；prelude 边界
- [x] **FR98 MVP 关闭仍有效**（NFR44）

---

## 门禁一句话

**缺 NFR14（或缺 a–d）⇒ 不得将 50.2–50.3 标 `ready`。**  
**不得以 FR98 四类近 VIP 冒充 GPIO（FR108）已交付；不得口头商业 VIP GPIO。**  
**Epic 50 / FR108 已关闭（Story 50.3）：** 近 VIP 完成面；FR98 Phase 12 MVP 关闭仍有效；商业 VIP GPIO 仍为 NFR47 非目标。
