# NFR14 风险记录 — Epic 37 互操作硬化与 HLS 外挂诚实（FR88 / NFR12 / NFR39）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；**AD-9** / **NFR12**（firtool↔Chisel 钉死）；**AD-25**（HLS 仅外挂）；**AD-27**（可编译机械 Chisel）；Phase 11 **NFR39**（禁止静默扩大子集）。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic36-contract-green.md`；`nfr14-risk-epic33-chisel-mem.md`。  
> **前置：** Epic 36 合同绿文本已合入（软依赖）；Correct Course `sprint-change-proposal-2026-09-09.md` approved。  
> **deferred：** `deferred-work.md` — CI 默认 Bambu stub；Epic 37 / FR88 **本阶段选 Path B**（关闭「可选夜间」；真机仍显式入口）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 37 后续故事 **37.2–37.3** 标为 `ready`，亦不得开工实现。
>
> **历史门禁（「禁树内」）：** 下文 AD-25「HLS 仅外挂 / 不得引入树内调度」为 **Epic 37 当时** 合同。**已被 AD-25 Path B 修订推翻**；现行树内 FR95 见 **Epic 41** / [`docs/fr35-hls.md`](../../docs/fr35-hls.md) / [`nfr14-risk-epic41-in-tree-hls.md`](nfr14-risk-epic41-in-tree-hls.md)。保留原文作历史证据，**不得**再当作现行产品禁令。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR88 / Epic 37；NFR14、NFR12、NFR39 |
| 记录日期 | 2026-09-09 |
| 状态 | closed — Story 37.3 Path B；Epic 37 关闭条件已勾选 |

### (a) 上游约束

- **FR88：** 文档化 CIRCT/`firtool` 钉死运维（继承 AD-9 / NFR12）；公开声明机械 Chisel emit = **可编译 ≠ idiomatic**；可选 CI/夜间真机 Bambu（失败不 ignore）或显式保持 stub 默认并写明。
- **NFR12 / AD-9 钉死对（当前）：** Chisel **7.14.0** ↔ firtool **1.155.0**。升钉须等上游正式配对并更新 ARCHITECTURE-SPINE Stack / AD-9 / addendum 后再改；覆盖入口文档化（如 `RHDL_FIRTOOL_PATH` 或等价），禁止默认裸 `PATH` firtool 冒充钉死。
- **漂移风险：** 上游已有更新 firtool 标签时，若未改合同就「顺手升版」交差，会破坏 NFR12 配对、Chisel 夹具与缓存哈希；Epic 37 须把运维清单与诚实声明钉在文档，而不是静默 bump。
- **AD-27 / FR28：** `emit_chisel` / 往返验收 = **可编译 + 端口/层次谓词**；机械风格可接受。**误读风险：** 对外或文档把机械生成 Scala 写成「可维护 / idiomatic 手写风格」——与 FR93（FIRRTL→idiomatic Scala 永久非目标）及 NFR39 冲突。
- **AD-25 / FR86：** HLS 仅外挂（Bambu 或等价）；树内/自研调度仍为非目标。`deferred-work.md`：CI 默认 `bambu-ci-stub.sh` 验证接线与非零覆盖，**非**真实 HLS 调度质量；Story **37.3 选 Path B**（显式保持 stub 默认；不落地夜间真机 job；真机 = `BITLOOM_HLS_USE_REAL=1`）。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。
- **NFR39：** 禁止静默扩大子集或用话术把 stub/机械路径冒充更深完成度。

### (b) 粗工期带

