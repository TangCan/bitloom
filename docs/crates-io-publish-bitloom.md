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

### A. crates.io (human, once)

1. Open https://crates.io/crates/bitloom/settings → **Trusted Publishing**
2. Add GitHub Actions publisher:
   - Repository: `TangCan/bitloom`
   - Workflow: `.github/workflows/release-plz.yml` (or the filename you enable)
   - Environment: optional
3. Optionally enable **Trusted Publishing Only** after first OIDC publish succeeds

### B. GitHub

1. Ensure this git remote points at `https://github.com/TangCan/bitloom.git` and Actions are enabled
2. Push `master` / tags so the workflow file is on the default branch
3. After Trusted Publishing is saved on crates.io, uncomment the live `release-plz` job in `.github/workflows/release-plz.yml` (see comments in that file)

### C. Verify

- Dispatch `release-plz` workflow manually once; confirm it can obtain an ephemeral crates.io token via OIDC (or open a release PR without long-lived `CARGO_REGISTRY_TOKEN`)

See closeout research: `_agile-output/planning-artifacts/research/technical-rhdl-clean-product-closeout-and-crates-i-2026-08-19/research.md` (package name = `bitloom`).
