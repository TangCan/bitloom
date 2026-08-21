# Code Review: Story 19.5 ClockDomain 产品叙事与夹具（FR52）

**Reviewer:** adversarial pass（blind / edge / verification-gap / intent-alignment）  
**Date:** 2026-08-21  
**Verdict:** accept

## Findings

### Medium — fixed during review

1. **产品夹具合法 CDC 未被测试钉死：** 去掉 `bind_domain`/`mark_cdc_bridge` 后 `legal_cdc_elaborate_emit_tick` 仍可绿。  
   **Fix:** `elaborate_cdc(with_bridge)` + `fixture_without_bridge_fails_with_e0220`；合法路径改为直接 `assign_net(y,a)` + bridge。
2. **仅展示 async，未并排 sync：** AC「同步·异步」只测了 `async_reset: true`。  
   **Fix:** `q_sync` / `q_async` 并排 + `sync_and_async_reset_shown_in_fixture_hir`。

### Low — fixed during review

3. **E0220 只断言 code、不断言文案点名 DoubleFlop/SyncFIFO。**  
   **Fix:** `assert_e0220` 检查 EN/ZH。
4. **emit/modules 空向量可能 panic。**  
   **Fix:** 非空守卫。
5. **language-surface 未披露 `assign_net`-only 域门禁与极性=AD-15。**  
   **Fix:** CAP-11 补 MVP 范围与极性说明。

### Deferred

- `assign_reg_d_*` 无跨域检查（预存；已写入 language-surface）
- ARCHITECTURE-SPINE AD-22 仍写 Clash `Signal<D,T>`（可选对齐，本故事非必须）
- 诊断码命名空间仍为 `rhdl::E0220`（Bitloom 品牌下预存）

### Rejected

- 真实 DoubleFlop/SyncFIFO RTL / Clash `Signal<D,T>` / per-domain tick 引擎 — Block If / Never
- ActiveLow 极性 API — 合同钉 AD-15 高有效文档化
- `cargo bitloom` 夹具 smoke — 验收面为 prelude 夹具 + docs，非 CLI

## AC checklist

| AC | Status |
| --- | --- |
| 用户可见文档 + ClockDomain 绑定 / sync·async（极性 AD-15） | pass |
| 非法跨域 freeze 失败（非仅文档） | pass（含夹具同形负向） |
| 合法路径经 DoubleFlop/SyncFIFO 文档等价 | pass（`mark_cdc_bridge`） |
| elaborate/emit/tick（或全局 tick 等价） | pass |
| Bitloom / `cargo bitloom` 品牌 | pass |

## Automate

- 加强：fixture twin 无 bridge → E0220；sync/async 并排；诊断文案；直接跨域 assign。

## Disposition

产品叙事 + 夹具 + review 补丁 → mark story done；epic-19 全部故事完成 → `epic-19: done`。
