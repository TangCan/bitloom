# FR153 — SemVer assume-published & release honesty

> **Contract:** Phase 18 / **FR153** / Epic 86 / Story 86.2.  
> **Policy:** [`docs/semver-1-0-policy.md`](semver-1-0-policy.md).  
> **Release checklist:** [`docs/fr146-bitloom-1-0-0-release.md`](fr146-bitloom-1-0-0-release.md).  
> **CLI publish:** [`docs/fr151-bitloom-cli-publish.md`](fr151-bitloom-cli-publish.md).

## What changed

- **Removed** the temporary `1.0.0` → `--release-type major` branch (and the need to set `BITLOOM_SEMVER_ASSUME_PUBLISHED=1`) in `scripts/semver-check.sh`.
- Workspace version **≥1.0.0** now defaults to `--release-type minor` under `just semver-check` / CI `semver-check`.
- Pre-1.0 (`0.x`) still defaults to **major**. Override with `BITLOOM_SEMVER_RELEASE_TYPE`.

## Honesty matrix (library vs CLI)

| Surface | Status | Evidence |
| --- | --- | --- |
| Library crates (`bitloom-macro` / `hir` / `builder` / `vlog` / `sim` / `prelude`) | **1.0.0 on crates.io** | Phase 17 / FR146 |
| CLI `bitloom` (+ `bitloom-firrtl` / `bitloom-viz`) | **1.0.0 on crates.io**; `cargo install bitloom` | Phase 18 / FR149–151 |
| SemVer gate default | **minor** for ≥1.0.0 | This FR / Story 86.2 |
| **NFR59** deepen leftovers | **Still deferred** | NFR67 — not cleared by CLI publish or SemVer honesty |

## Claims boundary

- 「CLI 已可从 crates.io 安装」→ cite **FR151** (not Phase 17 alone).
- 「SemVer 检查默认已按已发布 1.0 处理」→ cite **FR153**.
- Do **not** imply **NFR59** is empty; do **not** silently expand **FR142**.

## Brand

Public product **Bitloom**; crates.io / CLI **`bitloom`**.
