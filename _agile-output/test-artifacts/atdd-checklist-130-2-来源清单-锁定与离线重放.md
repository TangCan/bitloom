---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-generation-mode', 'step-03-test-strategy', 'step-04c-aggregate', 'step-05-validate-and-complete']
lastStep: 'step-05-validate-and-complete'
lastSaved: '2026-09-22T17:28:55+08:00'
storyId: '130.2'
storyKey: '130-2-来源清单-锁定与离线重放'
storyFile: '_agile-output/implementation-artifacts/130-2-来源清单-锁定与离线重放.md'
atddChecklistPath: '_agile-output/test-artifacts/atdd-checklist-130-2-来源清单-锁定与离线重放.md'
generatedTestFiles:
  - 'crates/bitloom/tests/fr199_external_ip_lock.rs'
  - '_agile-output/test-artifacts/130-2-atdd-replay.py'
inputDocuments:
  - '_agile-output/implementation-artifacts/130-2-来源清单-锁定与离线重放.md'
  - '_agile-output/implementation-artifacts/epic-130-nfr14.md'
  - '_agile-output/implementation-artifacts/epic-130-context.md'
  - 'docs/ip/phase24-contract.md'
  - '.agents/skills/bmad-testarch-atdd/resources/knowledge/data-factories.md'
  - '.agents/skills/bmad-testarch-atdd/resources/knowledge/component-tdd.md'
  - '.agents/skills/bmad-testarch-atdd/resources/knowledge/test-quality.md'
  - '.agents/skills/bmad-testarch-atdd/resources/knowledge/test-healing-patterns.md'
  - '.agents/skills/bmad-testarch-atdd/resources/knowledge/test-levels-framework.md'
  - '.agents/skills/bmad-testarch-atdd/resources/knowledge/test-priorities-matrix.md'
  - '.agents/skills/bmad-testarch-atdd/resources/knowledge/ci-burn-in.md'
  - '.agents/skills/bmad-testarch-atdd/resources/knowledge/playwright-utils-mandate.md'
  - '.agents/skills/bmad-testarch-atdd/resources/knowledge/overview.md'
  - '.agents/skills/bmad-testarch-atdd/resources/knowledge/api-request.md'
  - '.agents/skills/bmad-testarch-atdd/resources/knowledge/auth-session.md'
  - '.agents/skills/bmad-testarch-atdd/resources/knowledge/recurse.md'
  - '.agents/skills/bmad-testarch-atdd/resources/knowledge/pact-mcp.md'
---

# ATDD Checklist: Story 130.2 来源清单、锁定与离线重放

## Preflight And Context

- Detected stack: backend（Rust workspace / CLI / filesystem / external tool integration）。
- Test framework: Cargo unit/integration tests under `crates/*/tests`, plus repository gate scripts and actual external-tool probes.
- Story status: `ready-for-dev`; acceptance criteria and architecture boundaries are explicit.
- Primary affected boundary: CLI/build tooling, machine-readable manifest/lock, content-addressed cache, license evidence and isolated replay.
- Test levels: fast schema/path/hash/error unit tests; filesystem/process integration tests; one actual empty-cache online acquisition followed by network-isolated replay.
- Priority: identity/content/license/network/cache-escape failures are P0; diagnostics, deterministic serialization and scope checks are P1.
- Isolation: every mutation uses a copied manifest/lock/cache; tests must prove canonical artifacts and sprint bytes remain unchanged.
- Playwright Utils: N/A. The project has no Node/Playwright runner or HTTP/UI surface; the two applicability gates are false.
- Pact/Pact MCP: N/A. There is no consumer-provider service contract, so no Pact artifact or broker/provider-state query is generated.
- Browser/mobile/API contract testing: N/A; no invented endpoints, selectors, provider states or browser flows.

## Loaded Contract Summary

