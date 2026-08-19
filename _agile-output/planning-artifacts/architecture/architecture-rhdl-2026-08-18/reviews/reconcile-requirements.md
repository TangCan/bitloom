# Reconcile: requirements → architecture spine

- **Spine:** `ARCHITECTURE-SPINE.md` (draft, 2026-08-18)
- **Primary input:** `docs/requirements/RHDL 方案大纲（最终版）.md` (TOC) + numbered chapters, especially `4. 整体架构与工作流.md` and `19. 实施路线图.md`
- **Job:** what from the requirements did **not** land in the spine — especially quiet constraints. Do not rewrite the spine.
- **Lens:** a spine only keeps invariants that would let two downstream epics diverge. Feature catalog that is later-phase belongs in **Deferred**. Silence on a load-bearing constraint is **dropped-by-accident**.

This note is a gap report, not a rewrite.

---

## How to read this

| Bucket | Meaning |
| --- | --- |
| Landed | In an `AD`, convention, stack pin, or named Deferred — including intentional overrides of the requirements |
| Intentional Deferred | Named in spine `Deferred` (or clearly covered by one of those bullets) |
| Dropped-by-accident | Load-bearing in the requirements, not an `AD`, not named Deferred; two epics could choose incompatibly |
| Under-deferred | Requirements treat it as later-phase; spine neither adopts nor names it Deferred — catalog hole, not an invariant miss |

Quiet constraints (tone / “always / never / must”) are called out even when the matching feature is elsewhere deferred.

---

## 1. Landed — including intentional overrides

These are **not gaps**. They are recorded so a later reader does not re-open them as “missing.”

| Requirements claim | Spine landing | Kind |
| --- | --- | --- |
| 过程宏在 `cargo build`/`rustc` 期间构建 HIR；HIR 可从编译产物缓存加载 | **AD-1 / AD-7:** HIR only in generator `elaborate()`; freeze then immutable; `cargo test` must elaborate then `tick` | Override |
| 所有权/借用即多驱动门控（编译期 multiple drivers） | **AD-4:** unique-driver at `freeze()`; ownership may guide API, is not a soundness proof | Override |
| 与 Chisel 双向：FIRRTL→Chisel 生成器 + 可维护 Scala 作为互转路径 | **AD-3:** contract is HIR ↔ FIRRTL 6.0.0 text; no maintainable Chisel Scala; Chisel does not parse `.fir` | Override |
| 发布 crate 名 `rhdl`；`cargo add rhdl` | **AD-2:** crates.io `rhdl-rs`; repo may stay `rhdl` | Override |
| FIRRTL 3.0 锁定（路线图 19.11 风险） | Stack: FIRRTL spec **6.0.0**, firtool **1.155.0** | Override |
| 从 HIR 降低 TLM / 无定时功能模拟器 | **AD-5:** functional view is handwritten `#[functional_model]`; forbid HIR→TLM | Override |
| 路线图阶段七才做多视图 | **AD-5** pulls the *split* forward as an invariant (handwritten functional vs HIR `tick`); the rest of phase 7 is still later | Pull-forward |
| 阶段一 Verilog；FIRRTL 导出靠后 | **AD-8:** phase-1 Yosys-friendly Verilog; FIRRTL *emit* is phase 2; HIR semantics align FIRRTL 6 subset *now*; phase 1 single clock | Adopted |
| HIR 与 FIRRTL 语义对齐；可逆子集；导入有损 | **AD-3** + **AD-8** | Adopted |
| 互转只走周期精确视图，功能视图不进 FIRRTL | Implied by **AD-3** + **AD-5** | Adopted (implicit) |
| 嵌入式 DSL；设计即合法 Rust；无独立语言/解析器 | Paradigm + **AD-1** | Adopted |
| 后端纯从 HIR 出产物；单一事实来源 | **AD-6**, **AD-7** | Adopted |
| Cargo 本地工具链；设计是普通 Cargo 包 | **AD-11** | Adopted |
| 用户错误不得 `panic` / `custom attribute panicked` | **AD-10** | Adopted |
| HLS / IP / 可视化 / IDE / 形式验证 / SVA | Named **Deferred** | Deferred |
| CDC phantom 域、多时钟 HIR | Named **Deferred**; phase 1 single clock in **AD-8** | Deferred |
| 所有权作声音性证明 | Named **Deferred** (never as freeze gate) | Deferred |

