---
title: '18.4 更新大纲 + README 范围与品牌'
type: 'feature'
created: '2026-08-20'
status: 'done'
baseline_commit: '1804929c18414816fbff120f6367828b4beb85a1'
review_loop_iteration: 0
context:
  - '{project-root}/docs/tutorials/rv32-episode-ii/README.md'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** `99-episode-ii-outline` 仍像「纯延期空大纲」；根 README 未反映 Episode II 真实状态。

**Approach:** 更新 99 指向 episode-ii；根 README + episode-ii 首页写清范围/非目标与 Bitloom 品牌；CSR 可选、Epic 17 不依赖 18.3。

## Boundaries & Constraints

**Always:**
- Bitloom / 与 samitbasu/rhdl 无关
- 非目标：cache/MMU/Linux/动态预测；VexRiscv 仅对照

**Never:**
- 把 CSR 写成已实现流水依赖

</frozen-after-approval>

## Tasks & Acceptance

**Execution:**
- [x] `docs/tutorials/rv32-episode-i/99-episode-ii-outline.md` — 指向实现/教程状态
- [x] 根 `README.md` — Episode I/II 表 + 非目标
- [x] Episode I README 99 行；Episode II README 范围段
- [x] sprint-status 18-1..18-4 + epic-18 → done

**Acceptance Criteria:**
- Given 18.1–18.2 就位（18.3 延期），when 更新大纲与 README，then 反映真实状态与品牌
- And CSR 可选；Epic 17 不依赖 18.3

## Dev Agent Record

### Completion Notes

- 99 不再写「未实现」为主叙事；流水已实现、CSR 延期
- **未 git commit**

### File List

- `docs/tutorials/rv32-episode-i/99-episode-ii-outline.md`
- `docs/tutorials/rv32-episode-i/README.md`
- `docs/tutorials/rv32-episode-ii/README.md`
- `README.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `_agile-output/implementation-artifacts/18-4-更新大纲-readme-范围与品牌.md`
