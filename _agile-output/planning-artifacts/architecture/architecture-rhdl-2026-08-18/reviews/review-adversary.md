# Adversarial Review — Architecture Spine (rhdl)

- **Artifact:** `ARCHITECTURE-SPINE.md` (architecture-rhdl-2026-08-18)
- **Lens:** adversarial — one-level-down incompatibility
- **Date:** 2026-08-18
- **Constraint honored:** this review reads only the spine; no companions, research, or requirements were consulted
- **Verdict:** **FAIL** — the spine does not close a build-substrate. Two units one level down can obey every AD to the letter and still not compose.

## Attack method

An AD is closed if there is only one legal way to share a datum, own an entity, or mutate state. The attack is: for each shared thing the crates must jointly use, construct **Unit A** and **Unit B** (epics or crates) such that:

1. neither violates any ADOPTED rule as written;
2. their shared-data shapes, owners, or mutation paths cannot be linked.

Every such pair is a hole. Close it with a new AD or by tightening the existing one until one of the two units becomes illegal.

**Out of scope as binding law:** Design Paradigm prose, Consistency Conventions, Stack versions, Structural Seed, and the Capability map. Only `### AD-N — … [ADOPTED]` rules were treated as constraints. Where those non-AD sections contradict or over-specify an AD, that itself is a hole (conventions that do not bind will be ignored by one epic and treated as law by another).

---

## Constructed pairs

### Pair 1 — Two `Hir` shapes (shared-data)

**Unit A — epic `hir-as-firrtl-ast` (crate `rhdl-hir`).**  
`Hir` is a hierarchical FIRRTL-6 AST: `Circuit { name, modules: Vec<Module> }`, `Module { ports, body: Vec<Stmt> }`, `Stmt` ∈ {Connect, DefNode, DefRegister, DefRegisterWithReset, Instance, Conditionally}. Types include `UInt`, `SInt`, `Clock`, `Reset`, and `Bundle`. `freeze()` walks Connects for multi-drive and width/direction (AD-4). `elaborate()` in the generator process is the only builder of this AST (AD-1). Backends are `fn(&Hir) -> Artifact` (AD-7). Node meanings are “the FIRRTL 6.0.0 reversible subset” (AD-8).

**Unit B — epic `hir-as-flat-netlist` (crates `rhdl-vlog`, `rhdl-sim`).**  
These crates consume `Hir` as `{ nodes: Vec<Op>, driver: Vec<Option<NodeId>>, width: Vec<u32> }` with **no Instance** (flatten at elaborate). Types are only `{width: u32}`. `freeze()` is “each `NodeId` has ≤1 driver” (AD-4). Emitters remain pure `&Hir -> String` (AD-7). Alignment to FIRRTL 6 is a comment mapping `Op::And` → `and`, `Op::Reg` → `reg` (AD-8 letter: 节点语义对齐, not “HIR *is* the FIRRTL AST”).

**ADs obeyed:** AD-1, AD-4, AD-7, AD-8, and every other AD (neither unit publishes as `rhdl`, neither panics on user input, neither depends upward, etc.).

**Clash:** there is no shared `Hir` type. Verilog and sim written against B cannot take A’s AST; a later `rhdl-firrtl` written against A cannot import into B’s netlist. AD-8’s “align to FIRRTL spec 6.0.0 reversible subset” is not a schema: FIRRTL itself has both statement modules and expression trees; either unit can claim alignment.

**Close with AD-12 (HIR schema):** Freeze the HIR data model in `rhdl-hir` as the FIRRTL 6.0.0 reversible-subset AST (hierarchical modules, statements, ground types listed by name). Flattening, if any, is a backend-private view, not a second `Hir`.

---

### Pair 2 — Two owners of the circuit entity (builder graph vs `Hir`)

**Unit A — crate `rhdl-builder`.**  
Owns `CircuitBuilder`, `Wire`, `Reg`, `Mod`. `Wire` is `Rc<Cell<…>>` (cloneable) because AD-4 says Rust ownership **不是**声音性证明 — so move-only `Wire` is optional. `elaborate()` dumps the builder graph into public `hir::Hir` constructors, then calls `freeze()`. During the dump, the builder graph is a fully-formed circuit that freeze never sees.

