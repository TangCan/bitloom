---
title: '37.3 可选夜间真机 Bambu 或显式保持 stub（FR88）'
type: 'docs'
created: '2026-09-09'
status: 'done'
baseline_commit: 'b6f2a4b'
review_loop_iteration: 0
context:
  - '{project-root}/docs/fr35-hls.md'
  - '{project-root}/README.md'
  - '{project-root}/.github/workflows/ci.yml'
  - '{project-root}/scripts/hls-smoke.sh'
  - '{project-root}/scripts/fixtures/bambu-ci-stub.sh'
  - '{project-root}/_agile-output/implementation-artifacts/deferred-work.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic37-interop-hls.md'
  - '{project-root}/_agile-output/implementation-artifacts/37-2-firtool-chisel-钉死运维与机械-chisel-诚实声明-fr88.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
  - '{project-root}/_agile-output/implementation-artifacts/process-one-story-one-commit.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR88（HLS 诚实条）要求要么可验真机 Bambu（失败不 ignore），要么显式保持 stub 默认并写明；`docs/fr35-hls.md` / deferred-work 仍把「可选夜间真机 job」标为未实现 deferred，易被读成「迟早默认真机」或「stub 绿 = HLS 质量已验」。Epic 37 关闭条件尚未勾选。

**Approach:** **Path B（默认偏好）** — 不新增夜间真机 CI job。文档钉死：本阶段 / Epic 37 **显式选择保持 stub 默认**；真机仅 `BITLOOM_HLS_USE_REAL=1`（+ `BITLOOM_BAMBU_PATH` 或缓存 AppImage）；更新 `deferred-work.md` 关闭「可选夜间」为「本阶段选 B」；勾选 NFR14 Epic 37 关闭条件；ATDD 锁住选型与诚实句。树内 HLS 调度仍非目标（AD-25 / FR86）。**Path A** 仅在已半成品且易落地时可选——当前 `ci.yml` 仅有 stub `hls-smoke`，**无**夜间真机 job → **选 B**。

## Boundaries & Constraints

**Always:** Gate 37.1+37.2 done；选 A 或 B 之一并写明；若 B：stub≠质量 + 真机显式入口 + deferred 收口「本阶段选 B」；若 A：夜间/optional job **失败不得 ignore / continue-on-error**；勾选 NFR14 Epic 37 关闭条件；树内 HLS 调度仍非目标；品牌 Bitloom；NFR39；one-story-one-commit；全部 37 故事 done 后 `epic-37: done`。

**Ask First:** 无（本管道默认 Path B；若改选 A 须先确认 CI 配额与 AppImage 缓存策略）。

**Never:** 开工 Epic 38；用 `continue-on-error` 掩盖真机失败却勾选 FR88；把 stub CI 写成「HLS 质量已验」；引入树内 HLS 调度；私自升 Bambu 钉死版交差；宣称 idiomatic Chisel / FR93 已交付。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Path B 选型 | `docs/fr35-hls.md` + deferred-work | 明确「Epic 37 / 本阶段选 B」；stub 默认；真机 = `BITLOOM_HLS_USE_REAL=1` | ATDD 红直至落盘 |
| deferred 收口 | epic-24-retro-item-54 条 | status → closed / 本阶段选 B（非「仍 deferred 可选夜间」） | ATDD 锁关键词 |
| CI 现状 | `.github/workflows/ci.yml` | 保持 stub `hls-smoke`；**无**假绿夜间 job；无 `continue-on-error` 在 hls | 审查拒收假绿 |
| NFR14 关闭 | `nfr14-risk-epic37-interop-hls.md` | Epic 37 关闭条件全 `[x]`；注明 Path B | ATDD 可选 |
| 范围越界 | 树内 scheduler / Epic 38 | 拒收 | Never |

</frozen-after-approval>

## Code Map