- Manifest expresses intent; lock expresses immutable resolved identity.
- Stable candidate is PULP `common_cells` `v1.40.0`, lightweight tag/full commit `1281545696eb3fcba50ec5b4275993476a3c710e`, with actual `src/fifo_v3.sv` and Bender-declared dependencies requiring explicit closure treatment.
- Stage A must fetch from an empty cache with network allowed. Stage B must use only copied Stage A artifacts under actual network isolation and a clean environment.
- Floating refs, identity/content/license/tool drift, undeclared files, missing closure members, cache escape and network access must fail non-zero.
- This Story cannot claim FR200, actual Bitloom wrapper behavior, compiled/behavior-tested/maintained support, Epic130 close or Phase24 close.

## Generation Mode

AI generation. This is a backend/CLI and filesystem/process integration story with explicit acceptance criteria and inspectable source contracts. Browser recording is inapplicable; executable red-phase Rust/script scaffolds will be derived from the Story, architecture and existing repository patterns.

## Test Strategy

| Test ID | Pri | Level | AC | Red-phase scenario |
|---|---|---|---|---|
| 130.2-INT-001 | P0 | Rust integration | 2,3 | Product entry can parse the candidate manifest and emits a deterministic lock with schema, lightweight tag/full commit, exact top/module/interface, ordered files/includes/macros and complete dependency identities; currently no entry/schema exists. |
| 130.2-UNIT-001 | P0 | Unit/integration mutation matrix | 2,6 | Validator rejects floating refs, tag/commit mismatch, absent dependency identity, reordered/missing/extra file, source/archive hash drift, missing generator input and tool drift with distinct diagnostics. |
| 130.2-INT-002 | P0 | Filesystem integration | 4,6 | Exact LICENSE/NOTICE facts and hashes are required for every resolved repository; content drift, missing material and unreviewed obligation fail without rewriting canonical artifacts. |
| 130.2-E2E-001 | P0 | Backend process E2E | 5 | Empty-cache online acquisition produces a complete movable cache/lock, then a separate `bwrap --unshare-net` environment validates and actually parses/compiles the locked FIFO source using only copied artifacts. |
| 130.2-E2E-002 | P0 | Backend process negative E2E | 5,6 | Offline replay proves DNS/HTTP/Git unavailable and host HOME/XDG/Git/Cargo/tool caches invisible; attempted network or cache escape fails non-zero instead of falling back. |
| 130.2-INT-003 | P0 | Archive/cache integration | 5,6 | Cache publication is atomic and rejects absolute/parent traversal plus escaping symlink/hardlink and undeclared archive members; failed extraction leaves no usable partial entry. |
| 130.2-INT-004 | P1 | CLI/schema integration | 2,7 | Generate and verify are separate commands; verify is read-only, deterministic and never auto-locks. Invalid schema/version/path input exits non-zero with actionable errors. |
| 130.2-INT-005 | P1 | Architecture/scope audit | 1,7,8 | Design crates remain prelude-only, no third-party source enters HIR/repo product source, pins/package versions/public API/support claims do not expand, and 130.3 remains backlog. |
| 130.2-INT-006 | P1 | Evidence integrity | 5,6,8 | Evidence records exact commands, sanitized environment, UTC/duration/exit/tool identities and source hashes; normal and optimized harness modes agree and canonical manifest/lock/cache/sprint hashes remain unchanged by negative tests. |

### Coverage Boundaries

- Unit/integration mutation tests own deterministic schema, identity, path, hash and diagnostic branches; the expensive process E2E does not duplicate every mutation.
- The process E2E uniquely proves real upstream acquisition, movable cache, network isolation and actual HDL consumption. A mocked fetch cannot satisfy it.
- Source lock compilation proves FR199 replay only. FIFO functional behavior, Bitloom wrapper binding and support levels above `locked` stay out of this suite and remain Story130.3 work.
- All P0 scaffolds must fail before implementation because the product manifest/lock/replay entry and canonical candidate artifacts do not exist. Existing Story130.1 gates remain green and are not relabeled as RED.

## TDD Red Phase Scaffolds

