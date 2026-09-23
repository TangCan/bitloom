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
- **Phase 19 / FR154–FR165:** NFR59 full subset upgrade + FR152(a) lsp live publish (Correct Course 2026-09-12) — **Epic 87 / FR154 gate closed** (Story 87.4); **Epic 88 / FR155 closed** (Story 88.4); **Epic 89 / FR157 closed** (Story 89.3); **Epic 90 / FR158 closed** (Story 90.3; genhtml LCOV GUI); **Epic 91 / FR159 closed** (Story 91.3; MemRead full emit); **Epic 92 / FR160 closed** (Story 92.3; non-Cargo path scan); **Epic 93 / FR161 closed** (Story 93.3; formal-sby hygiene); **Epic 94 / FR162 closed** (Story 94.3; default wave GUI primary); **Epic 95 / FR163 closed** (Story 95.3; UartRx handwritten FL); **Epic 96 / FR164 closed** (Story 96.3; CIRCT sim gate); **Epic 97 / FR165 closed** (Story 97.3; Style Guide/linter deepen); **Epic 98 / FR156 closed** (Story 98.3; claim honesty); Phase 19 planning+implementation stories complete (Epic 87–98); Phase 12–18 closes remain valid (**NFR68**); claims cite FR154–165 (**NFR72**); leftover subsets need new contract (**NFR71**); `git push` is not an FR
- **Phase 20 / FR166–FR171:** NFR71 four leftovers upgrade (Correct Course 2026-09-12) — **Epic 99 / FR166 gate closed** (Story 99.4); **Epic 100 / FR167 closed** (Story 100.3; ChiselSim + Open VSX/JetBrains); **Epic 101 / FR168 closed** (Story 101.3; SPI+I2C+AXI handwritten FL); **Epic 102 / FR169 closed** (Story 102.3; multi-lower / HW dialect allocation @ firtool-1.155.0; firtool bump deferred NFR76); **Epic 103 / FR170 closed** (Story 103.3; update-mainline Parser / FIRRTL 6.0.0; AD-27 revised; unpaired HEAD still NFR76); **Epic 104 / FR171 closed** (Story 104.3; claim honesty); Phase 20 planning+implementation stories complete (Epic 99–104); Phase 12–19 closes remain valid (**NFR73**); AD-9/AD-25/AD-27 / store honesty (**NFR75**); claims cite FR166–171 (**NFR77**); beyond NFR14 subsets still need new contract (**NFR76**); must not claim NFR71 ledger empty; `git push` is not an FR
- **Phase 21 / FR172–FR177:** NFR76 leftovers upgrade (Correct Course 2026-09-12) — **Epic 105 / FR172 gate closed** (Story 105.4); **Epic 106 / FR173 closed** (Story 106.3; firtool-1.158.0 ↔ Chisel 7.15.0; AD-9 revised); **Epic 107 / FR174 closed** (Story 107.3; unpaired firtool-1.156.0 channel); **Epic 108 / FR175 closed** (Story 108.3; `--ir-sv`/`--ir-verilog`); **Epic 109 / FR176 closed** (Story 109.3; combined Style Guide + update-mainline ecosystem; AD-27 revised); **Epic 110 / FR177 closed** (Story 110.3; claim honesty); Phase 21 planning+implementation stories complete (Epic 105–110); Phase 12–20 closes remain valid (**NFR78**); AD-9/AD-27 / firtool·HEAD touch honesty (**NFR80**); claims cite FR172–177 (**NFR82**); beyond NFR14 subsets still need new contract (**NFR81**); must not claim Phase 20 alone delivers FR173–176; `git push` is not an FR
- **Phase 22 / FR178–FR184:** NFR81 leftovers upgrade (Correct Course 2026-09-12) — **Epic 111 / FR178 gate closed** (Story 111.4); **Epic 112 / FR179 closed** (Story 112.3; floating-track firtool-1.159.0); **Epic 113 / FR180 closed** (Story 113.3; Handshake fork+join deepen); **Epic 114 / FR181 closed** (Story 114.3; Style/linter wartremover deepen); **Epic 115 / FR182 closed** (Story 115.3; unpaired product-pin firtool-1.159.0); **Epic 116 / FR183 closed** (Story 116.3; explicit FR142 surface expand); **Epic 117 / FR184 closed** (Story 117.3; claim honesty); Phase 22 planning+implementation stories complete (Epic 111–117); Phase 12–21 closes remain valid (**NFR83**); claims cite FR178–184 (**NFR87**); AD-9 / AD-25 / AD-27 / FR142 touch honesty (**NFR85**); beyond NFR14 still **NFR86**; must not claim NFR81 ledger empty; `git push` is not an FR
- **Engineering / contract closeout (Correct Course 2026-09-14):** `engineeringCloseoutApproved` / `engineeringCloseoutStatus: complete` — Phase 12–22 closes remain valid; deepen leftovers authorized only via **Phase 23** (not by closeout alone); proposal `_agile-output/planning-artifacts/sprint-change-proposal-2026-09-14-engineering-closeout.md`; `git push` is not an FR
- **Phase 23 / FR185–FR191:** NFR86 leftovers upgrade (Correct Course 2026-09-14) — **Epic 118 / FR185 gate closed** (Story 118.4); **Epic 119 / FR186 closed** (Story 119.3; live-tip channel); **Epic 120 / FR187 closed** (Story 120.3; Handshake lower branch+merge); **Epic 121 / FR188 closed** (Story 121.3; community Style Guide pack); **Epic 123 / FR190 closed** (Story 123.3; further FR142 expand); **Epic 122 / FR189 deferred / not delivered** (Correct Course defer-close 2026-09-14; further product-pin bump → **NFR91**); **Epic 124 / FR191 closed** (Story 124.3; claim honesty); Phase 23 **planning** stories complete (Epic 118–124); implementation honesty keeps FR189 undelivered; Phase 12–22 + closeout remain valid (**NFR88**); claims cite FR185–191 (**NFR92**); beyond NFR14 still **NFR91**; must not claim NFR86 ledger empty; `git push` is not an FR
- **Phase 23 honest closeout (Correct Course 2026-09-14):** `phase23CloseoutApproved` / `phase23CloseoutStatus: complete` — live tip == firtool-1.159.0 (current latest; == FR182 pin) ⇒ FR189 (strictly >1.159.0) remains **undelivered**; sprint may stop; must not claim FR189 delivered or NFR91 ledger empty; proposal `_agile-output/planning-artifacts/sprint-change-proposal-2026-09-14-phase23-honest-closeout.md`; `git push` is not an FR

