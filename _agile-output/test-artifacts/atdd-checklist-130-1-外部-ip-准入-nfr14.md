---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-generation-mode', 'step-03-test-strategy', 'step-04-generate-tests', 'step-04c-aggregate', 'step-05-validate-and-complete']
lastStep: 'step-05-validate-and-complete'
lastSaved: '2026-09-22T15:56:20+08:00'
storyId: '130.1'
storyKey: '130-1-外部-ip-准入-nfr14'
storyFile: '_agile-output/implementation-artifacts/130-1-外部-ip-准入-nfr14.md'
atddChecklistPath: '_agile-output/test-artifacts/atdd-checklist-130-1-外部-ip-准入-nfr14.md'
generatedTestFiles:
  - '_agile-output/test-artifacts/130-1-atdd-presence.py'
  - '_agile-output/test-artifacts/130-1-atdd-gate.py'
  - '_agile-output/test-artifacts/130-1-atdd-manual-acceptance.md'
inputDocuments:
  - '_agile-output/implementation-artifacts/130-1-外部-ip-准入-nfr14.md'
  - '_agile-output/implementation-artifacts/sprint-status.yaml'
  - 'scripts/check_phase24_gate.py'
  - 'crates/bitloom-prelude/src/ip/blackbox.rs'
  - '.agents/skills/bmad-testarch-atdd/resources/knowledge/data-factories.md'
  - '.agents/skills/bmad-testarch-atdd/resources/knowledge/component-tdd.md'
  - '.agents/skills/bmad-testarch-atdd/resources/knowledge/test-quality.md'
  - '.agents/skills/bmad-testarch-atdd/resources/knowledge/test-healing-patterns.md'
  - '.agents/skills/bmad-testarch-atdd/resources/knowledge/test-levels-framework.md'
  - '.agents/skills/bmad-testarch-atdd/resources/knowledge/test-priorities-matrix.md'
  - '.agents/skills/bmad-testarch-atdd/resources/knowledge/ci-burn-in.md'
---

# ATDD Checklist: Story 130.1

## Preflight And Context

- Detected stack: backend (Rust workspace; root `Cargo.toml`, integration tests under `crates/bitloom/tests`).
- Story status: ready-for-dev with seven explicit acceptance criteria.
- Framework: Rust integration tests plus repository Python acceptance gates; no frontend, HTTP API, Playwright, Pact, or browser surface is relevant.
- Target: red-phase document/evidence and state-transition acceptance. Future Story130.2 source fetch/offline replay and Story130.3 external RTL behavior are out of scope for this ATDD run.
- Risk threshold: P1. Source identity, license evidence, offline integrity and false support promotion are treated as P0 assertions because failure invalidates FR199/FR200 claims.

## Generation Mode

AI generation selected. This is a backend/document-and-tooling risk gate with no browser workflow; tests derive from the Story contract, the existing Phase24 gate, and the current black-box source.

## Test Strategy

| ID | Priority | Level | Acceptance behavior | Red reason before build |
|---|---|---|---|---|
| 130.1-INT-001 | P0 | repository integration | `epic-130-nfr14.md` exists and contains NFR14 fields (a)-(d), FR199/FR200 scope, stable-vs-master rule, ownership, estimates, forbidden downgrade, stop actions and maintenance cost | Risk record does not exist |
| 130.1-INT-002 | P0 | repository integration | Source/lock/license/offline contract names complete immutable identity, recursive closure, ordered files/includes/macros, hashes, exact license/NOTICE, wrapper/tool identity and two-stage online-to-offline replay | Risk record does not exist |
| 130.1-INT-003 | P0 | repository integration | Empty black-box/zero model, compile-only, upstream-only test and core FR201 evidence are explicitly forbidden from promoting external support | Risk record does not exist |
| 130.1-INT-004 | P0 | process integration | Real sprint passes; temporary 130.2/130.3 activation without 130.1 done fails; early Epic130 done and open M0 fail; legal 130.1-done future state passes | Test scaffold not present |
| 130.1-INT-005 | P1 | repository integration | Current external support row remains all `no` / `not delivered`; 130.1 does not modify product API, tool pins, package versions or fetch third-party source | Baseline protection is not encoded for this story |
| 130.1-INT-006 | P1 | evidence integration | Tool/mechanism probe evidence records command, path/version, UTC/duration/exit and source identity; validates Git/hash/offline control plus real Icarus/Yosys black-box mechanism, while disclaiming FR199/FR200 | Probe evidence does not exist |
| 130.1-INT-007 | P1 | process integration | 130.3 dependency on 130.2 and 126.2 is explicitly audited because the generic gate only enforces `.1`; FR189/Epic122 remains deferred | Risk record does not exist |

