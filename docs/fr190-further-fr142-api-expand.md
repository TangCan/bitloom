# FR190 — Further explicit FR142 public API surface expand (beyond FR183)

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** Epic 123 / FR190 **in progress** (Story **123.2**). Product path: **explicit** additive promote of FIRRTL text `emit`/`import` + roundtrip predicates + documented `check_*` family into the FR142 surface list ([`docs/public-api-1-0-surface.md`](public-api-1-0-surface.md)); SemVer honesty per FR143 (**minor** for additive); **AD-6** unchanged.

Phase 22 **FR183** firrtl emit/Parser expand and Phase 17 **FR142** surface lock **remain closed and valid** (NFR88). Those closes alone ≠ FR190. Silent expand remains forbidden.

## Selected shape (NFR14)

| Layer | Role |
|-------|------|
| **FR183 baseline** | Documented `emit_chisel*` / pin constants / `BitloomFirrtlParser` |
| **FR190 S1** | `emit`, `import`, `ports_roundtrip_ok`, `instance_graph_roundtrip_ok` |
| **FR190 S2** | Documented `check_*` family paired with productized emit faces (through FR188 pack) |
| **Surface doc (S3 / NFR90)** | FR190 / v1.x expand section; design crates still **prelude-only** |
| **SemVer** | Additive → **minor** on next `bitloom-firrtl` publish |

**Forbidden closes:** FR183 alone; FR142 alone; “code already has `pub`” alone; surface claim without updating `public-api-1-0-surface.md`; **silent expand**; design crates depending on `bitloom-firrtl`; promoting lsp/hir/builder/vlog to 1.0-stable in this FR.

## Reproducible checks

```bash
cargo test -p bitloom --test fr190_further_fr142_api_expand
# surface file lists FR190 + emit/import + check_* + AD-6
# bitloom-prelude must not depend on bitloom-firrtl
```

## Standing honesty

Further API expands beyond this NFR14 subset need a new contract (**NFR91**).
Do **not** claim FR183 alone delivers this FR.

```text
cargo test -p bitloom --test fr190_further_fr142_api_expand
```
