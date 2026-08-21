---
title: '20.1 NFR14 风险记录（Chisel 双向）'
type: 'chore'
created: '2026-08-21'
status: 'done'
baseline_commit: '2a45130ef997c6a979dcd4141ab5f1822d10ee0a'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/epic-20-context.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-record-template.md'
  - '{project-root}/_agile-output/implementation-artifacts/19-1-nfr14-风险记录模板.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Epic 20（FR28/FR46）在标 ready / 开工前缺少 NFR14 风险记录，无法满足 AD-28 / PRD NFR14 门禁，且易静默降级回「尽力失败」。

**Approach:** 按 19.1 模板填写 Epic 20 / FR28+FR46 风险记录（字段 a–d 齐全），引用 addendum FR46 选项拟选方向，并用 ATDD 锁住文件存在与必填节。

## Boundaries & Constraints

**Always:** 字段 (a) 上游约束（至少含 Chisel/firtool 钉死版本、CIRCT 无 Scala FIRRTL Parser / issue#4899）、(b) 粗工期带、(c) 禁止静默降级（含不得把 FR28 改回「结构化尽力失败」而不改 PRD）、(d) 负责人；引用 FR46 选项 A/B/C 拟选（可 `[ASSUMPTION]`）；无此记录则 20.2–20.5 不得标 ready；品牌 Bitloom。

**Ask First:** 无。

**Never:** 实现 FR28/FR46 生成器或导入器；改脊柱 AD 合同（留给 20.2）；把本记录冒充 NFR14-crates。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 记录齐全 | `nfr14-risk-chisel-bidirectional.md` 含 (a)–(d) 与选项方向 | ATDD 绿 | N/A |
| 缺文件 | 路径不存在 | ATDD 红 / panic | 测试失败直至落盘 |
| 缺必填节 | 缺上游约束 / 工期 / 静默降级 / 负责人 | ATDD 红 | 补齐字段 |

</frozen-after-approval>

## Code Map

- `_agile-output/implementation-artifacts/nfr14-risk-record-template.md` — 复制源；字段 (a)–(d) 与门禁说明
- `crates/bitloom/tests/nfr14_risk_record_template.rs` — ATDD 样板（workspace_root + 读文件断言）
- `_agile-output/planning-artifacts/architecture/.../ARCHITECTURE-SPINE.md` — Stack：Chisel 7.14.0 / firtool 1.155.0；AD-28；AD-27
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md` — FR46 选项 A/B/C；issue chipsalliance/chisel#4899
- `_agile-output/planning-artifacts/epics.md` — Story 20.1 AC
- `_agile-output/implementation-artifacts/sprint-status.yaml` — `epic-20` / `20-1-nfr14-风险记录-chisel-双向`

## Tasks & Acceptance

**Execution:**
- [x] `_agile-output/implementation-artifacts/nfr14-risk-chisel-bidirectional.md` -- 按模板填写 Epic 20 / FR28+FR46 风险记录（a–d + 选项方向 + 门禁句）-- AC 正文
- [x] `crates/bitloom/tests/nfr14_risk_chisel_bidirectional.rs` -- ATDD：文件存在且含必填节/关键词 -- 门禁
- [x] `_agile-output/implementation-artifacts/sprint-status.yaml` -- `epic-20`→in-progress；本故事 in-progress→review→done -- 冲刺追踪
- [x] `_agile-output/implementation-artifacts/20-1-code-review.md` -- 对抗性审查笔记 -- 流水线
- [x] 故事文件 Dev Agent Record / File List / Status -- 收口

**Acceptance Criteria:**
- Given 19.1 模板存在，when 创建 Epic 20 / FR28+FR46 风险记录，then 含上游约束（Chisel/firtool 钉死、CIRCT 无 Scala FIRRTL Parser / #4899）、粗工期带、禁止静默降级（含不得把 FR28 改回结构化尽力失败而不改 PRD）、负责人
- Given 记录正文，when 对照 addendum，then 引用 FR46 选项 A/B/C 中拟选方向（可 `[ASSUMPTION]`）
- Given 无有效记录，when 评估 20.2–20.5，then 不得标 ready（文档门禁句明示）
- Given ATDD，when `cargo test -p bitloom --test nfr14_risk_chisel_bidirectional`，then 通过

## Spec Change Log

## Design Notes

拟选方向默认与脊柱 AD-27 一致：选项 **A**（自研 FIRRTL→Scala + 测试编译；不恢复 Parser）。工期带为粗估，供计划引用即可。

## Verification

**Commands:**
- `cargo test -p bitloom --test nfr14_risk_chisel_bidirectional` -- expected: pass
- `cargo fmt --all && just test` -- expected: pass（故事收口）

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- 填写 `nfr14-risk-chisel-bidirectional.md`：字段 a–d；拟选 FR46 选项 A（`[ASSUMPTION]`）；门禁 20.2–20.5
- ATDD `nfr14_risk_chisel_bidirectional`；审查后收紧 A/B/C 与 FR28 静默降级断言
- testarch-automate：文档门禁测试已落地（非 N/A）— 见 [20-1-code-review.md](20-1-code-review.md)
- `epic-20` → in-progress；本故事 → done

### File List

- `_agile-output/implementation-artifacts/nfr14-risk-chisel-bidirectional.md`
- `_agile-output/implementation-artifacts/20-1-nfr14-风险记录-chisel-双向.md`
- `_agile-output/implementation-artifacts/20-1-code-review.md`
- `_agile-output/implementation-artifacts/epic-20-context.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `crates/bitloom/tests/nfr14_risk_chisel_bidirectional.rs`

## Change Log

- 2026-08-21: Epic 20 NFR14 Chisel 双向风险记录 + ATDD 门禁（Story 20.1）

## Suggested Review Order

**风险记录正文**

- 字段 a–d、选项 A 拟选、20.2–20.5 ready 门禁
  [`nfr14-risk-chisel-bidirectional.md:1`](nfr14-risk-chisel-bidirectional.md#L1)

**ATDD 门禁**

- 断言钉死版本、#4899、A/B/C、静默降级
  [`nfr14_risk_chisel_bidirectional.rs:15`](../../crates/bitloom/tests/nfr14_risk_chisel_bidirectional.rs#L15)

**审查笔记**

- 对抗性发现与 Disposition
  [`20-1-code-review.md:1`](20-1-code-review.md#L1)
