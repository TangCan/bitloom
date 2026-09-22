---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-generation-mode', 'step-03-test-strategy', 'step-04c-aggregate', 'step-05-validate-and-complete']
lastStep: 'step-05-validate-and-complete'
lastSaved: '2026-09-22T13:05:49+08:00'
storyId: '129.3'
storyKey: '129-3-ci-兼容性-贡献模板与核心关闭'
storyFile: '_agile-output/implementation-artifacts/129-3-ci-兼容性-贡献模板与核心关闭.md'
atddChecklistPath: '_agile-output/test-artifacts/atdd-checklist-129-3-ci-兼容性-贡献模板与核心关闭.md'
generatedTestFiles:
  - '_agile-output/test-artifacts/129-3-atdd-gate.py'
  - 'crates/bitloom/tests/fr201_core_matrix.rs'
inputDocuments:
  - '_bmad/tea/config.yaml'
  - '_agile-output/implementation-artifacts/129-3-ci-兼容性-贡献模板与核心关闭.md'
  - '_agile-output/implementation-artifacts/129-2-外设子系统与使用配方.md'
  - '_agile-output/implementation-artifacts/epic-129-context.md'
  - '_agile-output/implementation-artifacts/epic-129-nfr14.md'
  - 'docs/ip/phase24-contract.md'
  - 'crates/bitloom/tests/fr198_peripheral_system.rs'
  - 'crates/bitloom/tests/fr198_peripheral_system/axi_tb.sv'
  - 'crates/bitloom/tests/fr198_peripheral_system/direct_tb.sv'
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
---

# ATDD Checklist - Story 129.3

## Step 1: Preflight And Context

- Detected stack: `backend`（Rust workspace + SystemVerilog/JVM/formal 外部工具链）。
- Story prerequisite: `ready-for-dev`，包含 9 项可验证 AC；129.1/129.2 与 M0-M3 已完成。
- Test framework: Cargo integration tests under `crates/bitloom/tests/`，现有 FR198 Rust runner、AXI/direct SV testbench、formal ignored gates 与 CI patterns 可复用。
- Development environment: Rust 1.97.1 已钉；本机已知 Yosys 0.33，Icarus/VVP 12.0 需显式 PATH；firtool/JVM/formal 工具在后续 preflight 中逐项硬检查。
- Affected components: FR198 system generator and benches, FIRRTL/Chisel lowering runners, formal/synthesis gates, `Justfile`, GitHub Actions, SemVer evidence, contribution template and Epic129 closeout.
- Integration boundaries: single-session AD-30 HIR, direct/FIRRTL/Chisel actual RTL execution, sby/Yosys, cargo-semver-checks, isolated checkout evidence.
- Playwright Utils: not applicable because this repository has no `package.json`, Playwright runner, or `@seontechnologies/playwright-utils` dependency. No browser/API test artifacts will be generated.
- Pact.js/Pact MCP: not relevant; Story has no HTTP service or consumer/provider contract.
- Priority: P0 for truthful three-backend execution and hard-fail evidence; P1 for formal/synthesis/CI/SemVer/contributor governance.
- Quality constraints: fixed seeds, no optional assertion path, no silent skip, non-zero negative controls, explicit counts, isolated artifact roots, and evidence archived before clean.

## Step 2: Generation Mode

- Mode: AI generation.
- Reason: backend-only Rust/RTL/JVM/formal workflow with explicit source contracts and no browser UI. Recording mode is inapplicable.
- Red-phase target: executable source/evidence gate that fails until three-backend behavior, formal/synthesis, CI, SemVer, contribution and closeout artifacts exist and contain falsifiable results.

## Step 3: Test Strategy

| Test ID | AC | Priority | Level | Scenario / falsifiable outcome |
|---|---:|---|---|---|
| 129.3-INT-001 | 1,2 | P0 | Integration | Build the existing complete AXI and direct four-leaf graphs; execute direct/FIRRTL/Chisel lowered RTL against independent expected behavior. Each route must report non-zero transactions/assertions and a non-empty VCD. |
| 129.3-INT-002 | 2 | P0 | Integration | Across each route, check fixed windows, SLVERR/DECERR, WSTRB, response backpressure, shared reset cancellation, Timer/GPIO/UART and all five IRQ sources. A route-specific emit or compile-only result fails. |
| 129.3-INT-003 | 3 | P0 | Integration | Verify exact Rust/firtool/Chisel/Scala/sbt identities; force each required-tool-missing path and require non-zero exit. Missing marker, timeout and zero-count output fail. |
| 129.3-INT-004 | 4 | P0 | Integration | Run bounded system prove and cover, record depths/status, and execute a mutation/negative control that must fail. Run Yosys hierarchy/check/stat on each final RTL route. |
| 129.3-UNIT-005 | 5 | P1 | Source contract | Parse `Justfile` and CI YAML structurally/textually enough to prove a dedicated required command/job exists, contains no ignored failure, pins required tool versions, and archives failure artifacts. Actual local command execution remains INT-001/004. |
| 129.3-INT-006 | 6 | P1 | Integration | Run SemVer failure-propagation test and real registry-backed checks for prelude/sim/firrtl; compare public API inventory and package versions to baseline so this story cannot silently expand or publish. |
| 129.3-UNIT-007 | 7 | P1 | Document contract | Validate contribution template required fields and support matrix state vocabulary; prove external rows cannot be PASS/maintained before Epic130 evidence. |
| 129.3-INT-008 | 8 | P0 | Integration | Clone HEAD, apply current diff, use isolated Cargo/RTL/evidence roots, then run the one-command gate. Assert source/tool/command/exit/hash/log/wave records and a real missing-tool negative result. |
| 129.3-UNIT-009 | 9 | P1 | Closeout contract | Validate story/epic status and claim partition: Epic129 core done only after all evidence; Epic130/FR199/FR200/external FR201/Phase24/FR189/NFR91 remain open or deferred. |

