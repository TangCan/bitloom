# Reconcile: docs/requirements overview → PRD §9.2 / FR46–FR52

- **Input:** `docs/requirements/1. 项目概述.md`
- **Against:** `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md` §9.2 + §5.7 FR46–FR52 (and cited baseline FR28/29/30/35/37/38 where §9.2 joins them)
- **Job:** whether overview design goals / §1.5 highlights are covered by the 2026-08-21 literal升格. No rewrite of either source; overview intentionally **not** edited this pass.
- **Lens:** gap-against-intent (unmapped or weaker/stronger contract mismatch). **RHDL vs Bitloom** wording drift is **deferred**, not a gap-against-intent.

This note is a gap report, not a rewrite.

---

## How to read this

| Bucket | Meaning |
| --- | --- |
| Landed | §9.2 row + FR body match overview intent (strength may be PRD-hardened) |
| Deferred wording | Brand / crate naming still says RHDL; contract is Bitloom — do not treat as intent hole |
| Gap | Overview load-bearing claim missing, contradicted, or over/under-stated vs FR46–52 / §9.2 |

---

## Deferred wording (not gap-against-intent)

Overview still brands the product **RHDL** everywhere (§1.1 name, diagrams, §1.6 summary) and shows example IP crates `rhdl-uart` / `rhdl-axi` (§1.5.4). PRD identity is **Bitloom** / `bitloom` / `bitloom-prelude` (FR21; FR48). PRD §0 / §7 already record: overview file unedited this revision; contract supersedes naming. Track as a later overview rewrite task — **not** an FR46–52 coverage miss.

---

## Landed (§9.2 → overview)

| 概述条目 | §9.2 FR | Overview gist | PRD landing |
| --- | --- | --- | --- |
| §1.3.1–6 类型/所有权/comb-seq/参数化/Cargo/可综合 | 阶段一 + FR22 等 | Strong typing, ownership single-drive, explicit comb/seq, generics/macros, Cargo/tests, synthesizable subset | Baseline; out of FR46–52 scope |
| §1.3.7 / §1.5.3 Chisel 双向 | FR28, **FR46** | FIRRTL bridge; RHDL↔Chisel bidirectional; mixed design | FR46: maintainable Chisel Scala + import surface/FrozenHir + round-trip predicates + mixed fixture |
| §1.3.8 / §1.5.4 HLS | FR35, **FR50** | Algorithm-level Rust → RTL; `#[hls]` in diagram | FR50: product path + CI fixture; external scheduler allowed |
| §1.3.9 / §1.5.4 IP 库 | FR37, **FR48** | UART / SPI / I2C / FIFO / AXI | FR48: all five instantiable under prelude; black-box retained |
| §1.3.10 可视化 | FR38, **FR49** | 模块层次图、时序图 | FR49: hierarchy + timing (or equivalent interactive) from product entry |
| §1.3.11 / §1.5.5 多视图与双模拟器 | FR29, FR30, **FR47** | Functional + cycle-accurate views; generate both sims; bridge + equivalence | FR47: CLI/API **generate** both artifacts; FR30联验 |
| §1.5.1 所有权防多驱动 | 阶段一 FR15 | Ownership / borrow → single driver | Baseline |
| §1.5.2 ClockDomain / 显式 CDC | FR23, FR24, **FR52** | ClockDomain binds clk/reset polarity/sync·async; explicit synchronizer | FR52: docs+fixtures + freeze fail without sync |
| §1.3.4 复合参数化缺口 | **FR51** | Generics / const generics / macros for reusable HW (no Bundle/Vec by name) | FR51: Bundle / `Vec<T,N>` (or equiv.) elaborate/emit/tick |

**Missing §1.3.7–11 / §1.5.1–5 → FR:** none for the升格 set. Every §9.2 overview-literal row has a FR body.

---

## Gaps

### G1 — §1.5.3 import regenerates product **source**; FR46 + NFR10 forbid that as interop contract

- **Overview:** FIRRTL→RHDL 导入器 → HIR → **生成 RHDL 源码**; round-trip drawn as source↔source.
- **PRD:** FR46 success allows Chisel/`.fir` → **可编辑模块表面或 FrozenHir** then emit; NFR10: HIR→源码再生仍可仅调试，**不得**冒充 Chisel 双向合同.
- **Drop:** epics can meet FR46 without overview’s productized source regen; overview readers still expect RHDL/Bitloom **source** round-trip as the interop climax.

