---
title: '42.2 Idiomatic / 可维护 Chisel 发射与验收（FR97）'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: '42baf55'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic42-idiomatic-chisel.md'
  - '{project-root}/_agile-output/implementation-artifacts/42-1-epic-42-nfr14-风险记录.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
  - '{project-root}/docs/fr28-chisel-compilable.md'
  - '{project-root}/crates/rhdl-firrtl/src/chisel.rs'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** 修订 AD-27 / FR97 要求 idiomatic / 可维护 Chisel Scala 验收面；当前仅有机械 `emit_chisel`（FR28/FR46「可编译 + 端口/层次」）。若只改文档把机械面标成 FR97，或等上游恢复 `Parser.parse`，会假绿关闭 Epic 42。

**Approach:** 在 `rhdl-firrtl` Chisel 发射路径上实现或升级 **idiomatic emit**（新 API 或显式模式），使至少一夹具满足 NFR14 Epic 42 钉死的命名/结构/可读性验收条，并附自动化断言（风格/结构黄金或等价检查）。文档明确区分历史机械可编译 vs FR97 idiomatic。至少一个负向/边界：不满足 idiomatic 条时失败可读，或显式降级为机械面（不得 silent 宣称 FR97）。公开品牌 Bitloom。

## Boundaries & Constraints

**Always:** 引用修订后 AD-27；NFR14 Epic 42 记录已存在（42.1）；至少一正例夹具过 idiomatic 断言；文档区分机械 vs FR97；负向可读失败或显式降级；机械 FR28/FR46 回归不回退；设计依赖 `bitloom-prelude`；品牌 Bitloom；钉死 Chisel 7.14.0 ↔ firtool 1.155.0。

**Ask First:** 若要把「仅机械可编译」改回 FR97 完成面 — Correct Course / 改 NFR14；若宣称「符合官方 Style Guide」— 须在文档钉死采纳条款（本 MVP 可不采纳官方子集）。

**Never:** 仅改文案把机械 emit 标成 idiomatic；要求恢复 `Parser.parse` 而无替代合同；勾选 Epic 42 关闭 / 改 README·deferred 永久非目标移除（→ **42.3**）；开工 42.3 / Epic 43–47；破坏 FR28 黄金 counter / JVM 合同；用 NFR10 调试再生冒充 FR97。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Idiomatic 正例 | FrozenHir 夹具（counter 或层次）→ idiomatic emit | Scala 含 FR97 标记；命名/结构/可读性过 `check_idiomatic`；黄金或结构断言绿 | N/A |
| 机械输出冒充 | 对 `emit_chisel`（FR28）结果跑 idiomatic check | **失败可读**（指出缺 FR97/结构/命名谓词） | 结构化 Err，非 panic 静默 Ok |
| 显式降级 | 调用方只要机械面 | 继续用 `emit_chisel`；文档标明 ≠ FR97 | 不得在产物头写 FR97 |
| 子集外 Mem | 非法 MemDecl | 仍 E0901（与 FR81 一致） | 不删诊断 |
| 仅改文档 | 无 API/断言 | **不合格** | 审查拒收 |

</frozen-after-approval>

## Code Map

- `crates/rhdl-firrtl/src/chisel.rs` — **UPDATE**：保留 `emit_chisel`（机械 FR28）；新增 `emit_chisel_idiomatic` + `check_idiomatic_chisel`（或同文件内等价 API）
- `crates/rhdl-firrtl/src/lib.rs` — **UPDATE**：`pub use` 新 API；机械单测不回归
- `crates/rhdl-firrtl/testdata/` — **NEW**（可选）：`fr97_idiomatic_*.scala` 黄金夹具
- `docs/fr97-idiomatic-chisel.md` — **NEW**：FR97 完成面、验收谓词、与 FR28 对照
- `docs/fr28-chisel-compilable.md` — **轻量 UPDATE**：交叉链到 FR97；保留「机械 ≠ idiomatic」诚实句（勿做 42.3 README/deferred 收口）
- `crates/bitloom/tests/fr97_idiomatic_chisel.rs` — **NEW** ATDD（正例 + 负向 + 文档区分）
- `_agile-output/implementation-artifacts/nfr14-risk-epic42-idiomatic-chisel.md` — **只读**；**不**勾选关闭条件
- CLI `--also-chisel`：可选接通 idiomatic；非 AC 必须（库 API + ATDD 即可）

## Story

As a Chisel 互操作用户,
I want FrozenHir/`.fir` 路径产出可维护的 idiomatic Chisel Scala（或文档等价往返合同）,
So that 验收超出「能编译即可」。

## Acceptance Criteria