---

## 2. Dropped-by-accident — load-bearing, silent in the spine

These should have been an `AD` **or** a named Deferred bullet. Silence lets epics fork.

### D1 — Explicit comb/seq split; no inferred latches

- **Where:** 原则 3.3–3.4；目标 2.4；`7.` 组合逻辑；`8.` 时序逻辑
- **Quiet rule:** `#[combinational]` vs `#[sequential]` is mandatory, not inferred. Comb may drive `Wire`/`Output` only; only seq may write `Reg.d`. Incomplete assignment in comb is an error (no implicit latch). Comb must not write `Reg.d`; seq must not drive combinational nets.
- **Spine:** capability map lists “comb·seq” under prelude/builder/macro; **no rule**.
- **Divergence:** one epic infers `always_comb` / allows incomplete assigns; another requires attributes and rejects latches at freeze.

### D2 — Synthesizable subset is a gate on the cycle-accurate path

- **Where:** 目标 2.7；原则 3.6；`15.` 可综合子集
- **Quiet rule:** cycle-accurate / generate path rejects heap (`Vec`/`Box`/`String`), unbounded recursion, `dyn Trait`, capturing closures, file/net/threads, default `f32`/`f64`. Functional view (`#[functional_model]`) may use them. Enforcement is *before* Verilog emit (reqs: compile-time / `Synthesizable` trait). Functional fields must be marked (`#[functional_state]`) or they are illegal on the hardware struct.
- **Spine:** HLS deferred; dual-model in **AD-5**; **no synthesizable-subset invariant** and no “functional fields never enter freeze/HIR.”
- **Divergence:** one epic dumps illegal constructs to Verilog; another rejects at elaborate; a third leaks `VecDeque` into the netlist.

### D3 — Strict width vs FIRRTL widening (unresolved conflict)

- **Where:** `5.2.1` 严格位宽：算术结果宽度与操作数相同，扩展/截断必须显式
- **Spine:** **AD-4** rejects width mismatch at freeze; **AD-8** aligns HIR node semantics to FIRRTL 6 *now*. FIRRTL `add` on two `UInt<n>` yields `UInt<n+1>`.
- **Quiet miss:** the requirements’ strict-same-width rule is neither adopted nor explicitly overridden. Aligning to FIRRTL 6 silently contradicts ch.5 unless the surface API still truncates.
- **Divergence:** prelude operators truncate; HIR/`rhdl-firrtl` widen; Verilog backend picks a third.

### D4 — No implicit global clock (even in phase-1 single clock)

- **Where:** 原则 3.3；`11.1`「每个寄存器必须明确属于某个时钟域，不允许隐式全局时钟」；模块示例 `pub clk: Clock` / `pub rst: Reset` 作为端口
- **Spine:** **AD-8** phase-1 single clock; CDC phantom / multi-clock **Deferred**. Does not say whether that one clock is an explicit port or a Chisel-style implicit `Module` clock.
- **Divergence:** one epic generates hidden `clk`; another requires `Clock`/`Reset` fields on every sequential module.

### D5 — Port direction is a type, not a FIRRTL-style naked wire

- **Where:** `5.3`「所有模块端口必须使用方向类型声明」`Input`/`Output`/`InOut`; `6.` 模块字段分类
- **Spine:** freeze checks 方向 (**AD-4**); no rule that the *surface* API wraps ports in direction types (vs raw `UInt` ports + FIRRTL direction on HIR only).
- **Divergence:** builder/macros disagree on `Input<UInt<8>>` vs `UInt<8>` + attribute.

### D6 — Connection completeness (undriven ports)

- **Where:** `9.4.2` 未连接输入为错误（除非显式悬空）；未连接输出为警告/错误
- **Spine:** **AD-4** is multi-drive + width/direction mismatch. **Undriven** is the dual and is absent.
- **Divergence:** one epic allows floating inputs; another requires a dangling marker.

### D7 — Interop metadata / names survive the FIRRTL hop

