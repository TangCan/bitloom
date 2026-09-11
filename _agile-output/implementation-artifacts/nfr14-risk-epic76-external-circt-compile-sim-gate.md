# NFR14 风险记录 — Epic 76 完整外部 CIRCT 编译 / 仿真门禁（FR137）

> **权威：** PRD NFR14；AD-28；Phase 16 **NFR56 / NFR57 / NFR58 / NFR59**；交付 **FR137**。  
> **前置：** Epic 72 **closed**（FR133）；FR129 C1–C4 **closed**（Epic 69）；FR121 / FR110 / FR95/96 **closed**。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **76.2–76.3** 标 `ready`。  
> **隔离：** ≠ FR95/FR96 alone；≠ FR110 alone；≠ FR121 ready/valid alone；≠ FR129 C1–C4 alone；≠ docs-only；≠ continue-on-error 静默跳过。  
> **NFR58：** 外部 CIRCT / 工具链运维合同须在 76.2 同步脊柱/文档/CI 前可检查。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR137 / Epic 76；对照 FR129 / FR121 / FR110 / FR95 / FR96 |
| 记录日期 | 2026-09-11 |
| 状态 | **76.2 implemented**（编译门禁 MVP + CI/`just`；仿真未选）— Epic 关闭勾选 → Story **76.3** |
| **选定加深子集** | **外部 CIRCT 编译与/或仿真门禁（E1–E4）** |

### (a) 上游约束

- Epic 72 / FR133 已关闭；Phase 12–15 关闭仍有效（**NFR56**）；不得改写 FR94–132「已关闭」为失败。
- FR129 C1–C4（CIRCT Handshake 方言 IR 子集标记 + 多时钟弹性缓冲 + 产品 API + ATDD；AD-25 已修订）**仍有效**；alone ≠ FR137。
- FR121 Handshake ready/valid / FR110 商业深度 / FR95 / FR96 树内 HLS MVP **仍有效**；alone ≠ FR137。
- **AD-9 / Stack：** 外部 CIRCT 调用须钉死 **firtool-1.155.0**（Chisel **7.14.0** 配对；发行渠道 = CIRCT/firtool **tag `firtool-1.155.0`** 资产 `firrtl-bin-linux-x64.tar.gz` + `.sha256`；**非** CIRCT HEAD；**非** PATH 随机 firtool）。升钉须等 Chisel 正式配对并修订 AD-9 / Stack（NFR12）。
- 覆盖路径：CLI 托管缓存（AD-9）与/或 `RHDL_FIRTOOL_PATH` / 文档钉死目录含 `firtool` 二进制；**禁止**默认信任裸 `PATH`。
- 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；外部 CIRCT 调用属 CLI/CI/工具链层，不得把运行时 CIRCT 依赖塞进设计 crate。
- 品牌 Bitloom / `bitloom-*`；禁止发布 `rhdl` / `rhdl-bits`。
- **NFR58：** 触及外部 CIRCT 运维时，76.2 须先/同步修订脊柱指针、运维文档与 CI 合同后再标实现完成。

### (b) 粗工期带

- Epic 76：约 **1–3 人周**（76.1 ≤0.25；76.2 0.75–2.5；76.3 ≤0.5）。置信度：中。
- 假设：76.2 交付下表 **E1–E4**；未列入更广 CIRCT/MLIR lower / 全 allocation 套件仍 **NFR59**。

### (c) 禁止的静默降级清单

- 不得缺本记录将 **76.2–76.3** 标 `ready`。
- **不得仅 FR95 / FR96 关闭 FR137。**
- **不得仅 FR110 关闭 FR137。**
- **不得仅 FR121 ready/valid 关闭 FR137。**
- **不得仅 FR129 C1–C4 关闭 FR137。**
- **不得 docs-only**（仅评估 defer ≠ 外部 CIRCT 编译/仿真门禁本身）。
- **不得 continue-on-error 静默跳过**（required job / 钉死路径缺工具或失败时禁止 silent skip）。
- 不得改写 FR95 / FR96 / FR110 / FR121 / FR129「已关闭」为失败（NFR56）。
- 不得因「终局」口号静默吞并 **NFR59**（未列入更广 CIRCT/MLIR lower 仍须新合同）。
- 不得冒充 **NFR14-crates**（本门禁 = NFR14 / AD-28）。
- 不得未履行 **NFR58** 运维同步（脊柱/文档/CI）即宣称 FR137 关闭。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR56** / **NFR57** / **NFR58（外部 CIRCT 运维同步）** / **NFR59**。
- 消歧：**NFR14-crates**（crates.io 名占用）≠ 本门禁。

---

### 外部 CIRCT 工具版本 / 发行渠道

