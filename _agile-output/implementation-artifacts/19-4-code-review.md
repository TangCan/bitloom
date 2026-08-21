# Code Review: Story 19.4 实现 Bundle / Vec 可综合路径（FR51）

**Reviewer:** adversarial pass（blind / edge / verification-gap）  
**Date:** 2026-08-21  
**Verdict:** accept

## Findings

### Medium — fixed during review

1. **Reg.d 宽门禁无测：** `assign_reg_d_expr` 已接 E0131，但负向测试只打 `assign_net`；删 Reg.d 门仍能绿。  
   **Fix:** `mismatched_assign_reg_d_width_rejected` + 夹具 `width_mismatch_on_reg_d_fails_before_emit`。

### Low — fixed during review

2. **crate 文档 HTML 实体：** `Vec&lt;T,N&gt;` 会弄脏 rustdoc。  
   **Fix:** 改为 `` `Vec<T,N>` ``。
3. **HwVec N=0** 静默零端口。  
   **Fix:** `const { assert!(N > 0) }`。
4. **MVP 边界未写进 language-surface：** 嵌套 / derive 缺失。  
   **Fix:** 补 N>0、ground-only、无 derive 边界。
5. **宏路径仅查端口名：** 未 assert emit。  
   **Fix:** `module_macro_registers_leaf_ports` 增加 emit 断言与 `out_stream_valid`。

### Deferred

- Bundle 叶仅 GroundType；无嵌套 Bundle / `HwVec<Bundle,_>`（已入 deferred-work）
- 无 `#[derive(Bundle)]`（已入 deferred-work）
- 展平叶名碰撞无门禁（已入 deferred-work）
- `check_connect` 只比位宽、不比 GroundType kind（Bool↔UInt 等同宽仍可能过）— 预存宽合同，非本故事引入

### Rejected

- 全路径（assign_add/mem）统一 E0131、Analog 叶校验、堆 Vec trybuild、脊柱再钉 HwVec 名、示例 README、prelude 单测重复 — 超出 FR51 最小夹具验收或属预存行为。

## AC checklist

| AC | Status |
| --- | --- |
| 文档化 Bundle / HwVec（Vec 等价）prelude/builder/emit | pass |
| 夹具 elaborate → emit `.v` → tick | pass |
| 位宽/方向 emit 前失败 | pass（含 Reg.d） |
| 设计 crate 仅 bitloom-prelude | pass |
| 相关 cargo test 通过 | pass |

## Automate

- 加强：Reg.d E0131 单测 + 夹具；宏路径 emit；`out_stream_valid`；HwVec N>0；language-surface MVP 边界。

## Disposition

展平路径 + 门禁 + 夹具 + review 补丁 → mark story done。
