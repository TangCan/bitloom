# Sprint Change Proposal — 2026-09-14 — FR189 / Epic 122 upstream block（Correct Course）

**Status:** **approved** — product **yes** = 方案 **A（Park Epic 122）** · 2026-09-14  
**Mode:** Batch  
**Product:** Bitloom  
**Authoring:** Correct Course (`bmad-correct-course`) · 2026-09-14  
**Scope:** Minor — Developer agent 直接落地  

---

## Section 1: Issue Summary

### Triggering story

- **Story 122.2** —「继续 firtool 产品钉升钉实现与验收（FR189）」  
- **前置：** Story **122.1** NFR14 已完成；钉死验收 = AD-9 **默认产品钉** 升至已发布 `firtool-*` **严格大于 1.159.0**。

### Core problem

| 类型 | 判定 |
|------|------|
| Issue type | **Technical limitation discovered during implementation**（上游发布面阻塞） |
| Problem | FR189 / Epic 122 要求诚实产品钉 **>1.159.0**；2026-09-14 上游无此 tag。按原 AC 开工只能：假造版本、silent-Ok 停留在 FR182 **1.159.0**，或把 FR186 live tip 冒充产品钉 — **三者均禁止**。 |

### Evidence（2026-09-14）

1. NFR14：`nfr14-risk-epic122-further-firtool-product-pin-fr189.md` — 「无新版本时不得 silent-Ok；须非零失败或 Correct Course」。  
2. `just circt-live-tip-check` → resolved tip identity **`firtool-1.159.0`**（与 FR182 产品钉重合；live tip ≠ 产品钉）。  
3. Phase 23 其余 deepen 已关（FR186–188 / FR190）；宣称门 **FR191 / Epic 124 已关闭**，诚实面已列出 **FR189 blocked-upstream**。  
4. Sprint：`122-2` = `ready-for-dev` + YAML 注释 `blocked-upstream`；`122-3` = `backlog`。

---

## Section 2: Impact Analysis

（清单 §1–§5 已完成于草案；§6.3–6.5 随本批准落地。）

### Epic Impact

| Epic | Impact |
|------|--------|
| **122 / FR189** | **Park**：122.2 保持 blocked；**不得**勾选关闭；上游发布后再实现 |
| **118–121, 123–124** | **无回滚**；关闭面仍有效（NFR88） |
| **Future / NFR91** | 更高升钉 / 恢复仅配对纪律仍须新合同 |

### Technical Impact

- **不**改默认 firtool 产品钉（仍 **1.159.0** / FR182）。  
- **不**把 live tip 写入 AD-9 默认钉。  
- CI / `firtool ensure` 路径无强制变更。

---

## Section 3: Approved Approach — **A Park Epic 122**

1. Epic 122 / FR189 = **blocked-upstream / parked**（本提案）。  
2. FR189 验收谓词 **不变**（仍须已发布 firtool **>1.159.0** + AD-9 修订）。  
3. 公开宣称面：FR189 **未交付**；不得「NFR86 账本已空」。  
4. 上游出现 `firtool-*` >1.159.0 后：直接开 Story 122.2（NFR90 先改 AD-9）。  
5. **不**关闭 Epic 122；**不**回滚 123–124。

---

## Section 4: Approved edits（落地清单）

见同日实现提交 / 工作区改动：

- `sprint-status.yaml` — park 戳  
- PRD `addendum.md` — FR189 park 短注  
- `epics.md` — `phase23Epic122Status: blocked-upstream` + note  
- Story 122.2 AC — 无钉阻塞分支  
- README / deferred / FR191 / AGENTS — Correct Course A approved  

---

## Section 5: Implementation Handoff

| 项 | 内容 |
|----|------|
| **Scope** | **Minor** |
| **Route** | Developer agent（本会话落地） |
| **Success criteria** | 提案 approved；park 戳齐全；FR189 未关；不得 1.159.0 冒充 |
| **Resume trigger** | 已发布 `firtool-*` **>1.159.0** → 开 122.2 |

---

## Approval record

- **2026-09-14：** Richard — **yes** = 方案 **A**  
- Checklist §6.3–6.5：批准后落地；sprint 已 stamp park  