| 角色 | 钉死 | 76.2 义务 |
| --- | --- | --- |
| **选定工具** | **firtool**（CIRCT 发行物；CLI 调用入口） | 门禁路径实际调用该二进制 |
| **选定版本** | **firtool-1.155.0**（与 Chisel 7.14.0 配对；AD-9） | 版本可检查；≠ CIRCT HEAD；≠ 未配对 1.156.0 |
| **发行渠道** | GitHub tag **`firtool-1.155.0`** → `firrtl-bin-linux-x64.tar.gz` + `.sha256`；或 AD-9 缓存等价物 | 文档/CI 写明渠道；校验 sha256 |
| 覆盖 / 钉死路径 | CLI 缓存 **或** `RHDL_FIRTOOL_PATH` / 文档钉死目录 | 缺目录/缺二进制 → 非零可读失败 |
| 已关闭对照 | FR129 C1–C4（树内方言标记，可不调用外部 firtool 完成面） | **仍有效；alone ≠ FR137** |

### 编译与/或仿真门禁形状（CI required 与/或文档钉死路径）

| 角色 | 钉死 | 76.2 义务 |
| --- | --- | --- |
| **选定（MVP）门禁形状** | **至少其一**（可同时）：(1) CI **required** job（无 `continue-on-error`）调用钉死 firtool 做 **编译与/或仿真**；(2) 文档钉死可复现路径（例：`just circt-external-check` / `cargo bitloom …` 等价）在 CI 或维护者验收中强制跑通 | 路径可检查；失败非零 |
| **编译门禁** | 外部 firtool 对代表性 `.fir` / Handshake 产物 **编译成功**（或文档钉死的等价 compile 谓词） | 输出可检查 |
| **仿真门禁（可选加深）** | 若选定含仿真：外部 CIRCT/firtool 仿真或文档钉死仿真驱动非零可失败 | 未选仿真时须在关闭文档写明「本 epic 选定 = 编译门禁」且仍满足 E1–E4 |
| 对照 FR127 | `formal-sby` required job 体例可复用；本 epic 目标 = **外部 CIRCT**，≠ sby | 不得用 sby alone 关闭 FR137 |

### 缺工具非零可读失败语义

| 场景 | 钉死行为 |
| --- | --- |
| firtool 二进制缺失 / 版本不匹配 / 渠道资产损坏 | **非零退出**；stderr/日志 **可读**说明缺工具或版本/渠道不符 |
| CI required job 失败 | job **失败**（红）；**禁止** `continue-on-error: true` / silent skip |
| 文档钉死路径在验收时失败 | 验收失败；不得标 FR137 关闭 |

### 验收谓词（E1–E4）

| ID | 钉死 |
| --- | --- |
| **E1 工具版本/渠道** | 钉死 **firtool-1.155.0** + tag/`firrtl-bin-linux-x64` 渠道（或 AD-9 等价缓存）；可检查；≠ PATH 随机；≠ CIRCT HEAD |
| **E2 门禁形状** | CI **required** job **与/或** 文档钉死路径：外部 CIRCT **编译与/或仿真**；无 continue-on-error |
| **E3 缺工具失败** | 缺工具 / 版本不符 → **非零** + **可读**失败；禁止静默跳过 |
| **E4 ATDD / 边界** | ATDD 覆盖 E1–E3；**≠ FR129 C1–C4 alone**；**≠ FR121 / FR110 / FR95 / FR96 alone**；关闭条件 = E1–E4，**≠** docs-only |

### 与 FR129 C1–C4 边界

| 层 | 完成面 | FR137 关系 |
| --- | --- | --- |
| **FR129 C1** | CIRCT Handshake 方言 IR 子集标记（`handshake.func` / `handshake.buffer` + `"fr129": true`） | **仍有效；alone ≠ FR137** |
| **FR129 C2** | 多时钟弹性缓冲（`clock_domains ≥ 2` + `elastic_buffers ≥ 1`） | **仍有效；alone ≠ FR137** |
| **FR129 C3** | 产品 API / CLI `--circt-handshake` | **仍有效；alone ≠ FR137** |
| **FR129 C4** | ATDD `fr129_circt_handshake` + AD-25 修订戳 | **仍有效；alone ≠ FR137**（本 FR 须 **外部** 编译/仿真门禁） |
| **FR137 E1–E4** | 外部工具版本/渠道 + 编译/仿真门禁形状 + 缺工具失败 | **本 epic** |

### 未列入（NFR59）

- 完整 CIRCT/MLIR lower 全家桶；全 allocation/binding 优化套件；任意时钟拓扑自动综合；未配对 firtool-1.156.0+；macos/windows/linux-aarch64 firtool 资产（NFR11）；以 FR129 C1–C4 alone 宣称外部门禁完成。

### Epic 76 关闭条件（76.3 勾选）

- [ ] **76.2 / FR137：** E1–E4 产品路径 + ATDD（外部工具渠道 + 编译/仿真门禁 + 缺工具失败）+ **NFR58** 运维同步
- [ ] **文档 / deferred / Phase 16 故事清单指针**
- [ ] **禁止事项未触发**
- [ ] **品牌 / 依赖：** Bitloom；设计 crate → `bitloom-prelude`
- [ ] **FR129 / FR121 / FR110 / FR95 / FR96 关闭仍有效**（NFR56）