**Unit B — crate `rhdl-hir`.**  
Owns the only circuit types. `elaborate()` is `hir::elaborate`. `Hir` node constructors are `pub(crate)`. Builder is a thin facade over `&mut Hir`. `Wire` is move-only (allowed, not required, by AD-4).

**ADs obeyed:** AD-1 (HIR is built in `elaborate()`), AD-4 (freeze still rejects multi-drive; A’s cloneable `Wire` is explicitly permitted), AD-7 (only `elaborate()` mutates **Hir**; A’s `CircuitBuilder` is not Hir, so AD-7 does not apply to it), AD-6 (builder → hir). Capability map *prose* puts 类型/模块 in prelude/builder/macro and 多驱动 in hir freeze — that split licenses Unit A.

**Clash:** two live owners of “the circuit” during elaboration. Macros (see Pair 8) expand to one API or the other. Freeze can pass on a dump that dropped a second driver that still exists on the builder graph — or the opposite, if dump invents nodes the builder never had. Design crates against A’s `Wire` do not compile against B.

**Close with AD-13 (single circuit owner):** The circuit entity is `Hir` and only `Hir`. Builder holds a session token / `&mut Hir`, not a parallel graph. No `Module`/`Wire` type may survive past `elaborate()` except as Hir nodes. Macros expand only to the builder API, never to `Hir` constructors.

---

### Pair 3 — Two mutation windows (`elaborate` vs `freeze` vs leftover `&mut Hir`)

**Unit A — type-flag `Hir`.**  
`fn elaborate() -> Hir` returns an unfrozen graph. `fn freeze(&self) -> Result<(), Diag>` sets a boolean. AD-7’s signature is literally `&Hir -> Artifact`, so backends accept the **same type** before and after freeze. `Hir` is `Clone`. A caller clones before freeze, freezes one copy, mutates the other — the mutable copy was never “after freeze”. Design crates may depend on `rhdl-hir`: AD-6 forbids 设计 crate → CLI/后端, and **Hir is not a backend**. So user code can mutate Hir outside `elaborate()`.

**Unit B — type-state.**  
`elaborate()` returns `FrozenHir`. Unfrozen Hir never leaves `rhdl-hir`. Backends take `&FrozenHir`. Consume-freeze: `freeze(self) -> Result<FrozenHir, Diag>`.

**ADs obeyed:**  
- A matches AD-7’s **letter** (`&Hir`, not `&FrozenHir`) and AD-1 (construction still happens in `elaborate()`).  
- B matches AD-7’s **intent** (freeze 之后不可变) and AD-1 (construction + freeze as one step).  
AD-1 and AD-7 never say that unfrozen Hir must not escape, nor that freeze is part of `elaborate()`, nor that design crates cannot depend on `rhdl-hir`.

**Clash:** CLI/tests written for A call `elaborate(); freeze(); emit(&hir)`. CLI/tests written for B call `emit(&elaborate()?)`. A’s backends will accept unfrozen graphs (multi-drive lowering is undefined). B’s `FrozenHir` is a different type. Two mutation paths: (1) builder during elaborate, (2) public `&mut Hir` / `Clone` after elaborate returns.

**Close with AD-7 tightened:** Unfrozen Hir is crate-private. `elaborate()` returns `Result<FrozenHir, Diagnostics>` and is the only function that mutates. `freeze` is a private last step of `elaborate`, not a public second window. Backends are `fn(&FrozenHir) -> Artifact`. Design crates must not depend on `rhdl-hir`.

---

### Pair 4 — Two HIR birth paths (AD-1 `elaborate` vs AD-3 import)

**Unit A — `rhdl-firrtl` parser.**  
`fn import(text: &str) -> Hir` builds Hir **not** by calling `elaborate()`. Justification: AD-1’s **Prevents** clause is only “rustc 编译期抽 HIR vs 独立生成器进程”; import is neither. AD-3 **requires** HIR ↔ FIRRTL 6.0.0 text, including 导入.

