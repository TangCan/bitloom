# Action-items sweep — 2026-09-10

Cleared **31** open Phase-12 retro action items (epics 40–47, items 99–129). Authoritative dispositions also landed in `deferred-work.md` and `process-one-story-one-commit.md`; `epics.md` frontmatter `phase12Epic40Status`–`phase12Epic47Status` → `complete`.

| ID | Short theme | Disposition |
|----|-------------|-------------|
| 99 | NFR42 / FR94 claim discipline | **done-deferred** — standing honesty |
| 100 | NFR41 / revised AD cite discipline | **done-deferred** — standing honesty |
| 101 | Sync epics.md phase12Epic40–47Status | **done-satisfied** — frontmatter → complete |
| 102 | FR95/FR96 honesty boundary | **done-deferred** — standing honesty |
| 103 | CLI ↔ library API clarity (`--in-tree` / FR96) | **done-implemented** — CLI help + fr35 docs |
| 104 | Continue 101 frontmatter sync | **done-satisfied** — same edit as 101 |
| 105 | Historical NFR14「禁树内」pointers | **done-implemented** — Epic 24/37 NFR14 banners |
| 106 | FR97 honesty boundary | **done-deferred** — standing honesty |
| 107 | Harden `check_idiomatic_chisel` scoped port/IO + empty | **done-implemented** — scoped check + E0904 empty |
| 108 | Continue 101 frontmatter sync | **done-satisfied** |
| 109 | NFR14 commit-subject convention | **done-deferred** — process note |
| 110 | FR98 honesty boundary | **done-deferred** — standing honesty |
| 111 | Assess split `ip.rs` (~2869 LOC) | **done-deferred** — assess-and-defer (past threshold; no split) |
| 112 | Continue 101 frontmatter sync | **done-satisfied** |
| 113 | NFR14/closeout commit-subject | **done-deferred** — process note |
| 114 | FR99 honesty boundary | **done-deferred** — standing honesty |
| 115 | Continue 101 frontmatter sync | **done-satisfied** |
| 116 | NFR14/closeout commit-subject | **done-deferred** — process note |
| 117 | Optional: Cargo-graph design-root discovery | **done-deferred** — optional product |
| 118 | FR100/102/103 honesty boundary | **done-deferred** — standing honesty |
| 119 | Continue 101 frontmatter sync | **done-satisfied** |
| 120 | NFR14/closeout commit-subject | **done-deferred** — process note |
| 121 | Optional: MemRead≡tick / SymbiYosys / GPIO VIP | **done-deferred** — optional product |
| 122 | FR101 honesty boundary | **done-deferred** — standing honesty |
| 123 | Continue 101 frontmatter sync | **done-satisfied** |
| 124 | NFR14/closeout commit-subject | **done-deferred** — process note |
| 125 | Optional: AT nb_transport product | **done-deferred** — optional product |
| 126 | FR104/FR105 honesty boundary | **done-deferred** — standing honesty |
| 127 | Continue 101 + stale deferred L20 | **done-satisfied** — frontmatter + deferred opener |
| 128 | NFR14/closeout commit-subject | **done-deferred** — process note |
| 129 | Optional: C3 FSM / Tywaves / LCOV GUI | **done-deferred** — optional product |

## Engineering

- **FR97 check:** `module_class_span` scopes port / `IO(new Bundle)` / instance checks per class; empty `modules` → E0904 (no exemption).
- **CLI HLS:** `--in-tree` help + `docs/fr35-hls.md` state CLI always dissolves `--dataflow` (FR96); bare FR95 → library API only.
- **ip.rs:** assessed **~2869** LOC; **no split this sweep** (dual-model / FR98 coupling); hygiene story required.

## Tests

- `cargo fmt --all` (touched Rust)
- `cargo test -p rhdl-firrtl --lib` + `cargo test -p bitloom --test fr97_idiomatic_chisel`
