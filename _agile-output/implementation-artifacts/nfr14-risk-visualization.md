# NFR14 风险记录 — 内置可视化（Epic 23 / FR38 + FR49）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（Phase 7 风险门禁）。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 23 后续故事 **23.2–23.5** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR38 + FR49（及 FR40 `visualize`/`doc`/`wave`）/ Epic 23 |
| 记录日期 | 2026-08-21 |
| 状态 | accepted |

### (a) 上游约束

- **既有 HIR HTML（Story 10.4）：** `rhdl-viz::to_html` 已能列出模块/端口与实例层次；本 epic 须将其**产品化**为 CLI/文档入口，而非另立第二套 HIR 可视化语义。
- **波形基座（AD-5 / AD-24 / FR31）：** 默认 dump 为 **VCD**；FST 为可选（`vcd2fst` / Verilator `--trace-fst`）；关闭 FST 时必须仍能走 VCD。允许基于 VCD/FST **渲染或转码** 产出可浏览时序视图。
- **完成定义（概述字面 / FR49）：** 「内置层次图 + 时序图」须有**产品命令或文档化入口**产出可查看工件；**不得**以「请用户自行打开 GTKWave」作为 FR49 唯一完成路径。
- **LSP：** 完整 hover/goto 仍可延期；不阻塞 FR38/FR49 可视化半程。
- **设计依赖边界：** 设计 crate 仍只依赖 `bitloom-prelude`；可视化生成属工具链 / `cargo bitloom`。

### (b) 粗工期带

- **预计：** Epic 23 整体约 **2–3.5 人周**（23.1 风险记录 ≤0.25 人周；23.2 层次产品入口 0.5–1 人周；23.3 时序/波形入口 0.75–1.25 人周；23.4 统一文档 / UJ-6 可视化半程 0.25–0.5 人周；23.5 联验 0.25–0.5 人周；缓冲含 smoke 与夹具）。
- **置信度 / 假设：** 中高；假设复用 `rhdl-viz` + `bitloom-sim` VCD，时序视图以 HTML（或等价可浏览）渲染为主。若要求 Tywaves 级类型化源级波形 IDE，工期显著上修。

### (c) 禁止的静默降级清单

- 不得以「用户自行打开 GTKWave / Surfer」作为 **FR49** 唯一完成路径，而不改 PRD。
- 不得删除 / 绕过产品 **`wave` / `visualize`（或等价）入口**，改回「仅库 API HTML dump」或「仅写 VCD 文件」并宣称 FR38/FR49 完成，而不改 PRD。
- 不得把层次视图交付成**空壳**（无模块/端口/实例层次）冒充 FR49 层次图。
- 不得在关闭 FST 时破坏默认 **VCD** 路径（须仍可用）。
- 不得在无本记录（或缺 a–d）时将 **23.2–23.5** 标 `ready` 或开工实现。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）
- 备份 / 升级路径：架构争议升级至 AD-24 / AD-28 维护者；CLI 动词改名须同步 FR40 文档与本记录。

### 产品入口形态（钉死偏好）

| 形态 | 说明 | 本记录 |
| --- | --- | --- |
| `cargo bitloom visualize` / `doc` | FrozenHir → 模块层次 HTML（或文档化交互视图） | **必需产品入口** |
| `cargo bitloom wave` | tick/仿真轨迹 → 时序图或可浏览波形 HTML（可基于 VCD/FST） | **必需产品入口** |
| 仅「请自行 gtkwave foo.vcd」 | 外部查看器可作补充，**不得**作唯一路径 | **禁止冒充 FR49 完成** |

### 并行 / 维护叠加（Chipyard 式）

- 与 Epic 20–22/24 并行时：CLI 动词表、文档交叉链接与 VCD/FST 回归面叠加；升钉仿真 API 须同步可视化 smoke。
- 语义最低公分母：层次视图须与 FrozenHir 实例树一致；时序视图信号名须与 VCD/tick dump 一致。
- 禁止把「LSP 未做」当作 FR38/FR49 可视化未完成的豁免（LSP 已明确可延期）。

### 引用

- AD-28 — Phase 7 风险门禁（NFR14）
- AD-24 — 可选 FST；默认 VCD
- PRD NFR14（`prd-rhdl-2026-08-19/prd.md` §6）
- PRD FR38 / FR49 / FR40；概述字面可视化
- 历史别名消歧：**NFR14-crates**（crates.io FCFS）≠ 本门禁 **NFR14**

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 23 故事 23.2–23.5 标 `ready`。**
