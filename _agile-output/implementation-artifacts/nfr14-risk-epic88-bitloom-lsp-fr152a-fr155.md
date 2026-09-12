# NFR14 风险记录 — Epic 88 从 crates.io 安装 bitloom-lsp（FR152(a) / FR155）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**；Phase 19 **NFR68–NFR72**；实现面 **FR155**；宣称须引对应 FR。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic87-phase19-nfr59-fr152a.md` / `nfr14-risk-epic85-cli-install-fr149-152.md`。  
> **前置：** Epic 87 / FR154 **closed**；Correct Course Phase 19 **approved**（Q2：FR152(a) live publish）；FR151 / CLI **1.0.0** 已关闭；`docs/fr152-*` 现仍为 (b) 直至本 epic。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 88 后续故事 **88.2–88.4** 标为 `ready`，亦不得开工实现。**不得**在未 live 成功时宣称 crates.io 已可装 lsp。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR155 / Epic 88；NFR14、NFR68、NFR69、NFR70、NFR72；对照 FR151 / FR152(b) / FR154 |
| 记录日期 | 2026-09-12 |
| 状态 | closed — Story 88.4 勾选完成；Epic 88 关闭；FR155 / FR152(a) 实现面可宣称；**FR157–165 / FR156** 仍属 Epic 89–98 |
| **选定** | 在保留 Phase 12–18 与 Epic 87 闸门的前提下，授权 `bitloom-lsp` FR152(a)：`publish=true` + version 依赖 + **live** `cargo publish` |

### Phase 18 / 87 关闭面 vs Epic 88 实现边界（NFR68 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **Phase 18 / FR151–153** | CLI 已上架；FR152 **(b)** 已关 | **仍有效；不得改写「已关闭」为失败** |
| **Epic 87 / FR154** | Phase 19 合同闸门 | **已关闭** |
| **Epic 88 / FR155** | FR152(a) lsp live crates.io | **本实现 epic** |
| **Epic 89–98** | NFR59 九条 + 宣称 | **本 epic 不做** |

**选定：完成 lsp 可发布化与 live 上架；不破坏 FR151；不扩大 FR142；不等于 LSP 功能加深。**

### FR155 范围（本 epic）

| 故事 | 交付 |
| --- | --- |
| **88.1** | 本 NFR14 |
| **88.2** | `publish=true` + 依赖 version 化；`docs/fr152-*` (b)→(a)；dry-run 绿 |
| **88.3** | **live** `cargo publish -p bitloom-lsp` |
| **88.4** | 文档/deferred 收口；勾选 Epic 88 |

### (a) 上游约束

- **Epic 87 已关闭：** FR154 Correct Course + PRD；README/deferred；脊柱/AGENTS Phase 19 指针。
- **目标 version：** 建议与工作区 **1.0.0** 对齐（Q5）；须在 88.2 NFR14/故事钉死最终号。
- **依赖：** `bitloom-builder` / `bitloom-hir`（及传递依赖）须全部为 **version/registry** 需求，方可 `cargo publish`；禁止留下挡 publish 的 path-only。
- **政策：** 重写 `docs/fr152-bitloom-lsp-publish-policy.md` 为 **(a)**；诚实注明 (b) 已升格；**(a) ≠ LSP 产品功能加深**。
- **FR151 边界：** 不得破坏已关闭的 `cargo install bitloom` / `cargo publish -p bitloom` 路径。
- **NFR68：** 不得改写 FR94–153「已关闭」。
- **NFR70：** crates.io live 发布须文档化凭证/顺序/失败回滚。
- **NFR72：** 未 live 成功不得宣称 crates.io 已可装 lsp。
- **AD-2：** 禁止 publish `rhdl` / `rhdl-bits`；对外名 **`bitloom-lsp`**。
- **品牌：** **Bitloom**；设计 crate 仍只依赖 **`bitloom-prelude`**（本 epic 不改设计依赖边界）。
- **软序：** 88.2 dry-run → 88.3 live → 88.4 收口。

### (b) 粗工期带

- **预计：** Epic 88 整体约 **0.75–2 人周**（88.1 ≤0.25；88.2 可发布化 0.25–0.75；88.3 live 0.25–0.5 + 凭证风险；88.4 收口 0.25）。
- **置信度 / 假设：** 中（crates.io index/凭证/依赖 version 图）。假设 `bitloom-hir`/`builder` 1.0.0 仍在 registry。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **88.2–88.4** 标 `ready` 或开工实现。
- **不得在未 live 成功时宣称 crates.io 已可装 `bitloom-lsp`。**
- **不得 publish `rhdl` / `rhdl-bits`。**
- **不得静默扩大 FR142。**
- **不得破坏已关闭 FR151 CLI 安装路径。**
- **不得把 LSP 功能加深冒充 FR155 完成面。**
- **不得改写 Phase 12–18 / FR152(b) 关闭证据为「失败」**（升格为 (a) 是新完成面，须诚实重写政策）。
- 不得把本记录冒充 **NFR14-crates**（FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 与 **NFR68 / NFR69 / NFR70 / NFR72** 共同责任人
- 备份 / 升级路径：撤回 FR152(a) / 凭证失败永久改 dry-run-only 须升级至产品 / Correct Course 批准人。

---

### Epic 88 关闭条件（Story 88.4 勾选）

- [x] **FR155 可发布化 + 政策 (a)** — Story 88.2
- [x] **live `cargo publish -p bitloom-lsp`** — Story 88.3（Published bitloom-lsp v1.0.0）
- [x] **文档 / deferred 收口** — Story 88.4
- [x] **NFR68/70/72：** 边界与诚实义务保持
- [x] **品牌 / AD-2：** Bitloom / `bitloom-lsp`；禁 `rhdl`/`rhdl-bits`
- [x] **Epic 89–98：** NFR59 / 宣称仍属后续 epic；本 epic 仅关 FR155