**Unit B — language/CLI epic.**  
The letter of AD-1 is “HIR 只在生成器进程的 `elaborate()` 中构建”. Import is therefore FIRRTL → generated Rust source → `cargo rhdl build` → `elaborate()`. AD-3 is satisfied by export + that round trip. Direct `parse → Hir` is illegal.

**ADs obeyed:** each unit satisfies one AD by interpreting the other as scoped by its Prevents/Binds line. The spine gives no resolution rule when two ADOPTED ADs collide.

**Clash:** two owners of “how HIR is born”. Parser-Hir preserves FIRRTL instance names, node names, and statement order. Re-elaborate-Hir goes through builder, mangles names, and may flatten. Round-trip tests, CLI `import` flags, and freeze obligations differ (does imported Hir go through freeze? AD-4 binds freeze but not import).

**Close with AD-1 tightened:** The only functions that may allocate `Hir` are (1) `elaborate()` on a design crate in the generator process, and (2) `rhdl-firrtl::import` of the AD-3 subset. Both must end in the same private `freeze` and return `FrozenHir`. No third constructor. State that AD-3 import is an explicit exception to “only elaborate”, not an informal one.

---

### Pair 5 — Two generator→backend handoffs (shared-data across the process boundary)

AD-1: HIR is built in the **generator process**.  
AD-6: 设计 crate 不得依赖 CLI 或后端. Backends live in the CLI process.  
Therefore FrozenHir must cross from a crate that must not link vlog/sim into a crate that does. The spine specifies neither the handoff nor the bytes.

**Unit A — JSON dump.**  
Design crate (deps: prelude only) is compiled as a bin. `elaborate()` serde-serializes a stringly module graph to stdout / `target/rhdl/top.hir.json`. CLI parses JSON and calls backends. Schema is Unit A’s invention (string names, widths as decimals).

**Unit B — CLI-generated shim crate.**  
`cargo rhdl build` writes a throwaway crate that depends on the design crate **and** `rhdl-vlog`/`rhdl-sim`. `elaborate()` + backends run in one rustc session. No serialization. `Hir` is the Rust type.

**ADs obeyed:** both keep HIR construction in a generator process (AD-1), both keep design.toml free of backend deps (AD-6: the shim is not the design crate), both backends are pure functions (AD-7), no cloud (AD-11).

**Clash:** CLI-A cannot run a B design; CLI-B cannot consume A’s JSON. The `Hir` in memory and the `Hir` on disk are different languages. Two owners of the “how backends receive Hir” protocol (CLI vs an implicit hir serde feature).

**Close with AD-14 (elaboration host):** `cargo rhdl build` generates a host/shim crate that depends on the design crate + backends, calls `elaborate() -> FrozenHir` in-process, then `emit`. Serialization of FrozenHir, if it exists, is a versioned schema owned by `rhdl-hir` and is not an alternative CLI protocol in phase 1.

---

### Pair 6 — Design-crate `tick` vs “no backend deps” (AD-1 ∩ AD-6)

AD-1: 设计 crate 的 `cargo test` 若要仿真，必须先 elaborate 再 `tick`.  
AD-5: 周期精确仿真只从冻结 HIR（原生 `tick`），and `tick` lives in `rhdl-sim`.  
AD-6: 设计 crate 不得依赖 CLI 或后端; mermaid places `rhdl-sim` next to vlog/firrtl under CLI; mermaid has no `DES → SIM`.

**Unit A — sim is not a “后端”.**  
“后端” means emitters (`rhdl-vlog`, `rhdl-firrtl`). Design crate `[dev-dependencies] rhdl-sim`. Tests call `elaborate(); tick()`. AD-6 letter forbids CLI and 后端, not sim.

**Unit B — sim is a backend.**  
Mermaid groups SIM with VLOG/FIR. Design crate must not depend on it. `cargo test` in the design crate only runs `#[functional_model]` (ordinary Rust, AD-5). Cycle-accurate `tick` lives in a sibling `*_sim_tests` crate or `cargo rhdl test` shim. AD-1’s “若要仿真” is read as “if you choose to tick in this crate”; they choose not to.

**ADs obeyed:** both readings are licensed by the text. “后端” is never defined.