- **Where:** 原则 3.5；`4.2` HIR `Metadata`（原类型名、参数、源位置、`functional_model_info`）；`12.2.3` FIRRTL annotations；`13.4.1` Verilog 可读性（保留原名、层次默认不展平）
- **Spine:** **AD-3** reversible *subset* (scalar ports; no property; no CHIRRTL mem). Conventions: stable module names + private mangling. **No** “preserve source names / parameters as annotations” and no “Verilog keeps hierarchy + original identifiers by default.”
- **Divergence:** import/export epic strips to anonymous `reg_0`; Verilog epic flattens; debug regen cannot round-trip names.

### D8 — Native sim waveform is part of the tick contract

- **Where:** `4.4`, `13.6`, `14.`（VCD/FST；`dump_vcd`）；阶段二交付物
- **Spine:** **AD-5** is cycle-accurate `tick` from frozen HIR. Waveform is not adopted, not Deferred.
- **Divergence:** `rhdl-sim` ships tick-only; tests/docs assume VCD; a later epic invents a second recorder API.

### D9 — Last-connect on import must become unique-drive HIR

- **Where:** `12.4.2` FIRRTL last-connect → mux/when 规范化后再进 RHDL
- **Spine:** **AD-3** + **AD-4** *imply* import cannot leave multi-drive, but the normalization rule is unstated.
- **Divergence:** import rejects last-connect FIR; another keeps FIRRTL last-connect in HIR and only the Verilog backend collapses it.

---

## 3. Under-deferred — later-phase in the requirements, unnamed in spine `Deferred`

Not every item needs an `AD`. These should still appear under **Deferred** so epics do not treat them as in-scope surprises.

| Gap | Requirements | Why it is not just “IP/HLS/IDE” |
| --- | --- | --- |
| **U1 Analog / InOut / tri-state** | `5.`: Analog 仅顶层 IO；InOut 仅顶层 | **AD-3** import subset lists scalar / no property / no CHIRRTL mem — not Analog/InOut |
| **U2 Mem / sync-read memory** | `5.` `Mem<T,DEPTH>`；`12.4.4` vs `cmem`/`smem` | Phase-1 language vs FIRRTL mem is a real fork if unnamed |
| **U3 Handwritten TLM↔RTL bridge, `#[abstraction]`, `#[functional_state]`, `#[bridge]`, mixed `both` sim** | `18.`, 阶段七 | **AD-5** forbids *generating* TLM from HIR; it does **not** defer handwritten bridges or mixed sim |
| **U4 C ABI / `cdylib` for functional and cycle-accurate sim** | `4.7`, `13.6`, `18.4` Rust 库 **和** C 共享库 | Software-integration path; easy for a sim epic to skip |
| **U5 Dual-view formal equivalence** | `18.6.3` 可选形式化等价 | **AD-5** pins random/contrast tests only; formal view-equiv unlisted |
| **U6 Coverage** | `14.` 代码/功能覆盖率 | Not under formal/SVA Deferred |
| **U7 Black-box / foreign IP wrapper** | `15.6` 黑盒模块 | Distinct from “IP crate” Deferred |
| **U8 Clock gating / enables** | `11.` | Not implied by “CDC phantom domain” |
| **U9 Reset polarity + sync/async as a type** | `8.`/`11.` `ClockDomain` + `Polarity`/`ResetKind` | Multi-clock Deferred; phase-1 Verilog still needs *a* reset style (sync vs async `always_ff`) |
| **U10 `cargo rhdl` subcommand surface** | `13.2.2`: `generate`, `check`, `import`, `test`, `visualize`, `wave`, `doc`, `build-sim` | Spine paradigm uses `cargo rhdl build`; other verbs unset. Visualize/wave/doc sit with Deferred tools; `import`/`check` will appear as soon as FIRRTL/HIR land |
| **U11 FIRRTL→Chisel generator as a *tool* (not the interop contract)** | 阶段四；`12.3.1`；`cargo rhdl generate --format chisel` | Spine Deferred says Scala is not the contract; does not say whether a best-effort generator exists later |
| **U12 Interpreter vs compiled native sim** | `13.6.1`, `18.4.3` | **AD-5** requires `tick` from HIR; execution engine (interp vs codegen) unnamed |
| **U13 `Synthesizable` float crate / `rhdl-float`** | `15.6` | Extension of synthesizable subset |

---

## 4. Quiet constraints that *almost* landed

