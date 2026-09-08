# D1 Requirements Capability Inventory

**Decision:** Gap between `docs/requirements` (incl. 受控泛型闭包) vs implementation vs PRD/Epic; produce full FR implementation plan.  
**Dimension:** D1 Requirements baseline — decision-grade capability inventory.  
**Evidence firewall:** only `docs/requirements/**/*.md` (this run).  
**Accessed:** 2026-09-08.  
**Source set:** `RHDL 方案大纲（最终版）.md` + chapters `1`–`19`.

**Cap-ID count:** 72  
**Closure-related (yes):** 26 · **partial:** 10 · **no:** 36

---

## How to read this inventory

| Field | Meaning |
| --- | --- |
| Cap-ID | Stable inventory ID (`Cap-R-NN`) — **not** a PRD FR ID |
| Bucket | `(A)` pre-existing RHDL capability (present before / independent of 泛型闭包 integration) · `(B)` NEW or materially changed by 受控泛型闭包 integration |
| Closure? | `yes` / `no` / `partial` |
| Phase | From doc **19** when stated; else `unspecified` or inferred note |
| Acceptance | Decision-grade, testable statement paraphrased from requirements |

Roadmap phases (doc 19): **P1** core language · **P2** param/hierarchy/sim + generator closures · **P3** clock/CDC/formal · **P4** Chisel interop · **P5** HLS/IP/viz + HLS/IP closures · **P6** eco/IDE · **P7** multi-view + bridge closures.

---

## (A) Pre-existing RHDL capabilities

Capabilities described as core RHDL design independent of the 泛型闭包 “optimization” layer, or present in the baseline language/toolchain before closure integration (outline note: closures were *added/adjusted* onto an existing scheme).