1. Given Story 42.1, when 实现或升级 emit/往返，使至少一夹具满足风险记录中的 idiomatic 验收条，并有自动化断言（风格/结构黄金或等价检查）, then 正例绿
2. And 文档区分：历史「机械可编译」vs FR97 idiomatic 完成面
3. And 至少一个负向或边界：不满足 idiomatic 条时失败可读或明确降级策略（不得 silent 宣称）
4. And 公开品牌仍为 Bitloom

## Tasks / Subtasks

- [x] T1: `emit_chisel_idiomatic`（或升级模式）+ 命名/结构/可读性谓词文档化（AC: 1）
- [x] T2: `check_idiomatic_chisel`（或等价）自动化断言；至少一正例夹具/黄金（AC: 1）
- [x] T3: 负向：机械 FR28 输出（或故意破坏）对 idiomatic check 失败可读（AC: 3）
- [x] T4: `docs/fr97-idiomatic-chisel.md` + fr28 交叉区分；引用 AD-27 / Bitloom（AC: 2, 4）
- [x] T5: ATDD `fr97_idiomatic_chisel`；机械路径回归不红；sprint → done；**不**做 42.3

## Dev Notes

### MVP 形状（务实，须诚实满足 AC）

NFR14 钉死四维：命名 / 结构 / 可读性 /（可选）官方风格子集。本故事 **钉死前三维**；**不**口头宣称官方 Style Guide，除非另写条款。

推荐实现：

1. **保留** `emit_chisel` = 机械 FR28（头注释含 `FR28 compilable`；**不得**写 `FR97`）。
2. **新增** `emit_chisel_idiomatic(hir) -> Result<Artifact, ChiselGenError>`：
   - 头注释显式：`FR97 idiomatic`、`Bitloom`、钉死 Chisel/firtool 对、引用 AD-27。
   - **命名：** `class {Module.name}`、端口 `val {port}`、实例 `val {inst}` 与 FrozenHir 公开名一致（clk/rst → clock/reset 映射与机械路径相同，属稳定映射）。
   - **结构：** 每模块 `extends Module` + `IO(new Bundle {…})`；层次用 `Module(new Child)` + 分节连接；正文按寄存器 / 线网 / 实例 / 逻辑分组（`// --- registers ---` 等节注释），禁止单行巨型扁平堆作为 FR97 通过标准。
   - **可读性：** 合理缩进与换行；可含节注释；仍须能在钉死栈下编译（可与机械共用 JVM 配方；本故事不强制新 JVM job）。
3. **新增** `check_idiomatic_chisel(scala: &str, hir: &FrozenHir) -> Result<(), IdiomaticCheckError>`：
   - 断言头含 `FR97`（或 `idiomatic`）且**不含**把机械冒充为完成面的 silent 路径。
   - 命名：每个非 clk/rst 公开端口与模块名出现在 Scala。
   - 结构：`extends Module`、`IO(new Bundle`、多模块时 `Module(new`。
   - 可读性：存在分节注释或等价结构标记；行数/换行合理（非单行 dump）。
   - 失败时 `code` + en/zh 可读（建议 `rhdl::E0904` 或 `IdiomaticCheckError`）。
4. **负向：** `check_idiomatic_chisel(emit_chisel(...), hir)` → Err（机械头 / 缺 FR97 标记）。
5. **显式降级：** 文档写明：只要机械面 → 继续 `emit_chisel`；产物不得标 FR97。

### 当前代码状态（UPDATE 须保留）

- `chisel.rs`：`emit_chisel` 已产出可读 `Module`/`IO`/`Bundle`，但是 **FR28 机械合同**（头写 FR28；无 idiomatic check）。
- 单测：`chisel_fr28_*`、`fr81_*`、`ad27_compilable_chisel`、`fr88_firtool_chisel_ops_honesty`（机械 ≠ idiomatic 诚实句）须继续绿。
- 黄金：`testdata/fr28_golden_counter.scala` **不得**因 FR97 变红或被替换。
- `docs/fr28-chisel-compilable.md` 仍含「可编译 ≠ idiomatic」；更新时保留机械诚实，并链到 `fr97-*.md`。**勿**在本故事清空 README/deferred 永久非目标列表（42.3）。

### 架构合规

- **AD-27（修订）：** 机械仍满足 FR28/FR46；另增 FR97；禁止仅机械宣称 FR97；不要求 Parser.parse。
- **AD-3：** `.fir` 文本契约不变；idiomatic Scala 是生成器产物，非等上游 FIRRTL→Scala。
- **AD-9 / NFR12：** 不改钉死对。
- **AD-6：** 设计 crate 只依赖 `bitloom-prelude`；实现落在 `rhdl-firrtl` / 测试在 `bitloom`。
- **NFR40 / NFR41：** 不得文档假绿；须引用修订 AD-27。
- **品牌：** Bitloom / `bitloom`；不发布 `rhdl`。

### 测试要求

