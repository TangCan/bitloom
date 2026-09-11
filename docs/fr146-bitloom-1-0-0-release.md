# FR146 — Bitloom 1.0.0 release checklist

> **Contract:** Phase 17 / **FR146** / Epic 83 / Story 83.2.  
> **Surface:** [`docs/public-api-1-0-surface.md`](public-api-1-0-surface.md).  
> **Policy:** [`docs/semver-1-0-policy.md`](semver-1-0-policy.md).

## Preconditions

- [x] FR141–FR144 closed; FR145-skip closed (`docs/fr145-pre-1-0-hygiene-skip.md`)
- [x] Workspace version **1.0.0**
- [x] `CHANGELOG.md` section `[1.0.0]`
- [x] `cargo fmt` / `just test` green on release commit
- [x] `cargo doc -p bitloom` succeeds
- [x] Annotated tag **`v1.0.0`** on the release commit

## Dry-run results (2026-09-11)

Preferred order (dependency-aware):

```bash
export PATH="$HOME/.cargo/bin:$PATH"
for p in bitloom-macro bitloom-hir bitloom-builder bitloom-vlog bitloom-prelude bitloom-sim bitloom; do
  cargo publish -p "$p" --dry-run
done
```

| Package | dry-run | Notes |
| --- | --- | --- |
| bitloom-macro | **OK** | leaf; `--allow-dirty` during WIP |
| bitloom-hir | **OK** | out-of-promise; leaf vs registry |
| bitloom-builder | blocked | needs `bitloom-hir` **1.0.0** on crates.io first |
| bitloom-vlog | blocked | needs `bitloom-hir` **1.0.0** on crates.io first |
| bitloom-prelude | blocked | needs builder/hir **1.0.0** on crates.io first |
| bitloom-sim | blocked | needs hir **1.0.0** on crates.io first |
| bitloom | **OK** (FR151) | live publish 2026-09-11 after `bitloom-firrtl` / `bitloom-viz` 1.0.0 |

**FR146 acceptance path used:** version bump + CHANGELOG + annotated tag + successful dry-run of publishable leaves + this checklist for sequential crates.io upload. Live `cargo publish` may follow when credentials and dependency publishes are available; absence of live upload does **not** undo the SemVer **1.0.0** tree state.

## Tag

```bash
git tag -a v1.0.0 -m "Bitloom 1.0.0 — public API stability gate (FR146)"
```

## crates.io publish (manual follow-up)

```bash
cargo publish -p bitloom-macro
cargo publish -p bitloom-hir
cargo publish -p bitloom-builder
cargo publish -p bitloom-vlog
cargo publish -p bitloom-prelude
cargo publish -p bitloom-sim
cargo publish -p bitloom  # FR151 — done 2026-09-11
```

After **1.0.0** is on crates.io, set `BITLOOM_SEMVER_ASSUME_PUBLISHED=1` (or remove the 1.0.0 special-case) so `just semver-check` defaults to `--release-type minor` — **FR153 / Epic 86**.

## Brand / honesty

- Public product **Bitloom** / `bitloom`
- 「1.0 / 公开 API 稳定」claims require FR141–146 (**FR147**)
- **NFR59** remains deferred (**NFR63**)
- **CLI `cargo install bitloom`:** **FR151** closed (Epic 85); SemVer assume-published honesty → **FR153**
