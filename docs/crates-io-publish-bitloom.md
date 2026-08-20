# crates.io publish runbook — `bitloom`

**Repository:** https://github.com/TangCan/bitloom

## Preconditions (Story 12.1)

- [x] AD-2 / README identity Bitloom
- [x] `SECURITY.md`, `CHANGELOG.md`, `docs/semver-0x-policy.md`
- [x] MSRV 1.97.1 via `rust-toolchain.toml` + `just test`
- [x] README deferred / non-goals honest
- [x] `repository` / `homepage` / `documentation` on `crates/bitloom/Cargo.toml`

## First publish (Story 12.2)

1. Re-probe: `curl -sA 'bitloom' -o /dev/null -w '%{http_code}\n' https://crates.io/api/v1/crates/bitloom` → **200** after 0.1.0
2. Review https://crates.io/policies
3. Metadata: description, license, repository, readme, keywords≤5, categories, authors, rust-version
4. `cargo publish -p bitloom --dry-run`
5. Manual: `cargo publish -p bitloom` — **done 2026-08-19** (`bitloom` 0.1.0)
6. Tag `v0.1.0` + CHANGELOG — **done**

## 0.1.1 metadata patch

1. Bump workspace `version` to `0.1.1`
2. `cargo publish -p bitloom --dry-run`
3. `cargo publish -p bitloom`
4. `git tag -a v0.1.1 -m "bitloom 0.1.1"`

## Trusted Publishing + release-plz (Story 12.3)

- [x] crates.io Trusted Publishing linked to `TangCan/bitloom` + `.github/workflows/release-plz.yml` (2026-08-19)
- [x] Workflow live: `workflow_dispatch` + push to `main`/`master`; uses OIDC (`id-token: write`), no long-lived registry token

### Verify

- Actions → **release-plz** → Run workflow, or push to `main`
- Prefer `release-pr` first; switch action `command` to `release` when ready for direct publish

## MVP library family (Story 13.2 / NFR22)

Published crates (lockstep with CLI when possible):

| crates.io name | role |
|---|---|
| `bitloom` | CLI (`cargo-bitloom`) |
| `bitloom-prelude` | design `[dependencies]` only |
| `bitloom-hir` / `bitloom-builder` / `bitloom-macro` | transitive via prelude |
| `bitloom-vlog` | host emit backend |

**Manual lockstep (when release-plz does not yet cover every crate):**

1. Bump workspace `version` once.
2. Dry-run then publish deps first: `bitloom-hir` → `bitloom-builder` → `bitloom-macro` → `bitloom-prelude` → `bitloom-vlog` → `bitloom`.
3. Ensure crates.io Trusted Publishing (or API token) is configured for **each** published package name, not only `bitloom`.
4. Never publish `rhdl` / `rhdl-bits` / user-facing `rhdl-prelude`.

`bitloom-sim` (optional Epic 14) follows the same lockstep when enabled. Publish order after `bitloom-hir`: `bitloom-sim` (depends only on hir).
