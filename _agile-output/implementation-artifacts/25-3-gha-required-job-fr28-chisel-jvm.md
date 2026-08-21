---
title: '25.3 GHA required job fr28-chisel-jvm'
type: 'feature'
created: '2026-08-21'
status: 'done'
baseline_commit: '6afcee4'
---

## Story

As a 贡献者 / 审查者,
I want 默认 GitHub Actions 上有独立 required job 对黄金夹具做真实 Chisel JVM 编译,
So that FR28「必须编译通过」不能只靠 Rust 谓词或 skip=0 交差。

## Tasks

- [x] T1: GHA job `fr28-chisel-jvm`
- [x] T2: docs
- [x] T3: ATDD workflow asserts
- [x] T4: sprint + epic-25 done

## Dev Agent Record

### Completion Notes List

- Parallel required job: Temurin 17 + cache sbt + setup-sbt + required compile
- timeout-minutes: 20 (tighten after cold/hot samples on first CI runs)
- ATDD `scripts/test-gha-fr28-chisel-jvm.sh`

### File List

- `.github/workflows/ci.yml`
- `docs/fr28-chisel-compilable.md`
- `scripts/test-gha-fr28-chisel-jvm.sh`
- `_agile-output/implementation-artifacts/25-3-gha-required-job-fr28-chisel-jvm.md`
- `_agile-output/implementation-artifacts/25-3-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
