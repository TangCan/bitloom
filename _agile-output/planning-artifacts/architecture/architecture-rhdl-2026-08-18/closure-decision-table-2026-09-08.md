# 闭包决策表（Wave 0 / FR72）— 2026-09-08

> **权威角色：** Phase 9 Wave 0 合同裁决页；消解 requirements / 调研 open questions，使 FR 验收不打架。  
> **绑定：** `epics.md` **FR72**；调研 `technical-requirements-implementation-gap-generic-2026-09-08` Open Q1/Q2/Q4。  
> **品牌：** Bitloom；设计 crate 只依赖 `bitloom-prelude`。  
> **状态：** **ADOPTED**（Story 26.2）

---

## 1. 裁决总表

| # | 开放问题 | 裁决 | 生效 Epic | 备注 |
|---|----------|------|-----------|------|
| D1 | HLS 闭包「自由」vs「可综合」 | **分裂路径：** 自由/无限制闭包**仅**允许在 **AD-25 外挂 HLS** 路径与**功能侧**；可综合 Bitloom 路径（comb/seq / elaborate→freeze）一律 **`SynthesizableClosure`**（或文档等价约束） | **29**（FR76；受本表约束）；合同面 **26** | **禁止**树内 HLS scheduler（AD-25 / FR86） |
| D2 | FR75（comb/seq 可综合闭包）是否进 Wave 2 | **进入 Wave 2 / Epic 28**；不永久延期；依赖 Epic 27（FR73）+ FR74 | **28**（FR74+FR75） | Wave 1（Epic 27）仍以生成器闭包为 MVP，不含 comb/seq 内联 |
| D3 | `const fn`（Cap-R-18）vs 生成器 `Fn`（FR73） | **双轨允许：** 能 `const fn` 的编译期表优先用 `const fn`；elaborate-time 表/工厂用生成器闭包（Epic 27）；**二者均须在 `freeze` 前消解**，不得残留闭包对象进 `tick`/后端 | **27**（生成器轨）；`const fn` 可与 27 并行/文档化，不另开阻塞 epic | 不得把双轨混成「一种 API 两种语义」而不诊断 |
| D4 | FIRRTL/Chisel 是否编码闭包 | **明确非目标（Cap-R-58）：** Verilog / FIRRTL / Chisel / tick **不**新增闭包 IR 节点 | **26–30**（NFR36） | 闭包只存在于 elaborate 期 |
| D5 | 术语消歧 | 见 §3 术语表；三者不得写成同一完成定义 | **26**（合同）→ 全 Phase 9 | 写入 FR72 / NFR35 |

---

## 2. 分项说明

### D1 — HLS：外挂自由 / 可综合约束

- **功能视图 / host / `#[functional_model]`：** 可继续使用普通 Rust 闭包（与现有功能侧一致）；不进入 FrozenHir。
- **AD-25 外挂 HLS（Bambu / Vitis）：** 允许将**无状态或本表允许的约束类**闭包作为数据流变换，在调度/降低**前**消解（FR76）；约束类以实现时 Epic 29 NFR14 记录为准，但**不得**弱于：调度前消解、无树内 scheduler。
- **可综合 Bitloom 路径（`#[combinational]` / `#[sequential]` / 生成器写入 HIR）：** 必须满足 **SynthesizableClosure**（FR74）；Epic 29 叠加 IP 时，可综合腿仍走 SynthesizableClosure，不得借「HLS 自由」绕过。
- **非目标：** 树内自研 HLS scheduler / allocation（AD-25、FR86）。

### D2 — FR75 进入 Wave 2 / Epic 28

- **裁定：** Cap-R-55/56/70（comb/seq 可综合闭包）**进入 Wave 2**，由 **Epic 28** 交付；**不是**「永远可选 / 永久 defer」。
- **顺序：** Epic 27（FR73 生成器）→ Epic 28（FR74 约束+诊断 → FR75 内联）。
- **与 MVP：** Wave 1 / Epic 27 **明确非目标** comb/seq 内联（留给本裁决的 Epic 28）。