**Clash:** two owners of the test-time sim API; prelude cannot re-export `tick` (that would be PRE → SIM, forbidden by the mermaid) unless sim is not a backend (A) or prelude grows a forbidden edge (illegal under B). IP crates (AD-5 **Binds:** rhdl-sim, 测试, IP) have the same unresolved dep.

**Close with AD-6 tightened:** Define 后端 = `{rhdl-vlog, rhdl-firrtl, rhdl-sim}`. Design crate `[dependencies]` may name only `rhdl-prelude`. Design crate `[dev-dependencies]` may additionally name `rhdl-sim` (and no other backend). Prelude must not depend on sim. `tick` is a public item of `rhdl-sim`.

---

### Pair 7 — Clock / reset / tick contract (shared sequential state)

AD-8: 阶段一单时钟. AD-5: `tick` from frozen HIR. No AD says whether clock/reset are HIR ports, implicit, sync, or async. FIRRTL 6 has `reg`, `regreset`, Clock, and async-reset variants; “对齐” does not pick one.

**Unit A — implicit clock.**  
`DefReg { width, init }` has no clock field. `tick(&Hir, &mut State)` = eval comb, then NBA-update all regs. Verilog emit **invents** `clk` at the module boundary. Reset is sync, active-high, also invented at emit, or omitted (init-only).

**Unit B — FIRRTL-faithful ports.**  
Every module has `clock: Clock` and `reset: UInt<1>` in HIR. `DefRegisterWithReset` references those nodes. `tick(clk, reset, ports)`. Verilog uses `always @(posedge clock)`.

**ADs obeyed:** both are single-clock (AD-8), both tick from frozen HIR (AD-5), both can claim FIRRTL-6 semantics (implicit clock is a subset; explicit ports are the spec). Phase-1 “no packed array / no automatic” is independent.

**Clash:** sim-A ignores ports that sim-B requires. Functional models (AD-5) disagree on whether `clock` is an argument. Verilog-A’s invented `clk` vs Verilog-B’s HIR port: filelists, testbenches, and dual-model comparison keys diverge. Three backends, three sequential contracts.

**Close with AD-15 (phase-1 sequential contract):** Every module has exactly one `Clock` port and one sync active-high `Reset` port, both present in FrozenHir. `Reg` is posedge + sync reset. `tick` is one posedge of that clock with reset sampled as a port. Verilog is `always @(posedge clock)`. No implicit ports added at emit.

---

### Pair 8 — Two writers of Hir nodes (macro → hir vs macro → builder)

AD-6 mermaid: `PRE → BLD`, `PRE → MAC`, `BLD → HIR`. **MAC has no outgoing edge.** Rule text: 宏不得依赖 vlog/firrtl/sim — it does **not** forbid `rhdl-macro → rhdl-hir`.

**Unit A — path-emitting macros.**  
`rhdl-macro` depends on nothing. Expansion is tokens calling `rhdl_builder::…`. Builder is the only mutation path into Hir.

**Unit B — hir-constructing macros.**  
`rhdl-macro` depends on `rhdl-hir`. Expansion is `Hir::add_node` / Connect statements. Builder is unused for macro-originated hardware. Still “HIR built in `elaborate()`” if the expanded code runs inside `elaborate()`.

**ADs obeyed:** AD-6 as written, AD-1, AD-10 (both return `compile_error` on bad input).

**Clash:** two mutation paths into the same entity; two naming/span conventions; prelude cannot be “设计 crate 唯一依赖” if macros inject `rhdl_hir::` paths the design crate did not declare (or, conversely, Unit B adds a hidden hir dep via the macro crate’s re-exports). Comb/seq regions (capability map: 类型/模块/comb·seq live in prelude, builder, **macro**) may exist only in A’s builder API and be absent from B’s Hir.

**Close with AD-6 / AD-13:** `rhdl-macro` may depend on `rhdl-builder` only (or on nothing, emitting builder paths). It must not depend on `rhdl-hir`. Comb/seq are builder API; Hir records the resulting statements, not a second region type.

---

### Pair 9 — Two `Artifact` / file identities (shared product shape)

AD-7: 后端是纯函数：`&Hir -> Artifact`. **Artifact is undefined.** Consistency Conventions (non-AD) say `<module>.sv` / `<module>.fir`.