- **Phase 24 / FR192–FR201 / NFR93–NFR99（2026-09-20）：** 完整规划已批准，Epic125–130 正式登记；**M0 / Epic125 已关闭**（Story125.3，FR192/FR193）；用户随后要求继续，当前仅授权 **Story126.1 NFR14 → 126.2 模块组合基础**；Epic126 in-progress，**126.1/126.2 done（FR194）**，组合与实测见 `docs/ip/module-composition.md` / `module-composition-evidence.md`；126.3/126.4 与 Epic127–130 保持 backlog，未授权自动执行，不宣称 M1 完成。接口 `docs/ip/phase24-contract.md`；M0 闭合后仍需各 Epic .1 NFR14 done。新增 API 不自动扩 FR142；整个 Phase24 尚未交付。历史关闭与 Epic122 / FR189 deferred 保持，NFR91 未清空。

## Process

- Future epics: **one story → one commit** (see `_agile-output/implementation-artifacts/process-one-story-one-commit.md`).

<!-- bmad:context -->
<!-- Verified 2026-09-20 against 7a8135f. Managed by bmad-project-context; edits inside this block are replaced on refresh. Keep anything you want preserved outside the markers. -->

## rhdl (workspace; public brand Bitloom)

Rust 嵌入式 RTL HDL：设计是生成器，冻结 HIR 后再降后端。需求在 `docs/requirements/`，规划在 `_agile-output/planning-artifacts/`。公开产品名 Bitloom（crates.io 包：`bitloom`；命令：`cargo bitloom`）。

## Policy

- 禁止向 crates.io 发布名 `rhdl` 或 `rhdl-bits`，不用 `rhdl-rs`；公开发布名用 `bitloom`。设计 crate 只依赖 `bitloom-prelude`，不得依赖 CLI 包。文档须声明与 `samitbasu/rhdl` 无关。

## Where things are

- 改语言或工具链（`crates/`）：先读 `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md`，按 AD 做；不要另立 HIR，不要在 rustc 编译期抽网表。
- 给人讲架构：同目录 `team-walkthrough.html`。
- 技术依据：`_agile-output/planning-artifacts/research/technical-rhdl-rust-edsl-hdl-implementation-archit-2026-08-18/research.md`。
- 命名依据：`_agile-output/planning-artifacts/research/technical-rhdl-rename-alternatives-product-naming-2026-08-19/research.md`。

## Running and verifying

- 使用 `rust-toolchain.toml` 与根 `Cargo.toml` 钉死的 Rust 1.97.1 / edition 2024。
- 工作区测试用 `just test`（`cargo test --workspace`）；裸 `cargo test` 只覆盖 `default-members`，会漏掉 CLI、LSP 等成员。
- `just check` 增加格式检查，但不覆盖独立的 JVM、formal、CIRCT 等门禁；改相关路径时，按 `Justfile` 和 `.github/workflows/ci.yml` 选择对应门禁，不能把工作区测试通过视为全部 CI 通过。
- 公开 CLI：`cargo bitloom`（包 `bitloom`，实际二进制 `cargo-bitloom`）；树内运行用 `cargo run -p bitloom -- …`。

