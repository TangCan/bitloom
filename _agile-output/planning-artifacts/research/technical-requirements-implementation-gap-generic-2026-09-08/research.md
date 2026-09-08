---
title: 'technical research: requirements-implementation-gap-generic-closures'
type: 'technical'
topic: 'requirements-implementation-gap-generic-closures'
decision: '弄清 docs/requirements（含受控泛型闭包）与当前实现及 PRD/Epic 的差距，产出全量功能需求落地计划'
source: 'deep-recon-run'
status: complete
preset: 'standard'
validation: 'normal'
created: '2026-09-08'
updated: '2026-09-08'
claims_verified: 7
claims_unverified: 0
---

# technical research: requirements-implementation-gap-generic-closures

**Decision this research serves:** 弄清 docs/requirements（含受控泛型闭包）与当前实现及 PRD/Epic 的差距，产出全量功能需求落地计划

## Executive summary

证据表明存在**三层错位**，而不是单一「实现落后需求」问题：

1. **`docs/requirements` 已写入受控泛型闭包**（Cap-R-47…72，共 26 项新/变更能力），并按路线图挂在 **P2 生成器 → P5 HLS/IP → P7 桥接**。[1]
2. **PRD / Epic / Spine 未承包该正向能力**：Epic 1–8 在规划面上均已 `complete`，但对「泛型闭包 / generator-closure」**零正向 FR**；契约反而以 FR16 / AD-18 **禁止周期精确路径上的捕获闭包**。Phase 7 英文 *closure* 指 overview-literal **闭环**，不是 Rust `Fn`。[2]
3. **crates 实现面无设计态 `impl Fn` / `const fn` / LUT-callback 生成器 API**（检索 NOT FOUND）；现有 FR47「generators」是 `FrozenHir→Rust crate` 代码生成，不是用户闭包展开硬件。[3]

**建议立刻做的事：** 先走 **Correct Course + PRD 增补**，把闭包能力收缩为「**elaborate-time / 冻结前消解**」并与 AD-18 对齐；再按下文 **Wave 0–5** 排期实现。外部 HDL 实践支持该模型（宿主 elaboration 生成器），并反对把硬件线网捕获为构造参数。[4]

**最大 caveat：** requirements 内部对 HLS 闭包「完全自由 vs 可综合约束」、以及可综合 comb/seq 闭包的**交付阶段未钉死**——签约前必须消解，否则 FR 验收会打架。

---

## D1 — Requirements baseline

从 `docs/requirements/**` 抽出 **72 Cap-IDs**（详见 `imports/requirements-capability-inventory.md`）：

| 桶 | 范围 | 数量 |
|----|------|------|
| (A) 闭包前基线 | Cap-R-01…46 | 46 |
| (B) 受控泛型闭包新增/变更 | Cap-R-47…72 | 26 |

闭包相关：`yes=26` / `partial=10` / `no=36`。[1]

**闭包横切主轴（路线图）：**

| 阶段（doc 19） | 能力 | Cap |
|----------------|------|-----|
| P2 | 生成器闭包（工厂 / LUT / 层次批量） | Cap-R-52…54, 69 |
| P3 | 同步策略可选闭包 | Cap-R-57 |
| P5 | HLS 数据流闭包 + IP 生成器闭包 | Cap-R-62, 63, 71 |
| P7 | 桥接适配器闭包模板 + 多视图自由/约束分裂 | Cap-R-65…68 |
| 未分期（R7：生成器之后） | 可综合 comb/seq 闭包 + `SynthesizableClosure` | Cap-R-48, 50, 55, 56, 70 |

**需求内矛盾（签约前必须处理）：** HLS「自由 vs 可综合」冲突；可综合闭包无明确阶段；多视图叙事 vs P7 交付；`rhdl` CLI 命名 vs 产品 Bitloom 品牌（需求文内稳定为 `rhdl`）。

---

## D2 — Contract surface (PRD / Epic / Spine)