| Cap-ID | Short name | Source | Acceptance-oriented statement | Closure? | Phase |
| --- | --- | --- | --- | --- | --- |
| Cap-R-01 | Embedded DSL / process macros | `1. 项目概述.md` §1.1–1.2; `3. 核心设计原则.md` §3.2; `4. 整体架构与工作流.md` §4.1 | Hardware is legal Rust annotated with `#[module]` / `#[combinational]` / `#[sequential]`; macros expand to HIR without a separate HDL parser. | no | P1 |
| Cap-R-02 | Strong hardware typing | `2. 设计目标.md` §2.2; `5. 类型系统.md` §5.2 | Compile-time width, direction, and (later) clock-domain attributes are encoded in types; mismatches fail at `rustc` time. | no | P1 (width/dir); P3 (CDC) |
| Cap-R-03 | Base hardware types | `5. 类型系统.md` §5.2; `19. 实施路线图.md` §19.3 | Provide `Bool`, `Bits<N>`, `UInt<N>`, `SInt<N>`, `Clock`, `Reset` (and `Analog` for top-level IO per §5.2) with defined ops. | no | P1 (Analog not listed in P1 deliverables — see contradictions) |
| Cap-R-04 | Direction wrappers | `5. 类型系统.md` §5.3; `6. 模块与端口.md` §6.2 | Ports use `Input<T>` / `Output<T>` / `InOut<T>`; illegal direction connections fail compile-time. | no | P1 |
| Cap-R-05 | Composite types Bundle/Vec/Mem | `5. 类型系统.md` §5.4; outline §5 | Support `Bundle`, fixed `Vec<T,N>`, `Mem<T,DEPTH>` on the hardware path with expansion to signals/memory. | no | P1–P2 (composites appear with language; Mem use grows with hierarchy) |
| Cap-R-06 | Ownership single-driver | `1. 项目概述.md` §1.5.1; `2. 设计目标.md` §2.3; `3. 核心设计原则.md` §3.4 | A signal may have only one mutable driver; multiple drivers → compile error (`multiple drivers`). | no | P2 (explicitly listed under P2 checks); design goal from start |
| Cap-R-07 | Comb/seq explicit split | `2. 设计目标.md` §2.4; `7. 组合逻辑.md` §7.1; `8. 时序逻辑.md` §8.1 | Combinational and sequential logic are declared separately; incomplete comb assignment → latch error. | no | P1 |
| Cap-R-08 | Combinational operators & control | `7. 组合逻辑.md` §7.3–7.5 | Map arithmetic/bitwise/shift/compare/`if`/`match`/static loops to hardware; enforce full assignment. | no | P1 |
| Cap-R-09 | Wire internal nets | `7. 组合逻辑.md` §7.6; `6. 模块与端口.md` §6.3 | Support internal `Wire` for intermediate comb nets distinct from ports/regs. | no | P1 |
| Cap-R-10 | Register abstraction Reg | `8. 时序逻辑.md` §8.2–8.3 | `Reg<T>` with `q`/`d` semantics updated only in `#[sequential]`. | no | P1 |
| Cap-R-11 | FSM via enum+match | `8. 时序逻辑.md` §8.5 | State machines expressed with Rust enums + match in sequential blocks with configurable encoding. | no | P1 |
| Cap-R-12 | Module struct + ports | `6. 模块与端口.md` §6.1–6.2 | Modules are `#[module]` structs; ports/internal signals are typed fields. | no | P1 |
| Cap-R-13 | Hierarchy & connect | `9. 层次化与连接.md` §9.2–9.4; `19` §19.4 | Instantiate submodules; `#[connect]` validates direction, width, completeness. | no | P2 |
| Cap-R-14 | Const-generic parameterization | `10. 参数化与生成.md` §10.2; `19` §19.4 | Modules parameterized by `const` generics (width/depth) specialize at compile time. | no | P2 |
| Cap-R-15 | Type-generic parameterization | `10. 参数化与生成.md` §10.3 | Modules parameterized by hardware type parameters under trait bounds. | no | P2 |
| Cap-R-16 | Function hardware generators | `10. 参数化与生成.md` §10.4 | Functions/methods return module instances / recursively build structure (without requiring closures). | no | P2 |
| Cap-R-17 | Proc-macro codegen | `10. 参数化与生成.md` §10.5; `4` §4.3 | Custom/process macros drive batch codegen and HIR construction during `cargo build`. | no | P1–P2 |
| Cap-R-18 | const fn compile-time compute | `10. 参数化与生成.md` §10.6 | `const fn` computes parameters/LUT init without closures. | no | P2 |
| Cap-R-19 | ClockDomain object | `11. 时钟域与复位.md` §11.2; `19` §19.5 | Bind clock, reset, polarity, sync/async kind into `ClockDomain`; regs bind explicitly. | no | P3 |
| Cap-R-20 | Sync/async reset semantics | `11. 时钟域与复位.md` §11.3–11.4 | Emit correct sync vs async reset behavior from domain config. | no | P3 |
| Cap-R-21 | CDC checks + primitives | `11. 时钟域与复位.md` §11.6; `19` §19.5 | Forbid unsynchronized CDC; provide `DoubleFlop` / `SyncFIFO` (and related) primitives. | no | P3 |
| Cap-R-22 | Clock gate / enable | `11. 时钟域与复位.md` §11.7 | Support clock gating / enable modeling under explicit domain rules. | no | P3 |
| Cap-R-23 | HIR as single IR | `3. 核心设计原则.md` §3.5; `4` §4.2; `13` §13.3 | All synth backends consume one HIR; backends share semantics. | no | P1 |
| Cap-R-24 | HIR↔FIRRTL alignment | `3` §3.5; `12. 与 Chisel 的互转设计.md` intro; `13` §13.5 | HIR semantics align with FIRRTL for export/import. | no | P2 (FIRRTL export); P4 (import/round-trip) |
| Cap-R-25 | Reversible subset | `3` §3.6; `12` (可逆子集); outline §12 | Define which constructs can round-trip via FIRRTL to regenerable RHDL/Chisel. | partial | P4 |
| Cap-R-26 | Verilog/SV backend | `13` §13.4; `19` §19.3 | Generate synthesizable Verilog-2001 / SV-2017 from HIR. | no | P1 |
| Cap-R-27 | FIRRTL backend | `13` §13.5; `19` §19.4 | Emit FIRRTL text from HIR for Chisel/`firtool` tooling. | no | P2 |
| Cap-R-28 | Cycle-accurate simulator | `14. 验证与仿真.md` §14.2.1; `13` §13.6.1; `19` §19.4 | Native sim with `tick` / set/get I/O; comb then edge update; VCD. | no | P2 |
| Cap-R-29 | cargo test integration | `2` §2.6; `14` §14.3 | Hardware tests are Rust `#[test]` via Cargo. | no | P2 |
| Cap-R-30 | Waveform export | `14` §14.5; `13` §13.2.2 | Export VCD/FST; optional auto-dump on failure. | no | P2 |
| Cap-R-31 | Property / random testing | `14` §14.6 | Integrate `proptest`-style property tests against DUT. | partial | P2–P5 (API grows; closures enhance) |
| Cap-R-32 | Coverage hooks | `14` §14.7; `19` §19.7 | Record functional coverage during sim (extended in P5). | no | P5 |
| Cap-R-33 | Assertions / formal interface | `14` §14.8; `13` §13.8; `19` §19.5 | `#[assert]` / SVA export or SymbiYosys interface. | no | P3 |
| Cap-R-34 | Synthesizable subset enforcement | `15. 可综合子集与限制.md` §15.1–15.4; `2` §2.7 | Whitelist/blacklist Rust features for synth path; reject illegal constructs at compile time. | partial | P1 (baseline); closure rules change subset — see Cap-R-50 |
| Cap-R-35 | Chisel FIRRTL bridge | `12` whole; `19` §19.6 | RHDL↔FIRRTL↔Chisel at IR level; `cargo rhdl import` / `generate --format chisel`. | no | P4 |
| Cap-R-36 | Metadata retention | `12` (元数据); `3` §3.5 | Preserve names/params/source locs for readable reverse gen. | no | P4 |
| Cap-R-37 | HLS `#[hls]` frontend | `17. 扩展功能整合.md` §17.2; `2` §2.9; `19` §19.7 | Algorithm Rust → DFG → schedule/bind → HIR RTL. | partial | P5 |
| Cap-R-38 | IP crates ecosystem | `17` §17.3; `2` §2.10; `19` §19.7–19.8 | Publish IP as crates (`rhdl-uart`, `rhdl-fifo`, later SPI/I2C/AXI/GPIO). | partial | P5–P6 |
| Cap-R-39 | Visualization backend | `17` §17.4; `13` §13.7; `19` §19.7 | Hierarchy graphs (Graphviz/Mermaid) and timing diagrams from HIR/waves. | no | P5 |
| Cap-R-40 | Cargo CLI `cargo rhdl` | `13` §13.2; `4` §4.3.2 | Subcommands: generate, test, visualize, wave, check, import, doc, build-sim. | partial | P1–P7 (commands accrue) |
| Cap-R-41 | Multi-view modeling (core) | `18. 多视图建模与模拟器生成（核心优化）.md` §18.2; `3` §3.8; `19` §19.9 | One module may declare functional + cycle-accurate views (`#[abstraction]`, `#[functional_model]`). | partial | P7 (integration); described earlier as design |
| Cap-R-42 | Functional simulator path | `18` §18.4; `13` §13.6.2; `14` §14.2.2 | Build/run functional sim from ordinary Rust view (no HIR). | partial | P7 |
| Cap-R-43 | Consistency verification | `18` §18.6; `14` §14.9 | Random compare and optional formal equiv between views. | partial | P7 |
| Cap-R-44 | Layered abstraction RTL/HLS/IP | `3` §3.7; outline §3 | Mix algorithm / RTL / IP instantiation with clear boundaries. | no | P5+ |
| Cap-R-45 | Explicit-over-implicit philosophy | `3` §3.3 | Clock domain, direction, reset kind must be explicit. | no | P1–P3 |
| Cap-R-46 | Compile-time checks first | `3` §3.4 | Width, direction, multi-drive, CDC, latch checks before emit. | partial | P1–P3 (+ closure check as Cap-R-51) |

