---
title: 'Story 130.2：来源清单、锁定与离线重放'
type: 'feature'
created: '2026-09-22'
status: 'done'
route: 'dispatch'
review_loop_iteration: 1
baseline_commit: '3e7fb44b4500e3cdd2f75198780d440f375d6ebd'
context:
  - '{project-root}/AGENTS.md'
  - '{project-root}/_agile-output/implementation-artifacts/130-2-来源清单-锁定与离线重放.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-130-context.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-130-nfr14.md'
  - '{project-root}/_agile-output/test-artifacts/atdd-checklist-130-2-来源清单-锁定与离线重放.md'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Bitloom 尚无可验证的外部 RTL 来源 manifest、不可变 lock、许可证归档和禁网重放入口；URL/tag 或一次联网成功无法满足 FR199/NFR98。

**Approach:** 在 CLI/构建层交付 versioned JSON manifest/lock、三仓库完整 tracked-file 闭包及内容寻址缓存；先空缓存联网获取，再在 bubblewrap 禁网环境中验证身份、内容、许可证并实际编译 stable `fifo_v3`。

## Boundaries & Constraints

**Always:** 固定 `common_cells v1.40.0`、`common_verification v0.2.0`、`tech_cells_generic v0.2.11` 的 tag object/peeled commit；逐仓记录完整文件 hash、LICENSE/NOTICE 事实、owner、工具身份、ordered compile filelist/include/宏。生成 lock 与只读 verify/replay 分离；临时目录、原子发布、路径越界和未声明文件 fail closed。设计 crate 仍只依赖 prelude。

**Never:** 不把第三方源码提交为产品源码或塞入 HIR，不创建第二 IR/通用包管理器，不回退联网或宿主缓存，不自动重锁/换 master/Taxi，不实现真实 Bitloom wrapper 或 FIFO 行为 oracle，不提升 `compiled`/`behavior-tested`/`maintained`，不改工具 pin、包版本、发布状态或 130.3。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|---|---|---|---|
| Lock | versioned manifest + empty cache + network | deterministic lock and complete movable cache; evidence separately records UTC/tool | floating/moved tag, missing license/dependency or Git failure is non-zero; no partial cache |
| Verify | manifest + lock + complete cache | read-only identity/file/license/tool validation; canonical bytes unchanged | extra/missing/reordered/path-escape/hash/tool drift is non-zero; never rewrites lock |
| Replay | copied cache + isolated HOME/XDG + no network | verify plus Yosys SystemVerilog parse/compile of locked FIFO filelist | network/cache escape or unavailable isolation/compiler is non-zero |
| Candidate | stable Bender.yml | three explicit source entries agree with declared versions/URLs | dependency declaration drift is non-zero; no silent pruning |

</frozen-after-approval>

## Code Map

- `crates/bitloom/src/main.rs` -- add `external-ip` command routing; preserve existing cargo-subcommand argument normalization.
- `crates/bitloom/src/external_ip.rs` -- new private manifest/lock/fetch/cache/verify/replay implementation; reuse `firtool.rs` SHA256, process-error and atomic-cache principles without turning it into a generic package manager.
- `crates/bitloom/tests/fr199_external_ip_lock.rs` -- ATDD CLI/schema RED scaffold; activate scenarios incrementally and retain mutation coverage.
- `scripts/phase24-external-ip-replay.py` -- deterministic two-stage orchestration/evidence wrapper; no source resolution logic duplicated from Rust.
- `ip/external/pulp-common-cells-fifo-v3.source.json` and `.source.lock.json` -- canonical intent and deterministic resolved identity; lock lists all 210 tracked files across three repos.
- `ip/external/licenses/` -- exact per-repository LICENSE text plus NOTICE absence facts referenced by lock.
- `Justfile` / `.github/workflows/ci.yml` -- one explicit FR199 replay gate only if runtime and cost are bounded; never `continue-on-error`.
- `docs/ip/phase24-support-matrix.md` -- at most mark source as catalogued/locked after actual replay; leave compiled/behavior/maintained false for 130.3.

## Tasks & Acceptance

