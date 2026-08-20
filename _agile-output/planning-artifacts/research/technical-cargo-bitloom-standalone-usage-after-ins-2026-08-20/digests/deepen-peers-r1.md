# Digest: Deepen — peer standalone patterns (subagent)

**Source:** Peer standalone publish patterns  
**Accessed:** 2026-08-20

## Additive claims

- Swim docs: per-project compiler pin (`swim.lock`); `swim init` scaffolds; git install — **not** crates.io-complete standalone.
- SUS: `cargo install sus_compiler` + companion crates; stdlib under XDG at install — binary+data layout pattern.
- wasm-bindgen / wasm-bindgen-cli: separate packages, lockstep versions; wasm-pack `new` via cargo-generate.
- crates.io no rename; product prefix when short names taken; `[lib] name` / `[[bin]] name` for ergonomics.
- Hard rule: published manifests must not use path/git into monorepo for user-facing artifacts.

## Tension with Bitloom AD-6
Subagent suggested user `cargo add bitloom` umbrella. Spine forbids design depending on CLI. Prefer **`bitloom-prelude` as the user-facing library** (umbrella re-exports ok *inside* prelude); keep package `bitloom` CLI-only or document clearly if lib+bin ever combined.

## Verdict alignment
Confirms deepen route A (bindgen-style registry family); Spade = interim git pattern only.