### D3 — `const fn` 与生成器闭包双轨

- **`const fn`：** 适合编译期可求值的常量表 / LUT（Cap-R-18 精神）；优先用于无 elaborate 副作用的纯表。
- **生成器闭包（FR73）：** 适合 elaborate-time 需 session/工厂/connect 的场景；在 `ElaborateSession` 内执行后写入 HIR。
- **共同硬约束：** freeze 前消解完毕；捕获 `Wire`/`Reg` 等硬件引用 → 诊断失败；后端无闭包 IR（D4）。
- **验收：** 不得要求「只实现其中一轨才算 Phase 9 完成」；双轨均合法，Epic 27 以生成器轨为 AC 主路径。

### D4 — Cap-R-58 非目标（后端无闭包）

- FIRRTL / Chisel / Verilog / tick **不得**编码闭包节点或 callback IR。
- 抽检 FrozenHir 与 emit 产物无闭包语义残留（NFR36）。
- 与 FR16 负例共存：捕获闭包仍失败。

---

## 3. 术语表（强制）

| 术语 | 含义 | 不是 |
|------|------|------|
| **生成器闭包** | Elaborate-time 非捕获 `Fn`（或等价），在 session 内展开为 Mem/ROM/工厂实例等，freeze 前消解（FR73） | FR47 产物；Phase 7「闭环」 |
| **FR47 sim generators** | 工具链从 FrozenHir **生成**功能/周期模拟器 **crate**（双视图） | 用户 `Fn` 展开硬件 |
| **Phase 7「闭环」** | 概述字面收口（Epic 19–24）；英文 overview 偶用 *closure* 指闭环 | 本主题 controlled generic closures |
| **SynthesizableClosure** | 可综合路径闭包约束（纯、无堆、无运行时捕获状态等，FR74） | 功能侧自由闭包；未消解的 Rust 闭包对象 |
| **HLS 自由闭包** | 仅 AD-25 外挂路径 / 功能侧允许的较宽闭包用法（D1） | 可综合 Bitloom comb/seq 默认表面 |

---

## 4. 与 FR / Epic 映射（验收指针）

| FR | 本表相关裁决 | 实现 Epic |
|----|--------------|-----------|
| FR72 | 本页为决策表交付物；链接进合同 | **26** |
| FR73 | 生成器轨（D3） | **27** |
| FR74 / FR75 | SynthesizableClosure + Wave 2 进入（D1 可综合腿、D2） | **28** |
| FR76 | HLS 腿按 D1（外挂自由 + 调度前消解；无树内 scheduler） | **29** |
| FR77 | IP 定制；可综合腿仍 SynthesizableClosure | **29** |
| FR78 | 桥接；功能自由 / 周期侧仅消解后信号 | **30** |
| FR86 | 重申 AD-25；可并入本表叙事，无新实现 epic | **26.2**（文档） |
| NFR35 / NFR36 | 术语可测区分；后端无闭包 IR（D4、§3） | **26–30** |

---

## 5. 引用

- `epics.md` — FR72、Epic 26–30、Story 26.2
- `ARCHITECTURE-SPINE.md` — AD-1 / AD-7 / AD-13 / AD-18（待 26.3）/ AD-25 / AD-28
- NFR14：`_agile-output/implementation-artifacts/nfr14-risk-phase9-closures.md`
- 调研：`technical-requirements-implementation-gap-generic-2026-09-08/research.md`（Open Q1/Q2/Q4）
- Cap-R-58（后端不编码闭包）；Cap-R-18（`const fn`）

---

## 6. 一句话门禁

**无本决策表（或缺 D1–D5 裁决）⇒ FR76/FR75 阶段与 HLS 约束类验收视为未签约；Epic 28/29 不得假装已消解 Wave 0 矛盾。**
