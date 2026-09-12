# NFR14 风险记录 — Epic 95 未列协议手写 FL（FR163）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**；Phase 19 **NFR68–NFR72**；实现面 **FR163**；宣称须引对应 FR。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic74-more-ip-handwritten-fl.md` / `nfr14-risk-epic94-deeper-gui-ide-fr162.md`。  
> **前置：** Epic 87 / FR154 **closed**；Epic 74 / **FR135** `UartTx` handwritten FL **closed**；Epic 66 / **FR126** Gpio handwritten **closed**；Epic 94 / FR162 **closed**。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 95 后续故事 **95.2–95.3** 标为 `ready`，亦不得开工实现。**不得**以 FR135 `UartTx` alone 勾选本 FR。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR163 / Epic 95；NFR14、NFR68、NFR69、NFR71、NFR72；对照 FR135 / FR126 / FR103 / FR112 |
| 记录日期 | 2026-09-12 |
| 状态 | open / in-progress — Story 95.1；Epic 95 未关闭 |
| **选定** | 在保留 FR135 `UartTxFunctional` / FR126 `GpioFunctional` 关闭面的前提下，授权 **MVP：至少一项超出 `UartTx` 的协议手写 FL** — 选定 **`UartRx`**（`UartRxFunctional` ≡ tick + `IpDualModelMatrix::verify_uart_rx_handwritten` 或等价）；SPI / I2C / AXI handwritten **不做**（须新合同 / NFR71） |

### Phase 12–18 / FR135 关闭面 vs Epic 95 实现边界（NFR68 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR103** | SyncFifo 手写 + UART/SPI/I2C/AXI **GeneratedFunctional** | **仍有效**；GeneratedFunctional alone ≠ FR163 |
| **FR126 / Epic 66** | `GpioFunctional` 手写 | **仍有效**；alone ≠ FR163 |
| **FR135 / Epic 74** | `UartTxFunctional` 手写 | **仍有效；不得改写为失败**；**alone ≠ FR163** |
| **Epic 87–94** | 闸门 / … / FR162 | **已关闭；本 epic 不做** |
| **Epic 95 / FR163** | **未列协议手写 FL**（本记录选定：`UartRx`） | **本实现 epic** |
| **Epic 96–98** | 其余 NFR59 + 宣称 | **本 epic 不做** |

**选定：关闭「仅 UartTx / Gpio 手写」缺口；≠ 重做 FR135；≠ 本批交付 SPI/I2C/AXI 全套手写 FL。**

### FR163 候选协议与本批选定（NFR71）

| 候选（Epic / README 明示） | 本 epic |
| --- | --- |
| **`UartRx` 手写 FL ≡ tick** | **选定 = FR163 MVP** |
| `SpiMaster` / SPI 手写 FL | **不做**（须新合同 / NFR71） |
| `I2cMaster` / I2C 手写 FL | **不做**（须新合同 / NFR71） |
| `Axi4LiteSlave` / AXI 手写 FL | **不做**（须新合同 / NFR71） |

### 选定子集钉死（95.2 必须）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **协议 / IP** | `bitloom_prelude::ip::UartRx` | SPI / I2C / AXI handwritten |
| **FL + API** | `bitloom_sim::UartRxFunctional`（或等价名）+ 钉死刺激 + `IpDualModelMatrix::verify_uart_rx_handwritten`（或等价公开 API） | 把 FL 运行时塞进设计 crate |
| **验收谓词** | FL ≡ `Sim::settle`+`tick`（或文档等价）Pass；故意错模型 → Fail；ATDD；`docs/fr163-*` | GeneratedFunctional alone；docs-only |
| **crate 边界** | 设计 crate → **`bitloom-prelude`**；手写 FL / 矩阵在 **`bitloom-sim`** | 改 prelude 公开表面为「须依赖 bitloom-sim」 |
| **与 FR135 边界** | FR135 = `UartTx`；FR163 = **超 UartTx** 至少 `UartRx`；FR135 关闭证据不得改写为失败 | 把 FR135 alone 写成已含 UartRx |

### 目标产品形状（95.2）

