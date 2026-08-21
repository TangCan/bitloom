---
title: '21.2 手写 bridge / abstraction / both 回归（FR29）'
type: 'feature'
created: '2026-08-21'
status: 'done'
baseline_commit: '5c30dd22c4497b755e69dbe1277e02e92be55f0d'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/epic-21-context.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-dual-sim-generation.md'
  - '{project-root}/docs/fr29-bridge-abstraction-both.md'
  - '{project-root}/examples/mixed_both/src/lib.rs'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Epic 21 生成路径建立前，须确认阶段二手写 `#[bridge]` / `#[abstraction]` / mixed `both` 仍可用且有回归；文档仍写旧「禁止 HIR→TLM」叙事，易与 FR47 冲突，且未说明生成不取代手写标注。

**Approach:** 验证既有 `examples/mixed_both` 与 `bitloom-sim` 夹具绿；修订 FR29 文档写清手写路径与即将到来的生成路径关系；用 ATDD 锁住文档合同与夹具存在。不实现 FR47 生成器。

## Boundaries & Constraints

**Always:** `PortValues` 对照；手写路径保持可用；文档说明生成不取代手写标注；品牌 Bitloom（`bitloom-prelude` / `bitloom_sim`）；相关 `cargo test` 通过。

**Ask First:** 无。

**Never:** 实现 FR47 生成器、FR30 生成路径联验、SystemC TLM；删除手写标注能力；把「仅手写绿」写成 FR47 完成。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 混合夹具一致 | mixed_both / check_mixed_both 同刺激 | PortValues 对照 pass | N/A |
| 故意不一致 | Wrong abstraction | 对照 fail | 测试断言 is_err |
| 文档合同 | fr29 文档 | 含手写↔生成关系、不取代手写、Bitloom 包名 | ATDD 红直至改正 |

</frozen-after-approval>

## Code Map

- `docs/fr29-bridge-abstraction-both.md` — FR29 产品文档（须修订）
- `examples/mixed_both/` — Epic 8 混合夹具；`both_fixture_matches_tick` / `mismatch_fails`
- `crates/bitloom-sim/src/lib.rs` — `check_mixed_both`；`mixed_both_*`；`no_hir_to_tlm_api`（无 emit_tlm）
- `crates/bitloom-sim/src/equiv.rs` — FR30 有界等价（本故事只确认手写路径，不接生成）
- `crates/bitloom/tests/nfr14_risk_dual_sim_generation.rs` — ATDD 样板
- `_agile-output/implementation-artifacts/sprint-status.yaml` — `21-2-手写-bridge-abstraction-both-回归-fr29`

## Tasks & Acceptance

**Execution:**
- [x] `docs/fr29-bridge-abstraction-both.md` -- 修订：Bitloom 包名；手写路径与 FR47 生成路径关系（生成不取代手写）；SystemC 非合同 -- AC 文档
- [x] `crates/bitloom/tests/fr29_handwritten_bridge_regression.rs` -- ATDD：文档合同 + mixed_both 夹具源存在 -- 门禁
- [x] `crates/bitloom-sim/src/lib.rs` -- 若需：更新 `no_hir_to_tlm_api` 注释（明确禁的是 SystemC emit，非 FR47 Rust 生成）-- 叙事对齐
- [x] 跑通 `cargo test -p mixed_both` 与 bitloom-sim mixed_both/equiv 相关测试 -- 回归
- [x] `_agile-output/implementation-artifacts/sprint-status.yaml` -- 本故事 in-progress→done -- 冲刺追踪
- [x] `_agile-output/implementation-artifacts/21-2-code-review.md` -- 对抗性审查 -- 流水线
- [x] 故事文件 Dev Agent Record / Status -- 收口

**Acceptance Criteria:**
- Given 阶段二 bridge/abstraction/both，when 跑混合夹具，then PortValues 对照通过且故意不一致 fail（FR29）
- Given 文档，when 阅读 FR29，then 说明手写路径与即将到来的生成路径关系，且生成不取代手写标注能力
- Given 相关 `cargo test`，when 收口，then 通过
- Given 本故事，when 完成，then 仍未实现 FR47 生成器

## Spec Change Log

## Design Notes

Epic 8 已交付夹具与 API；本故事以回归+文档合同为主，不强行重写运行时。

## Verification

**Commands:**
- `cargo test -p mixed_both` -- expected: pass
- `cargo test -p bitloom-sim --lib mixed_both` -- expected: pass
- `cargo test -p bitloom --test fr29_handwritten_bridge_regression` -- expected: pass
- `cargo fmt --all && just test` -- expected: pass

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- 修订 `docs/fr29-bridge-abstraction-both.md`：手写↔FR47 生成关系；Bitloom 包名；SystemC 非合同
- ATDD `fr29_handwritten_bridge_regression`；mixed_both / bitloom-sim 回归绿
- `no_hir_to_tlm_api` 注释对齐 AD-5（禁 SystemC emit，不禁 FR47 Rust 生成）
- 本故事 → done；未实现 FR47 生成器

### File List

- `docs/fr29-bridge-abstraction-both.md`
- `crates/bitloom/tests/fr29_handwritten_bridge_regression.rs`
- `crates/bitloom-sim/src/lib.rs`
- `_agile-output/implementation-artifacts/21-2-手写-bridge-abstraction-both-回归-fr29.md`
- `_agile-output/implementation-artifacts/21-2-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-08-21: FR29 手写 bridge/abstraction/both 回归文档 + ATDD（Story 21.2）

## Suggested Review Order

**文档合同**

- 手写 vs FR47 生成；不取代手写；Bitloom 包名
  [`fr29-bridge-abstraction-both.md:1`](../../docs/fr29-bridge-abstraction-both.md#L1)

**ATDD 门禁**

- 文档 + mixed_both 源 + 禁 FR47 生成入口
  [`fr29_handwritten_bridge_regression.rs:15`](../../crates/bitloom/tests/fr29_handwritten_bridge_regression.rs#L15)

**既有夹具**

- PortValues match / mismatch
  [`lib.rs:99`](../../examples/mixed_both/src/lib.rs#L99)
