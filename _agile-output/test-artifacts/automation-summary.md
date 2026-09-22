---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-identify-targets', 'step-03c-aggregate', 'step-04-validate-and-summarize']
lastStep: 'step-04-validate-and-summarize'
lastSaved: '2026-09-22T14:57:00+08:00'
inputDocuments:
  - '_agile-output/implementation-artifacts/129-3-ci-兼容性-贡献模板与核心关闭.md'
  - '_agile-output/implementation-artifacts/spec-129-3-core-closeout.md'
  - '_agile-output/test-artifacts/atdd-checklist-129-3-ci-兼容性-贡献模板与核心关闭.md'
  - '_agile-output/test-artifacts/129-3-build-evidence.md'
  - '_bmad/tea/config.yaml'
  - '.agents/skills/bmad-testarch-automate/resources/knowledge/test-levels-framework.md'
  - '.agents/skills/bmad-testarch-automate/resources/knowledge/test-priorities-matrix.md'
  - '.agents/skills/bmad-testarch-automate/resources/knowledge/data-factories.md'
  - '.agents/skills/bmad-testarch-automate/resources/knowledge/selective-testing.md'
  - '.agents/skills/bmad-testarch-automate/resources/knowledge/ci-burn-in.md'
  - '.agents/skills/bmad-testarch-automate/resources/knowledge/test-quality.md'
---

# Story129.3 Automation Summary

## Preflight

- Mode: BMad-integrated create.
- Stack: backend (Rust workspace).
- Framework: Cargo unit/integration tests under `crates/bitloom/tests`; `just test` is the workspace regression entry.
- Existing Story tests: `fr198_peripheral_system`, `fr201_core_matrix`, Python ATDD gate, strict build runner, isolated checkout replay, and required GitHub Actions job.
- Browser/mobile surfaces: none. Playwright Utils is out of scope because the repository does not use JavaScript/TypeScript Playwright for this Story.
- Pact relevance: none; no consumer/provider HTTP contract is changed. `pact_mcp_reachable=false` because SmartBear MCP tools are not available; no provider states are needed or inferred.
- Automation boundary: extend executable Rust/Python/Cargo checks only. Do not add a second framework or duplicate the 16,000-transaction system oracle.

## Coverage Plan

|ID|Priority|Level|Target|Current coverage|Automation action|
|---|---|---|---|---|---|
|129.3-INT-001|P0|Rust integration|direct/FIRRTL/Chisel execute the same complete system with frozen budget|`fr201_backend_matrix` + evidence assertions|Keep; do not duplicate slow oracle|
|129.3-INT-002|P0|Rust integration|formal prove/BMC/cover and real mutant counterexample|formal runner + evidence status|Assert counterexample VCD marker and exact formal depths|
|129.3-INT-003|P0|Rust integration|isolated replay cannot substitute deleted `/tmp` evidence|runner verifies replay status only|Assert every `source_evidence` path exists and belongs to the repository artifact roots|
|129.3-INT-004|P0|Rust integration|evidence names the RTL it actually hashed|SHA256 recorded, but consumer test does not recompute it|Recompute each `design.v` SHA256 from its source-evidence directory|
|129.3-INT-005|P1|Python contract gate|CI invokes strict runner and runner invokes isolated replay|source text partially checked|Require exact CI/runner wiring and no `continue-on-error`/failure swallowing|
|129.3-INT-006|P1|Rust integration|missing tools fail with the contract exit code|only nonzero asserted|Require exact exit 4 for all eight tool probes|
|129.3-INT-007|P1|Rust integration|SemVer covers all three surface crates and API/version claims remain unchanged|JSON section exists|Assert package set and PASS values explicitly|

Scope is selective: slow behavior/formal generation remains single-source in the build runner; automation consumers validate freshness, provenance and hard-failure semantics without creating a second RTL oracle. No UI, HTTP API, database, mobile, Pact or Playwright tests are applicable.

## Generation

- Requested mode: `auto`; capability probe initially selected subagent mode. Both dispatched workers timed out without outputs, so execution fell back to sequential worker contracts.
- API worker: 0 tests, 0 files. Story129.3 exposes no HTTP endpoint; fabricating one would be invalid coverage.
- Backend worker: 7 automated scenarios consolidated in the existing `crates/bitloom/tests/fr201_core_matrix.rs`; 5 P0 and 2 P1, one integration-test file.
- Fixtures: none. Tests consume deterministic machine evidence and repository artifacts.
- Added checks: exact formal modes/depths and counterexample VCD; exact missing-tool exit 4; owned, existing source-evidence paths; recomputed executed-RTL SHA256; exact SemVer package set and API/version claims; strict CI→runner→isolated wiring.
- No Playwright Utils or Pact deviations: their applicability/relevance gates do not hold for this Rust HDL Story.

## Validation

- Updated files: `crates/bitloom/tests/fr201_core_matrix.rs`, `129-3-atdd-gate.py`, and the producer-side provenance validation in `129-3-build-runner.py`.
- Generated structure: 5 Rust integration-test functions covering 7 planned scenarios; no fixtures, factories, helpers, UI/API specs or duplicate slow oracle.
- Dedicated producer gate: PASS, including a second detached-worktree replay, three backend runs, formal/mutation, synthesis and SemVer.
- Rust evidence-consumer burn-in: 10/10 iterations PASS; 50 test executions, 0 failed, 0 ignored.
- Python gate: normal and `python -O` PASS; `py_compile`, shell syntax and `git diff --check` PASS.
- Determinism: fixed seeds; no sleeps, retries, conditional skip, shared mutable state or committed focus. The generated evidence consumer remains fast and fresh-checkout compatible because raw-artifact ownership/SHA checks execute in the producer.
- CLI/browser sessions: N/A; none opened. Temp worker JSON files are transient orchestration data under `/tmp`; durable results are this document and `129-3-latest-results.json` under the configured test-artifact directory.
- Playwright Utils deviations: None. Applicability gates do not hold for non-Playwright Rust tests.
- Pact.js Utils deviations: N/A; no consumer-provider contract artifacts were generated.

## Assumptions And Risks

- The committed JSON is a latest successful evidence index, not a substitute for the required CI producer job. CI runs `just fr198-fr201-core-check` to recreate it and fail on stale/missing tool behavior.
- Formal remains bounded for reset/response behavior and blackboxes four CSR leaves under their published reset/hold contract; this is documented in the machine evidence.
- Original reviewer subagents were unavailable; automate execution also fell back from timed-out subagents to sequential worker contracts. No result is attributed to a reviewer that did not return.

## Next Workflow

Run the Story129.3 clean/fmt/workspace regression, then close the Story and commit its single-story change. A separate `test-review` is optional after the required regression; it is not substituted for the already-run `bmad-code-review` step.
