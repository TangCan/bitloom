# Story 12.2: 首次手动发布 bitloom

Status: done

## Story

As a 维护者,
I want 将至少一个对外 crate 以 `bitloom` 手动首发到 crates.io,
so that 产品有真实 registry 身份且不占用 `rhdl`。

## Acceptance Criteria

1. Re-probe `bitloom` free (404) — **done 2026-08-19**
2. C-METADATA + `cargo publish -p bitloom --dry-run` — **done**
3. Manual `cargo publish -p bitloom` — **done 2026-08-19** (`Published bitloom v0.1.0 at registry crates-io`)
4. Policies reviewed — see docs/crates-io-publish-bitloom.md

## Tasks

- [x] Remove unpublished path deps from bitloom (inline HLS)
- [x] dry-run green
- [x] Human: `cargo publish -p bitloom`
- [x] Tag `v0.1.0` + CHANGELOG dated

## Dev Agent Record

### Completion Notes List

- Live publish confirmed via maintainer terminal: Uploaded/Published bitloom v0.1.0
- https://crates.io/crates/bitloom

### File List

- crates/bitloom/**, CHANGELOG.md, sprint-status.yaml
