# Reconcile: research R1–R7 → PRD FR / NFR / phasing

- **PRD:** `prd.md` (draft, 2026-08-19)
- **Addendum:** none
- **Primary input:** `technical-rhdl-phase-two-later-product-fr21-nfr3-l-2026-08-19/research.md` §建议 R1–R7
- **Job:** whether each recommendation is reflected as a stable FR/NFR and in the P0→P1→P2 order. Not a rewrite.
- **Lens:** silent drops and phasing inversions. Implementation HOW (Maven fallback, cache path, `print-version`) belongs in architecture/addendum, not a PRD miss.

This note is a gap report, not a rewrite.

---

## How to read this

| Bucket | Meaning |
| --- | --- |
| Landed | Named FR/NFR and phase match the recommendation’s order |
| Intentional defer | Parked as Open Question / architecture HOW; not a product-scope hole |
| Gap | Load-bearing in R1–R7; PRD FR/NFR/phasing is silent, weaker, or inverted |

---

## Landed

| Rec | Research | PRD landing |
| --- | --- | --- |
| R1 | README：`rhdl-rs`、禁止 crates.io `rhdl`/`rhdl-bits`、与 samitbasu/rhdl 无关 | **FR21** P0; Vision; SM-4 |
| R2 | 钉 `firtool-1.155.0` + `firrtl-bin-linux-x64.tar.gz` + `.sha256` + 缓存 + `RHDL_FIRTOOL_PATH`；文档承认上游会漂 | **NFR3** P0; **NFR12** P0 (升钉政策); UJ-2 |
| R3 (phase) | 语言表面在 HLS/多时钟之前 | **FR22** P0; intent 写明不上 HLS/多时钟; UJ-1 仍单时钟 |
| R4 (core + order) | 单时钟 SyncReadMem → HIR → `.v`/`firrtl.mem`；Mem 先于多时钟 | **FR26** P1「先于 FR23」; SM-2 |
| R5 (order) | 多时钟 epic 在表面加厚与 Mem 之后 | **FR23** P1 after FR26; UJ-3 |
| R6 (phase) | FST 可选、不阻塞表面 sprint；VCD 仍默认 | **FR31** P2; VCD remains CAP-5 |
| R7 (phase + no scheduler) | HLS 不进下一主 epic；不自研调度 | **FR35** P2; Out of scope「自研 HLS 调度器」; Non-users |

P0 cites「调研 R1–R3」; P1 is Mem then clocks; P2 holds FST/HLS. Macro-order matches the research exec summary.

---

## Gaps

### G1 — R3 named surface, not only two fixtures (FR22)

- **Research:** 宏/builder **完整 if/match**、**同位宽算子**、**reg 复位语义**，外加计数器 + 单时钟 FIFO 形示例。
- **PRD:** FR22 success is only the two examples; intent is the vague「comb/seq/运算/控制流」. Same-width is inherited from phase 1, not an FR22 acceptance line. Register *reset semantics on the surface* is not FR22 (FR24 is async reset, P1, a different claim).
- **Drop:** an epic can ship two fixtures without complete if/match or documented reg-reset surface.

### G2 — R4 dual-port cross-clock only via named CDC FIFO

- **Research:** 「双口跨时钟仅经命名 CDC FIFO」.
- **PRD:** FR26 is single-clock SyncReadMem only. FR23 names SyncFIFO as language-level CDC but does **not** forbid dual-port / cross-clock `mem` except through that FIFO.
- **Drop:** P1 can grow a dual-port cross-clock mem path that bypasses the named CDC primitive.

### G3 — R5 three-way CDC pick vs FR23 Clash lean

- **Research:** after R3–R4, **explicitly choose one** of Spinal-style check / Clash phantom / Chisel library and write it into the spine.
- **PRD:** FR23 intent still says「Clash 风格域（或同等可执行检查）」. §7 Assumption + Open Question 1 defer the pick to architecture — correct as HOW, but the FR already leans Clash (same wording as `later-product.md`).
- **Drop:** the “don’t assume Clash; pick among three” constraint is not a phasing/architecture gate on FR23; epics can treat Clash phantom as the requirement.

### G4 — R6 Verilator/vcd2fst switch, not a first-party FST writer

- **Research:** FST as **sim/Verilator 可选开关** (or vcd2fst); default remains VCD; do not block R3.
- **PRD:** FR31 P2 optional switch + GTKWave/Surfer openable FST. Open Question 3 reopens「自研 writer vs Verilator/GTKWave」.
- **Drop:** R6’s recommended path (Verilator/conversion, not a product writer) is not in FR31; P2 can take on in-house FST.

### G5 — R7 spike vs FR35 product success

- **Research:** HLS is **only** a design spike「发射 + 调 Bambu/XLS」; **not** the next main epic.
- **PRD:** P2 is correct (not P0/P1). FR35 success is a real `#[hls]` call that produces documented Bambu/XLS artifacts — a deliverable FR, not a spike.
- **Drop:** later-product/HLS is fully promoted; spike-only / “don’t staff a scheduler epic” is weaker than a gated P2 FR.

---

## Not gaps (recorded so they are not re-opened)

- R2 Maven `llvm-firtool` fallback, XDG cache layout, `rhdl firtool --print-version` — HOW; NFR3 already has pin/fetch/sha256/cache/override.
- NFR10 in P0 — later-product item, not R1–R7.
- FR24/FR25 in P1 with clocks — later-product clocking extras; research did not sequence them against Mem.
- P0 listing FR21+NFR3+FR22 as one bucket vs research「立刻文档/firtool → 紧接着表面」: same phase, no inversion; intra-P0 order is epic sequencing, not a PRD hole.

---

## Compact (for parent)

**Input:** technical-rhdl-phase-two-later-product-fr21-nfr3-l-2026-08-19 (R1–R7)

**Gaps:**

1. FR22 omits R3 if/match, same-width ops, reg-reset surface (examples only).
2. No FR/NFR: dual-port cross-clock mem only via named CDC FIFO (R4).
3. FR23 still Clash-leans; R5’s three-way spine pick is not a gate.
4. FR31 does not pin Verilator/vcd2fst; OQ-3 allows a first-party FST writer (R6).
5. FR35 is a P2 product FR, not R7’s emit+Bambu/XLS spike.

**File:** `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/reconcile-research.md`