**Unit A — `rhdl-vlog`:** `fn emit(hir: &Hir) -> String` — one blob, top = first module in vec order.

**Unit B — CLI:** expects `struct Artifact { files: Vec<(AbiName, String)>, filelist: String }` and writes `<abi_name>.sv` per public module, mangling private modules (convention text, FIRRTL ABI).

**ADs obeyed:** both are pure `&Hir -> _`. AD-7 does not name `Artifact` fields. Conventions are not ADs, so Unit A may ignore filenames, mangling, and filelists.

**Clash:** CLI cannot write A’s String to the path B’s users/Yosys expect. Two owners of module ABI names: hir freeze vs vlog emit vs firrtl export. Private-module mangling can happen in freeze (A never heard of it) or in the backend (B), producing different symbols for the same entity.

**Close with AD-16 (artifact + ABI):** `Artifact` is owned by `rhdl-hir` (or a tiny `rhdl-artifact` crate): `{ files: Vec<EmittedFile>, filelist: Vec<String> }`. Freeze assigns `abi_name`: exactly one public top per elaborate; private modules mangled then, not in backends. Backends must use `abi_name` as the file stem. Conventions table is restated as this AD or deleted.

---

### Pair 10 — Two diagnostic catalogs / two span owners (AD-10)

**Unit A — `rhdl-macro`:** `compile_error!("rhdl::E0001: duplicate driver")` using proc-macro spans. Codes allocated ad hoc.

**Unit B — `rhdl-hir` freeze:** `enum HirError { MultiDrive { net: String } }` with `Display` sometimes printing `rhdl::E0001` for a **different** meaning (width mismatch). HIR nodes carry no spans (no AD requires spans on Hir), so freeze diagnostics have `code` but not `span`. AD-10 says 用户错误走结构化诊断（span + 码） and **Binds: 宏, HIR, 后端, CLI**.

**ADs obeyed:** both avoid panic on user input (AD-10’s Prevents). “span + 码” can be read as “when a span exists”. No crate owns the catalog. Conventions (`rhdl::E0xxx`; 英文码 + 中文说明) are not ADs.

**Clash:** one code, two meanings; CLI cannot format freeze errors next to design-crate source; backends invent `VlogError` without codes. Three owners of “the diagnostic entity”.

**Close with AD-17 (diagnostics):** A single `Diagnostic { span, code, en, zh }` type; code ranges allocated per crate; Hir nodes carry `Span` threaded by builder. Macros, freeze, backends, CLI all emit that type. No parallel error enums at the crate boundary.

---

### Pair 11 — Two dual-model state shapes (AD-5)

**Unit A — `rhdl-sim`:** `struct SimState { map: HashMap<String, BitVec> }`; `tick(&Hir, &mut SimState)`. Comparison is string-keyed.

**Unit B — `#[functional_model]` (macro + design crate):** ordinary Rust struct with typed fields; generated `fn step(&mut self, i: I) -> O`. The attribute is a no-op marker **or** a trait impl — AD-5 does not say which, or which crate owns the attribute (macro vs a new `rhdl-sim-macros`, the latter still legal: AD-6 forbids macro→backend, not sim→macro).

**ADs obeyed:** both are cycle-accurate-from-HIR vs handwritten Rust (AD-5). Consistency is “随机/对照测试” with no payload type, no reset protocol, no owner of the harness.

**Clash:** there is no shared value type to compare. Two owners of “circuit state at a cycle”. Random tests cannot be written without a third unofficial translation. `Bits<const W: usize>` in prelude vs runtime `width: u32` on imported FIRRTL (AD-3) vs `BitVec` in sim — three shapes for one ground value.

**Close with AD-18 (dual-model values):** `rhdl-hir` owns `PortValues` keyed by FrozenHir port list (name + width). `tick(&FrozenHir, &mut PortValues)`. `#[functional_model]` is a `rhdl-macro` attribute that impls `fn cycle(&mut self, ports: &PortValues) -> PortValues`. Differential tests compare `PortValues` only. Ground values in Hir are runtime widths; const-generic `Bits<W>` is a builder facade that lowers to those widths.

