<!-- Preserved outside bmad:context (not replaced on refresh) -->
## Brand lock (2026-08-19)

- **Public product name: Bitloom**
- crates.io package + CLI binary: **`bitloom`**
- Design crates depend only on **`bitloom-prelude`** (never on the CLI package)
- Never publish `rhdl` / `rhdl-bits`; do not use `rhdl-rs` as the publish name
- Naming research: `_agile-output/planning-artifacts/research/technical-rhdl-rename-alternatives-product-naming-2026-08-19/research.md`
- Standalone path: `_agile-output/planning-artifacts/research/technical-cargo-bitloom-standalone-usage-after-ins-2026-08-20/research.md`
- AD-5 / FR47: toolchain may generate Rust functional-sim crates; Phase 12 SystemC TLM-2.0 **product path** (**FR101** / revised AD-5) — **Epic 46 closed** (LT-only MVP; AT deferred)
- AD-18 / NFR35: cycle-accurate path still rejects **capturing** closures (and heap/`dyn`/etc.); elaborate-time **non-capturing** `Fn` (or equiv.) allowed only if dissolved to HIR before freeze — must not enter `tick` as a Rust closure object (see ARCHITECTURE-SPINE AD-18)
- AD-20 / FR51: documented `Bundle` / `Vec<T,N>` (or equiv.) allowed on synthesizable path; width/dir fail before emit; FR22 surface thicken does not deliver composites (see ARCHITECTURE-SPINE AD-20)
- AD-25 / FR95: in-tree `#[hls]` scheduling allowed as a product path (revised AD-25); external Bambu optional — **Epic 41 closed** (FR95/FR96 MVP); **FR110 / Epic 52 closed** (commercial depth Q1+Q2; AD-25 revised 2026-09-10); **FR121 / Epic 62 closed** (Handshake default synthesizable; AD-25 revised 2026-09-10; Story 62.3); **FR129 / Epic 69 closed** (CIRCT Handshake + multi-clock elastic; AD-25 revised 2026-09-10; Story 69.3)
- AD-27 / FR28+FR46: FrozenHir/`.fir` → **compilable** Chisel Scala (mechanical OK); **FR97** adds idiomatic/maintainable acceptance (revised AD-27); no Scala `Parser.parse` requirement for FR28–FR130 Style Guide closes; historical NFR9 “no maintainable Chisel” is overturned — **Epic 42 closed** (FR97 MVP); **FR111 / Epic 53 closed** (deepen D1+D3; AD-27 revised 2026-09-10); **FR122 / Epic 63 closed** (official-style pack; AD-27 revised 2026-09-10 FR122; default still no Parser); **FR130 / Epic 70 closed** (Style Guide S1–S4; AD-27 revised 2026-09-10; Story 70.3; Parser not restored for that FR); **FR138 / Epic 77 closed** (Story 77.3; Parser product close via **`BitloomFirrtlParser.parse`**; AD-27 revised 2026-09-11; Story 77.2)
- FR98 / Epic 43: UART/SPI/I2C/AXI4-Lite near-VIP first-class IP — **Epic 43 closed** (MVP; GPIO optional not included); **FR108 / Epic 50 closed** (GPIO near-VIP); **FR120 / Epic 61 closed** (commercial VIP GPIO / `GpioVip`; full SoC pad etc. deferred)
- FR99 / Epic 44: keystroke full-elaborate Bitloom LSP (`bitloom-lsp`) — **Epic 44 closed** (FR99 MVP; FR90 rust-analyzer remains available and does not substitute); **FR113 / Epic 55 closed** (Cargo-graph + metadata `design_roots`); **FR118 / Epic 59 closed** (full-tree `#[bitloom::top]` syn-scan without metadata)
- FR100 / FR102 / FR103 / Epic 45: formal FL≡RTL product + multi-view attribute matrix + first-class IP dual-model — **Epic 45 closed** (MVP); **FR112 / Epic 54 closed** (GeneratedFunctional MemRead≡tick deepen; A/C deferred); **FR119 / Epic 60 closed** (SymbiYosys/`sby` F1-(ii); branch C more IP FL still deferred)
- FR101 / Epic 46: SystemC TLM-2.0 product path — **Epic 46 closed** (LT-only MVP via `emit_systemc_tlm_lt` / `gen-tlm`; AT deferred; ≠ FR47 Rust FL)
- FR104 / FR105 / Epic 47: interactive rich waveform + sim coverage extension — **Epic 47 closed** (FR104 `interactive.html`; FR105 coverage v2 Mux branch; C3 FSM cropped); **FR114 / Epic 56 closed** (LCOV + in-tree coverage GUI; Tywaves deferred)
- **Phase 15 / FR124–FR132:** NFR51 leftover deepen contract (Correct Course 2026-09-10) — **Epic 64 / FR124+FR132 gate closed** (Story 64.4); Phase 12–14 closes remain valid (**NFR52**); deepen epics 65–71 still need per-epic NFR14 (**NFR53**); AD-25→FR129 / AD-27→FR130 / CI sby→FR127 (**NFR54**); claims only via FR132
- **Phase 16 / FR133–FR140:** NFR55 final-closeout contract (Correct Course 2026-09-11) — **Epic 72 / FR133+FR140 gate closed** (Story 72.4); **Epic 73 / FR134 closed** (Story 73.3; G1–G4 upstream Tywaves GUI/IDE); **Epic 74 / FR135 closed** (Story 74.3; UartTx handwritten FL beyond Gpio); **Epic 75 / FR136 closed** (Story 75.3; ChipPadRing multi-peripheral / full-chip pad ring); **Epic 76 / FR137 closed** (Story 76.3; external CIRCT compile gate beyond FR129); **Epic 77 / FR138 closed** (Story 77.3; `BitloomFirrtlParser.parse` + AD-27 revise); **Epic 78 / FR139 closed** (Story 78.3; C1 `ip/gpio/` split; C2 not selected); Phase 12–15 closes remain valid (**NFR56**); Phase 16 planning stories Epic 72–78 complete (**NFR57**); AD-27→FR138 / external CIRCT→FR137 / crate boundary→FR139 (**NFR58**); claims only via FR140；further IP layout / deeper GUI·IDE subsets / unlisted protocols / broader CIRCT·MLIR lower / sim-gate deepen / deeper Chisel·Parser ecosystem still **NFR59**
- **Phase 17 / FR141–FR147:** API stability gate / Bitloom 1.0 (Correct Course 2026-09-11) — **Epic 79 / FR141+FR147 gate closed** (Story 79.4); **Epic 80–83 closed** (FR146 `v1.0.0`; library crates on crates.io); Phase 12–16 closes remain valid (**NFR60**); must not swallow NFR59 (**NFR63**); claims only via FR147；**CLI publish ≠ Phase 17** (→ Phase 18)
- **Phase 18 / FR148–FR153:** CLI / dependency crate crates.io publishability (Correct Course 2026-09-11) — **Epic 84 / FR148 gate closed** (Story 84.4); **Epic 85 / FR149–152 closed** (Story 85.6; `bitloom-firrtl` / `bitloom-viz` / `bitloom` 1.0.0 on crates.io; FR152(b)); **Epic 86 / FR153 closed** (Story 86.3; SemVer minor default / honesty); Phase 12–17 closes remain valid (**NFR64**); AD-2 `bitloom-*` (**NFR66**); must not swallow NFR59 (**NFR67**); claims only via FR148–153；**Phase 18 implementation stories complete**
- **Phase 19 / FR154–FR165:** NFR59 full subset upgrade + FR152(a) lsp live publish (Correct Course 2026-09-12) — **Epic 87 / FR154 gate closed** (Story 87.4); **Epic 88 / FR155 closed** (Story 88.4); **Epic 89 / FR157 closed** (Story 89.3); **Epic 90 / FR158 closed** (Story 90.3; genhtml LCOV GUI); **Epic 91 / FR159 closed** (Story 91.3; MemRead full emit); **Epic 92 / FR160 closed** (Story 92.3; non-Cargo path scan); **Epic 93 / FR161 closed** (Story 93.3; formal-sby hygiene); **Epic 94 / FR162 closed** (Story 94.3; default wave GUI primary); Epic 95–98 implement FR163–FR165 / FR156; Phase 12–18 closes remain valid (**NFR68**); must not claim before FR close (**NFR72**); AD-25/AD-27 touch via **NFR70**; leftover subsets need new contract (**NFR71**); `git push` is not an FR

