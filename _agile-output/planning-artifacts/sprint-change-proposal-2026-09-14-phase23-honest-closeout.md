---
title: Sprint Change Proposal — Phase 23 诚实结项收口（FR189 仍未交付；保留 NFR91）
date: 2026-09-14
status: approved
approved: 2026-09-14
step5_user: yes（用户确认「当前 firtool 1.159.0 已经是最新版本了。所以请结项。」）
trigger: Phase 23 规划/实现诚实面齐（retro + action items done）；live tip = firtool-1.159.0 = FR182 产品钉；用户要求二次结项
mode: Batch
change_scope: Moderate
related_prior: |
  sprint-change-proposal-2026-09-14-engineering-closeout.md (Phase 12–22 结项 · complete);
  sprint-change-proposal-2026-09-14-phase23-nfr86-leftovers.md (Phase 23 · approved);
  sprint-change-proposal-2026-09-14-fr189-defer-close.md (FR189 deferred · approved)
epics_ref: 不新增 deepen epic；不把 epic-122/FR189 标为已交付；仅 stamp Phase 23 诚实结项 + NFR91
approval_defaults: |
  Q1 本批 = Phase 23 诚实结项收口；承认 tip==1.159.0 时 FR189（须严格 >1.159.0）上游不可交付；
  Q2 不得 epic-122: done / FR189 已交付；deferred 关账仍有效 → NFR91；
  Q3 Phase 12–22 结项 + Phase 23 已关 FR（185–188/190–191）仍有效；
  Q4 宣称：「Phase 23 诚实结项 / sprint 可停」≠「NFR86/NFR91 账本已空」≠「FR189 已升钉」；
  Q5 git push / 远程同步不在本合同内；
  Q6 未来 firtool 产品钉超 1.159.0 → 另开 Correct Course（NFR91）；不得从本结项 alone 冒充已授权。
---

# Sprint Change Proposal — Phase 23 诚实结项收口

## 1. Issue Summary

**触发：** 用户 2026-09-14：「当前 firtool 1.159.0 已经是最新版本了。所以请结项。」

**事实：**

1. `just circt-live-tip-check` → **resolved tip identity firtool-1.159.0**（与 AD-9 / FR182 产品钉重合）。
2. **FR189** 合同要求产品钉 **严格 > firtool-1.159.0**。tip 已是最新 ⇒ **当前无法诚实交付 FR189**（不是「已升钉」）。
3. Sprint：除 Epic 122 `deferred` 外，故事/回顾/action items **全 done**；Phase 23 规划齐；FR191 诚实门已关。
4. 既有 **工程/合同结项**（Phase 12–22）与 **FR189 延期关账** 仍有效。

**本提案批准：** 做 **Phase 23 诚实结项收口**——可停 sprint / 宣称「当前合同面诚实关账」；**必须**同时写明 FR189 **未交付**、**NFR91** standing。

**本提案不宣称：** FR189 已交付；产品钉已升超 1.159.0；NFR86/NFR91 账本已空；Phase 24 已开。

## 2. Impact Analysis

- Epic 1–121 / 123–124：**保持已关闭**。
- Epic 122 / FR189：**保持 `deferred` / 未交付**（不改 `done`）。
- 无新 deepen epic；无 AD 实质修订（AD-9 仍钉 1.159.0）。
- 文档：PRD addendum、epics stamp、README、deferred、AGENTS、SPINE 指针、fr191 结项一句。

## 3. Recommended Approach

1. Stamp `phase23CloseoutApproved` / `phase23CloseoutStatus: complete`（或等价键）。
2. 公开诚实面：tip==1.159.0 ⇒ FR189 上游不可交付；结项 ≠ 升钉。
3. 加深 leftover 仍 **NFR91**（含未来超 1.159.0 产品钉）。
4. `git push` 不是 FR。

## 4. Acceptance

- [x] 用户确认结项触发（tip 最新）  
- [x] 提案 `status: approved` 落盘  
- [x] 诚实面戳齐；**不得**把 FR189 写成已交付  
- [x] sprint 仍保留 `epic-122: deferred`

## 5. Brand

Public **Bitloom**；设计 crate 只依赖 **`bitloom-prelude`**。