### G2 — §1.2 functional sim via ordinary Rust compile vs FR47 **generation**

- **Overview §1.2 flowchart:** `A -->|普通 Rust 编译| G[功能模拟器]` (functional path = normal Rust build).
- **Overview §1.5.5:** toolchain **分别生成**功能与周期精确模拟器 (aligned with FR47).
- **PRD FR47:** CLI/API must **生成**功能模拟器工件与周期精确模拟器工件.
- **Drop:** overview self-conflicts; a reader of §1.2 alone can treat FR47’s generated functional-sim path as optional. Intent for升格 is §1.5.5 / FR47, but §1.2 still understates the contract.

### G3 — Mindmap「波形导出」not in §9.2; FR49 is 层次+时序 only

- **Overview §1.3 mindmap (可调试性):** 可视化层次图、**波形导出**、形式验证接口.
- **§9.2:** maps §1.3.10 可视化 → FR38, FR49 only (层次图 + 时序图). Waveform remains baseline **FR31**; formal **FR39** — neither appears in the 2026-08-21 概述→FR table.
- **Drop:** SM-6 / 概述字面闭环 can ship FR49 without closing overview’s co-listed「波形导出」under the same升格 narrative; traceability for that mindmap bullet is outside §9.2.

### G4 — Core pipeline still「过程宏展开 → HIR」at compile time

- **Overview §1.1–1.2:** process macros expand to HIR; 「编译期完成硬件检查与优化」; diagrams show macro → HIR → backends (including functional sim branching from source).
- **PRD glossary / product verbs:** elaborate → **FrozenHir** → emit / tick; FR46–52 assume that model.
- **Drop:** FR46–52 close vision bullets but do not correct the overview’s primary IR/lifecycle story. Implementers following overview alone can target rustc-time netlist extraction — a known spine non-goal — while still claiming §9.2「已覆盖」for §1.3.1–6.

### G5 — Unqualified「AXI」vs FR48 default AXI4-Lite

- **Overview §1.3.9 / §1.5.4:** IP list includes bare **AXI** (and diagram crates `rhdl-axi`).
- **PRD:** FR48 lists AXI as a category; §8 Q7 **[OPEN]** with default `[ASSUMPTION]=AXI4-Lite 从设备级最小从接口`.
- **Drop:** overview literal reads as full AXI family; acceptance can stop at AXI4-Lite without an overview amendment or an explicit FR48 scope line readers of 概述 would recognize.

---

## Not gaps (recorded so they are not re-opened)

- **RHDL vs Bitloom / `rhdl-*` crate labels** — deferred wording; see above.
- **FR51 Bundle/Vec not named in overview** — PRD fills §1.3.4 parameterization gap; overview→FR strength increase, not a dropped overview claim.
- **FR50 / FR35 external HLS scheduler** — overview「计划集成 HLS」does not require in-tree scheduler; matches PRD assumption.
- **FR47 Rust functional crate vs SystemC (OQ-6)** — overview functional view is ordinary Rust / 事务级; default assumption matches.
- **FR46 idiomatic vs mechanical Chisel (OQ-5)** — overview does not demand idiomatic Scala; open question is epic sizing, not overview hole.
- **§1.4 Verilog delivery / FPGA flow** — stage-one / emit backends; not FR46–52.
- **Overview file unedited** — explicit PRD assumption; gaps above are for a future overview sync task, not silent PRD incompleteness on the升格 IDs themselves (except G3 table omission and G1/G2/G4/G5 mismatches).

---

## Compact (for parent)

**Input:** `docs/requirements/1. 项目概述.md`

**Gaps:**

1. §1.5.3 productizes import→**source** regen; FR46+NFR10 allow surface/FrozenHir only (debug regen ≠ interop).
2. §1.2 functional sim = ordinary Rust compile; FR47 requires **generated** functional-sim artifacts (§1.5.5 agrees; §1.2 understates).
3. Mindmap「波形导出」absent from §9.2; FR49 is hierarchy+timing only (waveform stays FR31).
4. Overview still teaches compile-time macro→HIR pipeline; PRD product model is elaborate/FrozenHir — unfixed by FR46–52.
5. Overview bare「AXI」wider than FR48 default AXI4-Lite (OQ-7).

**File:** `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/reconcile-overview.md`
