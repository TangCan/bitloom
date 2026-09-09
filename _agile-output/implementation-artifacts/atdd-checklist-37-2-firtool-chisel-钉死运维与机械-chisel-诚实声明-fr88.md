# ATDD Checklist — 37-2 / FR88 firtool↔Chisel ops + honesty

**Story:** `37-2-firtool-chisel-钉死运维与机械-chisel-诚实声明-fr88`  
**Stack:** backend (Cargo / `bitloom` integration tests)  
**Phase:** green (docs lock landed)

## Acceptance → tests

| AC | Test | Expected red reason (pre-impl) |
|----|------|--------------------------------|
| 钉死版本对 | `fr88_fr28_documents_pinned_chisel_firtool_pair` | (likely green already — pair present) |
| 缓存/覆盖入口 | `fr88_fr28_documents_firtool_override_and_cache` | FR28 lacks `RHDL_FIRTOOL_PATH` / cache ops |
| 可编译 ≠ idiomatic | `fr88_fr28_states_compilable_not_idiomatic` | FR28 lacks explicit ≠ idiomatic honesty |
| README 交叉链 | `fr88_readme_cross_links_fr28_ops_honesty` | README lacks FR88 / ops honesty cross-link framing |
| FR46 无误导 | `fr88_fr46_no_handwritten_maintainable_claim` | (likely green — audit lock) |
| FR28 无误导 | `fr88_fr28_no_handwritten_maintainable_claim` | (likely green — audit lock) |
| 品牌 | `fr88_brand_remains_bitloom` | (should stay green) |

## Command

```bash
cargo test -p bitloom --test fr88_firtool_chisel_ops_honesty
```

## Out of scope (do not ATDD here)

- Story 37.3 nocturnal Bambu / stub selection
- Private firtool/Chisel version bumps
- Changing `emit_chisel` implementation (docs-only OK)
