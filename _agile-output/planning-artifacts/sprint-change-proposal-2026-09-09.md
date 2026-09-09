---
title: Sprint Change Proposal — Phase 11 合同绿（doc-19 重定义完成标准）
date: 2026-09-09
status: approved
approved: 2026-09-09
trigger: Phase 11 合同绿
mode: Batch
change_scope: Moderate
---

# Sprint Change Proposal — Phase 11 合同绿（doc-19 重定义完成标准）

## 1. Issue Summary

**触发：** Correct Course（2026-09-09）。用户确认：批准 **Phase 11 合同绿** — 用 research  
`technical-doc19-seven-stage-full-green-product-pla-2026-09-09` 的结论，将「产品做完 / `docs/requirements/19` 七阶段全绿」改为**可验收合同绿**（按同业重定义阶段五–七完成标准），并正式纳入 PRD / sprint。

**背景：**

1. Phase 1–10（Epic 1–35）实现与规划面已 `done`；Epic 12 口径下 **0.x 合同结项**已成立。
2. PRD 2026-08-21 **①C** 曾**拒绝**调研「重定义 done」，并把概述愿景升格为硬 FR（FR46–52 等）；其后 Phase 7–10 已按深度合同交付（含 FR79–86）。
3. 用户现要求对 **路线图文档「七阶段全绿」** 与剩余生态缺口采用 **合同绿**（非字面再开多年 VIP/自研 HLS/idiomatic Chisel/自动 TLM≡CA/全 elaborate LSP）。
4. Create Epics（2026-09-09）已起草 **Phase 11 / Epic 36–39 / FR87–FR93 / NFR38–NFR39**。

**问题陈述：** 若不正式修正 PRD，「产品做完 / 七阶段全绿」会继续与 ①C 字面叙事及 `19. 实施路线图.md` 旧甘特冲突。Correct Course **批准窄范围修正**：只改写路线图完成标签与剩余缺口验收，**不回滚**已交付 FR46–86。

**证据：** research.md；PRD ①C；epics Phase 11；原 sprint 止于 Epic 35。

---

## 2. Impact Analysis

### Checklist 记录（Step 2）

#### §1 Trigger & Context
- [x] **1.1** 非单 story；触发 = Phase 11 CE + 合同批准
- [x] **1.2** 类型：战略窄范围 pivot + 新 FR87–93
- [x] **1.3** 证据齐全

#### §2 Epic Impact
- [x] **2.1–2.5** 新增 Phase 11 Epic 36–39；不改 1–35；序 36→(37∥38)→39

#### §3 Artifact Conflicts
- [x] **3.1 PRD：** addendum + prd 指针（已落地）
- [x] **3.2 Architecture：** Deferred 指针（已落地）
- [x] **3.3 UX：** N/A
- [x] **3.4 sprint-status：** Epic 36–39 backlog（已落地）

#### §4 Path Forward
- [x] **4.4 Selected：** Hybrid = Option 1 + 局部 Option 3

---

## 3. Recommended Approach

**已批准路径：** Phase 11 正式 backlog；PRD 合同绿；①C 已交付不回滚；永久非目标 FR93。

---

## 4. Detailed Change Proposals — 落地状态

| ID | 状态 |
|----|------|
| P-P1 addendum Phase 11 节 | **done** |
| P-P2 prd frontmatter / §0 | **done** |
| P-P3 Rejected 旁注 | **done** |
| P-E1 epics 已齐 | **confirmed** |
| P-A1 spine Deferred 指针 | **done** |
| P-S1 sprint-status 36–39 | **done** |
| P-D1 doc-19/README | **Story 36.2–36.3**（实现期） |

---

## 5. Implementation Handoff

**变更范围：Moderate** — 已批准并部分落地合同/跟踪文件。

| 角色 | 下一步 |
|------|--------|
| Developer | Story **36.1** NFR14 → 36.2 → 36.3；再 37∥38 → 39 |
| Build | `bmad-build` 从 36.1 起 |

**成功标准：**

1. ~~addendum 含 2026-09-09 决议~~  
2. ~~sprint-status 含 Epic 36–39~~  
3. Story 36.2 后 doc-19 与 FR87 一致（待实现）  
4. 无回滚 FR46–86  

---

## Checklist §6

- [x] **6.3** 用户 **yes** 批准（2026-09-09）  
- [x] **6.4** sprint-status 已追加 Epic 36–39  
- [x] **6.5** handoff：下一动作为 Build 36.1  
