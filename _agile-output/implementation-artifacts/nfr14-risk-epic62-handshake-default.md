# NFR14 风险记录 — Epic 62 Handshake 默认可综合（FR121）

> **权威：** PRD NFR14；AD-28；Phase 14 **NFR48 / NFR49 / NFR50 / NFR51**；交付 **FR121**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic52-hls-commercial-depth.md`；`nfr14-risk-epic41-in-tree-hls.md`；`nfr14-risk-epic57-phase14-nfr47-deferred-deepen.md`。  
> **前置：** Epic 57 **closed**（FR116）；Epic 41 **closed**（FR95/FR96）；Epic 52 **closed**（FR110 Q1+Q2）；现行 **AD-25** 仍禁止 Handshake 为默认可综合语义。  
> **门禁：** 无本有效记录（或缺字段 a–d / 下方 H1–H4 清单）⇒ **不得**将 **62.2–62.3** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR121 / Epic 62；NFR14、NFR48、NFR49、NFR50、NFR51；对照 FR95/FR96、FR110、AD-25、AD-18 |
| 记录日期 | 2026-09-10 |
| 状态 | **closed** — Story 62.3 / Epic 62（2026-09-10） |

### (a) 上游约束

- **Epic 57 / FR116 已关闭：** Phase 14 加深合同已开闸；本 epic 将 NFR47「Handshake 默认可综合」升格为 **FR121** 必选。
- **FR95 / FR96 / Epic 41（已关闭 · 隔离）：** 树内 loop-unroll MVP + 闭包 dissolve + `in-tree-mvp` RTL stub **仍有效**（NFR48）；**不得**改写为失败，亦不得以 MVP / stub / 仅展开冒充 Handshake 默认语义。
- **FR110 / Epic 52（已关闭 · 隔离）：** Q1+Q2 pipeline/`ii` 商业深度 **仍有效**；**不得**以 FR110 alone 冒充 FR121。
- **AD-25（现行 → 须修订）：** 现行 Prevents/Rule **禁止** Handshake/动态数据流作为默认可综合 RTL 语义。**FR121 关闭前 Story 62.2 必须修订 ARCHITECTURE-SPINE AD-25**，允许 Handshake/动态 DF 为文档化默认可综合语义（NFR50）；未修订不得宣称 FR121。
- **AD-18：** 捕获闭包仍禁入 `tick`；elaborate-time 非捕获 `Fn` 须冻前溶解（见下 H2）。
- **设计依赖：** 设计 crate 只依赖 `bitloom-prelude`；HLS/Handshake 调度在工具链 crate（`bitloom`）。
- **品牌：** Bitloom / `bitloom-*`。

### (b) 粗工期带

- **预计：** Epic 62 整体约 **1–4 人周**（62.1 ≤0.25；62.2 AD 修订 + 产品路径 0.75–3；62.3 收口 0.25–0.75）。置信度：**中**。
- **假设：** 不交付完整 allocation/binding/全优化套件（NFR51）；FR95/96/FR110 回归不破。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **62.2–62.3** 标 `ready`。
- **不得仅以 loop-unroll / `in-tree-mvp` stub / FR95 MVP alone 关闭 FR121。**
- **不得仅以 FR110 Q1+Q2 / pipeline+ii alone 关闭 FR121。**
- **不得 docs-only / 仅改文档关闭。**
- **不得未修订 AD-25 即宣称 Handshake/动态 DF 为默认可综合语义。**
- 不得改写 FR95/96 / FR110「已关闭」为失败（NFR48）。
- 不得静默扩大超出下方 H1–H4 清单（NFR51）。
- 不得冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR50** / **NFR51** 共同责任人；NFR48/NFR49 共同注意人。
- 备份 / 升级路径：缩回「仅静态 schedule」口径或扩大至完整 CIRCT Handshake 方言全家桶须升级至产品 / Correct Course 批准人。

---

### Handshake 默认可综合清单（供 62.2 / FR121 · H1–H4）

| # | 钉死项 | 合同内容 | 62.2 验收证据 |
| --- | --- | --- | --- |
| **H1 · 默认语义范围** | Handshake（或等价动态数据流）作为**文档化默认**可综合 HLS/RTL 语义 | 数据通路以 **ready/valid（或文档等价 token 握手）** 为默认可综合通道合同；静态 loop-unroll / FR110 pipeline **仍可用**，但**不再**是宣称「默认可综合 DF」的唯一合法语义。文档/CLI/库须标明 Handshake 为 FR121 默认开关（`--handshake` 或等价；文档写清默认行为） | 文档 + schedule IR 含 `handshake`/`fr121`/`semantics=handshake-dynamic-df`（或等价）；ATDD 断言默认路径 |
| **H2 · 与 dissolve / AD-18 关系** | 冻前溶解义务不变 | Handshake 路径上的数据流闭包仍须 **elaborate-time 非捕获 dissolve**（FR96 族或等价入口）后再进入 Handshake schedule；**捕获 / 有状态闭包** → 调度前可读失败；溶解后 **不得**以 Rust `Fn` 对象进入 `tick`（AD-18） | 正向：dissolve → Handshake schedule；负向：capturing → Err 且消息含 AD-18/captur |
| **H3 · 可综合发射 / 验收谓词** | 可检查产物 | （1）schedule IR 声明 Handshake/动态 DF（`fr121: true` 且 `handshake: true` 或文档钉死等价字段）；（2）RTL/发射面暴露 **ready/valid**（或文档等价）端口对，诚实标注 Handshake 路径（≠ 仅 `in-tree-mvp` 无握手端口）；（3）公开品牌 Bitloom | `cargo test -p bitloom --test fr121_handshake_default`（或等价）纳入 `just test` |
| **H4 · 失败语义** | 可读失败，不得 silent 成功 | 未满足 H1–H3 时不得打印/标记 `fr121=true`；捕获闭包失败可读；非法通道配置失败可读；**缺 AD-25 修订戳时文档/ATDD 不得宣称 FR121 closed** | 负向 ATDD；AD-25 Revised 戳含 FR121 |

**与既有路径边界：**

| 路径 | 角色 | 可否单独关闭 FR121 |
| --- | --- | --- |
| FR95 loop-unroll + `in-tree-mvp` stub | Phase 12 MVP | **否** |
| FR96 dissolve → FR95 | Phase 12 MVP | **否** |
| FR110 Q1+Q2 pipeline/`ii` | Phase 13 商业深度 | **否** |
| 外挂 Bambu stub alone | FR35 诚实路径 | **否** |
| 本记录 H1–H4 + **修订 AD-25** + ATDD | FR121 | **是**（须 62.2→62.3） |

### 明确非目标 / deferred（NFR51 · 诚实披露）

下列 **未**列入本 FR 关闭清单；**不得 silent 宣称已交付：**

- 完整 CIRCT Handshake 方言 / 全 MLIR 降低全家桶
- 完整 allocation/binding/商业 HLS 编译器优化套件
- 多时钟域 Handshake 网络 / 弹性缓冲全家桶自动插入
- 将 Handshake 强制替换为唯一合法路径（静态 FR95/FR110 须保留可回归）

### Epic 62 关闭条件（Story 62.3 勾选）

- [x] **62.2 / FR121：** H1–H4 + 修订 AD-25 + ATDD；FR95/96/FR110 回归不破
- [x] **文档 / deferred / HLS 文档 / AD-25 修订戳**
- [x] **禁止事项未触发**
- [x] **品牌 / 依赖：** Bitloom；prelude 边界
- [x] **FR95/96 / FR110 关闭仍有效**（NFR48）

---

## 门禁一句话

**缺 NFR14（或缺 a–d / H1–H4）⇒ 不得将 62.2–62.3 标 `ready`。**  
**不得以 loop-unroll / in-tree-mvp stub / FR110 alone / docs-only / 未修订 AD-25 冒充 Handshake 默认可综合。**  
**未列入协议保持 deferred（NFR51）。**  
**Epic 62 / FR121 已关闭（Story 62.3）：** Handshake 默认可综合（ready/valid；AD-25 修订）；FR95/96 / FR110 关闭仍有效；全优化 / CIRCT Handshake 方言全家桶仍 deferred。