---

## (B) NEW / changed by 受控泛型闭包 integration

Marked **新增/修订/引入** in outline, ch.2–3, 5, 10, 15, 19, or described as closure-driven capability.

| Cap-ID | Short name | Source | Acceptance-oriented statement | Closure? | Phase |
| --- | --- | --- | --- | --- | --- |
| Cap-R-47 | Controlled-closure principle | `3` §3.9; outline §3; `RHDL 方案大纲` 说明 | Non-synth paths may use free Rust closures; synth paths only constrained closures that fully dissolve before HIR. | yes | Cross-cutting (policy); enforcement phased |
| Cap-R-48 | SynthesizableClosure trait | `5` §5.8; outline §5; `15` §15.4.3 | Define `SynthesizableClosure`: no runtime capture (const-only), `HardwareType` args/returns, synthesizable body, inlineable. | yes | Unspecified in gantt; risk R7 after generator (see contradictions) |
| Cap-R-49 | Synth-path closure compile check | `3` §3.4; `4` §4.3.1; `5` §5.8.3; `13` intro | Macros detect closures on synth path; fail with `unsynthesizable closure` + suggest functional view / named fn. | yes | After P1 “reserve macro hooks”; before full use |
| Cap-R-50 | Subset: allow constrained closures | `15` §15.2.7; outline §15; `2` §2.7 | Synth subset **adds** constrained closures; **revises** blanket closure ban to ban only non-inlineable / capturing closures. | yes | With Cap-R-48/49 |
| Cap-R-51 | Closure dissolve before HIR | `3` §3.5; `4` §4.2; `12` intro; `13` §13.3.1 | After processing, HIR contains **no** closure nodes; backends are closure-unaware. | yes | P2+ whenever closures exist |
| Cap-R-52 | Generator closures (behavior params) | `10` §10.7; outline §10; `19` §19.4 | Generators accept `Fn` closures executed at compile time to produce ROM/init/structure variants (LUT, FIR coeffs, ALU datapath). | yes | **P2** |
| Cap-R-53 | Module factory + closure | `6` §6.8; outline §6 | Factory fn takes closure params, returns `#[module]` instance with compile-time-customized internals. | yes | P2 (with generators) |
| Cap-R-54 | Hierarchy connect via generator closures | `9` §9.10.2–9.11; outline §9 | Closures batch-instantiate/connect arrays of children inside `#[connect]`; expand to fixed wiring. | yes | P2 |
| Cap-R-55 | Comb synthesizable closures | `7` §7.10; outline §7 | In `#[combinational]`, no-capture pure closures inline to comb logic for reuse (e.g. bit-reverse, CRC helper). | yes | Risk R7: after generator; **not** in P1 tasks |
| Cap-R-56 | Seq synthesizable closures | `8` §8.9; outline §8 | In `#[sequential]`, constrained closures compute next-state values; no extra regs; cannot capture `self`. | yes | Same as Cap-R-55 |
| Cap-R-57 | CDC sync strategy closures | `11` §11.1, §11.9; outline §11 | Optional closures customize synchronizer stages / reset filter logic at generate time. | yes | P3 (with CDC primitives) |
| Cap-R-58 | Interop unaffected by closures | `12` intro + 总结; outline §12 | Synth closures inlined; generator closures finished before FIRRTL — interop needs no special FIRRTL closure encoding. | yes | P4 (verify invariance) |
| Cap-R-59 | Macro triple-mode closure handling | `13` intro, §13.3.1; outline §13 | Classify: synth→inline/check · generator→execute · non-synth→keep for runtime. | yes | P2 (generator) then later modes |
| Cap-R-60 | `cargo rhdl check` includes closures | `13` §13.2.2 | Static check command covers closure synthesizability. | yes | With Cap-R-49 |
| Cap-R-61 | Test/bridge free closures | `14` intro, §14.3.3, §14.4, §14.6.2; outline §14 | Tests freely use closures for stimulus, asserts, property predicates, test helpers. | yes | P2+ (tests); bridge templates P7 |
| Cap-R-62 | HLS closures as DF transforms | `17` §17.2.5; `2` §2.9.2; `19` §19.7 | Stateless closures in `#[hls]` map/inline to comb or pipeline stages before schedule. | yes | **P5** |
| Cap-R-63 | IP generator closure customization | `17` §17.3.6; `2` §2.10.2; `19` §19.7–19.8 | IP builders take closures for algorithms (CRC poly tables, filter coeffs); execute at generate time. | yes | **P5** (UART/FIFO); **P6** (more IP) |
| Cap-R-64 | Viz closure-transparent | `17` §17.4.4 | Visualization consumes post-dissolve HIR; must not require closure awareness (func call graphs may show runtime closures via Rust tools). | yes | P5 |
| Cap-R-65 | Bridge adapter closure templates | `18` §18.5.2–18.5.3, §18.8.4; `2` §2.12.2; `19` §19.9 | Reusable `start_wait_complete`-style generic closures convert TLM↔signal protocols. | yes | **P7** |
| Cap-R-66 | Multi-view free vs constrained split | `5` §5.10; `3` §3.8; `15` §15.7; `18` §18.2.1 | Functional view: free closures; cycle-accurate: only constrained closures. | yes | P7 (+ earlier local rules) |
| Cap-R-67 | Functional-model free closures | `14` §14.2.2; `18` §18.2.1 | `#[functional_model]` methods may use dynamic data + free closures. | yes | P7 |
| Cap-R-68 | Consistency tests with closures | `14` §14.9; `18` §18.6 | Closure helpers wrap cycle-accurate transaction driving during random compare / co-sim. | yes | P7 |
| Cap-R-69 | Generator vs const fn coexistence | `10` §10.7.6 | Documented alternative: const fn vs generator closures; both valid compile-time parameterization styles. | yes | P2 |
| Cap-R-70 | Closure ownership compatibility | `3` §3.9.2; `7` §7.10.5 | Synth closures must not introduce extra mutable signal borrows; checker validates ownership inside body. | yes | With Cap-R-55/56 |
| Cap-R-71 | HLS path constraint coupling | `15` §15.5; `17` §17.2.5 | HLS-used transform closures must meet synth constraints and dissolve before HIR. | yes | P5 |
| Cap-R-72 | Phased closure rollout | `19` intro, §19.2, §19.4, §19.7, §19.9, §19.11 R7 | Implement generator closures (P2) → HLS/IP closures (P5) → bridge framework (P7); risk plan: generator before synthesizable closures. | yes | P2 → P5 → P7 (+ synth after generator) |

