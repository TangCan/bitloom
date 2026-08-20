# Digest: Integration — round 1 (subagent)

**Source agent:** Integration crates.io bitloom  
**Accessed:** 2026-08-20

## Additive claims (beyond lead digest)

- C10: cargo-nextest documents `cargo install --locked cargo-nextest` then use on any project without cloning — https://nexte.st/docs/installation/from-source/
- C11: Spade Swim: `cargo install --git …/swim` then `swim install-tools`; user projects via Swim — https://docs.spade-lang.org/guide_installation.html
- C12: sus_compiler: `cargo install sus_compiler` then compile user `.sus` files; stdlib via install/XDG — https://crates.io/crates/sus_compiler
- Confirmed: trustpub_data null / trustpub_only false — no end-user install UX impact
- GitHub raw README matches crates.io README monorepo quickstart

## Verdict alignment
Agrees with lead: bin-only install; no honest standalone HDL tutorial on published surface.

## Gaps noted by agent
- Did not live-run install (lead later did empty-dir build fail)
- Sibling crate inventory beyond bitloom not fully scanned
