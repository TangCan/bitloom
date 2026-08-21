---
title: '24.1 NFR14 风险记录（HLS）'
type: 'chore'
created: '2026-08-21'
status: 'done'
baseline_commit: '35cde2f'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/epic-24-context.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-record-template.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Epic 24（FR35/FR50）在标 ready / 开工前缺少 NFR14 风险记录，易静默把 HLS 做成「永久 unsupported」。

**Approach:** 按 19.1 模板填写 Epic 24 风险记录（字段 a–d），钉死 Bambu 2024.10，并用 ATDD 锁住文件存在与必填节。

## Boundaries & Constraints

**Always:** 字段 (a)–(d)；单一后端名称与版本策略；禁止「永久 unsupported」与树内 scheduler；无此记录则 24.2–24.4 不得标 ready；品牌 Bitloom。

**Ask First:** 无。

**Never:** 实现产品 HLS 路径或 CI（留给 24.2–24.3）；引入树内调度；把本记录冒充 NFR14-crates。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 记录齐全 | `nfr14-risk-hls.md` 含 (a)–(d) 与 Bambu 2024.10 | ATDD 绿 | N/A |
| 缺文件 | 路径不存在 | ATDD 红 | 测试失败直至落盘 |
| 缺必填节 | 缺上游约束 / 工期 / 静默降级 / 负责人 | ATDD 红 | 补齐字段 |

</frozen-after-approval>

## Code Map

- `_agile-output/implementation-artifacts/nfr14-risk-record-template.md` — 复制源
- `crates/bitloom/tests/nfr14_risk_chisel_bidirectional.rs` — ATDD 样板
- `_agile-output/planning-artifacts/epics.md` — Story 24.1 AC
- `_agile-output/implementation-artifacts/sprint-status.yaml` — epic-24 键

## Tasks & Acceptance

**Execution:**
- [x] `nfr14-risk-hls.md` -- 填写 a–d + Bambu 2024.10 + 门禁 -- AC
- [x] `nfr14_risk_hls.rs` -- ATDD -- 门禁
- [x] `sprint-status.yaml` -- 仅更新 epic-24 / 24-1 键 -- 冲刺追踪
- [x] `24-1-code-review.md` -- 对抗性审查 -- 流水线
- [x] 故事文件收口 -- done

**Acceptance Criteria:**
- Given 19.1 模板存在，when 创建 Epic 24 风险记录，then 含上游约束（钉死 Bambu 或 Vitis、许可/安装、CI）、粗工期带、禁止静默降级、负责人
- Given 记录正文，when 阅读版本策略，then 明确所选单一后端名称与版本
- Given 无有效记录，when 评估 24.2–24.4，then 不得标 ready

## Spec Change Log

## Design Notes

选定 **Bambu 2024.10**（非 Vitis）：开源、可 AppImage、与 Story 9.2 既有钩子一致。

## Verification

**Commands:**
- `cargo test -p bitloom --test nfr14_risk_hls` -- expected: pass
- `cargo fmt --all && just test` -- expected: pass

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- 填写 `nfr14-risk-hls.md`：钉死 Bambu 2024.10；门禁 24.2–24.4
- ATDD `nfr14_risk_hls`；审查 accept
- 仅更新 sprint 的 epic-24 / 24-1 键（不碰 epic-23）

### File List

- `_agile-output/implementation-artifacts/nfr14-risk-hls.md`
- `_agile-output/implementation-artifacts/24-1-nfr14-风险记录-hls.md`
- `_agile-output/implementation-artifacts/24-1-code-review.md`
- `_agile-output/implementation-artifacts/epic-24-context.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `crates/bitloom/tests/nfr14_risk_hls.rs`

## Change Log

- 2026-08-21: Epic 24 NFR14 HLS 风险记录 + ATDD（Story 24.1）

## Suggested Review Order

**风险记录** → **ATDD** → **sprint 键**
