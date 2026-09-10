# NFR14 风险记录 — Epic 49 SystemC TLM-2.0 AT 产品支（FR107）

> **权威：** PRD NFR14；AD-28；Phase 13 **NFR44–NFR47**；交付 **FR107**。  
> **前置：** Epic 48 **closed**（FR106）；Epic 46 **closed**（FR101 LT-only MVP）；ARCHITECTURE-SPINE Deferred 已指向 FR107→AD-5（Story 48.4）。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **49.2–49.3** 标 `ready`。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR107 / Epic 49；NFR14、NFR44、NFR45、NFR46、NFR47；对照 FR101 / FR47 / AD-5 |
| 记录日期 | 2026-09-10 |
| 状态 | **closed** — Story 49.3（FR107 / Epic 49 收口；AT documented subset；LT 回归保留） |

### (a) 上游约束

- **Epic 48 / FR106 已关闭：** Phase 13 加深合同已开闸；本 epic 实现 **FR107**（AT / `nb_transport`），超出 FR101 LT-only。
- **FR101 / Epic 46（已关闭 · 隔离）：** LT-only MVP（`b_transport` / LT 风格）**仍有效**（NFR44）；**不得**把 LT-only 冒充 AT 完成。
- **AD-5 / NFR46：** 实现须引用现行 AD-5，并在本 epic 进一步修订以允许 AT / `nb_transport_fw`/`bw`（或文档等价子集）为产品路径。
- **FR47：** host Rust FL **≠** SystemC TLM AT。
- **设计依赖：** 设计 crate 只依赖 `bitloom-prelude`；SystemC 依赖不得泄漏进设计 crate。
- **品牌：** Bitloom / `bitloom-*`。

### (b) 粗工期带

- **预计：** Epic 49 整体约 **3–10 人周**（49.1 ≤0.25；49.2 AT 路径 2.5–8；49.3 收口 0.5–1.5）。置信度：**低–中**。
- **假设：** 保留 LT 回归；不要求全 timing/quantum 全家桶（未列入须新合同 / NFR47）。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **49.2–49.3** 标 `ready`。
- **不得仅以文档口号关闭 FR107。**
- **不得把 LT-only / host Rust FL / FR47 标成 AT。**
- 不得改写 FR101「已关闭」为失败（NFR44）。
- 不得静默扩大超出下方 AT 交付物钉死子集（NFR47）。
- 不得冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR46**（AD-5 修订引用）共同责任人；NFR44/45/47 共同注意人。

---

### AT 交付物（本记录钉死 · 供 49.2 / FR107）

| # | 交付元 | 钉死值（MVP 合同） |
| --- | --- | --- |
| **A1** | AT-style 接口 | 至少 `nb_transport_fw` **或** `nb_transport_bw`（或文档等价子集）可检查 |
| **A2** | 与 LT 关系 | 保留 FR101 LT 路径回归；文档写明 AT 与 LT 并存/选择 |
| **A3** | 生成或一等集成 | CLI/`cargo bitloom` 或文档产品入口接到 AT 工件 |
| **A4** | 夹具 + 依赖 | ≥1 可构建/可运行烟测；SystemC/TLM 版本钉死；缺依赖失败可读 |

### Epic 49 关闭条件（Story 49.3 勾选）

- [x] **49.2 / FR107：** AT 产品路径可检查
- [x] **文档 / deferred：** 边界诚实；LT 回归保留
- [x] **禁止事项未触发**
- [x] **品牌 / 依赖：** Bitloom；prelude 边界

---

## 门禁一句话

**缺 NFR14（或缺 a–d）⇒ 不得将 49.2–49.3 标 `ready`。**  
**不得以 FR101 LT-only / Rust FL 冒充 FR107 AT。**
