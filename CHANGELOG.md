# Changelog

All notable changes to the **Bitloom** (`bitloom`) publish surface are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
From **1.0.0**, in-surface breaking changes require a **major** bump
([`docs/semver-1-0-policy.md`](docs/semver-1-0-policy.md) / FR143). Pre-1.0 history
followed Cargo **0.x** compatibility rules.

## [Unreleased]

### 1.2.0 发布候选（尚未上传 crates.io）

#### 新增

- Phase24 可组合 IP：共享展开 session、模块实例校验、两槽 ready/valid 注册切片、参数同步 FIFO（FR194–195）。
- 静态 CSR 描述及软件地址产物、AXI-Lite 桥、四窗译码（FR196）；Timer32、五源 IRQ、GPIO32 CSR、双 FIFO UART CSR（FR197）。
- 两种外设子系统拓扑、组合与后端验收配方（FR198/FR201）。
- 外部 IP 来源清单、锁定、离线验证/隔离重放及一个真实外部 FIFO 试点（FR199–200）。

#### 修复与变更

- AXI 独立请求捕获与事务保持；外部缓存重锁失败保留已有闭包，拒绝缓存祖先符号链接越界。
- 发布家族统一至 **1.2.0**，生成的功能/周期仿真 crate 依赖跟随同版家族。
- 新增公开接口按 FR142/FR143 登记为 SemVer **minor**；历史 1.1.x 发布保持有效。

#### 范围

- 发布候选验证与最终操作见[1.2.0 发布清单](docs/bitloom-1-2-0-release.md)。此段不表示已发布。
- FR189/Epic122 仍 deferred，NFR91 保留；层级 native/generated 仿真仍 unsupported，物理签核/上板未完成，不扩展外部 IP 试点支持矩阵。

## [1.1.0] - 2026-09-14

### Added

- Phase 18–23 deepen shipped since 1.0.0 (additive / minor under FR143):
  - CLI + `bitloom-firrtl` / `bitloom-viz` / `bitloom-lsp` crates.io install paths (FR149–151 / FR155)
  - Coverage / wave / IP / CIRCT / Chisel Style·Parser deepen (FR157–165, FR167–176, FR179–181, FR186–188)
  - Explicit FR142 in-surface expands for `bitloom-firrtl` interop (`emit` / `import` / `check_*` family) — FR183 + FR190
- AD-9 product pin / channels through **firtool-1.159.0** (FR173 / FR182; live-tip channel FR186)
- Phase 19–23 claim-honesty docs (`docs/fr156`…`fr191`)

### Changed

- Workspace package version **1.0.0 → 1.1.0** (lockstep for publishable Bitloom crates)
- Public surface doc: [`docs/public-api-1-0-surface.md`](docs/public-api-1-0-surface.md) (FR183 / FR190)

### Notes

- **Minor** release: additive in-surface expands; no intentional breaking of the FR142 1.0 promise
- **FR189** (firtool product pin **strictly > 1.159.0**) remains **undelivered** / deferred → **NFR91**
- Beyond-NFR14 deepen leftovers remain **NFR91**; must not claim NFR86/NFR91 ledger empty
- crates.io upload: see [`docs/bitloom-1-1-0-release.md`](docs/bitloom-1-1-0-release.md)

## [1.0.0] - 2026-09-11

### Added

- Phase 17 API stability gate: public surface (`docs/public-api-1-0-surface.md`), SemVer 1.0 policy, `just semver-check` / CI FR144 gate, FR145-skip hygiene decision
- SemVer **1.0.0** promise for in-surface crates: `bitloom`, `bitloom-prelude`, `bitloom-macro`, `bitloom-sim`

### Changed

- Workspace package version **0.1.2 → 1.0.0** (lockstep for publishable Bitloom crates)
- Post-1.0 default `cargo-semver-checks` release-type **minor** (override via `BITLOOM_SEMVER_RELEASE_TYPE`)

### Notes

- Out-of-promise crates (`bitloom-hir` / `bitloom-builder` / `bitloom-vlog`) may share the workspace version but are **not** covered by the 1.0 SemVer stability promise (Q2)
- **NFR59** product deepen items remain deferred (NFR63); 1.0 ≠ empty backlog
- crates.io upload: see `docs/fr146-bitloom-1-0-0-release.md` (dry-run + manual checklist)

## [0.1.2] - 2026-08-20

### Added

- True-standalone path: `cargo install bitloom` → `cargo bitloom new` → `cargo bitloom build` without cloning the monorepo
- Host shim resolves `--package` via `cargo metadata` and pins `bitloom-vlog` / `bitloom-hir` from crates.io outside the monorepo
- MVP library family on crates.io: `bitloom-{hir,builder,macro,prelude,vlog}` (lockstep 0.1.2)
- Optional `bitloom-sim` 0.1.2 for `cargo add bitloom-sim --dev` cycle-accurate tick

### Changed

- Design crates depend on **`bitloom-prelude`** (not CLI `bitloom`); README quick start is install-first

## [0.1.1] - 2026-08-19

### Added

- Crate metadata: `repository` / `homepage` / `documentation` → https://github.com/TangCan/bitloom

## [0.1.0] - 2026-08-19

### Added

- Initial crates.io registration: [`bitloom`](https://crates.io/crates/bitloom) 0.1.0
- Public brand **Bitloom**; binary `cargo-bitloom` (`cargo bitloom`)
- Architecture AD-2 publish identity locked to `bitloom`
- Maturity contract: SECURITY.md, SemVer 0.x policy, CI on rustc 1.97.1
