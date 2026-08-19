# PRD Quality Review — RHDL 阶段二（later-product → FR）

## Overall verdict
This PRD does the job it claims: every `later-product.md` bullet gets a stable FR/NFR, P0/P1 order is evidence-backed (surface → Mem → clocks; HLS/FST stay P2), and Non-goals / Non-users are honest. Sequencing and the pin-sharp P0 host NFRs (NFR3, NFR12) hold up. What is at risk is treating this as a build contract for the actual language bet: FR22 is two fixtures with no construct inventory, SPEC / `epics.md` / this PRD still share authority (Open Question 4), and P2 FRs are ID-complete but story-thin. Green-light for architecture on P0/P1 order; not yet green-light for epic decomposition of FR22 or the P2 cluster.

## Decision-readiness — adequate
A decision-maker can act on **order**: §4 names P0 (FR21, NFR3, FR22, NFR10, NFR12), P1 as “Mem 先于多时钟” (FR26 then FR23/24/25), and P2 as the rest, with addendum *Rejected* refusing to promote HLS/FST to P0. Trade-offs are stated as refusals, not balances: no in-house HLS scheduler, no Chisel Scala as interop contract, no crates.io `rhdl`/`rhdl-bits`, no cloud. Open Question 1 is actually open (“phase-blocker for architecture”). The CDC options matrix lives correctly in `addendum.md`.

The document does not close who owns the contract after this draft. Working title and §0 say stage-one CAP-1…CAP-9 / FR1–FR20 “仍有效” and this file is “阶段二及以后的产品范围,” while §8 Q4 still asks whether stage two merges into a single SPEC or stays SPEC + PRD. Meanwhile FR21 and NFR3 already exist in `_agile-output/planning-artifacts/epics.md` (Epic 1 inventory) as stage-one requirements; this PRD re-lists them as “阶段一缺口” without saying whether Epic 1 still owns them. Someone pushing back with “isn’t FR21 already in the epic breakdown?” would find the objection only as an unresolved question, not a decision.

### Findings
- **high** Triple source of truth (§0, §4 [NOTE FOR PM], §8 Q4) — SPEC.md still calls itself the canonical contract; `epics.md` already enumerates FR1–FR21 and NFR1–NFR9; this PRD claims stage-two product range without choosing the survivor. FR21/NFR3 are duplicated as P0 “缺口” while remaining Epic 1 coverage. *Fix:* Answer Q4 in the PRD: name the canonical stage-two file, and state whether FR21/NFR3 stay Epic 1 work or move here (one owner, one phase).

## Substance over theater — strong
Vision (§1) would not swap into another HDL PRD: generator elaboration, no in-house HLS, crates.io `rhdl-rs`, FrozenHir, Yosys `.v`, `tick`/VCD, identity vs `samitbasu/rhdl`. JTBD emotion is product-specific (“不会静默截断位宽、不会 silently 变成 latch”). Three named protagonists each drive a UJ that maps to FRs; none are furniture. NFRs carry versions and assets (`firtool-1.155.0`, `firrtl-bin-linux-x64.tar.gz`, sibling `.sha256`, MSRV 1.98.0), not “must be scalable.” SM-3’s counter-metric (“重新出现无 ID 的 later 清单”) is earned given this PRD’s promotion job. P2 looks like a backlog because promoting the whole later-product list *is* the stated intent, not innovation theater.

### Findings
None.

## Strategic coherence — adequate
The thesis is explicit: stage one is a skeleton; stage two makes small real RTL writable without becoming an HLS product, and evidence order is surface → Mem → multi-clock → optional FST/HLS (§1, §4, addendum “表面加厚顺序”). Feature phasing follows that thesis, not “what’s easy.” SM-2’s counter-metric (“多时钟合并时仍无 Mem 语义锚”) actually tests the thesis. Scope kind is platform/capability and matches the toolchain.

Success Metrics do not validate “可写真实小设计.” SM-1 is P0 demoability, SM-3 is process hygiene, SM-4 is identity. The language bet is only in FR22’s fixture sentence. P2 (FR27–FR40 plus NFR11/NFR13) is a flat remainder: §4’s assumption that “P2 内项可并行拆 epic” is a scheduling note, not prioritization derived from the thesis. A reader can sequence P0/P1; they cannot tell whether analog IO, LSP, or `rhdl-float` is the next move after Mem/CDC.