---

## Closure-related capability index (yes / partial)

### Yes (26)

Cap-R-47 … Cap-R-72 (all bucket B). Primary phase anchors: Cap-R-52 (P2 generator), Cap-R-62/63 (P5 HLS/IP), Cap-R-65 (P7 bridge).

### Partial (10)

| Cap-ID | Why partial |
| --- | --- |
| Cap-R-25 | Reversible subset **includes** inlined constrained closures; capturing closures irreversible |
| Cap-R-31 | PBT exists without closures; closures are an enhancement |
| Cap-R-34 | Subset exists pre-closure; membership list **changed** by Cap-R-50 |
| Cap-R-37 | HLS exists; closure DF transforms are the closure delta |
| Cap-R-38 | IP crates exist; closure customization is the delta |
| Cap-R-40 | CLI exists; `check`/sim paths gain closure modes |
| Cap-R-41 | Multi-view design; bridge/free-vs-constrained split is closure-touched |
| Cap-R-42 | Functional sim path; free closures allowed in view code |
| Cap-R-43 | Consistency framework; closure helpers used in compare/co-sim |
| Cap-R-46 | Compile-time checks exist; synthesizable-closure check is additive |

---

## Roadmap phase map (closure-heavy)

| Phase | Closure-related work per doc 19 | Caps |
| --- | --- | --- |
| P1 | No closures; reserve macro expansion hooks | Cap-R-01..12, 23, 26; hooks for 49/59 |
| P2 | **Generator closure basics**; hierarchy; sim; FIRRTL | Cap-R-52, 53, 54, 51, 59 (generator mode), 69 |
| P3 | CDC; optional sync-strategy closures | Cap-R-19..22, 57 |
| P4 | Interop; closures already eliminated | Cap-R-35, 36, 58 |
| P5 | **HLS DF closures**; **IP generator closures**; viz | Cap-R-62, 63, 64, 37–39 |
| P6 | More IP with closure customization | Cap-R-63 extended |
| P7 | **Bridge adapter closure framework**; multi-view sim | Cap-R-65–68, 41–43 |
| Unphased / R7 | Constrained synth closures in comb/seq | Cap-R-48, 50, 55, 56, 70 |

