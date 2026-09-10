# NFR14 风险记录 — Epic 60 SymbiYosys/SMT 形式路径（FR119）

> **权威：** PRD NFR14；AD-28；Phase 14 **NFR48 / NFR49 / NFR50 / NFR51**；交付 **FR119**。  
> **前置：** Epic 57 **closed**（FR116）；Epic 45 **closed**（FR100 F1-(i) 有界穷举）；Epic 54 **closed**（FR112 分支 B MemRead≡tick）。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **60.2–60.3** 标 `ready`。  
> **隔离：** ≠ FR100 F1-(i) alone；≠ FR112 分支 B alone；≠ FR92 记分板 alone；≠ FR85 Verilator lint  alone；≠ FR107 SystemC AT。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR119 / Epic 60；NFR14、NFR48、NFR49、NFR50、NFR51；对照 FR100、FR112、FR92、FR85、FR107 |
| 记录日期 | 2026-09-10 |
| 状态 | **accepted** — Story **60.1** 门禁；实现 → 60.2；收口 → 60.3 |
| **选定绑定** | **(A) SymbiYosys（`sby`）绑定** |

### (a) 上游约束

- **Epic 57 / FR116 已关闭：** Phase 14 加深合同已开闸。
- **FR100 / Epic 45（已关闭 · 隔离）：** F1-(i) 树内有界穷举 `FormalEquivProduct` **仍有效**（NFR48）；**不得**改写为失败，亦不得把 F1-(i) alone 冒充 FR119 / F1-(ii)。
- **FR112 / Epic 54（已关闭 · 隔离）：** 分支 B GeneratedFunctional MemRead≡tick **仍有效**（NFR48）；**不得**以 FR112-B alone 冒充分支 A / FR119。
- **FR92：** `SharedStimulusScoreboard` 为配套；**alone ≠ FR119**（亦 ≠ FR100）。
- **FR85 / FR39：** Verilator `--lint-only --assert` / 可选 `sby` 钩子为 SVA 外部 checker 模式；**alone ≠ FR119**（FR119 须钉死 **SymbiYosys 形式证明路径**，含 assume/assert 义务与专用夹具，而非仅 lint）。
- **≠ FR107：** SystemC TLM AT / `nb_transport` **不是**本 epic 完成面（≠ FR107 混淆）。
- **设计依赖：** 设计 crate 只依赖 `bitloom-prelude`；形式路径在工具链脚本/文档化产品入口，不泄漏进用户设计包。
- **品牌：** Bitloom / `bitloom-*`。
- **NFR50：** formal/SBY 工具链形状由本记录钉死；脊柱不预钉 crate 切分；触及 AD 修订时须先改脊柱再标实现 story ready（本选定不强制改 AD-25/27）。

### (b) 粗工期带

- **预计：** Epic 60 整体约 **1–4 人周**（60.1 ≤0.25；60.2 产品路径+夹具+ATDD 0.75–3；60.3 收口 0.25–0.75）。置信度：**中**（CI 未必安装 `sby`；以「有则跑通、无则可读失败」合同为准）。
- **假设：** FR100 / FR112-B 回归不破；默认 `just test` **不**要求主机安装 SymbiYosys；完整证明在本地/可选 CI job 复现。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **60.2–60.3** 标 `ready`。
- **不得以 FR92 记分板 alone 关闭 FR119。**
- **不得以 FR100 F1-(i) alone 关闭 FR119。**
- **不得以 FR112 分支 B MemRead≡tick alone 关闭 FR119。**
- **不得仅改文档（docs-only）关闭 FR119。**
- **不得把 FR107 SystemC AT 写成 FR119 / 冒充形式路径。**
- **不得以 FR85 Verilator lint alone / toy `check_sva_text` 冒充 SymbiYosys F1-(ii) 完成面。**
- 不得改写 FR100 / FR112「已关闭」为失败（NFR48）。
- **不得 silent 宣称未选分支 C（更多 IP 手写 FL）已交付**（NFR51；保持 deferred）。
- 工具缺失时 **不得 silent 成功**（须可读失败）。
- 不得冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR50** / **NFR51** 共同责任人；NFR48/49 共同注意人。
- 备份 / 升级路径：缩回或改写 (A) SymbiYosys 义务、与 FR85 路径合并争议、或触及 formal/AD 修订时，升级至架构（AD-28）维护者 / Correct Course 批准人。

---

### 并行 / 维护叠加（Chipyard 式 · NFR49）

- Epic 58–63 **可并行**；本 epic 引入外挂 `sby`/Yosys/SMT 引擎安装与可选 CI job 时，与 LSP/波形/VIP 等并行会膨胀回归面 — 须保持默认 `just test` **不**硬依赖 `sby`，并把 FR100/FR112-B 回归列入 60.2 验收。
- 文档面：`docs/fr119-*`（或扩展 fr100/fr112）、`deferred-work.md`、README 须区分 FR100 F1-(i) / FR112-B / FR85 / FR119，禁止混用关闭口径。

### 绑定路径（本 epic 钉死）

