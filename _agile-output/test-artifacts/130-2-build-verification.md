# Story130.2 Build Verification

Date: 2026-09-22

## Accepted Source Closure

- Manifest SHA256: `5eb5cf32013737fdd0857287ad0f933b0f5e814d40f7c864aba0043065a03356`.
- Lock SHA256: `a0249918c9928d59c133936352d2cbef18e68cf950e8bf4627d9113bb6fe03df`.
- Recursive closure digest: `c6776ed391fa8b67a7ab8703e9c360c66cbd973ed35c5522784589ef32902a0e`.
- `common_cells v1.40.0`: lightweight tag/commit `1281545696eb3fcba50ec5b4275993476a3c710e`, 169 tracked files.
- `common_verification v0.2.0`: annotated tag object `70351333c358c3bbf8ec34bee23250ee9fdb9e33`, commit `6fc76fb013315af9fabbb90b431863d498df2d6d`, 13 tracked files.
- `tech_cells_generic v0.2.11`: lightweight tag/commit `a9cae21902e75b1434328ecf36f85327ba5717de`, 28 tracked files.
- All three root LICENSE files are exact SHL-0.51 text with SHA256 `6527a46225891b976fa94f634f6ee0cd22e1c7129d6f11c5c98c997627625fc7`; no tracked NOTICE-named file exists. The lock preserves the conditional NOTICE obligation instead of claiming that no obligation exists.

## Two-Stage Result

- Online empty-cache evidence: `130-2-online-fetch.json`, exit 0, complete 210-file closure. Bitloom CLI, Git, `timeout`, Yosys and adapter paths/versions/binary hashes are embedded in the lock and evidence.
- Offline copied-cache evidence: `130-2-offline-replay.json`, exit 0. The replay bundle copied manifest, lock, licenses and the CLI binary into the transported cache before entering bubblewrap.
- The recorded host and replay network namespace IDs differ; HOME/XDG/Git configuration were redirected inside the copied cache.
- Yosys consumed the locked `fifo_v3` through `bitloom-yosys-sv-compat@1`: input SHA256 `861ea44cbd3129731b77977da36fb0dac1a1350d7df664a1b4310423e7d272bc`, adapted output SHA256 `7512ebaf5b925e85d7dd32409eedc4dc02e1813ec5af3ff0cdd98d773af7178d`, `hierarchy/proc/check` passed.

## Focused Tests

- Dedicated Rust online gate final run: 1 passed / 0 failed / 0 ignored in 36.52s. Two immediately preceding runs failed non-zero after the test helper cleared this environment's required proxy variables and all three exact-tag attempts reached GNU timeout exit 124; the helper now preserves only standard proxy/no-proxy variables after `env_clear`. Those failures were not recorded as PASS.
- Rust ordinary mode: 3 passed / 0 failed, covering floating ref, mandatory explicit `--offline`, and CLI separation; the online gate remains intentionally ignored so workspace regression does not silently depend on the network. Its ignored result is not counted as PASS.
- Rust private unit mode: 5 passed / 0 failed for exact adapter shape, adapter drift, relative-path safety, Yosys argument safety and closure identity binding.
- Python backend normal mode: 3 passed / 0 failed / 0 skipped.
- Python backend optimized mode: 3 passed / 0 failed / 0 skipped.
- Independent evidence consumer normal and optimized modes: 7 passed / 0 failed each. Five-round burn-in passed for Rust unit, offline integration, normal Python and optimized Python suites.
- Mutation coverage includes tag/commit/cache identity, source and license hash, NOTICE facts, dependency/include/define/tool drift, reordered/extra files, absolute/parent path, symlink/hardlink, empty cache, host-cache fallback, lock immutability and distinct network namespace.

## Boundary

This is FR199 source governance and replay evidence only. The Yosys parse/compile is a replay integrity check and does not promote the external support matrix. No Bitloom wrapper, FIFO behavior oracle, upstream behavior test, `compiled` support-level claim, FR200, Epic130 close, Phase24 close, push or publish is included.