### Coverage Partition

- Behavior is asserted only in integration tests against executed RTL; source-contract tests may verify wiring and CI declarations but cannot grant behavior PASS.
- Formal proves selected invariants and cover reachability; it does not duplicate or replace transaction behavior.
- Yosys synthesis checks elaborated structure and reports raw stats; it does not claim PPA.
- SemVer checks API compatibility; source inventory checks claim discipline. Neither proves RTL behavior.
- Document gates validate maintainer usability and status honesty; they cannot close Epic130.

### Red Phase

- A new ATDD gate will require not-yet-created backend/formal/CI/contribution/closeout artifacts and therefore fail before build implementation.
- Existing Story129.2 behavior remains green and is not weakened to manufacture red.
- Red execution must show a specific missing-artifact/contract failure and non-zero exit; syntax errors or unavailable baseline tools are not accepted as the intended red signal.

## Step 4: Red-Phase Scaffolds

- Requested mode: `auto`; initially resolved to `subagent`.
- Runtime result: two parallel attempts produced no output and were terminated. Deterministic fallback: `sequential` using the same worker contracts; the timeouts are not counted as review or test success.
- Source/evidence gate: `_agile-output/test-artifacts/129-3-atdd-gate.py` (1 P0/P1 aggregate gate). It is intentionally active when invoked, is not wired into default CI during red phase, and currently exits non-zero for missing build artifacts.
- RTL integration scaffold: `crates/bitloom/tests/fr201_core_matrix.rs` (3 P0 scenarios), each marked `#[ignore = "ATDD RED: ..."]` until its corresponding implementation task is activated.
- TDD mapping deviation: this repository does not use Playwright, so `test.skip()` maps to Rust `#[ignore]`; the standalone Python gate remains executable to demonstrate the intended red failure.
- No fixtures were generated: required inputs are product evidence artifacts created during build, not synthetic test data.
- Acceptance coverage: AC1–9 represented; behavior and tool hard-failure checks are integration-level, CI/contribution/claim checks are source-contract level.

### Activation Order

1. Run `python3 _agile-output/test-artifacts/129-3-atdd-gate.py` and preserve the expected non-zero red output.
2. Implement the real backend runner, then activate only `p0_three_backends_execute_the_complete_four_peripheral_system` and drive it green.
3. Implement formal/synthesis and activate the second Rust scenario.
4. Implement missing-tool negatives and isolated replay, then activate the third scenario.
5. Add CI/SemVer/contribution/support/closeout artifacts and drive the Python gate green.

## Step 5: Validation And Handoff

- Prerequisites: PASS. Story AC、Cargo integration framework、existing FR198 patterns and development environment are present.
- Scaffold compile: PASS. `cargo test -p bitloom --test fr201_core_matrix` compiled and reported exactly `0 passed / 0 failed / 3 ignored` with explicit RED reasons.
- Python RED: PASS. Normal Python and `python -O` each exited 1 with `ATDD RED` and concrete missing artifact/contract messages; no removable `assert` controls the gate.
- Activated Rust RED: PASS. Running the first ignored scenario alone exited 101 because `129-3-latest-results.json` does not exist. This is the expected pre-build failure.
- Metadata/handoff: PASS. Story ID/key/file, checklist path, generated paths and Story `ATDD Artifacts` links are populated.
- Determinism: PASS by inspection. No unseeded randomness, sleeps, network, shared mutable state or order dependency exists in the scaffolds.
- Placeholder assertions: none. Each Rust assertion checks a required count/status/coverage fact; Python requirements name a contract field or artifact.
- Browser/API/component/data-testid/factory/mock sections from the generic checklist: N/A. This Story has no UI or HTTP service and introducing Playwright/faker/Pact would violate project scope.
- CLI browser sessions: N/A; none opened. Temporary orchestration JSON remains in `/tmp` only as workflow transport; all durable artifacts are under the repository test-artifact/test paths.

### Commands

```bash
# Demonstrate Python RED, then later GREEN after implementation
python3 _agile-output/test-artifacts/129-3-atdd-gate.py

# Compile all Rust scaffolds without activating them
cargo test -p bitloom --test fr201_core_matrix

# Activate one task at a time during build
cargo test -p bitloom --test fr201_core_matrix -- --ignored \
  p0_three_backends_execute_the_complete_four_peripheral_system
```

### Completion Summary

- Story: `129.3`; primary level: backend integration with source-contract supplements.
- Generated: 1 Python aggregate RED gate and 3 Rust P0 ignored scenarios; 0 browser/API/component tests, 0 factories, 0 fixtures, 0 mocks, 0 data-testid requirements.
- Main risks: external tool availability and matrix runtime; CI must pin tools, use explicit timeout, preserve failure artifacts, and never convert missing tools to skip.
- Estimated implementation: 3–5 person-days per Epic definition; matrix and isolated replay dominate wall clock.
- Next workflow: `bmad-build` after this ATDD handoff; `bmad-testarch-automate` remains the fifth user-mandated Story step after code review.