**Execution:**
- [x] `crates/bitloom/src/external_ip.rs`, `main.rs` -- implement strict schema, deterministic lock, full tracked-file cache, read-only verify and replay with actionable error classes.
- [x] `ip/external/**` -- add explicit three-source manifest, generated lock and exact license material; verify Bender dependency declarations against locked source.
- [x] `scripts/phase24-external-ip-replay.py` -- run empty-cache fetch then copied-cache bubblewrap replay, sanitize HOME/XDG/Git environment, record commands/UTC/duration/exit/tool/source identities.
- [x] `crates/bitloom/tests/fr199_external_ip_lock.rs` and focused negative tests -- cover tag/commit, hash/license, extra/missing/order/path/cache/network/tool drift and immutability in normal execution.
- [x] `_agile-output/test-artifacts/130-2-*` -- archive online/offline and mutation evidence; update Story record without claiming FR200.

**Acceptance Criteria:**
- Given an empty cache and canonical manifest, when lock runs online, then all three immutable Git identities, all tracked-file hashes, compile metadata and license facts produce a deterministic lock and atomically complete cache.
- Given only copied lock/cache in an isolated checkout, when replay runs with network disabled, then it performs read-only validation and actual Yosys consumption of locked `fifo_v3`; hidden HOME/XDG caches are not required.
- Given each P0 mutation, when verify/replay runs, then it exits non-zero with a specific diagnostic and canonical manifest/lock/cache/sprint hashes remain unchanged.
- Given Story130.2 closes, when scope is audited, then only FR199/catalogued+locked evidence changes; FR200, real wrapper behavior, higher support levels, Epic130 and Phase24 remain open.

## Implementation Notes

Intent gaps: none; Story130.1/130.2 contracts fix observable behavior and candidate. Irreversibles: only temporary upstream network reads; no push, publish, deploy or upstream write. Footprint: one private Rust module/CLI command family, canonical manifest/lock/license files, one orchestration script, focused tests/evidence and optional dedicated gate. The dirty worktree contains only this Story's create-story/ATDD artifacts.

Implementation used explicit Git refspec resolution instead of requiring absent Bender/FuseSoC executables. Every Bender dependency declaration is checked against the three-source manifest, all 210 tracked files are hashed, and a schema/URL/dependency/source closure digest names the atomically published cache root. The implementation subagent timed out without edits; the permitted inline fallback produced and verified the diff.

Yosys 0.33 cannot parse upstream `parameter type dtype`. Replay therefore uses a fail-closed versioned compatibility adapter that recognizes six exact source fragments, concretizes the locked dtype, lowers whole unpacked-array assignments to loops, records input/output SHA256, and then runs Yosys hierarchy/process/check. This is replay parse/compile evidence only, not FR200 behavior or a support-level promotion.

## Spec Change Log

## Review Triage Log

| Source | Verdict | Route | Finding and evidence |
|---|---|---|---|
| Build local diff audit | medium | patch | `lock()` published a new closure before comparing an existing immutable lock, so identity drift could fail non-zero while leaving an unreferenced published cache entry. The implementation now validates staging, computes/compares deterministic lock bytes, removes staging on mismatch, and only then atomically publishes the complete closure. |
| Edge/verification | high | patch | `verify_source` joined `license.source_path` without relative-path validation, allowing cache escape; it now requires a relative `LICENSE` path and has traversal mutation coverage. |
| Edge/blind | medium | patch | Bender dependency checks used substring matches, allowing comments or unrelated text to satisfy closure requirements; they now parse exact dependency blocks and versions. |
| Edge/blind | medium | patch | Multi-source lock failure could leave published license archives; archives now remain in staging until all sources and preflight checks succeed, with cleanup coverage. |
| Blind | medium | patch | License archive symlinks or hardlinks could escape the declared archive; verification now requires a regular single-link file. |
| Edge/blind | medium | patch | Absolute `bitloom_path` prevented relocated replay despite identical binary content; the lock now records a relocatable identity and retains the binary hash. |
| Blind | maybe-false | defer | The runner relies on the caller-provided isolation environment marker; proving an unforgeable namespace check requires a broader launcher contract not exercised by this story's current CLI tests. |
| Blind | maybe-false | defer | The bubblewrap runner exposes a read-only host root for required tools and libraries; narrowing mounts needs an environment-specific toolchain contract beyond this story. |
| Blind | maybe-false | defer | Empty-cache proof and per-repository legal-fact extraction were not independently demonstrated in the restricted environment; existing archived evidence covers the canonical run, while a fresh network gate is required to settle them. |
| Blind | false | reject | Compile-contract defaults are bound by the exact locked pilot shape and mutation tests; the cited missing comparison does not occur at the reviewed implementation. |
| Blind | false | reject | Tool identity hashes are compared for replay; the relocated Bitloom path issue was fixed while external tool paths remain diagnostic metadata, so the cited end-user failure is not shown. |
| Blind | false | reject | The lock intentionally records deterministic evidence location rather than a run timestamp; timestamps are in the separately hashed evidence records by design. |

