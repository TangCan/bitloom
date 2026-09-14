# Bitloom 1.1.0 release checklist

> **Kind:** SemVer **minor** after 1.0.0 (FR143 / FR153).  
> **Surface:** [`docs/public-api-1-0-surface.md`](public-api-1-0-surface.md) (FR142 + FR183 + FR190).  
> **Policy:** [`docs/semver-1-0-policy.md`](semver-1-0-policy.md).  
> **Prior:** [`docs/fr146-bitloom-1-0-0-release.md`](fr146-bitloom-1-0-0-release.md).

## Preconditions

- [x] Phase 18–23 planning/honesty closed (FR189 deferred / undelivered)
- [x] Workspace version **1.1.0**
- [x] `CHANGELOG.md` section `[1.1.0]`
- [x] Annotated tag **`v1.1.0`** on the release commit
- [x] `just semver-check` green (`--release-type minor`)
- [x] Sequential `cargo publish` for publishable packages (2026-09-14)

## Live publish results (2026-09-14)

| Package | Result |
| --- | --- |
| bitloom-macro | **Published** 1.1.0 |
| bitloom-hir | **Published** 1.1.0 |
| bitloom-builder | **Published** 1.1.0 |
| bitloom-vlog | **Published** 1.1.0 |
| bitloom-sim | **Published** 1.1.0 |
| bitloom-prelude | **Published** 1.1.0 |
| bitloom-firrtl | **Published** 1.1.0 |
| bitloom-viz | **Published** 1.1.0 |
| bitloom | **Published** 1.1.0 |
| bitloom-lsp | **Published** 1.1.0 |

Install: `cargo install bitloom` / `cargo install bitloom-lsp` (after index propagate).

## Publish order (dependency-aware)

```bash
export PATH="$HOME/.cargo/bin:$PATH"
for p in \
  bitloom-macro \
  bitloom-hir \
  bitloom-builder \
  bitloom-vlog \
  bitloom-sim \
  bitloom-prelude \
  bitloom-firrtl \
  bitloom-viz \
  bitloom \
  bitloom-lsp
do
  cargo publish -p "$p" --dry-run
done
# then the same loop without --dry-run
```

| Package | Role |
| --- | --- |
| bitloom-macro | leaf |
| bitloom-hir | leaf (out-of-promise Q2) |
| bitloom-builder | needs hir |
| bitloom-vlog | needs hir/builder |
| bitloom-sim | needs hir/builder |
| bitloom-prelude | design-facing |
| bitloom-firrtl | FR183/FR190 surface |
| bitloom-viz | viz |
| bitloom | CLI |
| bitloom-lsp | FR155 |

## Tag

```bash
git tag -a v1.1.0 -m "Bitloom 1.1.0 — Phase 18–23 additive deepen (minor)"
```

## Honesty

- Public product **Bitloom** / `bitloom-*`
- **1.1.0** = additive deepen since 1.0.0; **≠** FR189 delivered; **≠** NFR91 empty
- `git push` is ops, not an FR (already requested by maintainer for this release)
