# Action-items sweep — 2026-09-10 (Phase 13)

Cleared **36** open Phase-13 retro action items (epics 48–56, items 130–165). Authoritative dispositions also landed in `deferred-work.md` and `process-one-story-one-commit.md`; `epics.md` frontmatter `phase13Epic50Status`–`phase13Epic56Status` → `complete`.

| ID | Short theme | Disposition |
|----|-------------|-------------|
| 130 | FR106/FR115 honesty | **done-deferred** — standing honesty |
| 131 | Gate closeout ATDD convention | **done-implemented** — process note |
| 132 | Commit subject Story N.M | **done-deferred** — process note |
| 133 | NFR46/NFR47 AD cite / no silent expand | **done-deferred** — standing honesty |
| 134 | FR107 AT honesty | **done-deferred** — standing honesty |
| 135 | Keep AT ∥ LT gen-tlm regression | **done-deferred** — standing NFR44 |
| 136 | Commit subject Story N.M | **done-deferred** — process note |
| 137 | Brand host dirs `rhdl-gen-tlm-*` → `bitloom-gen-tlm-*` | **done-implemented** — CLI paths |
| 138 | FR108 GPIO honesty | **done-deferred** — standing honesty |
| 139 | Keep FR98 near-VIP on ip.rs touch | **done-deferred** — standing NFR44 |
| 140 | Commit subject Story N.M | **done-deferred** — process note |
| 141 | Assess ip.rs ~2975 LOC split | **done-deferred** — assess-and-defer (no split) |
| 142 | FR109 C3 honesty | **done-deferred** — standing honesty |
| 143 | Keep FR105/FR34 coverage regression | **done-deferred** — standing NFR44 |
| 144 | Commit subject Story N.M | **done-deferred** — process note |
| 145 | Auto FSM label extract needs new contract | **done-deferred** — optional / NFR47 |
| 146 | FR110 honesty | **done-deferred** — standing honesty |
| 147 | Keep FR95/96 HLS regression | **done-deferred** — standing NFR44 |
| 148 | Commit subject Story N.M | **done-deferred** — process note |
| 149 | allocation/binding/Handshake needs new contract | **done-deferred** — optional / NFR47 |
| 150 | FR111 honesty | **done-deferred** — standing honesty |
| 151 | Keep FR97 idiomatic regression | **done-deferred** — standing NFR44 |
| 152 | Commit subject Story N.M | **done-deferred** — process note |
| 153 | Full style pack / Parser needs new contract | **done-deferred** — optional / NFR47 |
| 154 | FR112 honesty | **done-deferred** — standing honesty |
| 155 | Keep FR100/FR103 dual-model regression | **done-deferred** — standing NFR44 |
| 156 | Commit subject Story N.M | **done-deferred** — process note |
| 157 | SymbiYosys / more IP FL needs new contract | **done-deferred** — optional / NFR47 |
| 158 | FR113 honesty | **done-deferred** — standing honesty |
| 159 | Keep FR99 DesignFixture regression | **done-deferred** — standing NFR44 |
| 160 | Commit subject Story N.M | **done-deferred** — process note |
| 161 | Full syn-scan needs new contract | **done-deferred** — optional / NFR47 |
| 162 | FR114 honesty | **done-deferred** — standing honesty |
| 163 | Keep FR104/FR105 coverage CLI regression | **done-deferred** — standing NFR44 |
| 164 | Commit subject Story N.M | **done-deferred** — process note |
| 165 | Tywaves / 3rd-party LCOV GUI needs new contract | **done-deferred** — optional / NFR47 |

## Engineering

- **CLI TLM host dirs:** `target/rhdl-gen-tlm-host` / `target/rhdl-gen-tlm-at-host` → `target/bitloom-gen-tlm-host` / `target/bitloom-gen-tlm-at-host`.
- **ip.rs:** assessed **~2975** LOC; **no split this sweep** (FR98/FR108 dual-model coupling); hygiene story required.
- **Frontmatter:** `phase13Epic50–56Status` → `complete`.

## Tests

- `cargo fmt --all` (touched Rust)
- targeted compile/check of `bitloom` CLI if needed (`cargo check -p bitloom`)
