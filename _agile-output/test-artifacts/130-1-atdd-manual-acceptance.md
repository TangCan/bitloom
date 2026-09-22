# Story130.1 ATDD Manual Acceptance

Scope: review the Epic130 NFR14 risk gate. This is not browser E2E, FR199 source-lock evidence, FR200 external-core behavior, or proof that a future offline replay succeeded. Build review and the final seven-step closeout were performed 2026-09-22 by Codex against the full risk record, raw probe results, final diff, and regression evidence.

Target: `_agile-output/implementation-artifacts/epic-130-nfr14.md` plus Story130.1 probe evidence.

| ID / AC / Priority | Acceptance question | Build disposition | Evidence |
|---|---|---|---|
| M01 / AC1 / P0 | Does the record preserve Epic126–129 closure, FR201 core-only closure, external `no/not delivered`, FR189/Epic122 deferred, and no push/publish? | PASS | Risk record opening and (a); support matrix independently remains unchanged |
| M02 / AC2 / P0 | Are NFR14 fields (a)–(d) substantive: upstream constraints, estimate, forbidden downgrades, and named ownership? | PASS | Full risk record (a)–(d), reviewed as prose rather than keyword presence |
| M03 / AC2 / P0 | Are per-story dependencies, support parameters, failure actions, stop conditions, and maintenance costs assigned? | PASS | Dependency table, (b), risk/stop table and (d) |
| M04 / AC3 / P0 | Is PULP common_cells v1.40.0 explicitly only a candidate pending tag object, peeled commit, stable `fifo_v3` API/path, closure and license verification? | PASS | `(a) 候选与稳定版本边界` |
| M05 / AC3 / P0 | Does candidate rejection stop rather than silently switch to Taxi, floating master, copied files, or a second IR? | PASS | Candidate boundary and (c) forbidden downgrade list |
| M06 / AC3 / P0 | Is 130.3 responsible for the real module, parameters, flat ports, clock/reset, filelist, wrapper and minimal-HIR escalation? | PASS | Architecture/tool boundary, FR200 contract and dependency table |
| M07 / AC4 / P1 | Does tool discovery record command, path/version, UTC, duration, exit, logs and source identity for Git, SHA256, network denial, Icarus/vvp, Yosys and source-closure tooling? | PASS | Both build-probe result JSONs: 17 commands each; paths/hashes/logs/timing/exits, SHA cross-check, recursive submodule and archive identity |
| M08 / AC4 / P0 | Does the real external-shell compile probe prove only file-binding mechanics and explicitly reject FR199/FR200/support promotion? | PASS | `rtl-{compile,simulate,synthesis}.log`, result limitations and risk-record probe boundary |
| M09 / AC5 / P0 | Are manifest intent and immutable lock identity distinct? | PASS | FR199 contract opening |
| M10 / AC5 / P0 | Does immutable identity include URL, tag intent/object, peeled/full commit, recursive commits, ordered files, includes, macros, generators, hashes and tool identity? | PASS | FR199 field table |
| M11 / AC5 / P0 | Are top module, parameters, ports, clock/reset, wrapper version, tested configuration, owner, evidence and last-pass time bound? | PASS | FR199 binding/maintenance rows and FR200 contract |
| M12 / AC5 / P0 | Are exact license text path/hash, NOTICE and redistribution duties mandatory and fail closed? | PASS | FR199 license row plus license risk/stop action |
| M13 / AC5 / P0 | Is replay two-stage: empty-cache online closure capture, then isolated checkout/cache with network denied for build and test? | PASS | FR199 two-stage replay list |
| M14 / AC5 / P0 | Do undeclared files, floating refs, cache-external reads, network attempts and identity drift hard-fail? | PASS | FR199 hard-failure paragraph and (c) |
| M15 / AC5 / P0 | Does source acquisition stay in CLI/build ownership while `bitloom-prelude` remains source/network-free? | PASS | Architecture/tool boundary and AD-31 alignment |
| M16 / AC6 / P0 | Are upstream tests and independent Bitloom composed-RTL behavior both required for FR200? | PASS | FR200 contract opening |
| M17 / AC6 / P0 | Are empty/zero blackboxes, compile-only, upstream-only tests and core FR201 evidence forbidden from `behavior-tested`? | PASS | Support-level non-evidence paragraph and actual probe limitations |
| M18 / AC6 / P1 | Are `catalogued -> locked -> compiled -> behavior-tested -> maintained` cumulative, with CI/owner/parameters/upgrade policy required for maintained? | PASS | Support-level transition table |
| M19 / AC7 / P0 | Do actual and mutated sprint gates produce exact expected exits/diagnostics without changing the real sprint bytes? | PASS | ATDD result: normal 84/84 and optimized 84/84; source SHA unchanged |
| M20 / AC7 / P0 | Is 130.3 -> 130.2 and 126.2 manually audited, since the generic gate only enforces 130.1? | PASS | Dependency table and probe-boundary closing paragraph |
| M21 / AC7 / P0 | Are unresolved provenance/tool/license/representation/behavior blockers stop conditions that keep 130.2 backlog? | PASS | Risk/stop table; sprint still shows 130.2/130.3 backlog |
| M22 / AC7 / P0 | After all seven workflow steps, is only 130.1 closed, with no API/tool pin/package/version/source/support expansion? | PASS | `130-1-final-regression.exit` = 0; 484 result blocks, 1878 passed / 0 failed / 49 ignored; final diff has no product source, API, support matrix, Cargo/toolchain/package/version or publish changes; Epic130 remains in-progress and 130.2/130.3 remain backlog |

Reviewers must reject a row that cites only a filename, heading, tool version, successful download, compile-only result, or prior core evidence. Unknown and not-run are valid outcomes; they are not PASS.
