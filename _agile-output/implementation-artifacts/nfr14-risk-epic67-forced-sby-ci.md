# NFR14 风险记录 — Epic 67 默认 CI 强制真 sby（FR127）

> **前置：** Epic 64 closed；FR119 本机路径 closed。  
> **门禁：** 无本记录 ⇒ 不得标 67.2–67.3 ready。

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR127 / Epic 67；NFR54 CI 运维 |
| 记录日期 | 2026-09-10 |
| 状态 | **closed** — Story **67.3** |

### (a) 上游约束

- FR119 `just formal-sby-check` **仍有效**（本机可选）；alone ≠ FR127。
- 设计依赖不变；CI 安装 sby+yosys+z3（见 `scripts/ci-install-sby.sh`）。
- 品牌 Bitloom。

### (b) 粗工期带

- 0.5–2 人周。

### (c) 禁止的静默降级清单

- 不得缺本记录标 67.2–67.3 ready。
- **不得仅文档关闭。**
- **不得 silent skip 当 required（禁止 continue-on-error）。**
- **不得改写 FR119 关闭为失败（NFR52）。**
- 不得冒充 NFR14-crates。
- 公开品牌 **Bitloom** / `bitloom-*`；设计 crate 只依赖 `bitloom-prelude`。

### (d) 负责人

- Richard — NFR14 / NFR54 CI 运维 / NFR52 / NFR55。
- 消歧：**NFR14-crates**（crates.io FCFS）≠ 本门禁 NFR14。

### CI 形状（S1–S4）

| ID | 钉死 |
| --- | --- |
| S1 | `.github/workflows/ci.yml` job `formal-sby`（required；无 continue-on-error） |
| S2 | 安装：`bash scripts/ci-install-sby.sh`（yosys+z3+sby）；timeout ≤20m |
| S3 | 运行：`just formal-sby-check`；缺工具非零可读 |
| S4 | 边界：FR119 本机路径仍可用；≠ 仅本机可选 |

### Epic 67 关闭条件（67.3）

- [x] **67.2 / FR127**
- [x] **文档 / deferred / CI README**
- [x] **禁止事项未触发**
- [x] **品牌 / 依赖**
- [x] **FR119 关闭仍有效**
