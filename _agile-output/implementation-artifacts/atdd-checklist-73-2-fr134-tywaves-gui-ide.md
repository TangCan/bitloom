# ATDD Checklist — Story 73.2 / FR134

| AC / predicate | Test | Notes |
|----------------|------|-------|
| G1 版本/渠道 | `fr134_gui_install_descriptor_pins_version_channel` | install.json 含 GUI version + channel；≠ BITLOOM_TYWAVES_BIN alone |
| G2 元数据契约 | `fr134_wave_tywaves_gui_writes_manifest` | manifest 含 Bitloom / schemaVersion / tywaves.gui / tywaves.ide-plugin |
| G3 失败语义 | `fr134_force_missing_is_nonzero_readable` | FORCE_MISSING → 非零 + bitloom.tywaves* |
| G4 / alone | `fr134_not_satisfied_by_fr125_alone` | 仅 --tywaves 无 FR134 manifest |
| NFR56 | `fr134_keeps_fr125_fr117_fr104_artifacts` | sidecar + typed-wave + interactive 仍写出 |
| Standing honesty | `fr134_tywaves_not_in_design_crate_deps` | bitloom-prelude Cargo.toml 无 tywaves |
| Docs | `fr134_docs_and_nfr14_g1_g4` | docs/fr134 + NFR14 G1–G4 |

```bash
cargo test -p bitloom --test fr134_tywaves_gui_ide
```

Expected before Story 73.2 build: **FAIL**（缺 `--tywaves-gui` / emit / docs）。