### Findings
- **medium** Metrics and P2 do not serve the language thesis (§3, §4 P2 row, FR27–FR40) — SM-1…SM-4 never ask whether comb/seq is actually writable beyond two fixtures; P2 is “FR27–FR40, NFR11, NFR13, FR31 等” with no intra-phase order. *Fix:* Add an SM that names the FR22 construct bar (or a small-design suite), and split P2 into a post-P1 sequence (e.g. FST/sim engines vs ecosystem vs host ports) or mark items as stretch with a revisit condition.

## Done-ness clarity — thin
P0 host rows are engineer-ready: NFR3 specifies tarball, checksum, cache, `RHDL_FIRTOOL_PATH`, and “默认不信任 PATH firtool”; FR21’s README/crates.io success is binary. Most P1 FRs have a fixture plus a fail-closed check (illegal CDC at freeze, async-reset edge-sensitive Verilog, SyncReadMem read-latency gold).

FR22, the P0 language bet, does not. Intent is “comb/seq/运算/控制流可写真实小 RTL”; success is only “计数器 + 单时钟 FIFO 形示例均可 elaborate、emit `.v`、`tick` 对齐黄金值.” Research R3 named the thickening (`if`/`match`, same-width ops, reg reset, two non-toy examples); `language-surface.md` already catalogs allowed vs deferred constructs. Neither inventory appears here. Two fixtures can pass on the stage-one skeleton (CAP-5 already requires a counter gold). An engineer can ship FR22 without adding a single new construct.

P2 repeats a pattern of OR-success and bundled products: FR28 “产出…Scala，或结构化尽力失败”; FR35 “Bambu 或 XLS” / “文档化产物”; FR39 “SVA 或文档化 formal 输入”; FR38 is HTML + LSP hover/goto + hierarchy diagram as one FR. FR34 “格式稳定” and FR40 “smoke” are adjectives with a fixture nearby. NFR11 lists platforms but not the fetch/verify bar NFR3 set.

### Findings
- **high** FR22 success is two fixtures, not a language bar (§5.2 FR22) — Research called for complete `if`/`match`, same-width operators, and reg-reset semantics; the PRD kept only the examples. Stage-one CAP-5 already has a counter. *Fix:* List the constructs that must elaborate/emit/tick (and which `language-surface.md` deferred types stay out), then keep the FIFO-shaped fixture as the integration check.
- **medium** OR-success lets P2 FRs pass by failing (§5.4 FR28, §5.6 FR35, §5.6 FR39) — “尽力失败,” “Bambu 或 XLS,” “SVA 或文档化 formal 输入” each give a path that never ships the named capability. *Fix:* Split happy-path vs documented-unsupported into two consequences, or name one required backend/format per FR.
- **medium** FR38 is three products under one ID (§5.6 FR38) — HTML docs, LSP hover/goto, and a hierarchy graph do not share a done line. *Fix:* Split into three FRs (or one FR with three independently testable success bullets) so stories can ship docs without LSP.
- **low** NFR11 has platforms but no pin/verify bar (§6 NFR11) — Unlike NFR3, macos / windows / linux-aarch64 have no tarball, checksum, or cache rule. *Fix:* Reuse NFR3’s fetch/verify/cache/`RHDL_FIRTOOL_PATH` sentence per platform, or say “same mechanism as NFR3, additional triples.”
- **low** FR37 intent lists five IPs; success is 1+1 (§5.6 FR37) — UART/SPI/I2C/FIFO/AXI can collapse to whatever is cheapest plus one opaque wrapper. *Fix:* Name the required in-tree IP (or “any one from {…}”) and the black-box contract (unchanged ports, no HIR inlining).

## Scope honesty — adequate
Omissions that could be silently assumed are mostly named: §2.2 Non-users, §4 “Out of scope（仍禁止）” (in-house HLS, Chisel as contract, HIR→TLM, publish `rhdl`/`rhdl-bits`, cloud), and the assumption that P2 items remain real requirements, not a later-product dump. Traceability §9 maps every later-product bullet, including the research-only FR21/NFR3/FR22 rows. Open-item density (four Open Questions, five assumptions, one `[NOTE FOR PM]`) is acceptable for a Fast-path launch PRD if Q1 stays an architecture blocker and Q4 is resolved before stories.