- 新 ATDD：`cargo test -p bitloom --test fr97_idiomatic_chisel`
  - 正例：idiomatic emit + check Ok；断言命名/结构/FR97 头；可选黄金片段
  - 负向：机械 emit 对 check → Err 且消息可读（含 idiomatic/FR97/mechanical 之一）
  - 文档：`docs/fr97-idiomatic-chisel.md` 存在且区分机械 vs FR97；含 AD-27 / Bitloom
  - fr28 交叉：仍声明机械 ≠ idiomatic 完成面
- 回归：`cargo test -p rhdl-firrtl -- chisel_fr28`；相关 fr88 / ad27 / nfr14_epic42 不红
- **不做：** NFR14 Epic 42 关闭勾选；README/deferred 永久非目标移除

### Project Structure Notes

- 发射与检查优先同文件 `chisel.rs`（或 `chisel/idiomatic.rs` + re-export）
- 文档权威：新建 `docs/fr97-idiomatic-chisel.md`；fr28 仅交叉
- 一故事一提交；本 pipeline 结束标 `42-2: done`，`42-3` 仍 backlog，`epic-42` in-progress

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 42 / Story 42.2]
- [Source: `ARCHITECTURE-SPINE.md` — AD-27 revised 2026-09-09]
- [Source: `nfr14-risk-epic42-idiomatic-chisel.md` — 验收条]
- [Source: `crates/rhdl-firrtl/src/chisel.rs` — emit_chisel baseline]
- [Source: `docs/fr28-chisel-compilable.md` — 机械诚实]
- [Source: `process-one-story-one-commit.md`]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

### Completion Notes List

- 实现 `emit_chisel_idiomatic` + `check_idiomatic_chisel`（E0904）；机械 `emit_chisel` 保留 FR28
- 正例层次夹具过命名/结构/分节；负向：机械产出失败 + 剥除分节失败
- `docs/fr97-idiomatic-chisel.md` + fr28 交叉；未做 42.3 收口
- 审查修补：机械头去 FR97 字样；按 HIR 要求正文分节；端口 `val name = Dir(`；品牌 Bitloom
- ATDD 5 测绿；机械 chisel_fr28 / fr88 回归绿

### File List

- `crates/rhdl-firrtl/src/chisel.rs`
- `crates/rhdl-firrtl/src/lib.rs`
- `crates/bitloom/tests/fr97_idiomatic_chisel.rs`
- `docs/fr97-idiomatic-chisel.md`
- `docs/fr28-chisel-compilable.md`
- `_agile-output/implementation-artifacts/42-2-idiomatic-chisel-发射与验收-fr97.md`
- `_agile-output/implementation-artifacts/atdd-checklist-42-2-idiomatic-chisel-发射与验收-fr97.md`
- `_agile-output/implementation-artifacts/42-2-code-review.md`
- `_agile-output/implementation-artifacts/42-2-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: Story context created (ready-for-dev)
- 2026-09-09: FR97 idiomatic emit + check + docs + ATDD（Story 42.2）

## Review Triage Log

| Finding | Verdict | Route | Evidence |
| ---- | ---- | ---- | ---- |
| Mechanical header embeds `FR97` | high | patch | Fixed: downgrade line no longer contains `FR97` |
| Section check accepts only `--- IO ---` | high | patch | Fixed: require body markers from HIR stmt kinds; ATDD asserts registers/instances/logic |
| Port `val name` substring false pass | medium | patch | Fixed: require `val {name} = {Dir}(` |
| Bitloom not required by checker | medium | patch | Fixed: check requires bitloom |
| Mutilated idiomatic lacks negative ATDD | medium | patch | Added `fr97_mutilated_idiomatic_fails_section_check` |
| Story tasks unchecked / sprint not done | medium | patch | Tasks `[x]`; sprint → done at closeout |
| clk/rst → clock/reset not asserted | low | reject | Header documents mapping; Chisel implicit clock/reset; everyday emit always has clk/rst ports |
| Empty-circuit idiomatic Ok | maybe-false | defer | Unreachable via normal elaborate finish; record deferred |
| Whole-file vs per-module scoping | medium | defer | Hierarchy fixture still fails if Child class missing; deepen later |
| Instance connects not in check | low | reject | FR28 hierarchy predicates cover mechanical; FR97 focuses naming/structure/readability MVP |
| No rhdl-firrtl crate unit tests | low | reject | bitloom ATDD is the product acceptance surface |
| fr28 dropped FR93 永久非目标 pointer | false | reject | Intentional 42.2 docs distinguish; README/deferred cleanup is 42.3 |
| Chinese `可维护` dead path in checker | low | reject | English `FR97 idiomatic` is the emit contract |
| Line-count padding bypass | low | reject | Unlikely everyday; HIR-driven section markers are the real pin |

## Suggested Review Order

**idiomatic emit API** → **check 正例/负向** → **docs fr97 + fr28 交叉** → **机械回归** → **sprint（勿关 42.3）**