| 发现 | 证据 |
|------|------|
| Epic Phase 1–8 均为 `complete`；Epic 1–25；无 Phase 9+ | `epics.md` frontmatter [2] |
| **正向 泛型闭包 ABSENT** | prd / addendum / epics FR 表 / spine [2] |
| **捕获闭包 BAN** | FR16；AD-18；Epic 1 AC [2] |
| Phase 7 *closure* = 闭环（FR46–52） | phase7Scope；review-rubric [2] |
| 闭包若落地须服从 AD-1（无 rustc 期抽网表）、AD-7（冻前入 HIR）、AD-13（仅 builder）、AD-18（禁捕获） | spine § via import [2] |

**结论：** 完成的 Phase 1–8 **不能**被解读为已满足 requirements 闭包章节；缺的是**产品合同与架构修订**，然后才是代码。

---

## D3 — Implementation reality (crates)

13 个 workspace crate；核心 RTL 环路（macros → builder → FrozenHir → Verilog / FIRRTL / tick / FR28 Chisel / FR47 sim crate gen）**有实现证据**。[3]

| 主题桶 | n | 说明 |
|--------|--:|------|
| Implemented | 8 | 表面/HIR/vlog/sim/FIRRTL 子集/Chisel 子集/FR47 |
| Partial | 9 | HLS 外挂、SVA、viz、CDC 标记、Bundle flatten、IP stubs… |
| **Absent（设计面闭包）** | **硬缺口** | `impl Fn` / `const fn` / LUT+callback：**NOT FOUND** |

注意：FR47 `generate_*` 与「生成器闭包」**同名不同物**——前者是工具链产物生成，后者是用户 `Fn` 在 elaborate 期展开硬件。

---

## D4 — External generator / closure patterns

跨 HDL 可行模型：[4]

1. **宿主 elaboration 生成器**（Chisel / Spinal / Amaranth / Hardcaml）——闭包在构造期执行并 *emit* 网表，不是任意宿主语言 HLS。
2. **Clash**：源级可高阶，但 `Synthesize` 顶必须**单态一阶**。
3. **Rust eDSL**（kaze / rust-hdl）：多为 graph API 或受限 `#[hdl_gen]`，非自由可综合闭包。

**反模式：** 多态/高阶 synth 顶；数据依赖递归当「硬件闭包」；**把 wire 捕获为模块构造参数**；匿名未命名生成器局部。

→ 与 Bitloom 脊柱一致的落点是：**elaborate-time `Fn`，冻前消解进 FrozenHir；周期精确路径继续拒绝捕获闭包**。

---

## Cross-dimension insights

1. **文档已写、合同未签、代码未做** 三者同时成立 → 最大风险是团队按 requirements 实现却与 AD-18/FR16 冲突，或按已完成 Epic 误判「需求已交付」。
2. **命名碰撞：** requirements「生成器闭包」、产品 FR47「sim generators」、Phase 7 英文 closure（闭环）——计划与 FR 编号必须显式消歧。
3. **外部实践强化 AD-1：** 闭包应是宿主生成器，不是 rustc 中途抽网表；与现有 FrozenHir 管道兼容。
4. **桶 (A) 并非「已完成」：** D3 显示 CDC RTL、嵌套 Bundle、完整 HLS 调度、LSP 等仍为 Partial/Absent——全量计划必须同时覆盖 **非闭包缺口**，不能只做 Cap-R-47+。

---

## Gap matrix（决策级压缩）

图例：C=已签约 · I=已实现（有测试/代码证据）· R=requirements 要求 · △=部分