## Process

- Future epics: **one story → one commit** (see `_agile-output/implementation-artifacts/process-one-story-one-commit.md`).

<!-- bmad:context -->
<!-- Verified 2026-08-18 against 7fe8d78. Managed by bmad-project-context; edits inside this block are replaced on refresh. Keep anything you want preserved outside the markers. -->

## rhdl (workspace; public brand Bitloom)

Rust 嵌入式 RTL HDL：设计是生成器，冻结 HIR 后再降后端。需求在 `docs/requirements/`，规划在 `_agile-output/planning-artifacts/`。公开产品名 **Bitloom**（crates.io / CLI：`bitloom`）。

## Policy

- 禁止向 crates.io 发布名 `rhdl` 或 `rhdl-bits`；发布名与 CLI 用 **`bitloom`**（不用 `rhdl-rs`）。设计 crate 只依赖 **`bitloom-prelude`**。文档须声明与 `samitbasu/rhdl` 无关。

## Where things are

- 改语言或工具链（将来的 `crates/`）：先读 `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md`，按 AD 做；不要另立 HIR，不要在 rustc 编译期抽网表。
- 给人讲架构：同目录 `team-walkthrough.html`。
- 技术依据：`_agile-output/planning-artifacts/research/technical-rhdl-rust-edsl-hdl-implementation-archit-2026-08-18/research.md`。
- 命名依据：`_agile-output/planning-artifacts/research/technical-rhdl-rename-alternatives-product-naming-2026-08-19/research.md`。

## Running and verifying

- TODO：按脊柱钉死的 rustc 1.97.1 / edition 2024 跑 `just test`，并 refresh 本块。公开 CLI：`cargo bitloom`（包 `bitloom`，二进制 `cargo-bitloom`）。

<!-- /bmad:context -->
