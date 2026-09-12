# FR158 — Third-party LCOV GUI (`genhtml`)

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.  
**Status:** Epic 90 / Story **90.2** (implementation). Closeout → Story **90.3**.  
**NFR14:** `_agile-output/implementation-artifacts/nfr14-risk-epic90-third-party-lcov-gui-fr158.md`.

## Contract (MVP)

| # | Item | Product evidence |
|---|------|------------------|
| **Tool** | Third-party **`genhtml`** (`lcov` package) | PATH probe + run |
| **Input** | Bitloom **`coverage.lcov`** (FR114) | Same dialect as FR114 |
| **Integration** | `cargo bitloom coverage --genhtml` → `<out-dir>/genhtml/` HTML | CLI flag (extends existing `coverage`; FR142 note below) |
| **Failure** | `genhtml` missing → **non-zero** + install hint | Must not silent-Ok |

**FR114 alone ≠ FR158.** In-tree `coverage.html` remains FR114; this FR is the **third-party** path.

## Product path

```bash
# FR114 artifacts (always)
cargo bitloom coverage --out-dir target/cov --ticks 4

# FR158: also run genhtml (requires `lcov` / genhtml on PATH)
cargo bitloom coverage --out-dir target/cov --ticks 4 --genhtml
# optional: --genhtml-out target/cov/third-party-html
```

Library helper (`bitloom` package — **not** for design crates; AD-6):

```rust
use bitloom::lcov_gui::{find_genhtml, run_genhtml};
run_genhtml(Path::new("coverage.lcov"), Path::new("genhtml_out"))?;
```

## FR142 note

Extends the existing in-surface `coverage` command with optional `--genhtml` / `--genhtml-out`.
Does **not** add a new top-level subcommand.

## Missing tool

```text
error: FR158: `genhtml` not found on PATH (install `lcov` package, e.g. `apt install lcov`); ...
```

FR114 tree GUI is **not** a substitute when `--genhtml` was requested.

## Cross-links

- FR114 in-tree: [`fr114-lcov-coverage-gui.md`](fr114-lcov-coverage-gui.md)
- Phase 19 README / deferred

```text
cargo test -p bitloom --test fr158_third_party_lcov_gui
```
