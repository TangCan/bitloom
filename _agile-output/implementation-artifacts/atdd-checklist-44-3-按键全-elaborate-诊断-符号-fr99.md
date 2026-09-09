# ATDD Checklist — Story 44.3 按键全 elaborate 诊断 / 符号（FR99）

**Story:** 44-3-按键全-elaborate-诊断-符号-fr99  
**Phase:** red → green  
**Primary test:** `crates/bitloom/tests/fr99_bitloom_lsp_full_elaborate.rs`

| Acceptance | Test / guard | Failure mode |
|------------|--------------|--------------|
| Edit-trigger path runs full-design elaborate | `fr99_full_vs_shallow_elaborate_contract` | shallow-only / no `finish()` |
| Diagnostics + symbols (P6) | `fr99_full_elaborate_diagnostics_and_symbols` | missing diag/symbols |
| Readable failures | same + E0142 fixture | empty / opaque messages |
| Timeout / oversized documented (P3/P4) | `fr99_full_elaborate_p3_p4_docs_and_behavior` | silent hang / fake success |
| P1 trigger documented | `fr99_full_elaborate_trigger_docs` | undocumented trigger |
| Scope: no 44.4 / epic close | `fr99_full_elaborate_scope_guards` | premature FR99 close |
| Sprint 44-3 done; 44-4 backlog; epic-44 in-progress | same | wrong sprint keys |

```text
cargo test -p bitloom --test fr99_bitloom_lsp_full_elaborate
```
