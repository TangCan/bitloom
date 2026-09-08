# NFR14 风险记录 — Epic 34 一级 IP 可综合基线（无闭包 / FR82）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 10 **NFR37**（规划 done ≠ 深度 done）。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **历史最小合同：** `nfr14-risk-first-class-ip.md`（Epic 22 / FR37+FR48 — stub 级 smoke）。  
> **闭包叠加：** `nfr14-risk-epic29-hls-ip-closures.md`（Epic 29 / FR77 — **须**叠在本 epic 非 stub 基线之上）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 34 后续故事 **34.2–34.4** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR82（加深 FR37/FR48）/ Epic 34；NFR14、NFR37 |
| 记录日期 | 2026-09-08 |
| 状态 | accepted — Epic 34 关闭条件已勾选（Story 34.4） |

### (a) 上游约束

- **五类一级 IP 范围（FR82 / 继承 FR48）：** UART、SPI、I2C、FIFO、AXI 均须达到可 **elaborate → emit → tick**（或文档等价）的**非 stub** 可综合基线；本 epic API **不**接受生成器闭包（闭包定制留给 Epic 29 / FR77）。
- **AXI 范围（PRD Open Q7 已关闭，不变）：** AXI 类 = **AXI4-Lite 最小从接口**；非 Full AXI、非完整互联。
- **与历史最小合同（Epic 22）的区别（NFR37）：** Epic 22 / `bitloom-prelude` IP 表面交付的是**最小端口语义 stub**（可 smoke，但协议/硬件深度不足）。Epic 22 `done` **≠** FR37/FR48 **深度**关闭。本 epic（FR82）须把五类加深到诚实的非 stub 硬件路径（或书面钉死本 epic 交付子集并仍满足 FR82 关闭条）；**不得**继续用 stub 冒充 FR37/48 完成。
- **相对 Epic 22 FR37 起步条：** 历史要求 FIFO + UART + 黑盒 wrapper；本 epic 仍可保留黑盒不透明边界，但 FIFO/UART（及 34.3 其余类）须升级为非 stub 可综合路径，而非仅端口壳。
- **与 Epic 29 依赖顺序：** 本 epic（无闭包 IP 基线）**应先于** Story **29.3**（IP 生成器闭包定制）。29.2（HLS）可与 34 并行；**不得**在 34 基线未就绪时把 29.3 标 `ready` 或用 stub 包一层闭包冒充 FR77。用户排程偏好：**优先本 epic 于 29.2–29.3**。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**；公开品牌 **Bitloom**。
- **NFR37：** 公开/内部文档不得仅以 Epic 22 / 历史 sprint `done` 关闭 FR37/48 深度；深度缺口以 **FR82** 验收。

### (b) 粗工期带

- **预计：** Epic 34 整体约 **1.5–3 人周**（34.1 本风险记录 ≤0.25 人周；34.2 FIFO+UART 非 stub 0.5–1 人周；34.3 SPI/I2C/AXI 或文档最小子集 0.5–1.5 人周；34.4 ATDD + Epic 29 衔接文档 0.25–0.5 人周）。
- **置信度 / 假设：** 中；假设沿用既有 `ElaborateSession` / emit / `Sim::tick`，且「非 stub」定义为可演示的最小真实硬件语义（非全协议栈）。若要求 VIP 对拍或完整协议 FSM，工期显著上修。若 34.3 书面降级部分类，须显式记入决策/文档（NFR37），不得静默声称五类全完成。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **34.2–34.4** 标 `ready` 或开工实现。
- **不得仅重命名 / 包一层 / 改注释**现有 `bitloom-prelude` IP stubs 并宣称 FR82 / FR37/48 深度完成。
- 不得把 Epic 22 stub smoke 或历史 epic `done` 冒充本 epic 深度关闭（**NFR37：规划 done ≠ 深度 done**）。
- 不得将五类缩成「仅 FIFO」（或仅一类）并宣称 FR48/FR82 完成，而不改 PRD / 本记录。
- 不得在本 epic 引入生成器闭包定制 API（留给 Epic 29）；亦不得在无本基线时把 **29.3** 标 ready。
- 不得把 AXI 从「AXI4-Lite 最小从」偷换成 Full AXI / 完整互联验收，或反过来用「只有文档无模块」冒充 AXI 类达标（除非 34.3 书面降级且仍满足 FR82 关闭条）。
- 不得交付无端口语义 / 跳过 elaborate·emit·tick 的空壳冒充非 stub 基线。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR37** 深度诚实度共同责任人
- 备份 / 升级路径：五类砍改或「非 stub」定义争议升级至产品 / Phase 10 排程负责人；与 Epic 29.3 测序冲突升级至 AD-28 / 闭包 epic 维护者。

### 历史 stub vs FR82 基线（对照）

| 维度 | Epic 22 历史最小合同 | Epic 34 / FR82 |
| --- | --- | --- |
| 完成话术 | stub + smoke 可勾 FR37/48 当时 AC | 非 stub 可综合基线；NFR37 禁止用历史 done 关深度 |
| 闭包 | 无 | 仍无（留给 Epic 29） |
| 五类 | UART/SPI/I2C/FIFO/AXI（stub） | 同五类，加深或书面子集 |
| 测序 | 独立 P3 | **先于** 29.3；优先于 29.2–29.3 排程 |

### 并行 / 维护叠加（Chipyard 式）

- 与 Epic 29/31（CDC SyncFifo）并行时：文档须区分「CDC 跨域 FIFO」与「一级 IP SyncFifo 基线」，避免两边各自发明完成话术。
- 与 prelude 发布表面叠加时：新鲜加深 API 宜稳定后再深绑发布表面；树内夹具仍须可演示。
- 升钉 / 端口约定变更须同步五类（或已书面降级的子集）夹具，禁止「只修一类」。

### 引用

- AD-28 — 风险门禁（NFR14）
- PRD NFR14 / FR37 / FR48 / FR82；NFR37（规划 done ≠ 深度 done）
- Open Q7 — AXI4-Lite 最小从
- 历史记录：`nfr14-risk-first-class-ip.md`（Epic 22）
- 闭包叠加：`nfr14-risk-epic29-hls-ip-closures.md`（Epic 34 → 29.3）
- `bitloom-prelude` `ip` — FR82 非 stub 基线（Epic 34.2–34.3）；全协议仍非默认
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 34 关闭条件（Story 34.4 勾选）

- [x] **FR82 五类非 stub：** UART / SPI / I2C / FIFO / AXI 均可 elaborate → emit → tick（文档最小子集；见 `docs/ip/README.md`）
- [x] **本 epic 无生成器闭包 API**（闭包定制叠加点 = **Epic 29** / FR77 / Story 29.3）
- [x] **NFR37：** 相对 Epic 22 stub 历史已文档化；不得用 stub / 历史 `done` 冒充深度关闭
- [x] **ATDD / 配方：** `fr82_ip_baseline_matrix` + `fr82_fifo_uart_baseline` + `fr82_spi_i2c_axi_baseline`（亦含于 `just test`）
- [x] **Epic 29 handoff：** `docs/ip/README.md` 声明无闭包基线 vs 闭包 overlay 测序（34 → 29.3）
- [x] **禁止事项未触发：** 无仅重命名 stub 交差；无静默砍类；无提前标 29.3 ready

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 34 故事 34.2–34.4 标 `ready`。**  
**Epic 34 关闭条件（上节）已于 Story 34.4 勾选。**
