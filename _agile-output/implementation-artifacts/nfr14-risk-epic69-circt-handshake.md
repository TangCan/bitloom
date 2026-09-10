# NFR14 风险记录 — Epic 69 CIRCT Handshake / 多时钟弹性缓冲（FR129）

> **前置：** Epic 64 closed；FR121 ready/valid closed；须评估/修订 AD-25。  
> **门禁：** 无本记录 ⇒ 不得标 69.2–69.3 ready。

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR129 / Epic 69；NFR14 / NFR52 / NFR54 / NFR55 |
| 记录日期 | 2026-09-10 |
| 状态 | **closed** — Story **69.3** |

### (a) 上游约束

- FR121 Handshake ready/valid **仍有效**（NFR52）；alone ≠ FR129。
- FR95/96 / FR110 关闭仍有效。
- **须修订 AD-25** 以允许 CIRCT Handshake 方言子集 + 多时钟弹性缓冲产品路径（NFR54）。
- 品牌 Bitloom；设计依赖不变。

### (b) 粗工期带

- 1–3 人周。

### (c) 禁止的静默降级清单

- 不得缺本记录标 69.2–69.3 ready。
- **不得仅 FR95/96。**
- **不得仅 FR110。**
- **不得仅 FR121。**
- **不得 docs-only。**
- **不得未评估/未修订 AD-25 即宣称全家桶。**
- 不得冒充 NFR14-crates。

### (d) 负责人

- Richard — NFR14 / NFR54（AD-25）/ NFR52 / NFR55。消歧：NFR14-crates ≠ 本门禁。

### 验收面（C1–C4）

| ID | 钉死 |
| --- | --- |
| C1 | CIRCT Handshake 方言 IR 子集标记：`handshake.func` / `handshake.buffer` + `"fr129": true` |
| C2 | 多时钟弹性缓冲：`clock_domains ≥ 2` + `elastic_buffers ≥ 1` |
| C3 | 产品 API：`schedule_circt_handshake` / `meets_fr129_circt_handshake`；CLI `--circt-handshake` |
| C4 | ATDD：`cargo test -p bitloom --test fr129_circt_handshake`；AD-25 修订戳 |

### AD-25 决策

**须修订** — 允许 FR129 CIRCT Handshake 方言子集 + 多时钟弹性缓冲为合法产品完成面；FR121 ready/valid 默认路径保留可回归。

### 明确非目标（NFR55）

完整 CIRCT/MLIR lower 全家桶；全 allocation/binding 优化套件；任意时钟拓扑自动综合。

### Epic 69 关闭条件（69.3）

- [x] **69.2 / FR129**
- [x] **文档 / deferred / AD-25**
- [x] **禁止事项未触发**
- [x] **品牌 / 依赖**
- [x] **FR121 关闭仍有效**