- Rust CLI/schema: `crates/bitloom/tests/fr199_external_ip_lock.rs` — 4 tests, all protected by `#[ignore = "ATDD RED: Story130.2 implementation pending"]`.
- Backend process replay: `_agile-output/test-artifacts/130-2-atdd-replay.py` — 3 tests, all protected by `@unittest.skipUnless(RUN_RED, ...)`; activate only with `--run-red`.
- Domain-equivalent skip validation: the workflow's literal `test.skip()` check targets TypeScript. Rust `#[ignore]` and Python `skipUnless` provide the same default-red isolation while keeping each scaffold parseable by the repository's native toolchain.
- Total: 7 RED scenarios; P0=6, P1=1. No placeholder assertions, active passing feature tests, HTTP endpoints, UI selectors or Pact interactions were generated.
- Fixtures are local temporary directories and copied manifest/lock/cache trees. No Playwright merged fixture is created because both Playwright applicability gates are false.

### Worker Execution Evidence

Requested mode was `auto`; capability resolution selected subagent mode. Three worker dispatch variants produced no output and were terminated after timeouts. The workflow then used its deterministic sequential fallback, preserving failed worker metadata in `/tmp` and generating the same API/E2E output contracts without claiming parallel completion or speedup.

## Task-by-Task Activation

1. Implement the manifest/lock schema and CLI contract, then remove `#[ignore]` only from the current Rust scenario. Confirm it fails before the corresponding implementation and passes afterward.
2. Implement the fetch/replay runner and canonical candidate artifacts, then run `python3 _agile-output/test-artifacts/130-2-atdd-replay.py --run-red`. It must fail before implementation and pass only with actual online acquisition plus isolated offline replay.
3. Keep mutation tests on copied fixtures; canonical manifest, lock, cache and sprint hashes must remain unchanged.
4. Missing tools, zero executed tests, ignored tests, unavailable network isolation and timeouts are not PASS.
5. Story130.3 behavior and support-level tests remain separate; do not activate them through this Story.

## Fixture Needs

- Isolated temporary manifest/lock/cache factory.
- Canonical `ip/external/pulp-common-cells-fifo-v3.source.json` and `.source.lock.json` produced during build.
- `scripts/phase24-external-ip-replay.py` with explicit fetch/replay/negative-test surfaces.
- Git/SHA256, bubblewrap and an actual HDL parser/compiler with recorded identity.

## Playwright Utils Deviations

None. Playwright is not installed or used, and the Story has no HTTP/UI surface.

## Validation Result

PASS for the ATDD RED phase.

- Rust default: exit 0; 4 discovered, 0 passed, 0 failed, 4 ignored. Log: `130-2-atdd-rust-default.log`.
- Rust explicit RED: exit 101; 4/4 failed because `cargo-bitloom` rejects the not-yet-implemented `external-ip` subcommand. Log: `130-2-atdd-rust-red.log`.
- Python default: exit 0; 3 discovered, 0 passed, 0 failed, 3 skipped. Log: `130-2-atdd-python-default.log`.
- Python explicit RED: exit 1; 3/3 failed because `scripts/phase24-external-ip-replay.py` does not yet exist. Log: `130-2-atdd-python-red.log`.
- `cargo fmt --all -- --check`, Python compile and `git diff --check` pass after scaffold repair.
- Ignored/skipped scenarios are not counted as product PASS. Explicit activation demonstrates the intended missing-feature RED state rather than a syntax/import failure.
- No browser session was opened, so no CLI browser cleanup is required. Persistent evidence is stored under `_agile-output/test-artifacts/`; `/tmp` contains only orchestration JSON and is not product evidence.

## Completion Summary

- Story: 130.2 / `130-2-来源清单-锁定与离线重放`.
- Primary levels: Rust CLI/schema integration plus backend process integration.
- Generated tests: 7 total (4 Rust, 3 Python); factory/fixture mechanism: 2 isolated temporary-directory helpers; mocks/data-testid: N/A.
- Key assumption to resolve in build: final CLI and runner shape may be refined, but generate/verify/replay must remain separate and fail closed. Update scaffolds and checklist together if command names change.
- Next workflow: `bmad-build`; activate each current scenario before implementing its slice, obtain RED, implement to GREEN, then retain the tests as normal regression coverage.