- **入口：** `UartRx::elaborate()` + `verify_uart_rx_handwritten`（命名可微调，须在 docs 钉死）。
- **产物：** 手写 FL ≡ tick；`docs/fr163-*`；ATDD。
- **语义锚：** 可宣称「超出 UartTx 的协议手写 FL」须引 **FR163**，不得只引 FR135。

### 验收谓词 / 失败语义（95.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | `UartRx` 手写 FL ≡ tick Pass；文档对照 FR135；ATDD 可复现 |
| **负向 / 失败** | 故意错模型 → Fail / 可读；**不得** silent-Ok 宣称 FR163 |
| **禁止勾选** | 仅 FR135 `UartTx`；仅 FR126 Gpio；仅 GeneratedFunctional；仅 docs |

### `ip/` 布局与软序

- 触碰 `crates/bitloom-prelude/src/ip/`（UART 模块）与/或 `crates/bitloom-sim/src/ip_dual.rs`。
- **软序：** 与其它并行改 `ip/` 的 epic（若有）串行或分目录；本批仅 UART RX 加深，避免同 PR 改 SPI/I2C/AXI 手写。
- **Epic 78 / FR139** crate 边界仍有效（NFR68）；本 epic **不**重做跨 crate 拆分。

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **95.1** | 本 NFR14 |
| **95.2** | `UartRx` 手写 FL + ATDD；`docs/fr163-*` |
| **95.3** | README / `docs/ip/` / deferred 收口；勾选 Epic 95；写明未选 SPI/I2C/AXI 仍须新合同 |

### (a) 上游约束

- **Epic 87 已关闭；Epic 88–94 已关闭。**
- **FR135：** `UartTxFunctional` **仍有效**；alone ≠ FR163。
- **FR126 / FR103：** 仍有效；alone ≠ FR163。
- **NFR68：** 不得改写 FR94–162 / FR135「已关闭」。
- **NFR71：** 禁止静默扩大到 SPI / I2C / AXI 全套手写。
- **NFR72：** 未关 FR163 前不得宣称未列协议手写已交付；**不得以 FR135 alone 冒充 FR163**。
- **品牌：** **Bitloom**；设计 crate → **`bitloom-prelude`**（AD-6）。
- **软序：** 95.2 → 95.3；`ip/` 触碰面互斥见上。

### (b) 粗工期带

- **预计：** Epic 95 整体约 **0.75–2.5 人周**（95.1 ≤0.25；95.2 FL+ATDD 0.5–2；95.3 收口 0.25）。
- **置信度 / 假设：** 中（可复用 FR135 `UartTxFunctional` 体例；`UartRx` RTL 已存在）。假设不改 prelude 公共 elaboratable 表面契约。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **95.2–95.3** 标 `ready` 或开工实现。
- **不得以 FR135 `UartTx` alone 勾选 FR163。**
- **不得以 FR126 Gpio / FR103 SyncFifo / GeneratedFunctional alone 冒充 FR163。**
- **不得仅改文档关闭 FR163。**
- **不得静默扩大到 SPI / I2C / AXI 全套手写。**
- **不得把手写 FL 运行时依赖塞进设计 crate。**
- **不得改写 Phase 12–18 / FR135 / FR126 关闭证据为失败。**
- 不得把本记录冒充 **NFR14-crates**（FCFS）完成定义。
- 不得在 FR163 未关时宣称 NFR59「全清」。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 与 **NFR68 / NFR69 / NFR71 / NFR72** 共同责任人
- 备份 / 升级路径：改选 SPI/I2C/AXI 代替 `UartRx`，或扩大到多协议同批，须升级至产品 / Correct Course 批准人并修订本记录。

---

### Epic 95 关闭条件（Story 95.3 勾选）

- [ ] **FR163 钉死子集实现 + 验收** — Story 95.2
- [ ] **文档 / deferred / README / docs/ip 收口** — Story 95.3
- [ ] **NFR68/71/72：** 边界与诚实义务保持；未选 SPI/I2C/AXI 须新合同
- [ ] **品牌 / AD-6：** Bitloom；设计 crate → `bitloom-prelude`
- [ ] **其余 FR164–165：** 未关前不得宣称 NFR59 全清