---

### Pair 12 — Two Verilog products in phase 1 (AD-8 vs seed/CLI firtool)

Paradigm diagram: `rhdl-rs` 拉 firtool from day one. Structural seed includes `rhdl-firrtl`. AD-9 pins firtool-1.155.0 and binds CLI **and** `rhdl-firrtl`. AD-8: 阶段一必出 Yosys 友好 Verilog; **FIRRTL 导出是阶段二**.

**Unit A — phase-1 vlog only.**  
`rhdl-firrtl` is a stub. CLI never writes `.fir`. firtool is downloaded (AD-9) and unused until phase 2. Gold Verilog is `rhdl-vlog` only.

**Unit B — prove AD-8 alignment now.**  
“HIR 节点语义现在就按 FIRRTL spec 6.0.0 可逆子集对齐” is untestable without emission. Unit B emits `.fir` as a **debug dump** (not “the phase-2 product”) and may run firtool to get a second `.sv`. AD-8 letter: phase 1 **must** emit Yosys-friendly Verilog; it does not say **only** that. AD-9 binds `rhdl-firrtl`, so that crate may invoke firtool itself (CLI is not named as exclusive owner — “CLI 下载/缓存” vs firrtl crate also bound).

**ADs obeyed:** A honors “导出是阶段二”. B honors “现在就对齐” plus AD-9’s firrtl bind. Cache location is unspecified (`~/.cache` vs `target/` vs `RHDL_FIRTOOL_PATH` only as override).

**Clash:** two owners of firtool invocation and two files named `<module>.sv`. Yosys/CI may pick the wrong one. Direct vlog vs firtool lowering differ (reset style, bit-select, `assign` vs `always`); without a gold rule, “alignment” is theatrical.

**Close with AD-8 / AD-9 tightened:** Phase 1: `rhdl-vlog` is the unique owner of `.sv` build products; `rhdl-firrtl` must not emit user-facing `.fir` or invoke firtool. CLI is the unique firtool downloader/caller. Phase 2: firtool SV is written as `<module>.firtool.sv`, never as `<module>.sv`. Alignment tests in phase 1 are HIR-level (shared schema from AD-12), not a second Verilog path.

---

## Findings

Each finding is a hole: two legal units, incompatible composition. `guard_snippet` is the AD to add or tighten.

### F1

- **lens:** adversarial
- **location:** AD-7; AD-8; Structural Seed `rhdl-hir`; missing HIR schema
- **trigger_condition:** `Hir` has no frozen schema — hierarchical FIRRTL AST and flattened `NodeId` netlist are both legal.
- **guard_snippet:** AD-12 — HIR is the FIRRTL 6.0.0 reversible-subset AST in `rhdl-hir`; backends may flatten privately.
- **potential_consequence:** `rhdl-vlog`, `rhdl-sim`, and `rhdl-firrtl` cannot take the same value; the toolchain splits into two IRs.

### F2

- **lens:** adversarial
- **location:** AD-4; AD-7; Capability map “类型/模块 → prelude, builder, macro”
- **trigger_condition:** Builder may own a parallel mutable circuit because AD-7 only constrains **Hir**, and AD-4 says ownership is not soundness.
- **guard_snippet:** AD-13 — sole circuit owner is `Hir`; builder is a session over `&mut Hir`; macros expand only to builder.
- **potential_consequence:** freeze checks a dump, not the circuit the user built; macros and builder diverge.

### F3

- **lens:** adversarial
- **location:** AD-1; AD-7 (`仅 elaborate() 可变` vs `&Hir -> Artifact`); AD-6 (design ↛ hir not forbidden)
- **trigger_condition:** Unfrozen `Hir` may escape; freeze may be a flag on the same type; design crates may depend on `rhdl-hir` and mutate after `elaborate()`.
- **guard_snippet:** Tighten AD-7 — private unfrozen state; `elaborate() -> Result<FrozenHir, _>`; backends `fn(&FrozenHir) -> Artifact`; design crates must not depend on `rhdl-hir`.
- **potential_consequence:** backends lower multi-driven graphs; two public mutation paths for one entity.

