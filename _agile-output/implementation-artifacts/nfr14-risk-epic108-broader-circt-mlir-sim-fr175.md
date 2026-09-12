# NFR14 风险记录 — Epic 108 更广 CIRCT/MLIR/sim（FR175）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28** / **AD-9**；Phase 21 **NFR78–NFR82**；实现面 **FR175**。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic102-circt-mlir-firtool-fr169.md`。  
> **前置：** Epic 105 / FR172 **closed**；Epic 106 / FR173 **closed**（AD-9 **firtool-1.158.0**）；Epic 102 / **FR169** multi-lower **closed**；Epic 96 / **FR164**、Epic 76 / **FR137**、Epic 69 / **FR129** **closed**。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **108.2–108.3** 标 `ready`。**不得**以 FR169 alone / FR164 alone / FR137 alone / FR129 alone 勾选本 FR。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR175 / Epic 108；NFR14、NFR78、NFR79、NFR80、NFR81；对照 FR169 / FR164 / FR137 / FR129 / AD-9 |
| 记录日期 | 2026-09-12 |
| 状态 | open — Story 108.1；Epic 108 进行中；108.2–108.3 待本记录后 ready |
| **选定** | 在保留 FR169(A) `--ir-fir`+`--ir-hw`+Verilog 关闭面的前提下，授权 **更广 multi-lower**：增加 **`--ir-sv`（SV dialect）+ `--ir-verilog`（Verilog-lowering IR）** 产品门禁 @ AD-9 **firtool-1.158.0**；可选与 sim 证据同夹具串联；禁止仅再跑 FR169/FR164 冒充加深 |

### Phase 12–20 关闭面 vs Epic 108（NFR78）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR129 / Epic 69** | Handshake | **仍有效**；alone ≠ FR175 |
| **FR137 / Epic 76** | 编译门禁 | **仍有效**；alone ≠ FR175 |
| **FR164 / Epic 96** | 仿真门禁 | **仍有效**；alone ≠ FR175 |
| **FR169 / Epic 102** | `--ir-fir` + `--ir-hw` + Verilog | **仍有效**；**alone ≠ FR175** |
| **FR173 / Epic 106** | 配对升钉 1.158.0 | **仍有效**（本 epic 使用现行 AD-9 pin） |
| **Epic 108 / FR175** | 更广 CIRCT/MLIR lower | **本实现 epic** |

### 本批钉死（108.2 必须）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **加深子集** | 外部 firtool @ **1.158.0**：在 FR169 三档之外增加 **`--ir-sv`** 与 **`--ir-verilog`** 发射，产物可检（含 `sv.` 与/或 post-verilog IR 证据）+ Verilog；`just` / CI / ATDD | 仅重跑 FR169 alloc / FR164 sim |
| **工具 / 失败** | AD-9 ensure / `RHDL_FIRTOOL_PATH`；版本不符 / `BITLOOM_CIRCT_SV_FORCE_MISSING`（或文档等价）→ **非零可读** | PATH 随机；`continue-on-error`；silent-Ok |
| **软依赖** | 依赖 Epic 106 现行 pin（已满足）；触及 AD 时遵守 NFR80（本批默认不改 AD-9 版本，仅可补指针） | 把 FR174 unpaired 1.156.0 冒充本门禁 |
| **与 FR169 边界** | FR169 = fir+hw+v；FR175 = **+ sv + ir-verilog** 加深 | FR169 alone 勾选 |

### 验收谓词 / 失败语义（108.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | `docs/fr175-*`；`just circt-external-sv-check`（名可调）；CI required；ATDD 绿 |
| **负向** | 缺工具 / 错版本 / force-missing → 可读非零 |
| **禁止勾选** | FR169 alone；FR164 alone；FR137 alone；FR129 alone；仅 docs |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **108.1** | 本 NFR14 |
| **108.2** | SV/ir-verilog 加深实现 + ATDD/CI；`docs/fr175-*` |
| **108.3** | README/deferred 收口；勾选 Epic 108；未写入子集仍 **NFR81** |

### (a) 上游约束

- **Epic 105 / 106 已关闭。**
- **FR169 / FR164 / FR137 / FR129：** **仍有效**；alone ≠ FR175。
- **NFR78 / NFR81：** 不得改写已关闭；不得静默超子集（例如 Handshake 全方言全家桶）。
- **CIRCT 运行时不得进入设计 crate（AD-6）。**
- **品牌：** Bitloom。

### (b) 粗工期带

- **预计：** Epic 108 **Medium**；本 NFR14 ≤0.25 人周；108.2 约 0.5–1.5 人周。
- **置信度：** 中–高（firtool 已支持 `--ir-sv` / `--ir-verilog`）。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **108.2–108.3** 标 ready。
- **不得**以 FR169 / FR164 / FR137 / FR129 alone 勾选 FR175。
- **不得** PATH 随机 firtool / unpaired HEAD 冒充本加深。
- **不得**静默扩大 FR142；**不得**把 `git push` 当 FR。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR78** / **NFR81** / **AD-9**
- 升级：缩回「仅再跑 FR169」须 Correct Course / 产品批准。

### 引用

- AD-28；AD-9；FR175；对照 FR169 / FR164 / FR137 / FR129；Phase 21 Correct Course approved 2026-09-12
- **NFR14-crates** ≠ 本门禁

---

### Epic 108 关闭条件（Story 108.3 勾选）

- [ ] **FR175 钉死子集实现 + 验收** — Story 108.2
- [ ] **文档 / deferred / README / CI 收口** — Story 108.3
- [ ] **NFR78/81：** 边界与诚实义务保持
- [ ] **品牌 / AD-6：** Bitloom；CIRCT 运行时不得进入设计 crate
- [ ] **其余 FR176–177：** 未关前不得宣称 Phase 21 全清
