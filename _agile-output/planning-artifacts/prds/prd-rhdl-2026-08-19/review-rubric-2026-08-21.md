# PRD Quality Review — Bitloom 阶段二 PRD（overview-literal ①C / 2026-08-21）

**Scope of review:** `prd.md` + `addendum.md`（amendment `overview-literal-C-2026-08-21`）。  
**Stakes:** launch。**Intentional high-risk scope (①C):** treated as **accepted product risk**, not an automatic fail — judged on whether risk is **visible and controllable** (esp. NFR14), and whether remaining text is epic-safe.

## Overall verdict

This revision earns its thesis: §0’s overturn table, SM-6/SM-7 counter-metrics, UJ-4…6, and the addendum’s CIRCT/TLM/IP realities make the ①C bet honest rather than theatrical. What is at risk for epic work is not the ambition of FR46–FR52, but (1) baseline FR21–FR40 bodies collapsed to「同原」so P0–P2 stories cannot be sourced from this file alone, and (2) NFR14 naming risk without a gate-shaped artifact — accepted ①C scope is visible in prose, not yet adequate as a change-control contract before P3 epics.

## Decision-readiness — strong

A decision-maker can act on *what changed*: §0’s overturn table pairs each prior non-goal with the new FR, and explicitly records that research’s「重定义 done」was **rejected**. Trade-offs are not smoothed — Vision and Success Metrics say「尽力 / deferred / 非目标」must not masquerade as overview completion, and the addendum lists upstream facts (Chisel#4899 / no FIRRTL→Scala parse; industrial TLM↔RTL rarity; IP maintenance surface) that the PRD refuses to re-bury as non-goals.

Open Questions 5–7 are real blockers with documented default `[ASSUMPTION]`s; Open Q5 is correctly called out as blocking P3 epic sizing. The addendum’s `[NOTE FOR PM]` on FR46 option A/B/C before a spine AD is the right tension callout. ①C multi-year conflict with CIRCT direction is acknowledged, not dodged.

### Findings

- **high** Launch vs P3 completion still ambiguous (§3 SM-6, §4 P3 row) — Stakes are「launch」while SM-6 requires FR46–FR52 *and* strengthened FR28/30/35/37/38 before overview-literal is “done.” A PM can green-light the bet and still not know whether *launch* means “ship Bitloom identity + P0/P1” or “do not claim launch until P3 closes.” *Fix:* One sentence: launch milestone = which phase bucket; P3 items labeled accepted post-launch roadmap vs launch-blocking.
- **medium** Open Q5 default not locked as acceptance language (§8 Q5, FR28/FR46 「可维护」) — Default ASSUMPTION (compile + port/hierarchy predicate, not idiomatic style) is present but still `[OPEN]`. Epics will argue option B vs 「可维护」字面. *Fix:* Either close Q5 in-PRD or elevate the default ASSUMPTION into FR28/FR46 success bullets as the binding bar.

## Substance over theater — strong

UJ-4…6 drive FR46–FR49/FR30/FR47; they are not decorative. SM-6/SM-7 and the contract-level 反指标 against「同业未做满」as permanent waiver are product-specific, not boilerplate. Vision names Bitloom-specific deliverables (bidirectional Chisel, product HLS path, first-class IP, built-in hierarchy+timing, dual generated simulators) — not a swappable “modern HDL” slogan. NFR theater is largely absent: NFR3/NFR12 remain pin-sharp; NFR14 is thin (see Scope honesty) but at least scoped to P3升格, not generic “be reliable.”

Persona/JTBD section stays light and job-shaped; Non-Users honestly *remove* prior exclusions under ①C instead of papering over the flip.

### Findings

- *(none material — dimension holds.)*

## Strategic coherence — adequate

Thesis is clear: map overview §1.3.7–11 / §1.5 into hard FRs under user ①C, keep FR21–FR40 IDs stable, Bitloom identity supersession. Prioritization still follows the earlier evidence-backed spine (P0 surface/firtool → P1 Mem before CDC → P2a sim → P2b HLS/Chisel/formal → P2c IO/IP/viz → **P3 overview-literal closure**). New SMs validate the new thesis (SM-6/SM-7), not only activity.

Weakness: P3 is a coherent *bucket* for the overview bet, but internally FR46/47/48/49/51/52 are peer-listed without a thesis-derived order beyond assumptions (FR46 needs FIRRTL; FR47 needs FR29/30; FR48 needs prelude). That is acceptable for a Fast-path amendment if architecture owns sequencing — but launch-stakes readers may mistake P3 for a single epic.

### Findings

- **medium** P3 is a thesis bucket, not a sequenced bet (§4 P3, §7 ASSUMPTION on deps) — Dependencies are assumed, not ordered as delivery logic. *Fix:* Minimal P3 order line (e.g. FR52/FR51 with language; FR46 after FR28; FR47 after FR30; FR48/FR49 last) or mark parallelization explicitly per FR.

## Done-ness clarity — thin

Strengthened FRs (FR21, FR28, FR29/30, FR35, FR37/38, FR40, FR46–FR52) generally carry testable success: compile under pinned Chisel/firtool, pass/fail equivalence, smoke elaborate→emit→tick, CI HLS fixture, hierarchy+timing product entry (not GTKWave-only). That part of the amendment is epic-usable.

The regression is baseline coverage: FR22, FR23–FR27, FR31–FR34, FR36, FR39 are reduced to「同原」/「同原技术条」. An engineer opening *this* `prd.md` cannot recover construct bars, Mem semantics, FST policy, or formal export success without another artifact. Prior rubric already judged FR22 fixture-thin; this update makes that bar *invisible* rather than fixed. For launch-stakes epic decomposition, that is unsafe.

FR40’s “P3 前须交付…如 `import`/`visualize`/…” lists examples without a mandatory verb set. FR48’s AXI class is rescued only by Open Q7’s default ASSUMPTION (AXI4-Lite slave) — workable if locked.

### Findings

- **critical** Baseline FR bodies collapsed to「同原」(§5.2–5.6 FR22–27, FR31–34, FR36, FR39) — Intent/success text for the still-in-scope phase-two baseline is not in this file. Epics for P0–P2c cannot source-extract done-ness; 「同原」points at overwritten content, not a live companion. *Fix:* Inline intent/success for every in-scope FR (or point each stub to a frozen path/commit of the pre-amendment PRD / SPEC companion that still holds the bar).
- **high** FR22 construct bar still unresolved under stub (§5.2, SM-5) — Even the stub’s hint (“构造条 + 计数器/FIFO fixture”) re-imports the prior defect: fixtures without an explicit construct inventory. *Fix:* Restore a construct checklist in FR22 success (and keep Bundle/Vec on FR51 only).
- **medium** FR40 mandatory CLI surface underspecified (§5.6 FR40) — “为升格所必需者” is circular for story writers. *Fix:* Enumerate required verbs/subcommands mapped to FR46–FR49 (even if names are provisional).

## Scope honesty — adequate (①C risk accepted; NFR14 visibility thin)

Omissions and flips are unusually honest: Non-Users revised in place; §4 names what moved from forbidden → must-deliver; Rejected section in addendum marks which historical refusals ①C voids; Assumptions Index covers identity, HLS scheduler out-of-tree, CIRCT-era FR46 strategy, UJ sufficiency. Open-item density (3 open + multiple ASSUMPTIONs + NOTE FOR PM) is high for green-light-to-build, but appropriate *because* stakes are launch and the user chose ①C — not a silent scope creep.

**①C as accepted risk:** Not scored as fail. The PRD and addendum correctly refuse to convert engineering difficulty into non-goals.

**NFR14 adequacy (required for this review):** Present in spirit — addendum §2026-08-21 lists the three load-bearing realities; NFR14 forbids silent downgrade and demands「风险与工期记录」for P3. **Not yet adequate as a control:** no required fields (upstream dependency, estimate band, acceptance fallback *forbidden*), no gate (“before epic commit / before phase exit”), no named owner artifact path. At launch stakes, “leave a note somewhere” is weaker than the overturn table’s rigor.

### Findings

- **high** NFR14 is a policy sentence, not a risk gate (§6 NFR14; addendum NFR14 line) — Visibility of ①C risk is prose-level; epics can still start FR46/47/48 without a recorded estimate or explicit “no silent downgrade” checklist. *Fix:* Specify mandatory risk record shape (per FR46/47/48/49 at minimum: upstream constraint, rough calendar band, forbidden silent fallbacks) and when it must exist (e.g. before P3 epic status → ready).
- **low** Overview file remains non-authoritative while Vision quotes it (§0, §8 Q4 closed note) — Honest, but dual sources (概述 vs PRD) will confuse external readers until the promised rewrite task lands. *Fix:* Keep as-is for this “②只改 PRD” pass; track overview sync as explicit follow-up ID outside this PRD if not already.

## Downstream usability — thin

Chain-top (§0: PM, architecture, epic split) — this dimension matters. New FRs and strengthened success lines are extractable. Traceability §9.1–9.2 is clear for overview mapping and later-product continuity. UJ-1…6 have named protagonists.

Failures:「同原」stubs break standalone FR extraction; Glossary lacks load-bearing nouns now in FRs (Chisel 互转, HLS, Bundle/Vec, ClockDomain, 一级 IP); FR41 is cited in Document Purpose as stage-three identity while this PRD jumps FR40→FR46 (intentional gap, but unexplained in Glossary/ID note). Open Q5–Q7 defaults help, but unresolved opens on FR46/47/48 will fork architecture and stories.

### Findings

- **high** Same as Done-ness critical:「同原」blocks clean FR pull for stories (§5) — Downstream workflows cannot treat this PRD as the sole FR source. *Fix:* Same as above — restore full FR text or authoritative external anchors per ID.
- **medium** Glossary under-covers ①C vocabulary (§ Glossary vs FR46–FR52) — “可维护 Chisel,” HLS path, Bundle/Vec, ClockDomain, 功能/周期精确模拟器生成 appear in FRs without Glossary rows. *Fix:* Add rows so UX/architecture/story agents share terms.

## Shape fit — strong

Capability-spec shape matches a Cargo CLI / eDSL toolchain. UJs are used where they earn acceptance scenes for the overview upgrade (Dana/Ellis/Flynn), not as consumer-novel padding. Brownfield authority (阶段一 SPEC + FR1–20; 阶段二 baseline IDs retained) is marked. Chain-top is declared; addendum correctly holds HOW/options. Not over-formalized; not under-formalized for launch of a technical contract. ①C expands scope violently, but the *shape* still fits — the stress shows up in done-ness and risk gates, not in wrong template.

### Findings

- *(none material — shape matches product type and amendment intent.)*

## Mechanical notes

- **Glossary drift:** Public name Bitloom/`bitloom` consistent in revised FR21; overview still RHDL by ASSUMPTION — called out. `rhdl-float` → `bitloom-float` noted. Residual `RHDL_FIRTOOL_PATH` alias ASSUMPTION OK.
- **ID continuity:** FR21–FR40 retained; FR46–FR52 new; **FR41–FR45 gap** unexplained in-body (FR41 mentioned only as stage-three identity in §0). NFR14 appended after NFR13 — fine. No duplicate FR/NFR IDs spotted.
- **Assumptions Index roundtrip:** Inline ASSUMPTIONs (identity/overview sync; HLS scheduler; UJ env alias; float dual name; Bundle naming; P3 deps; FR46 CIRCT strategy in §7) largely indexed. §7’s CIRCT/Scala ASSUMPTION is index-only richness vs thinner inline at FR46 — acceptable. Open Q5–Q7 defaults are ASSUMPTIONs living under Open Questions (good) but easy to miss in Index — consider indexing them.
- **UJ protagonists:** UJ-1…6 named (Alex, Blair, Casey, Dana, Ellis, Flynn) — OK.
- **Typos / polish:** FR35 success has a stray space in `** reproducibly**`; not verdict-driving.
- **Prior review (`review-rubric.md`):** Pre-①C review’s FR22 construct-bar **high** remains unresolved and is worsened by stubbing; P2 sequencing concern partially addressed by P2a/b/c + P3 but P3 internal order still thin.

## Severity summary (epic-unsafe focus)

| Severity | Item | Epic impact |
|----------|------|-------------|
| **critical** |「同原」baseline FR bodies | P0–P2c stories cannot define done from this PRD alone |
| **high** | NFR14 not gate-shaped | ①C risk visible in narrative, weak as pre-epic control |
| **high** | Launch vs P3 “done” undefined | SM-6 can be read as launch-blocking years of work without milestone split |
| **high** | Open Q5 /「可维护」bar | FR28/FR46 epics fork on idiomatic vs mechanical |
| **medium** | FR22 construct inventory; FR40 verbs; P3 order; Glossary | Story churn / rework |

**Not automatic fails (accepted ①C risk):** Bidirectional Chisel against CIRCT reality; generated dual simulators; full UART/SPI/I2C/FIFO/AXI IP line — provided NFR14-class visibility is strengthened before P3 commit.