- **预计：** Epic 37 整体约 **0.5–1.5 人周**（37.1 本风险记录 ≤0.25 人周；37.2 firtool/Chisel 运维清单 + 机械≠idiomatic 公开声明 0.25–0.75 人周；37.3 夜间 Bambu **或** 显式 stub 选型 + deferred 收口 0.25–0.75 人周）。
- **置信度 / 假设：** 中高；37.2 以文档合同为主（不强制改 emit）。若选 37.3 路径 A（真机夜间 job + AppImage 缓存），墙钟与 CI 配额上修；路径 B 工期更短。Epic 38–39 工期**不**计入本记录。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **37.2–37.3** 标 `ready` 或开工实现。
- **不得私自升 firtool（或 Chisel）钉死对交差**，而不更新 AD-9 / NFR12 / 脊柱 Stack / 文档运维清单。
- **不得把 stub CI 写成「HLS 质量已验」** 或等价完成话术（stub ≠ 真机调度质量）。
- **不得用 `continue-on-error`（或 ignore 失败）掩盖真机 Bambu / 夜间 job 失败**，却勾选 FR88 HLS 诚实条。
- 不得把机械可编译 Chisel 表述为 idiomatic / 可维护手写风格，或悄悄把 FR93 idiomatic 非目标标成 done。
- 不得引入树内 HLS 调度器（继承 FR86 / AD-25 / FR93）。
- 不得在未改 PRD / 本记录的前提下把 FR88 静默砍掉或改回「PATH firtool + stub 即全绿」。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁、**NFR12** firtool/Chisel 钉死诚实度、**NFR39** 子集边界共同责任人
- 备份 / 升级路径：钉死对升版争议升级至架构（AD-9）；HLS A/B 选型或 stub 话术争议升级至产品 / PM；永久非目标（idiomatic / 树内 HLS）增删须 **新 PRD**。

---

### 风险主题对照（本 epic）

| 主题 | 风险 | 缓解（本 epic 故事） |
| --- | --- | --- |
| firtool/Chisel 钉死对漂移 | 私自升版破坏 NFR12 配对与缓存 | 37.2 文档钉死版本对 + 覆盖入口；禁私自升 |
| 机械 Chisel 误读为 idiomatic | 对外夸大 FR28/FR46 完成度 | 37.2 公开「可编译 ≠ idiomatic」；抽检误导表述（NFR39） |
| 夜间真机 vs stub 默认 | stub CI 冒充 HLS 质量；真机失败被 ignore | 37.3 选 A（真机且失败不 ignore）或 B（显式 stub）；禁 `continue-on-error` |

### Epic 37 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **37.1** | 本 NFR14 风险记录 + ATDD | **本故事** |
| **37.2** | firtool/Chisel 钉死运维 + 机械 Chisel 诚实声明（FR88） | Gate：须本记录后才可 ready |
| **37.3** | 可选夜间真机 Bambu **或** 显式保持 stub（FR88） | Gate：须本记录后才可 ready；关闭时勾选下节 |

### 并行 / 维护叠加（Chipyard 式）

- 与 Epic 38–39 并行前：建议先合入 37.2 文档诚实声明（软依赖）；不得在缺 FR88 声明时对外宣称「idiomatic Chisel」或「HLS 已验」。
- 文档面叠加：`docs/fr28-chisel-compilable.md`（或等价）、`docs/fr35-hls.md`、README、`deferred-work.md` 须同一叙事；禁止「代码 done、文档仍 PATH/stub 假绿」。
- 不扩大 VIP IP / 树内 HLS / idiomatic Scala；触及处遵守 FR93 与各 epic 自有 NFR14。

### 引用

- AD-28 — 风险门禁（NFR14）
- AD-9 / NFR12 — firtool↔Chisel 钉死对
- AD-25 / FR86 — HLS 外挂；禁树内调度
- AD-27 / FR28 — 可编译机械 Chisel
- PRD NFR14 / FR88；NFR39（禁止静默扩大子集）
- deferred-work：CI 默认 Bambu stub；可选夜间真机
- 体例：`nfr14-risk-epic36-contract-green.md`
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 37 关闭条件（Story 37.3 勾选）

- [x] **FR88（firtool/Chisel）：** 用户/维护者文档写明钉死版本对与缓存/覆盖入口；公开「可编译 ≠ idiomatic」— Story 37.2
- [x] **FR88（HLS）：** 落地 A（夜间真机 Bambu，失败不 ignore）**或** B（显式保持 stub 默认 + deferred 收口）— Story 37.3 **选 Path B**
- [x] **NFR12 / NFR39：** 无私自升钉死对；无机械冒充 idiomatic；无 stub/continue-on-error 假绿
- [x] **禁止事项未触发：** 无私自升 firtool 交差；无 stub CI 写成 HLS 质量已验；无 `continue-on-error` 掩盖真机失败；无提前标 37.2–37.3 ready（对本记录而言）
- [x] **品牌：** 仍为 Bitloom / `bitloom-*`

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 37 故事 37.2–37.3 标 `ready`。**  
**Epic 37 关闭条件（上节）已由 Story 37.3（Path B）勾选。**