- `docs/fr35-hls.md` — **UPDATE** 钉死 Path B：默认 stub；真机显式入口；Epic 37 FR88 HLS 诚实条选 B；交叉链 deferred 收口
- `README.md` — **UPDATE** HLS 节一句交叉：本阶段 stub 默认 / FR88 Path B（链 fr35）
- `_agile-output/implementation-artifacts/deferred-work.md` — **UPDATE** 关闭「可选夜间」为「本阶段选 B」；stub≠质量条可同步注明 Epic 37 收口
- `_agile-output/implementation-artifacts/nfr14-risk-epic37-interop-hls.md` — **UPDATE** 勾选 Epic 37 关闭条件；状态 → closed；注明 Path B
- `.github/workflows/ci.yml` — **不改**（Path B：不新增夜间 job）；确认无 continue-on-error
- `crates/bitloom/tests/fr88_hls_stub_path_b_honesty.rs` — **NEW** docs/deferred ATDD
- `_agile-output/implementation-artifacts/sprint-status.yaml` — `37-3` → done；`epic-37` → done

## Story

As a 维护者,
I want HLS 真机路径要么可验、要么诚实保持 stub 默认,
So that 外挂 HLS 合同不被 stub CI 冒充。

## Acceptance Criteria

1. Given Story 37.2；`docs/fr35-hls.md`（或等价）；AD-25，when 二选一落地：（A）增加 CI optional/夜间 job 跑真实 Bambu（缓存 AppImage 或文档钉死入口），**失败不得 ignore**；或（B）文档确认默认路径保持 stub，真机仍为显式环境变量入口，并更新 deferred-work 关闭「可选夜间」为「本阶段选 B」，then FR88 的 HLS 诚实条可勾选（A 或 B）
2. And 树内 HLS 调度仍为非目标（继承 FR86 / AD-25）
3. And NFR14 记录勾选 Epic 37 关闭条件

## Tasks / Subtasks

- [x] T1: 确认选型 **Path B**（无半成品夜间 job）；更新 `docs/fr35-hls.md` + README 交叉（AC: 1–2）
- [x] T2: `deferred-work.md` 关闭「可选夜间」为「本阶段选 B」；stub≠质量叙事一致（AC: 1）
- [x] T3: 勾选 `nfr14-risk-epic37-interop-hls.md` Epic 37 关闭条件；状态 closed（AC: 3）
- [x] T4: ATDD `fr88_hls_stub_path_b_honesty.rs`（AC: 1–3）
- [x] T5: sprint `37-3` + `epic-37` → done；**不**开工 epic-38

## Dev Notes

### 选型裁决

| 路径 | 现状 | 裁决 |
|------|------|------|
| A 夜间真机 | `ci.yml` **无**夜间/optional 真机 Bambu job；仅 stub `hls-smoke` | **不选** |
| B 显式 stub | stub 默认 + 真机 env 已存在；deferred item-54 已收口 | **选 B** |

### Previous story intelligence（37.2）

- firtool/Chisel 运维 + 可编译≠idiomatic 已 done；HLS 条本故事关闭。
- ATDD 体例：读文件 assert 关键词。

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 37 / Story 37.3 / FR88]
- [Source: ARCHITECTURE-SPINE AD-25]
- [Source: `_agile-output/implementation-artifacts/process-one-story-one-commit.md`]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

- ATDD red→green: `cargo test -p bitloom --test fr88_hls_stub_path_b_honesty` — 6 passed（红相 3 失败：缺 Path B / deferred open / NFR14 未勾选）

### Completion Notes List

- Path B：fr35 + README 钉死 stub 默认 / 无夜间真机 / `BITLOOM_HLS_USE_REAL=1`
- deferred item-53/54 → closed「本阶段选 B」
- NFR14 Epic 37 关闭条件全勾；status closed
- ATDD `fr88_hls_stub_path_b_honesty`；未改 CI；未开工 epic-38；`epic-37: done`

### File List

- `docs/fr35-hls.md`
- `README.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic37-interop-hls.md`
- `crates/bitloom/tests/fr88_hls_stub_path_b_honesty.rs`
- `_agile-output/implementation-artifacts/37-3-可选夜间真机-bambu-或显式保持-stub-fr88.md`
- `_agile-output/implementation-artifacts/atdd-checklist-37-3-可选夜间真机-bambu-或显式保持-stub-fr88.md`
- `_agile-output/implementation-artifacts/37-3-code-review.md`
- `_agile-output/implementation-artifacts/37-3-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: FR88 HLS Path B（显式 stub 默认）+ deferred 收口 + NFR14 Epic 37 关闭（Story 37.3）

## Suggested Review Order

**Path B 文档句（fr35）** → **deferred 收口** → **NFR14 关闭勾选** → **ATDD** → **sprint epic-37 done（未开工 38）**