Tone the `AD` structure flattened. Not full drops if a nearby `AD` already prevents the fork — listed so Finalize can tighten wording if desired.

| Quiet constraint | Nearby landing | Residual risk |
| --- | --- | --- |
| 「编译期检查前置」— fail before synthesis/sim | Checks moved to `elaborate`/`freeze` (**AD-1**, **AD-4**) + structured diagnostics (**AD-10**) | Latch / synth-subset / undriven (D1, D2, D6) still homeless |
| 「显式优于隐式」 | CDC/multi-clock Deferred; comb/seq only in capability map | Phase-1 implicit clock (D4) still open |
| 功能视图永不进 FIRRTL / Verilog | **AD-3** + **AD-5** | Needs an explicit “functional fields stripped before freeze” if D2 is not added |
| 互转是**模块级 IP**，不是设计迁移 / 生成器还原 | **AD-3** | OK |
| 用户依赖 `rhdl::prelude::*` | **AD-2** `rhdl-rs` + **AD-6** design crate → prelude | Unstated whether `rhdl-rs` re-exports prelude or users depend on `rhdl-prelude` directly |
| 阶段一类型：`Bool`/`Bits`/`UInt`/`SInt`/`Clock`/`Reset`（路线图 19.3）；Analog/Mem 靠后 | Capability map “类型” | Seed-level unless Analog/Mem stay unnamed (U1/U2) |
| 仿真「事件驱动」心智图 vs `tick()` API | **AD-5** cycle-accurate `tick` | Roadmap 19.4 mindmap “事件驱动” is overridden; ch.14 already describes tick + comb-then-seq |
| HIR 序列化缓存供 `generate` 复用 | Overturned by **AD-1** (run the generator) | Do not revive a rustc-side HIR cache as a contract |
| 诊断语言：需求示例为英文 | Convention: `rhdl::E0xxx` 英文码 + 中文说明 | Spine addition, not a requirements gap |
| Yosys/Vivado/Quartus/Verilator/Icarus/GTKWave 为外部可选 | **AD-11** Yosys/Verilator optional; stack pins Yosys 0.68 / Verilator 5.050 | Vivado/Quartus unnamed — fine |

---

## 5. Roadmap vs spine (phase table)

Requirements `19.` vs what the spine actually binds.

| Req phase | Req payload | Spine |
| --- | --- | --- |
| 一 | types, `#[module]`, comb/seq, simple Verilog, width/direction checks | Paradigm + **AD-8** Verilog; width/direction in **AD-4**; comb/seq/latch **not** bound (D1) |
| 二 | generics, `#[connect]`, native sim + VCD, FIRRTL *emit*, multi-drive | FIRRTL emit phase 2 (**AD-8**); unique-drive at freeze already (**AD-4**); VCD unnamed (D8); connect completeness unnamed (D6) |
| 三 | ClockDomain, CDC, DoubleFlop/SyncFIFO, `#[assert]`, SVA | CDC / multi-clock / formal **Deferred** |
| 四 | FIRRTL import, FIRRTL→Chisel, HIR→RHDL source, `import` CLI | Import in **AD-3**; Chisel Scala **not** the contract; HIR→RHDL source allowed as debug; Chisel *generator tool* under-deferred (U11) |
| 五–六 | HLS, IP crates, viz, LSP, docs | Named **Deferred** |
| 七 | `#[abstraction]`, bridges, consistency framework, `build-sim` | Split pulled to **AD-5**; bridges/mixed/C-ABI/formal-equiv **under-deferred** (U3–U5) |

---

## 6. Verdict

The spine captured the **divergent-paradigm** calls (runtime elaboration, publish name, HIR↔FIRRTL text, freeze unique-drive, dual-model without TLM lowering, downward deps, firtool pin, local CLI).

What did **not** land is mostly **language-safety tone** the requirements treat as non-negotiable even in phase 1: explicit comb/seq and latch rejection, synthesizable-subset gating, explicit clocks, direction wrappers, undriven-port completeness, and the strict-width vs FIRRTL-widen clash.

Later-phase product (bridges, C shlib, Analog/InOut/Mem, waveforms, coverage, black-box, CLI verbs) is largely **intentional later work** that is only **half-listed** in `Deferred`.