---

## Contradictions & ambiguities (requirements-internal)

1. **Principle count:** `3. 核心设计原则.md` intro says “七项核心原则（…新增受控闭包抽象原则）” then §3.10 says “这八项原则” — multi-view was also added; counting is inconsistent.

2. **HLS closures free vs constrained:** `3` §3.9.1 diagram routes “HLS 数据流变换” to **完全自由** non-synth handling; `15` §15.5 and `17` §17.2.5 require HLS transform closures to satisfy **可综合闭包** constraints and inline before HIR. **Conflict on constraint class.**

3. **When synthesizable (comb/seq) closures ship:** Doc 19 P1: “初始阶段不涉及闭包”; R7: “先支持生成器闭包，再支持可综合闭包”; chapters 7–8 specify comb/seq closures as language features. **No explicit phase for Cap-R-55/56** despite being core language surface.

4. **Multi-view timing vs narrative:** Docs 1–18 treat multi-view as a first-class design pillar; doc 19 places integration in **P7** only. Early chapters still write APIs (`#[functional_model]`, dual sim) as if present — **capability vs delivery phase ambiguity**.

5. **Analog in P1:** Type system requires `Analog` (`5` §5.2); P1 task list omits it.

6. **Generator closure “freedom”:** Generator/IP/bridge closures are “non-synth / free” yet must be compile-time executable, pure, hardware-typed returns (`6` §6.8.3, `10` §10.7.5) — **“free” ≠ unconstrained**.