The scaffold uses one deterministic Python integration gate with explicit `require()` checks that remain active under `python -O`. It does not add HTTP/Pact/UI tests or execute future external-source acquisition. Negative sprint cases use temporary copies and must leave the real status file byte-identical.

## Test Generation And Aggregation

- Capability probe: subagents available, agent-team unavailable; requested `auto` initially resolved to parallel subagents.
- Both narrowly scoped workers remained unresponsive after repeated requests, so the workflow stopped them and used the deterministic sequential fallback. The timeout is not reported as a clean worker review.
- API worker result: N/A, zero tests. Story130.1 exposes no HTTP API or Pact consumer/provider contract.
- E2E worker result: N/A, zero tests. Story130.1 exposes no UI, browser, authentication journey, or Playwright surface.
- Repository/process scaffolds: `130-1-atdd-presence.py`, `130-1-atdd-gate.py`, and `130-1-atdd-manual-acceptance.md`.
- Fixtures: zero. No JS dependencies, merged Playwright fixtures, mocks, selectors, or generated credentials are applicable.
- Worker outputs are persisted as `130-1-atdd-api-worker.json` and `130-1-atdd-e2e-worker.json`; the `/tmp` copies are not the only record.

The presence check is the genuine RED observation until build creates `epic-130-nfr14.md`. The 84 state scenarios deliberately verify an existing GREEN gate because AC7 requires the real and mutated status behavior; they do not claim FR199/FR200 acceptance. The 22 semantic review rows remain `unreviewed` until build/review supplies concrete evidence. No test uses removable Python `assert` as its acceptance control.

## Acceptance Criteria Coverage

| Acceptance criteria | Scaffold coverage |
|---|---|
| AC1–AC3 | Missing-record RED plus manual rows M01–M06 |
| AC4 | Manual tool/mechanism evidence rows M07–M08 |
| AC5 | Manual provenance/license/offline rows M09–M15 |
| AC6 | Manual behavior/support-level rows M16–M18 |
| AC7 | 84 isolated status scenarios plus manual rows M19–M22 |

## Playwright Utils Deviations

None. Playwright is not part of this Rust/Python repository-governance surface. Pact is likewise N/A because no consumer/provider boundary exists.

## Build Handoff

1. Run the presence check and retain its expected non-zero RED result.
2. Run the status gate in normal and optimized Python; both must pass exactly 84 scenarios without changing the real sprint file.
3. During build, create the NFR14 record, execute actual probes, and review all 22 manual rows with evidence rather than keyword matching.
4. Re-run the presence check after implementation; its transition to GREEN proves only that the target exists. Content and external-support claims require the separate evidence rows.

## Validation And Completion

- Expected RED: presence check exited 1 and named the missing NFR14 target.
- Existing process gate: 84/84 scenarios passed under ordinary Python and 84/84 under `python -O`.
- Isolation: source sprint bytes and SHA256 `f417ac120f4b3bbf399741433c270bccf63a962ee92500b08d53668ec8fe766c` remained unchanged.
- Static checks: both Python files compiled; `git diff --check` passed.
- Temp hygiene: worker results and validation summary are represented in `_agile-output/test-artifacts`; no browser sessions or long-running CLI sessions remain.
- Explicit N/A: HTTP API, Pact, browser E2E, Playwright fixtures, auth, selectors, network mocks, and frontend components.
- Remaining RED work: build the risk record and actual probes, then substantively review all 22 semantic rows. No FR199/FR200 or external support claim is made here.

Next workflow: `bmad-build` for Story130.1. The ATDD paths are linked from the Story and the result summary is `130-1-atdd-results.md`.
