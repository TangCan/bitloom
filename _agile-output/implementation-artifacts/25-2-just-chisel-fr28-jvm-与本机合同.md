---
title: '25.2 just chisel-fr28-jvm 与本机合同'
type: 'feature'
created: '2026-08-21'
status: 'done'
baseline_commit: 'b0a0f68'
context:
  - '{project-root}/_agile-output/implementation-artifacts/25-1-必失败-fr28-jvm-编译脚本.md'
warnings: []
deferred: []
---

## Story

As a 维护者 / 贡献者,
I want 有文档化的 `just chisel-fr28-jvm`,
So that 本机不必改 `just test` 默认门槛，又能与 CI 同一路径对齐。

## Tasks / Subtasks

- [x] T1: Justfile `chisel-fr28-jvm` + `chisel-fr28-atdd`
- [x] T2: docs + README
- [x] T3: ATDD `scripts/test-just-chisel-fr28-jvm.sh`
- [x] T4: sprint → done

## Dev Agent Record

### Completion Notes List

- `just chisel-fr28-jvm` → required 脚本 + 黄金夹具；`just test` 仍仅 cargo
- 文档/README；ATDD 绿；审查 Approve

### File List

- `Justfile`
- `docs/fr28-chisel-compilable.md`
- `README.md`
- `scripts/test-just-chisel-fr28-jvm.sh`
- `_agile-output/implementation-artifacts/25-2-just-chisel-fr28-jvm-与本机合同.md`
- `_agile-output/implementation-artifacts/25-2-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