7. **Package/CLI names:** Requirements consistently use `rhdl` / `cargo rhdl` / `rhdl-uart` (e.g. `13` §13.2, `19` §19.7). Branding elsewhere is out of firewall; **within** requirements, naming is stable as `rhdl` (note for later contract gap vs Bitloom).

8. **Reversible subset vs generator closures:** Constrained closures are reversible *after inline*; generators are “元编程 / 与可逆性无关” (`12`). Acceptance of “round-trip preserves generator intent” is **explicitly out of scope** — easy to misread as full source-level reversibility.

9. **Formal/panic synthesizability:** `15` §15.3.7 says `panic!` may be ignored or forbidden on synth path — **underspecified**.

10. **Trait naming:** `SynthesizableClosure` (`5` §5.8) vs `Synthesizable` (`15` §15.4.3) — relationship (blanket impl? marker?) not fully specified.

---

## Evidence notes (outline as index)

Outline `RHDL 方案大纲（最终版）.md` §说明 states the integration deltas: new principle 受控闭包抽象; type constraints; applications in parameterization / HLS / multi-view bridging; revised synthesizable subset; adjusted roadmap. Chapters 10, 17, 18, 19 are the primary closure application specs; 5, 7, 8, 15 define constraints; 12–13 define dissolve-before-HIR invariant.

---

## Cap-ID summary

| Metric | Value |
| --- | --- |
| Total Cap-IDs | **72** |
| Bucket (A) | Cap-R-01 … Cap-R-46 (46) |
| Bucket (B) | Cap-R-47 … Cap-R-72 (26) |
| Closure yes | **26** |
| Closure partial | **10** |
| Closure no | **36** |