| 主题簇 | R | C | I | Gap 类型 | 优先 |
|--------|:-:|:-:|:-:|----------|------|
| 核心 RTL / Verilog / 所有权 | ✓ | ✓ | ✓ | 维护 | — |
| const 泛型 / 层次 | ✓ | ✓ | ✓/△ | 加深 | P |
| Bundle/Vec（FR51） | ✓ | ✓ | △ | 嵌套/derive | P |
| Mem / CDC 原语 | ✓ | ✓ | △/弱 | DoubleFlop 真 RTL 等 | P |
| FIRRTL / Chisel FR28 | ✓ | ✓ | △ | Mem→Chisel 等 | P |
| 多视图 / FR47 gen | ✓ | ✓ | ✓/△ | 桥接模板深度 | P |
| HLS / IP / viz | ✓ | ✓ | △ | 产品化 | P2 |
| **受控泛型闭包（Cap-R-47…72）** | **✓** | **✗** | **✗** | **合同+架构+实现** | **P0** |
| 可综合 comb/seq 闭包 | ✓（章节） | ✗ | ✗ | 阶段未钉 + 实现 | P1 after 生成器 |
| 生成器闭包（工厂/LUT） | ✓ P2 | ✗ | ✗ | 合同+实现 | **P0** |
| HLS/IP 闭包 | ✓ P5 | ✗ | ✗ | 合同+实现 | after 生成器 |
| 桥接闭包模板 | ✓ P7 | ✗ | ✗ | 合同+实现 | after 多视图稳定 |

---

## Recommendations — 全量功能需求落地计划

### Wave 0 — 合同与架构对齐（阻塞实现）· 建议 1–2 周

**下游绑定：** Correct Course → PRD amendment → ARCHITECTURE-SPINE AD 修订 → 新 Epic Phase 9（或 Epic 26+）。

1. **消解 requirements 内矛盾**（HLS 自由 vs 可综合；可综合闭包阶段；多视图交付叙事）。产出一页决策表进 PRD。
2. **PRD 增补建议 FR 族（示例编号，签约时正式分配，避开 FR46/47/51 历史碰撞）：**
   - **FR-GC-1** Elaborate-time generator closures（Cap-R-52…54, 59-generator, 69）
   - **FR-GC-2** SynthesizableClosure 约束与诊断（Cap-R-48…50, 60）
   - **FR-GC-3** Comb/seq 可综合闭包（Cap-R-55, 56, 70）— 明确 **依赖 FR-GC-1**
   - **FR-GC-4** HLS DF closures（Cap-R-62, 71）
   - **FR-GC-5** IP generator closures（Cap-R-63）
   - **FR-GC-6** Bridge adapter closure templates（Cap-R-65…68）
3. **修订 AD-18：** 保留「周期精确路径禁止 *捕获* 闭包」；新增「允许 *非捕获 / elaborate-time* 闭包，且必须在 `freeze` 前消解为 HIR，不得进入 `tick` 为 Rust 闭包对象」。
4. **明确非目标：** 不在 FIRRTL/Chisel 中编码闭包；与 Cap-R-58 一致。

### Wave 1 — 生成器闭包 MVP（对应 requirements P2）· 建议 Epic 26

**验收（建议）：**

- prelude/builder API：`fn with_init<F: Fn(…) -> Bits<N>>(…)` 或等价，在 `ElaborateSession` 内执行并写入 Mem/ROM/常量。
- 模块工厂：`Fn` 返回子模块实例并完成 connect（Cap-R-53）。
- 宏/检查：生成器模式执行后 HIR 无闭包残留（Cap-R-59）。
- 负例：捕获 `Wire`/`Reg` 引用 → 诊断（对齐 D4 反模式 + AD-18）。
- ATDD：LUT/CRC 多项式表由闭包生成并与手写表 golden 一致。

**非目标：** comb 内联闭包（留给 Wave 2）。

### Wave 2 — 可综合闭包（R7）· 建议 Epic 27

- `SynthesizableClosure` / 纯函数、无堆、无捕获运行时状态（Cap-R-50）。
- `#[combinational]` / `#[sequential]` 内联展开（Cap-R-55/56）。
- 所有权检查扩展（Cap-R-70）。
- 与 FR16 负例测试矩阵共存。

### Wave 3 — 非闭包高优先缺口（与闭包并行或交错）

保持产品诚实：桶 (A) 未做完的部分仍要排期：

| 项 | 建议动作 |
|----|----------|
| CDC / DoubleFlop 真 RTL | 补齐 prelude 原语 → HIR → vlog/sim |
| Bundle 嵌套 / derive | 扩展 FR51 深度（AD-20 已允许文档化表面） |
| Chisel Mem / FR28 缺口 | 按现有 E0901 清单收敛 |
| HLS 真调度 vs 外挂 | 产品路径决策（FR-GC-4 依赖此决策） |
| IP stubs → 可综合 IP | 先无闭包版本，再接 FR-GC-5 |
| 形式化 / viz / C ABI | 按 PRD 残余 Partial 逐项 |