The hole is types deferred in `language-surface.md` that never appeared in `later-product.md` and therefore were not promoted: `Bundle`, `Vec<T,N>`, and multi-clock `Polarity` / `ResetKind`. A downstream reader can infer they hide inside FR22 “加厚” or FR23, or that silence means “don’t build.” This PRD’s purpose was to kill that inference.

### Findings
- **medium** Deferred language-surface types not in §9 (§5.2, §9 vs `language-surface.md` Deferred types) — `Bundle` and `Vec<T,N>` (and clock `Polarity` / `ResetKind`) are reserved in the companion catalog, absent from later-product, and unmapped here. *Fix:* Add FRs, fold them into FR22/FR23/FR26 with an explicit sentence, or mark `[NON-GOAL for this PRD]` so silence cannot mean permission.

## Downstream usability — adequate
This is chain-top (§0: PM, architecture, epic split). IDs FR22–FR40 are contiguous; FR21/NFR3 reuse is explained. UJ-1…3 have named protagonists (Alex, Blair, Casey) with Entry/Path/Climax. §9 is extractable as a promotion table. FRs are mostly pullable alone (intent / success / phase).

There is no Glossary. FrozenHir, `tick`, elaborate, emit, `PortValues`, `rhdl-prelude`, `rhdl::E0xxx`, CAP-5, and “freeze” are used as if the reader has SPEC + spine loaded. Stage-one numbering is CAP in SPEC and FR in `epics.md`; this PRD inherits both (“CAP-1…CAP-9 / FR1–FR20”) without a term map. UJ-2’s Edge cites NFR11 (P2) while the journey is the P0 firtool path (NFR3) — a story writer could attach the wrong NFR. Assumption that “不设独立 UX 规格” is fine for a CLI toolchain.

### Findings
- **medium** No Glossary; CAP vs FR dual numbering (§0, §2.3 UJ-2, §6) — Downstream cannot extract nouns without SPEC/epics in hand; UJ-2 Edge points at NFR11 instead of NFR3. *Fix:* Add a short Glossary (FrozenHir, tick, elaborate/emit, PortValues, CAP vs FR); retarget UJ-2 Edge to NFR3 and mention NFR11 as the later host expansion.

## Shape fit — strong
Capability-spec shape matches a solo/small-team CLI toolchain: FRs nested under feature groups, three UJs as acceptance scenes rather than a consumer-journey novel, operational SMs, no standalone persona section. Brownfield is marked (stage-one CAPs still valid; later-product promotion). Chain-top is declared. Not over-formalized (no fake UX spec) and not under-formalized for this product type (UJs exist where they earn a demo: FIFO, firtool offline, CDC catch). The miss on FR22 is a done-ness failure, not a wrong template.

### Findings
None.

## Mechanical notes
- **Assumptions Index roundtrip:** §7 lists five `[ASSUMPTION]` entries; only two are tagged inline (§2.3 journeys-as-capability, §4 P2 parallelism). FR numbering, “升格含分阶段,” CDC-mechanism-deferred, and later-product-becomes-index appear only in the index.
- **ID continuity:** FR21, then FR22–FR40 with no gaps. NFR3 then NFR10–NFR13 is intentional reuse/extension of stage-one NFRs, but NFR1–NFR2 / NFR4–NFR9 are not restated beyond “继承阶段一约束.”
- **Cross-refs:** §9 covers all `later-product.md` bullets. `language-surface.md` is not cited. Research path in §0 is named, not linked. UJ-2 → NFR11 mismatch noted above.
- **UJ protagonists:** Alex, Blair, Casey — all named, context inline.
- **Glossary drift:** “HIR” vs “FrozenHir” vs “冻结”; “Mem” vs `SyncReadMem` vs `firrtl.mem`; later-product’s “firtool-1.156.0 until Chisel pairs it” is correctly inverted in NFR12 (stay on 1.155.0 until a recorded pairing).
- **Required sections:** Vision, users/UJs, SMs with counter-metrics, scope/phasing, FRs, NFRs, assumptions, open questions, traceability. Glossary absent (see Downstream). No competitive section — appropriate; differentiation is in Vision and Non-users.
