# Epic 19 Context: 语言表面与合同解锁

<!-- Generated from planning artifacts. Regenerate with compile-epic-context if planning docs change. -->

## Goal

Unlock synthesizable `Bundle`/`Vec` and ship an acceptably documented ClockDomain/CDC story; align architecture ADs with the overview-literal contract; and land the NFR14 risk-record template that gates Epic 20–24 before they can be marked ready.

## Stories

- Story 19.1: NFR14 风险记录模板
- Story 19.2: 修订 AD-5（允许生成 Rust 功能模拟器）
- Story 19.3: 修订 AD-20（允许 Bundle/Vec）
- Story 19.4: 实现 Bundle / Vec 可综合路径（FR51）
- Story 19.5: ClockDomain 产品叙事与夹具（FR52）

## Requirements & Constraints

- Documented `Bundle` and `Vec<T,N>` (or equivalent) must elaborate → emit → `tick`; width/direction mismatches fail before emit. Design crates depend only on `bitloom-prelude`.
- Compound types are out of scope for the single-clock thicken surface; they must not become silently usable under that earlier bar—this epic delivers them.
- Product docs + at least one demo fixture show ClockDomain (or equivalent) binding clock/reset polarity and sync/async; illegal cross-domain freezes fail; legal paths use DoubleFlop or SyncFIFO (or same-named primitives). Docs alone without a freeze-fail path do not count.
- NFR14 template for P3: fields for upstream constraints, rough schedule band, forbidden silent-downgrade list, and owner; missing record blocks marking FR46/47/48/49 (and FR50 when applicable) epics ready. Distinct from historical **NFR14-crates** (crates.io FCFS). Parallel P3 work should note maintenance/Chipyard-style stacking risk.
- Public brand/CLI: Bitloom / `cargo bitloom`. Do not implement the dual-simulator generator here—only unblock it via AD revision.

## Technical Decisions

- Dual-model: cycle-accurate `tick` remains from FrozenHir; functional view may be handwritten `#[functional_model]` **or** a toolchain-generated Rust functional-sim crate; no SystemC TLM-2.0 contract. `#[functional_state]` never enters freeze/HIR.
- Bundle/Vec allowed on the synthesizable path (Story 19.3 revised AD-20; HIR may stay scalar via flatten); whether HIR grows Bundle/Vector nodes is an implementation choice, but public surface and emit semantics must stay consistent. Width/dir checks before emit.
- Multi-clock: Clash-style phantom domain markers in the type system; illegal crossing rejected at freeze; legal crossing only via language-level DoubleFlop/SyncFIFO. Default modules stay single Clock + sync active-high Reset unless multi-clock is declared.
- AD revisions (allow generated Rust functional sim; allow Bundle/Vec) and the NFR14 ready-gate are architecture contract work for this epic; project context that still restates the old bans must be corrected or pointed at the spine.
- Dependency rule unchanged: design crates → `bitloom-prelude` only (no backends/CLI).

## Cross-Story Dependencies

- Requires Phase 1–6 language/CDC baseline (multi-clock + DoubleFlop/SyncFIFO already present). Does not depend on Epic 20–24.
- 19.3 (AD allow Bundle/Vec) before 19.4 (implementation). 19.2 revises AD only—does not ship FR47 generators.
- 19.1 template is the ready-gate for Epic 20–24. Epic 20/21 need this epic’s contract/AD work; Epic 22 may need Bundle surface; Epic 24 needs NFR14.
