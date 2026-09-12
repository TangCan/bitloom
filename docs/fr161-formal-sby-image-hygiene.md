# FR161 — formal-sby image / tooling hygiene

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 93 / FR161 delivered in Story 93.2** (closeout Story **93.3**).
Phase 15 FR127 required CI `formal-sby` job remains closed (NFR68).

## Selected face (NFR14)

Pin GHA `formal-sby` **host bootstrap** install (strategy B): `scripts/ci-sby-pins.env`
records SymbiYosys git **ref + SHA**; `scripts/ci-install-sby.sh` clones that pin;
`scripts/ci-sby-hygiene-check.sh` asserts pins + install obedience.

| Layer | Role |
|-------|------|
| **FR119** | Local-optional `just formal-sby-check` — **still closed**; alone ≠ FR161 |
| **FR127** | Required CI job runs true sby — **still closed**; alone ≠ FR161 |
| **FR161** | Pin + hygiene tracking — **this page** |

**Forbidden closes:** FR127 alone; FR119 alone; docs-only; silent-Ok on missing pins;
floating `git clone` HEAD without pin.

## Pins (authority)

File: [`scripts/ci-sby-pins.env`](../scripts/ci-sby-pins.env)

| Variable | Meaning |
|----------|---------|
| `SBY_GIT_URL` | YosysHQ/sby remote |
| `SBY_GIT_REF` | Tag / branch cloned (`--branch`) |
| `SBY_GIT_SHA` | Expected `git rev-parse HEAD` after clone |

Current MVP pin: **`yosys-0.47`** @ `bfc1c47eb786496fe794481ff88e75728f0529a6`.

## Recipe

```text
bash scripts/ci-sby-hygiene-check.sh
# CI also runs this before ci-install-sby.sh (see .github/workflows/ci.yml formal-sby)
cargo test -p bitloom --test fr161_formal_sby_image_hygiene
```

Negative (readable fail):

```text
BITLOOM_SBY_HYGIENE_FORCE_FAIL=1 bash scripts/ci-sby-hygiene-check.sh   # exit 1
```

## Cross-links

| Doc | Role |
|-----|------|
| [`fr127-forced-sby-ci.md`](fr127-forced-sby-ci.md) | FR127 required job (still closed; ≠ FR161 alone) |
| [`fr119-symbiyosys-smt.md`](fr119-symbiyosys-smt.md) | FR119 local path |
| NFR14 | `_agile-output/implementation-artifacts/nfr14-risk-epic93-formal-sby-image-hygiene-fr161.md` |

## Non-goals

- Private registry / commercial formal SaaS
- Expanding formal proof fixtures beyond FR119
- Claiming NFR59 “fully cleared” before FR157–FR165 all close
