# FR119 — SymbiYosys / SMT formal path (F1-(ii))

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 60 / FR119 closed** (Story **60.3**). Product path delivered in Story **60.2**
(`just formal-sby-check`). Phase 12 FR100 F1-(i) and Phase 13 FR112 branch B remain closed
(NFR48). Branch C (more IP handwritten FL) stays **deferred** (NFR51).

This page is the **FR119 completion surface**. It delivers a documented **SymbiYosys (`sby`)**
binding with assume/assert fixtures. It is **beyond** FR100 F1-(i) bounded exhaustive,
FR112 branch B MemRead≡tick, FR92 scoreboard, and FR85 Verilator lint.

## Selected binding (NFR14)

**Selected: (A) SymbiYosys (`sby`)** — see
`_agile-output/implementation-artifacts/nfr14-risk-epic60-symbiyosys-smt.md`.

- Independent SMT solver product entry **(B)** is **not** selected as an equal close;
  SMT engines (e.g. `z3` via `yosys-smtbmc`) are backends of `sby`.
- Branch C (more IP handwritten FL) remains **deferred** (NFR51).

## Product entry

```bash
just formal-sby-check
# or: bash scripts/formal-sby-check.sh

# Readable fail fixture (when sby is installed):
BITLOOM_SBY_MODE=fail just formal-sby-check

# Force missing-tool path (always non-zero; never silent success):
BITLOOM_SBY_FORCE_MISSING=1 just formal-sby-check
```

| Path | Role |
|------|------|
| `just formal-sby-check` / `scripts/formal-sby-check.sh` | **FR119** first-class entry |
| `just formal-sva-check` / `scripts/formal-sva-check.sh` | **FR85** only (≠ FR119) |
| `FormalEquivProduct` | **FR100** F1-(i) (≠ FR119) |

Default `just test` does **not** require `sby` on the host.

## Fixtures (assume + assert)

| Fixture | Path | Expectation |
|---------|------|-------------|
| Pass | `crates/rhdl-formal/fixtures/fr119/fr119_pass.{sv,sby}` | BMC `expect pass` |
| Fail | `crates/rhdl-formal/fixtures/fr119/fr119_fail.{sv,sby}` | BMC `expect fail` (readable non-pass) |

Each SV fixture includes at least one `assume property` and one `assert property`,
inside a clocked process supported by Yosys. The initial clock edge assumes reset.
The wrapper checks the actual task status: only PASS in pass mode returns zero.
Although SBY itself returns zero for an expected FAIL, the fail-mode wrapper reports
`actual SBY status=FAIL` and returns nonzero. Regression requires a real counterexample;
syntax errors, UNKNOWN, missing status, and timeouts cannot satisfy that negative test.

## Tool versions / known-good band

Detection: `command -v sby`; script prints `sby --version` / `yosys -V` when available.

| Tool | Obligation |
|------|------------|
| SymbiYosys (`sby`) | Required for a green local/optional-CI proof run |
| Yosys | Required by typical `sby` installs |
| SMT engine (e.g. z3 via `smtbmc z3`) | Required by the committed `.sby` `[engines]` block |

**Measured 2026-09-20 (Story126.3 prerequisite repair):** SBY `yosys-0.47`,
Yosys `0.33 (2584903a060)`, Z3 `4.8.12`. Real pass fixture PASS; fail fixture
FAIL with a step-3 counterexample and wrapper exit 1. Evidence:
`_agile-output/test-artifacts/126-3-fr119-{pass,fail}.log`.
Fresh pinned clone and upstream `make install` into an isolated `/tmp` prefix were
verified with PYTHONPATH unset; full CI sudo/apt provisioning was not run locally.
See [implementation evidence](../_agile-output/test-artifacts/126-3-build-evidence.md).

CI without SymbiYosys must fail readably on this script (or use
`BITLOOM_SBY_FORCE_MISSING=1`); it must **not** silent-succeed.

## Isolation (must not close FR119 alone)

| Prior path | Status for FR119 |
|------------|------------------|
| FR92 scoreboard | Supporting; **alone ≠ FR119** |
| FR100 F1-(i) | Closed MVP; **alone ≠ FR119** (NFR48) |
| FR112 branch B MemRead≡tick | Closed; **alone ≠ FR119** (NFR48) |
| FR85 Verilator / `formal-sva-check` | External SVA checker; **alone ≠ FR119** |
| FR107 SystemC AT | **≠ FR119** |

## Recipe (ATDD)

```text
cargo test -p bitloom --test fr119_symbiyosys_smt_path
cargo test -p bitloom --test fr100_formal_equiv_product
cargo test -p bitloom --test fr112_memread_equiv_tick
cargo test -p bitloom --test fr112_epic54_closeout
```

## Cross-links

| Doc | Role |
|-----|------|
| [`fr100-formal-equiv.md`](fr100-formal-equiv.md) | FR100 F1-(i) (still valid) |
| [`fr112-generated-functional-memread-equiv.md`](fr112-generated-functional-memread-equiv.md) | FR112-B (still valid) |
| [`fr39-formal-sva.md`](fr39-formal-sva.md) | FR85 path (≠ FR119) |
| NFR14 Epic 60 | `_agile-output/implementation-artifacts/nfr14-risk-epic60-symbiyosys-smt.md` |