### F4

- **lens:** adversarial
- **location:** AD-1 vs AD-3
- **trigger_condition:** Import must build Hir, but AD-1 says Hir is only built in `elaborate()`; Prevents clauses do not resolve the collision.
- **guard_snippet:** Tighten AD-1 — allowed allocators are `elaborate()` and `rhdl-firrtl::import`; both end in the same private `freeze` → `FrozenHir`.
- **potential_consequence:** parser-Hir and re-elaborate-Hir for the same `.fir` are different circuits; round-trip is undefined.

### F5

- **lens:** adversarial
- **location:** AD-1 (generator process) ∩ AD-6 (design crate ↛ backends)
- **trigger_condition:** FrozenHir must leave the design process and enter CLI backends; no handoff protocol exists.
- **guard_snippet:** AD-14 — CLI host/shim crate links design + backends in-process; no ad-hoc JSON/dylib protocol in phase 1.
- **potential_consequence:** two CLIs (serialize vs shim) that cannot build each other’s designs.

### F6

- **lens:** adversarial
- **location:** AD-1 (`cargo test` … `tick`); AD-5 (`tick` in `rhdl-sim`); AD-6 (设计 crate 不得依赖后端; mermaid DES ↛ SIM)
- **trigger_condition:** “后端” is undefined, so design crates both may and may not depend on `rhdl-sim`; prelude cannot re-export `tick` without a forbidden PRE→SIM edge.
- **guard_snippet:** Tighten AD-6 — 后端 = vlog, firrtl, sim; `[dev-dependencies]` may name `rhdl-sim` only; prelude must not depend on sim.
- **potential_consequence:** tests that AD-1 requires cannot be written, or they violate AD-6; IP (bound by AD-5) repeats the fork.

### F7

- **lens:** adversarial
- **location:** AD-5; AD-8 阶段一单时钟; Deferred 多时钟
- **trigger_condition:** Clock/reset may be implicit (invented at Verilog emit) or explicit FIRRTL ports; `tick` arity is unspecified.
- **guard_snippet:** AD-15 — one Clock + one sync active-high Reset port in FrozenHir; `tick` is that posedge; no implicit ports at emit.
- **potential_consequence:** sim, vlog, firrtl, and `#[functional_model]` disagree on the sequential interface of the same module.

### F8

- **lens:** adversarial
- **location:** AD-6 mermaid (MAC has no edge); AD-6 rule text (宏不得依赖 vlog/firrtl/sim only)
- **trigger_condition:** `rhdl-macro` may depend on `rhdl-hir` and write nodes directly, bypassing builder.
- **guard_snippet:** AD-6 — macro → builder only (or path-emit builder); never macro → hir.
- **potential_consequence:** two mutation paths and two comb/seq representations for one circuit.

### F9

- **lens:** adversarial
- **location:** AD-7 `Artifact`; Consistency Conventions (non-AD) 文件 / HIR 标识
- **trigger_condition:** `Artifact` is unnamed; filename/ABI/mangling rules are conventions, so one backend returns `String` and another a filelist of ABI names.
- **guard_snippet:** AD-16 — typed `Artifact`; freeze assigns `abi_name` (one public top; private mangled in hir); backends use that stem.
- **potential_consequence:** CLI cannot place outputs; two owners of module identity; Yosys filelists miss instances.

### F10

- **lens:** adversarial
- **location:** AD-10; Consistency Conventions 错误
- **trigger_condition:** No diagnostic type or code owner; spans are not required on Hir nodes; `rhdl::E0001` can be issued twice with different meanings.
- **guard_snippet:** AD-17 — shared `Diagnostic` + span-on-Hir + allocated code ranges.
- **potential_consequence:** freeze errors cannot point at source; codes collide; AD-10’s “span + 码” is unenforceable in hir/backends.

### F11

- **lens:** adversarial
- **location:** AD-5 一致性用随机/对照测试; `#[functional_model]`
- **trigger_condition:** No shared `PortValues`; `tick` state, functional structs, and const-generic `Bits<W>` vs runtime import widths are three shapes.
- **guard_snippet:** AD-18 — `PortValues` owned by hir; `tick` and `#[functional_model]` both speak it; const generics lower to runtime widths.
- **potential_consequence:** dual-model testing required by AD-5 cannot be implemented without an unofficial translation crate.