The required blind-hunter, edge-case-hunter and verification-gap layers were launched against the complete unified diff; findings were triaged below and the actionable cache-boundary, dependency, transactional-license, symlink, and relocation defects were patched.

## Design Notes

Lock determinism excludes last-run UTC; execution timestamps belong to evidence. Cache entries are immutable directories keyed by repository commit plus tree-content digest, populated in a sibling temporary directory and renamed only after all files validate. The lock records all tracked files, while ordered compile files are a separate subset (`include/common_cells/assertions.svh`, `src/fifo_v3.sv`) with `include` and synthesis macros.

## Verification

**Commands:**
- `cargo test -p bitloom --test fr199_external_ip_lock p0_online -- --ignored --nocapture` -- dedicated real-network lock/determinism/mutation gate runs and passes; default ignored discovery is not counted as PASS.
- `cargo test -p bitloom --test fr199_external_ip_lock` -- offline-safe floating-ref/CLI checks pass; the one network gate is discovered but intentionally not run.
- `python3 _agile-output/test-artifacts/130-2-atdd-replay.py --run-red` -- online fetch, copied-cache network-isolated replay and negative cases pass with nonzero test count.
- `python3 scripts/check_phase24_gate.py && cargo fmt --all -- --check && just test` -- Phase24 state, formatting and workspace regression pass; ignored tests are not counted as PASS.

## 2026-09-23 关闭复核更正

提交 `abc61b4653ad4823ab2645af6097674a8c62a8f3` 不是七步全部完成的证据。对应 Story 的 clean/regression 项未闭合，sprint 仍 review；本 spec 原 done 更正为 in-review。旧日志中有关路径/依赖/许可证修复的“已覆盖”陈述，须以当前新增负测实跑重新证实；旧 false/defer 分类中宿主文件隔离、工具身份、依赖匹配均已由 130.3 审查识别为需修复问题，不能用旧分类绕过 FR199 合同。最终须重新执行空缓存获取、复制产物禁网验证、完整负测、独立审查和干净回归，保持 FR199 与 FR200 证据分责。

## 2026-09-23 独立快照最终验证

当前实现已在独立FR199快照完成实际clean/fmt/just test（485结果块、1889 passed / 0 failed / 50 ignored）、fresh online closure与210文件身份比对、source-only禁网/原输入只读重放，以及专用联网1/1、ATDD普通/优化各3/3、consumer普通/优化各11/11。证据见 `_agile-output/test-artifacts/130-2-final-verification.md` 与 `130-2-final-regression/commands.json`。旧false/defer隔离、依赖和默认参数项已按原合同修复并实测；当前仍in-review，最终done与单Story提交由主代理处理。不提升外部试点支持行，不交付FR200。

## 最终关闭复核

2026-09-23：独立来源快照的 review 修复、automate 复验、真实 clean/fmt/workspace 及全部来源专用门禁通过。旧 false/defer 不再用于绕过隔离、精确依赖和工具身份要求。最终 1889 passed / 0 failed / 50 ignored，raw 命令与 source-only 阶段证据见 `130-2-final-verification.md` 与 `130-2-final-regression/`。只关闭 FR199/Story130.2；支持矩阵仍按 NFR14 全 no，FR200 保持后续验收。
