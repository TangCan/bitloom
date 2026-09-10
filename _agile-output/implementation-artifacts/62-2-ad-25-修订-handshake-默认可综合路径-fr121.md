---
title: '62.2 AD-25 修订 + Handshake 默认可综合路径（FR121）'
type: 'feature'
created: '2026-09-10'
status: 'done'
route: 'oneshot'
baseline_commit: '64a94b8'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic62-handshake-default.md'
  - '{project-root}/_agile-output/implementation-artifacts/62-1-epic-62-nfr14-风险记录.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-62-context.md'
  - '{project-root}/crates/bitloom/src/hls.rs'
  - '{project-root}/docs/fr35-hls.md'
  - '{project-root}/docs/fr110-hls-commercial-depth.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** AD-25 仍禁止 Handshake/动态 DF 为默认可综合语义；产品路径与 ATDD 尚未交付，无法关闭 FR121。

**Approach:** 修订 ARCHITECTURE-SPINE **AD-25**（允许 Handshake/动态 DF 为文档化默认可综合语义）；交付库/CLI Handshake 产品路径（H1–H4：ready/valid 通道、AD-18 dissolve、schedule IR/`fr121` 谓词、可读失败）；ATDD `fr121_handshake_default`；FR95/96/FR110 回归不破（NFR48）。未列入全优化套件仍 deferred（NFR51）。公开品牌 Bitloom。本故事**不**做 62.3 收口勾选。

</frozen-after-approval>

## Implementation Notes

- AD-25：移除 Prevents「禁止 Handshake 默认」；Rule 允许 FR121；Revised 戳 2026-09-10。
- API：`InTreeScheduleKind::Handshake`；`schedule_handshake_default` / `schedule_handshake_from_transform`；CLI `--handshake`。
## Review Triage Log

- AD-25 是否充分允许 Handshake 默认 — **accept**（Rule 允许 + Prevents 改防冒充 + Revised FR121 戳）。
- RTL 仅组合 ready/valid 透传 — **accept**（H3 MVP 验收谓词；弹性缓冲全家桶 NFR51 deferred）。
- AGENTS brand lock 误改 AD-5 行 — **patch**（已恢复 AD-5；FR121 写入正确 AD-25 行）。

## Dev Agent Record

### Completion Notes List

- 修订 AD-25；交付 Handshake 产品路径 + ATDD；FR95/96/FR110 不破
- sprint `62-2: done`；code-review Approve

### File List

- `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md`
- `crates/bitloom/src/hls.rs`
- `crates/bitloom/src/main.rs`
- `crates/bitloom/tests/fr121_handshake_default.rs`
- `docs/fr121-handshake-default.md`
- `docs/fr35-hls.md`
- `docs/fr110-hls-commercial-depth.md`
- `AGENTS.md`
- `_agile-output/implementation-artifacts/62-2-ad-25-修订-handshake-默认可综合路径-fr121.md`
- `_agile-output/implementation-artifacts/62-2-code-review.md`
- `_agile-output/implementation-artifacts/62-2-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