### Wave 4 — HLS / IP 闭包（requirements P5）· Epic 28+

依赖 Wave 1 + HLS 产品路径决策。IP 生成器闭包在 generate-time 执行（Cap-R-63）；viz 保持闭包透明（Cap-R-64）。

### Wave 5 — 桥接闭包框架（requirements P7）· Epic 29+

`start_wait_complete` 类模板（Cap-R-65）；功能视图自由闭包已部分存在于测试/host view——产品化为可复用 adapter（Cap-R-66…68）。

### 推荐工作顺序（一张图）

```text
Wave0 合同/AD ──► Wave1 生成器闭包 MVP ──► Wave2 可综合闭包
                         │
                         ├─► Wave3 非闭包缺口（并行）
                         ├─► Wave4 HLS/IP 闭包
                         └─► Wave5 桥接闭包
```

**置信度：** Wave 0–1 高（D1–D4 交叉一致）；Wave 2–5 中高，依赖 Wave 0 对 HLS/阶段矛盾的裁决。

---

## Contrary evidence / caveats

- Red-team 未开；未发现「闭包已在 prelude 隐藏实现」的反证——D3 全仓库检索为 NOT FOUND。[3]
- requirements 可能被解读为「愿景」而非当前合同；本决策按用户指定 **Scope B（全量 requirements）** 处理，因此强调必须先签约。
- Epic `complete` 不保证每个 Partial 主题已达 requirements 全文深度（D3 Implemented/Partial/Absent 分层）。

---

## Open questions

1. HLS 变换闭包最终归「自由」还是「可综合」？（阻塞 FR-GC-4）
2. Cap-R-55/56 是否进入 Wave 2，或降级为 post-MVP 可选？
3. 新 FR 编号策略：独立 FR-GC-* 还是并入下一阶段 PRD 连续号（需避开历史碰撞）？
4. `const fn`（Cap-R-18）与生成器闭包（Cap-R-69）是否双轨交付，还是先闭包后 const？
5. 品牌文案：requirements 的 `cargo rhdl` 是否批量改为 `cargo bitloom`（文档债，非闭包阻塞）？

---

## Source appendix

| Ref | Finding | Publisher | Pub | Accessed | Conf |
|-----|---------|-----------|-----|----------|------|
| [1] | 72 Cap；26 闭包新增；P2/P5/P7 分期 | [requirements inventory](imports/requirements-capability-inventory.md) | project-docs | 2026-09-08 | high |
| [2] | 闭包未签约；FR16/AD-18 禁捕获；Phase1–8 complete | [prd-epic-spine contract](imports/prd-epic-spine-contract.md) | project-planning | 2026-09-08 | high |
| [3] | crates 无设计面 Fn 生成器；核心环路已实现 | [crates inventory](imports/crates-implementation-inventory.md) | project-code | 2026-09-08 | high |
| [4] | Elaboration 生成器模式；Clash 一阶顶；禁捕获 wire | [D4 digest](digests/d4-external-patterns-r1-1.md) / [external import](imports/external-generator-closure-patterns.md) | Chisel/Spinal/Clash/Hardcaml/kaze docs | 2026-09-08 | high |

Supporting digests: `digests/d1-*.md` … `d4-*.md`.

---

## Staleness map

| Claim class | Freshness bar | Re-check |
|-------------|---------------|----------|
| Contract IDs / epic status | ≤ 1 mo after next PRD/epic edit | 下一轮 Correct Course 后立即 |
| crates Fn 检索 | ≤ 1 mo during Wave 1 | Wave 1 每个 story 结束 |
| External HDL docs | ≤ 12 mo (patterns) | 2027-03 或 Clash/Chisel 大版本时 |
| requirements Cap 表 | 需求文每次修订 | Refresh 本 run |

Earliest actionable re-check: **PRD/AD 修订落地当日**（刷新 [2]）。
