# Story 12.3: Trusted Publishing 与后续自动化

Status: done

## Story

As a 维护者,
I want 首发后用 OIDC Trusted Publishing + release-plz（可选 cargo-dist）,
so that 后续发版不必长期持有 crates.io token。

## Tasks

- [x] Document Trusted Publishing path in docs/crates-io-publish-bitloom.md
- [x] Add `.github/workflows/release-plz.yml` scaffold + dry documentation job
- [x] Note: enable Trusted Publishing on crates.io **after** Story 12.2 live publish

## Dev Agent Record

### File List

- .github/workflows/release-plz.yml
- docs/crates-io-publish-bitloom.md