<!-- /bmad:context -->

## 当前整体执行授权（2026-09-20）

用户明确要求按create-story→ATDD→build→code-review→automate→clean/fmt/regression→commit七步连续处理sprint-status全部未完成Story。此授权取代旧的M0-only或仅126.1/126.2执行安排；各故事仍须满足自身依赖、NFR14及真实验证，一故事一提交。已有done保留；deferred不得假交付。122.2/122.3在2026-09-20实时核验后仍因没有已发布firtool>1.159.0而阻塞；继续其他17个可执行故事，待前置满足后再恢复FR189。整体目标尚未完成，不推送或发布。

## Phase24 当前交付状态（2026-09-20，Story126.4）

Epic126/M1已关闭：126.1–126.4 done，FR194共享session组合与FR195 RvRegSlice/ParamSyncFifo真实验收；见 `_agile-output/implementation-artifacts/epic-126-closeout.md`。用户全部未完成故事七步连续执行授权仍有效，下一故事127.1；Epic127–130未交付，整个Phase24未完成。FR189/Epic122仍deferred，NFR91与既有明确遗留保持。

## M2 / Epic127 实施关闭（2026-09-21）

Story127.1–127.4随各自单故事提交完成，FR196静态CSR描述/软件地址产物、AXI-Lite桥和固定四窗CsrDecoder交付。127.4实际clean/fmt/justtest为470结果块、1823 passed / 0 failed / 25 ignored，专用decoder/CSR/bridge形式与综合本轮另跑通过，不把ignored计PASS。关闭映射 `_agile-output/implementation-artifacts/epic-127-closeout.md`，完整验收 `_agile-output/test-artifacts/127-4-final-verification.md`。新增API逐符号登记FR142/minor，未改工具钉、包版本或发布。

真实七模块夹具验证总线/CSR语义，不代表Epic128外设算法或Epic129系统；native/generated层级仍unsupported。Epic128–130/FR197–201及整个Phase24未交付；下一故事128.1 NFR14，全部未完成故事七步授权继续有效。FR189/Epic122 deferred和NFR91保持，不push、不publish。此前各时点交付状态保留为历史。

## M3 / Epic128 实施关闭（2026-09-21）

128.1–128.5七步完成，随Story128.5单独提交关闭Epic128/FR197/M3：Timer32、五源IRQ、GPIO32 CSR及双FIFO UART CSR交付。UART最终实际clean/fmt/just test：482个结果块、1870 passed / 0 failed / 47 ignored；7项新专用backend/formal入口另有实际PASS，未把ignored计PASS。当前39命令完整定向证据、独立原始归档与关闭映射见`_agile-output/test-artifacts/128-5-final-verification.md`及`_agile-output/implementation-artifacts/epic-128-closeout.md`。

新增符号逐项登记FR142/minor，旧UART/GPIO/VIP/手写FL保持。只关闭M3，Epic129/FR198与核心FR201、Epic130/FR199–200及整个Phase24仍开放，下一129.1；native/generated层级与有限形式/物理边界保持。用户全部未完故事七步授权继续有效，FR189 deferred/NFR91保留；未改工具钉/包版本、不push/publish。此前各时点记录保留为历史。

## M5 / Epic130 外部试点关闭（2026-09-23）

Story130.1–130.3完成；FR199来源锁/只读禁网重放与FR200真实Bitloom父模块/FIFO独立oracle、上游六组各100000比较交付。130.2独立clean/fmt/justtest：1889 passed / 0 failed / 50 ignored；130.3：1893 passed / 0 failed / 55 ignored，专用真实工具门禁另跑，ignored不计PASS。见`_agile-output/implementation-artifacts/epic-130-closeout.md`与两Story最终验证记录。

外部支持行仅固定32-bit/depth8/fall-through0提升maintained；native/generated层级unsupported。CLI新增入口逐项FR142/minor登记，无产品工具钉/包版本/push/publish。FR189/Epic122继续deferred，NFR91保留；本记录不宣称整个Phase24关闭。全部未完成Story七步授权继续有效。


## Phase24 当前结项状态（2026-09-23）

2026-09-23：Phase24 已批准范围 FR192–FR201 / NFR93–NFR99 完成交付与结项，Epic125–130 共22故事done。129.3四层独立补审及修复、两拓扑六路线实际RTL/原始综合、有限请求/复位形式、耐清理的主/隔离原始证据和新鲜证据校验已完成；FR199/FR200外部试点按130.2/130.3最终记录交付。FR189/Epic122/122.2/122.3继续deferred、未交付；NFR91及既有明确遗留保留。无产品工具钉/包版本变更，不push、不publish，不声称远端CI已执行或物理签核。

[总验收与FR/NFR映射](docs/ip/phase24-closeout.md)。先前M0-only、126.1/126.2-only、Epic130开放或Phase24未完成的分日期段落保留为历史，本条为当前状态。