| 分支 | 内容 | 本 epic |
| --- | --- | --- |
| **(A) SymbiYosys（`sby`）绑定** | 文档化一等产品入口：调用 `sby` 跑 assume/assert 夹具；有工具则 pass/fail 可读；缺工具则非零退出+可读错误 | **选定 = FR119 完成面** |
| **(B) 独立 SMT 求解器绑定** | 直接对接 z3/boolector 等、绕过 `sby` 的产品入口 | **未选为独立完成面**；SMT 可作为 `sby`/`yosys-smtbmc` **后端**记载，不得 silent 宣称「独立 SMT 产品支已交付」 |
| 分支 C（更多 IP 手写 FL） | 原 FR112 未选加深 | **deferred**（NFR51；≠ FR119） |

### 选定绑定 (A) — 工具版本 / 义务 / 夹具 / 可复现

| 项 | 钉死 |
| --- | --- |
| **工具** | **SymbiYosys** 命令行入口 `sby`（依赖 Yosys + 至少一个 SMT/BMC 引擎，常见：`yosys-smtbmc` + z3/boolector/yices 等，由本机 `sby` 安装决定）。**不**要求默认 CI/`just test` 预装 `sby`。 |
| **版本义务** | 文档/脚本记录检测：`command -v sby`；跑通时打印 `sby --version` 与 `yosys -V`（若可得）。**不**钉死单一发行版号为关闭硬条件；**Story 60.2 必须在 `docs/fr119-*`（或脚本头注释）记录一次已知可通版本带**（本机实测的 sby/yosys/引擎）。无 `sby` → exit ≠ 0 + 可读错误，**禁止 silent 成功**。 |
| **assume / assert 义务** | 夹具须含至少一条 **assume**（约束输入/环境）与至少一条 **assert**（待证性质）；`.sby`（或等价）配置须指向该设计/网表与属性；证明模式以 **bmc**（或文档钉死的 `mode`）为 MVP。故意破坏 assert → **可读 FAIL**；工具缺失 → **可读 missing-tool 失败**。 |
| **夹具范围** | ≥1 最小可复现夹具（pass）；≥1 负向/缺失工具或故意失败路径（可读 fail）。可复用 FR85 Counter SVA **导出物**作输入，但 **FR119 验收以独立 `sby` 证明合同为准**，不得仅用 Verilator lint / FR85 默认路径关单。 |
| **产品入口（相对 FR85）** | **Fork 新入口，不扩展 FR85 默认 checker 为完成面。** 既有 `just formal-sva-check` / `scripts/formal-sva-check.sh`（`BITLOOM_FORMAL_CHECKER=sby` + `BITLOOM_FORMAL_SBY_FILE`）保留为 **FR85**；FR119 须新增文档化一等路径（建议名：`just formal-sby-check` / `scripts/formal-sby-check.sh`，缺工具强制：`BITLOOM_SBY_FORCE_MISSING=1`）。60.2 可共享导出夹具文件，但**不得**把「跑通 FR85 Verilator lint」或「FR85 可选 sby 钩子 alone」写成 FR119 关闭。Story **60.2** 落地脚本；本故事只钉死合同。 |
| **CI / 本地可复现** | **本地：** 安装 SymbiYosys 后按 FR119 文档跑通 pass；`BITLOOM_SBY_FORCE_MISSING=1` 强制缺失路径。 **CI：** 默认 `just test` **不**依赖 `sby`；ATDD 断言合同/脚本/文档存在与缺失失败语义；可选独立 job 在有 `sby` 镜像时跑真证明（非关闭硬条件）。 |
| **证明义务（60.2）** | 公开品牌 Bitloom；产品或文档化一等路径 + 夹具（pass + 可读 fail）；FR100 F1-(i) 与 FR112-B 回归不破；≠ FR107。 |

### Epic 60 关闭条件（Story 60.3 勾选）

- [ ] **60.2 / FR119：** 选定 (A) `sby` 路径可复现 + ATDD + ≥1 夹具；缺工具可读失败；pass/fail 可读；FR100/FR112-B 回归
- [ ] **文档 / deferred / README / FR100·FR112 交叉链**（分支 C 仍 deferred）
- [ ] **禁止事项未触发**
- [ ] **品牌 / 依赖：** Bitloom；prelude 边界
- [ ] **FR100 / FR112 分支 B 关闭仍有效**（NFR48）

---

## 门禁一句话

**缺 NFR14（或缺 a–d / 未钉死 SymbiYosys 绑定与 assume/assert 义务）⇒ 不得将 60.2–60.3 标 `ready`。**  
**FR119 完成面 = (A) SymbiYosys（`sby`）绑定可复现路径；不得以 FR92 / FR100 F1-(i) / FR112-B / FR85 Verilator alone / docs-only / FR107 关闭；缺工具不得 silent 成功；分支 C 仍 deferred。**

---

### 引用

- AD-28 — 风险门禁（NFR14）；Phase 14 NFR48–NFR51
- PRD FR119 / NFR14；对照 FR100、FR112、FR92、FR85、FR107
- Correct Course / Epic 57 闸门：`nfr14-risk-epic57-phase14-nfr47-deferred-deepen.md`
- 体例：`nfr14-risk-record-template.md`；`nfr14-risk-epic54-formal-dual-model-depth.md`
- 隔离文档：`docs/fr100-formal-equiv.md`；`docs/fr112-generated-functional-memread-equiv.md`；`docs/fr39-formal-sva.md`（FR85 ≠ FR119）
- FR85 入口（勿冒充 FR119）：`scripts/formal-sva-check.sh` / `just formal-sva-check`
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**
