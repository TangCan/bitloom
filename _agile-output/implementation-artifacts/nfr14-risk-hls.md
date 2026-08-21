# NFR14 风险记录 — HLS 产品路径（Epic 24 / FR35 + FR50）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（Phase 7 风险门禁）。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 24 后续故事 **24.2–24.4** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR35 + FR50 / Epic 24 |
| 记录日期 | 2026-08-21 |
| 状态 | accepted |

### (a) 上游约束

- **钉死单一后端：** **PandA Bambu `2024.10`**（AppImage：`https://release.bambuhls.eu/bambu-2024.10.AppImage`；源码标签 `2024.10`）。**不选** Vitis/XLS。升钉须改本记录 + 文档 + 烟测脚本中的版本常量，不得静默换后端。
- **许可：** Bambu / PandA 为 **GPLv3**；Bitloom 仅进程外调用，不把 Bambu 链进发布 crate；设计者本地安装责任在文档写明。
- **安装 / 覆盖：** 默认解析 `BITLOOM_BAMBU_PATH`（兼容 `RHDL_BAMBU_PATH`）或 `PATH` 上的 `bambu`；CI/烟测可缓存 AppImage 或使用文档化 stub 验证接线（真实合成质量以钉死 Bambu 为准）。
- **CI 可用性：** 完整 Bambu AppImage 偏重 → 允许 **optional** 真机 job；但必须有**非零**常驻覆盖（emit + 缺后端可读失败 + 夹具脚本）。optional job 失败不得 `continue-on-error` / ignore。

### (b) 粗工期带

- **预计：** Epic 24 整体约 **1–2 人周**（24.1 风险记录 ≤0.25 人周；24.2 产品路径加固 0.5–1 人周；24.3 CI/烟测 0.25–0.5 人周；24.4 文档收口 ≤0.25 人周）。
- **置信度 / 假设：** 中高；假设沿用 Story 9.2 的 emit-C + 外挂 CLI 形状，不引入树内调度；真 Bambu 回归依赖本机/可选 runner，不阻塞主 `just test` 墙钟。

### (c) 禁止的静默降级清单

- 不得以「未启用则永久 unsupported」交差，而不改 PRD / AD-25。
- 不得引入树内自研 scheduler / allocation（AD-25）。
- 不得把「可选实验钩子」冒充 FR35/FR50 产品完成，却无默认文档路径与 CI/烟测覆盖。
- 不得在后端缺失时 silent 成功（必须失败可读）。
- 不得静默从钉死 Bambu 换成 Vitis（或反之）而不改本记录与文档。
- 不得在无本记录（或缺 a–d）时将 **24.2–24.4** 标 `ready` 或开工实现。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）
- 备份 / 升级路径：架构争议升级至脊柱 AD-25 维护者；Bambu 升钉跟本记录 + 文档同步。

### 所选后端与版本策略

| 项 | 决策 |
| --- | --- |
| 后端 | **Bambu**（PandA） |
| 钉死版本 | **2024.10** |
| 获取 | 官方 AppImage / 源码构建；路径经 `BITLOOM_BAMBU_PATH` |
| 升钉策略 | 改 NFR14 记录 + `HLS_BACKEND_VERSION` 常量 + `docs/fr35-hls.md` + 烟测脚本；跑通夹具后再合入 |

### 并行 / 维护叠加（Chipyard 式）

- 与 Epic 20–23 并行时：HLS 不共享 Chisel/firtool 钉死栈，但文档与 README「支持功能」表会同时膨胀；避免各 epic 各自发明「永久 unsupported」话术。
- Bambu GPLv3 与 Bitloom MIT/Apache 边界：仅 CLI 外挂，禁止把 Bambu 源码 vendoring 进发布 crate。
- CI 墙钟：真 Bambu job 与主 test job 分离，防止拖垮 MSRV 矩阵。

### 引用

- AD-28 — Phase 7 风险门禁（NFR14）
- AD-25 — HLS 仅外挂（产品路径修订）
- PRD NFR14 / FR35 / FR50（`prd-rhdl-2026-08-19/prd.md`）
- 历史别名消歧：**NFR14-crates**（crates.io FCFS）≠ 本门禁 **NFR14**；历史 Epic 13 **FR50**（host shim registry）≠ 本 **FR50**（HLS 产品路径）

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 24 故事 24.2–24.4 标 `ready`。**