### F12

- **lens:** adversarial
- **location:** AD-8 阶段一 Verilog / 导出是阶段二; AD-9 binds CLI **and** `rhdl-firrtl`; paradigm “拉 firtool”
- **trigger_condition:** Phase 1 may stub firrtl or emit debug `.fir` + second `.sv` via firtool; two callers may cache/invoke firtool.
- **guard_snippet:** Tighten AD-8/9 — phase-1 unique `.sv` owner is `rhdl-vlog`; unique firtool owner is CLI; firtool SV named `*.firtool.sv` in phase 2.
- **potential_consequence:** two Verilog products, two firtool caches, alignment theater instead of a single gold lowering.

### F13

- **lens:** adversarial
- **location:** general — only `[ADOPTED]` blocks bind; Consistency Conventions / Design Paradigm / Capability map / Structural Seed do not
- **trigger_condition:** Package names, file stems, error-code shape, “设计 crate 只依赖 prelude”, tracing, and crate layout can be ignored by any epic that reads ADs only.
- **guard_snippet:** Promote every convention that two crates must share (prelude-only user dep, crate names, `rhdl::E0xxx`, tracing quietness) to ADs, or state in the spine that Conventions bind as ADs. Delete the rest.
- **potential_consequence:** Unit A implements “the ADs”; Unit B implements “the document”; they fail to compose while both claiming compliance.

### F14

- **lens:** adversarial
- **location:** AD-5 Binds `IP`; Deferred `IP crate`; Capability map CDC/HLS/IP/IDE = Deferred
- **trigger_condition:** AD-5 binds IP crates that the spine also defers; no rule says whether an IP epic may exist, depend on sim, or ship a second `tick`.
- **guard_snippet:** Remove IP from AD-5 Binds until an IP AD exists; or AD-19 — phase-1 IP is ordinary design crates under the same prelude-only rule, no extra lowering.
- **potential_consequence:** an “IP” epic ships a parallel simulation or Verilog path and still cites AD-5.

---

## AD patch list (holes → closes)

| Hole | Close |
| --- | --- |
| F1 two Hir shapes | **AD-12** HIR = FIRRTL 6 reversible-subset AST |
| F2 two circuit owners | **AD-13** builder sessions `&mut Hir`; no parallel graph |
| F3 unfrozen escape / `&Hir` vs freeze | **Tighten AD-7** `FrozenHir` only; design ↛ hir |
| F4 import vs elaborate | **Tighten AD-1** two allocators, one `freeze` |
| F5 process handoff | **AD-14** CLI host/shim in-process |
| F6 tick vs no-backend-dep | **Tighten AD-6** define 后端; allow sim as dev-dep |
| F7 clock/reset/tick | **AD-15** explicit Clock+Reset; posedge `tick` |
| F8 macro writes Hir | **Tighten AD-6** macro ↛ hir |
| F9 Artifact / ABI names | **AD-16** typed Artifact; freeze owns `abi_name` |
| F10 diagnostics | **AD-17** shared Diagnostic + spans |
| F11 dual-model values | **AD-18** `PortValues` join type |
| F12 two Verilog/firtool owners | **Tighten AD-8/9** unique `.sv` and firtool owners |
| F13 conventions non-binding | Promote or delete conventions |
| F14 IP bound and deferred | Drop IP from AD-5 or add AD-19 |

Until AD-12, AD-13, tightened AD-7, AD-14, and AD-15 exist, the spine is not a substrate: a hir epic and a backend epic can each be “correct” and still not build one toolchain.

---

## Verdict

**FAIL.** Do not break the spine into epics/crates on this text. The ADOPTED set blocks rustc-time HIR, crates.io name collision, TLM, Chisel-Scala as contract, upward deps among *named* backends, and PATH firtool — and leaves the actual shared objects (`Hir` schema, Frozen vs mut, birth paths, host handoff, clock/reset, Artifact, diagnostics, dual-model values) unspecified. Those are the composition surface. Close the holes above, then re-attack.
