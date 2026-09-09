# Nested Bundle — FR80 follow-along

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

This walkthrough is the **UJ-level「嵌套 Bundle」** close-out for Epic 32 / FR80
(AD-20): one-level nested `Bundle` + `#[derive(Bundle)]` — **not** the Epic 19 /
FR51 ground-leaf-only minimum (nested was OUT OF SCOPE).

Design crates depend only on **`bitloom-prelude`**.

## Historical minimal vs FR80 depth (NFR37)

| Contract | What it proves | Fixture |
|----------|----------------|---------|
| **FR51 / Epic 19** | ground-leaf `Bundle` / `HwVec` flatten + width/dir before emit | `examples/bundle_vec_skel` (flat `Stream`) |
| **FR80 / Epic 32** | **one-level** nested Bundle → emit `.v` → tick; derive via prelude | same skel nested/`Derived*` paths |

Do **not** close FR80 by deleting OUT OF SCOPE comments alone, or by citing
FR51/`done` talk (NFR37).

## Minimal example — hand-written nested Bundle

```rust
use bitloom_prelude::{Bool, Bundle, GroundType, UInt};

struct Stream;
impl Bundle for Stream {
    fn leaves() -> &'static [(&'static str, GroundType)] {
        &[
            ("data", GroundType::UInt { width: 8 }),
            ("valid", GroundType::Bool),
        ]
    }
}

struct Packet;
impl Bundle for Packet {
    fn leaves() -> &'static [(&'static str, GroundType)] {
        &[("ready", GroundType::Bool)]
    }
    fn nested_bundles()
        -> &'static [(&'static str, fn() -> &'static [(&'static str, GroundType)])]
    {
        &[("stream", Stream::leaves)]
    }
}
```

Leaf names after flatten: `pkt_ready`, `pkt_stream_data`, `pkt_stream_valid`.

## Minimal example — `#[derive(Bundle)]`

```rust
use bitloom_prelude::{Bool, Bundle, UInt};

#[derive(Bundle)]
struct DerivedStream {
    data: UInt<8>,
    valid: Bool,
}

#[derive(Bundle)]
struct DerivedPacket {
    ready: Bool,
    stream: DerivedStream, // one-level nest
}
```

Re-export is via **`bitloom-prelude`** (AD-6). Do **not** depend on
`bitloom-macro` or the CLI package `bitloom` from a design crate.

## Limits table

| Surface | Status |
|---------|--------|
| Ground-leaf Bundle / `HwVec<AsGround,_>` | Supported (FR51) |
| One-level nested Bundle (`nested_bundles` / derive nest field) | Supported (FR80) |
| Nested leaf width mismatch | Fail before emit (`rhdl::E0131`) |
| Nested leaf direction mismatch | Fail before emit (`rhdl::E0112`) |
| Leaf name collision | Fail before emit (`rhdl::E0152`) |
| `#[derive(Bundle)]` named struct + grounds / one-level nest | Supported |
| derive enum / tuple / unit / generics / `HwVec` / `Input` / `Output` fields | Reject (`rhdl::E0180`) |
| Nest depth ≥ 2 | **Non-goal** this epic — do not claim arbitrary depth |
| `HwVec<Bundle,_>` | **OUT OF SCOPE** |

Product doc: [`docs/fr80-nested-bundle.md`](../fr80-nested-bundle.md).
Spec: `_agile-output/specs/spec-rhdl/language-surface.md` (Composite types).

## Fixture goldens + negatives

```bash
cargo test -p bundle_vec_skel
```

Expect: nested hand-written + derived paths elaborate → emit → `Sim::tick`;
nested width/dir mismatches fail before emit; trybuild rejects
`HwVec<Bundle,_>` and unsupported derive shapes (`rhdl::E0180`).

## Close-out ATDD + contributor recipe

Documented Epic 32 depth matrix (Story 32.4):

```bash
cargo test -p bitloom --test fr80_nested_bundle
cargo test -p bundle_vec_skel
```

Full workspace (contributors): `just test` (or `cargo test --workspace`).
This story does **not** require `cargo clean && just test` as a gate.

## Cross-links

| Topic | Doc |
|-------|-----|
| FR80 product note | [`docs/fr80-nested-bundle.md`](../fr80-nested-bundle.md) |
| Language surface (CAP-10 / FR80) | `_agile-output/specs/spec-rhdl/language-surface.md` |
| Risk gate / Epic 32 close | `nfr14-risk-epic32-nested-bundle.md` |
| Fixture | `examples/bundle_vec_skel` |

## Out of scope here

- Arbitrary nest depth (≥2 levels).
- `HwVec<Bundle,_>`.
- Public HIR Bundle nodes (flatten to scalar leaves remains the contract).
