# Digest: Landscape — round 1 (subagent)

**Source agent:** Landscape cargo-install patterns  
**Accessed:** 2026-08-20

## Additive claims (beyond lead digest)

- Rust Book ch14-04: cargo install framed for binary crates — https://doc.rust-lang.org/book/ch14-04-installing-binaries.html
- Rust CLI Book packaging: multi-channel distribution (cargo install → prebuilts → OS packages) — https://rust-cli.github.io/book/tutorial/packaging.html
- cargo-expand: install then optional rustfmt component; may need nightly for expansion — https://crates.io/crates/cargo-expand
- wasm-pack: CLI install still requires rustc (+ npm for some flows) — https://rustwasm.github.io/docs/wasm-pack/prerequisites/index.html
- rustc/toolchain via rustup, not cargo install — rustup book / Rust Book install
- cargo-binstall as prebuilt alternative — https://docs.rs/crate/cargo-binstall/latest

## Verdict alignment
Agrees: install-to-PATH / cargo-subcommand is dominant; “fully standalone” only if binary embeds needs OR published library crates exist for user projects. Bitloom today is installable subcommand but not mdBook-complete.

## Tutorial skeleton (landscape)
rustup → cargo install bitloom → verify help → user project → cargo add language crates (when published) → first emit. Do not promise no Rust toolchain.
