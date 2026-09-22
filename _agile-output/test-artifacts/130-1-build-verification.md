# Story130.1 Build Verification

Date: 2026-09-22. Baseline: `baf07e739af2b36f4e0befddda9826964d16118e`.

- NFR14 content review M01–M21: PASS. M22 remains pending for independent review, automate, full regression, status close and commit.
- Presence transition: expected RED before build; PASS after `epic-130-nfr14.md` was created. Presence alone is not content validation.
- Process gate: normal Python 84/84 PASS; optimized Python 84/84 PASS; source sprint bytes/hash preserved in each run.
- Build probe: normal and optimized Python each executed 17 commands and PASSed. Results bind script/source/tool paths and SHA256, cross-check SHA256sum, full baseline commit, HEAD archive, recursive submodule status, network isolation, locked/offline Rust contract, Verilog compile/sim/synthesis and raw logs.
- Network isolation: host `unshare -n` returned 1 due permission and is recorded as unsupported; `bwrap --unshare-net` actually ran and exposed loopback only. This proves an available denial mechanism, not future offline replay.
- RTL boundary: `vendor_ext_ip` contains no behavior. Simulation emitted `behavior=absent`; compile/sim/synthesis are mechanism evidence only.
- `python3 scripts/check_phase24_gate.py`, Python compilation and `git diff --check`: PASS.
- No diff in `crates/`, `docs/ip/phase24-support-matrix.md`, `Cargo.toml`, `Cargo.lock`, or `rust-toolchain.toml`.

FR199 source locking, license verification, candidate acquisition and offline replay remain Story130.2. FR200 real wrapper and independent RTL behavior remain Story130.3. No external support level was promoted.
