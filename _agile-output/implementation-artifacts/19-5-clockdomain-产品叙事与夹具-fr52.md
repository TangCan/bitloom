---
title: '19.5 ClockDomain 产品叙事与夹具（FR52）'
type: 'feature'
created: '2026-08-21'
status: 'in-review'
baseline_revision: '0e20fd05983756ba1f571a9d79c3bd3ad59c253d'
baseline_commit: '0e20fd05983756ba1f571a9d79c3bd3ad59c253d'
review_loop_iteration: 0
followup_review_recommended: false
context:
  - '{project-root}/_agile-output/implementation-artifacts/epic-19-context.md'
  - '{project-root}/_agile-output/implementation-artifacts/19-4-实现-bundle-vec-可综合路径-fr51.md'
warnings: []
deferred: []
---

<intent-contract>

## Intent

**Problem:** FR23/AD-22 的 phantom 域与 `rhdl::E0220` 已存在，但用户可见文档与演示夹具未收口；概述 ClockDomain 叙事无法验收。

**Approach:** 产品化文档 + 至少一夹具：绑定域/同步·异步复位；非法跨域 freeze 失败；合法路径经 `mark_cdc_bridge`（文档等价 DoubleFlop/SyncFIFO）；elaborate→emit→tick（或文档化全局 tick 等价）。

## Boundaries & Constraints

**Always:** 非法跨域须有 freeze 失败路径（不得仅文档纪律）；品牌 Bitloom / `cargo bitloom`；设计 crate 仅依赖 `bitloom-prelude`；复用既有 `bind_domain` / `mark_cdc_bridge` / `declare_reg_ex` / E0220。

**Block If:** 必须新建 Clash 式 `Signal<D,T>` 或独立 per-domain tick 引擎才能验收；或必须实现真实 DoubleFlop/SyncFIFO RTL 单元（非 marker + bridge）才能满足 AC。

**Never:** 静默允许无 bridge 的跨域；把 RHDL 愿景文档（`docs/requirements/11.*`）当唯一验收面；扩展 FR48 一级 SyncFIFO IP；重做 FR51 Bundle。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Illegal CDC | 两域 `assign_net` 无 bridge | `finish`/`freeze` → `rhdl::E0220` | 不得 emit |
| Legal CDC | `mark_cdc_bridge` + 跨域 assign | elaborate → emit `.v` → tick | N/A |
| Async reset | `declare_reg_ex(..., async_reset: true, ...)` | HIR/夹具可展示 sync·async 绑定 | N/A |
| Domain bind | `bind_domain` + `ClockDomain<ID>` 叙事 | 文档/夹具可发现绑定 | N/A |

</intent-contract>

## Code Map

- `crates/bitloom-prelude/src/lib.rs:100-109` — `ClockDomain` / `DoubleFlop` / `SyncFIFO` ZST；可加文档/薄辅助，不改 HIR
- `crates/bitloom-builder/src/lib.rs:85-93` — `bind_domain` / `mark_cdc_bridge`
- `crates/bitloom-builder/src/lib.rs:824-845` — 跨域门禁 `rhdl::E0220`；复用勿重写核心
- `crates/bitloom-builder/src/lib.rs:187-218` — `declare_reg_ex` async_reset / enable
- `crates/bitloom-builder/src/lib.rs:1584-1617` — 既有单测样板（夹具应产品化同等路径）
- `crates/bitloom-sim` / `bitloom-vlog` — 全局 `tick` + async_reset 注释形 emit（per-domain tick 文档等价）
- `examples/bundle_vec_skel/` — 夹具布局样板（prelude-only + dev hir/sim/vlog）
- `_agile-output/specs/spec-rhdl/language-surface.md:21-25` — CAP-11 收口钉 API 映射
- `README.md` — 点出 ClockDomain/CDC 产品入口（Bitloom）
- `ARCHITECTURE-SPINE.md` AD-22 — 可选对齐「session 域标签 + E0220」现状（勿另立 HIR）

## Tasks & Acceptance

**Execution:**
- `examples/clockdomain_skel/` -- 新夹具：多域绑定 + 非法 E0220 + 合法 mark_cdc_bridge（DoubleFlop/SyncFIFO 叙事）+ async_reset 展示；elaborate→emit→tick -- FR52 演示
- `Cargo.toml` -- 注册 example member -- 工作区可测
- `crates/bitloom-prelude/src/lib.rs` -- 文档化 ClockDomain/CDC 表面与 bridge 等价；极性=默认高有效 Reset（AD-15）-- 用户可见 API 叙事
- `_agile-output/specs/spec-rhdl/language-surface.md` -- 钉死 bind_domain / mark_cdc_bridge / E0220 / declare_reg_ex 与全局 tick 等价 -- 合同
- `README.md` -- Bitloom 入口提及 ClockDomain/CDC 夹具路径 -- 产品发现
- `_agile-output/implementation-artifacts/sprint-status.yaml` -- 19-5 追踪；epic-19 全完成后标 done -- 冲刺

**Acceptance Criteria:**
- Given FR23/AD-22 已存在，when 更新用户可见文档 + 至少一演示夹具，then 展示 ClockDomain（或等价）绑定域与同步·异步复位（极性按 AD-15 高有效文档化）
- Given 非法跨域无 bridge，when elaborate/finish，then freeze 失败（`rhdl::E0220`），不得仅靠文档
- Given 合法路径，when 经 DoubleFlop/SyncFIFO 文档等价（`mark_cdc_bridge`），then elaborate→emit→tick（或文档化全局 tick 等价）通过
- Given 相关路径，when `cargo test` / 夹具测试，then 通过；品牌为 Bitloom / `cargo bitloom`

## Spec Change Log

- 2026-08-21: FR52 产品叙事 + 夹具 `examples/clockdomain_skel`；prelude / language-surface / README 钉死 bind_domain、mark_cdc_bridge、E0220、declare_reg_ex、全局 tick 等价。未改 intent-contract；未重做 CDC 核心。

## Review Triage Log

## Design Notes

合法跨域今日为 session 级 `mark_cdc_bridge`（诊断文案指向 DoubleFlop/SyncFIFO）；prelude ZST 为叙事锚点，不要求真实同步器实例。极性：默认同步高有效 `Reset`（AD-15）；异步经 `declare_reg_ex`。`Sim::tick` 为单全局时钟步进——夹具文档声明为「按域 tick」MVP 等价。

```rust
s.bind_domain("a", 0); // ClockDomain::<0>
s.bind_domain("y", 1); // ClockDomain::<1>
s.mark_cdc_bridge("y"); // DoubleFlop / SyncFIFO 合法路径
s.assign_net("y", "a", span); // OK；无 bridge → E0220
```

## Verification

**Commands:**
- `cargo test -p clockdomain_skel` -- expected: PASS（非法 E0220 + 合法 emit/tick + async 展示）
- `cargo fmt --all && just test` -- expected: 全绿
