# NFR14 风险记录 — Epic 81 SemVer 1.0 政策与破坏性变更 CI（FR143 / FR144）

> **权威：** PRD NFR14；AD-28；Phase 17 **NFR60 / NFR61 / NFR62**；交付 **FR143 / FR144**。  
> **前置：** Epic 80 **closed**（FR142；`5b0fa25`）；表面清单 `docs/public-api-1-0-surface.md`。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **81.2–81.4** 标 `ready`。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR143 / FR144 / Epic 81；对照 NFR15 / Q5 MSRV |
| 记录日期 | 2026-09-11 |
| 状态 | accepted — Story 81.1 |
| **选定** | 政策 `docs/semver-1-0-policy.md` + `just semver-check` / CI `semver-check`（`cargo-semver-checks`） |

### (a) 上游约束

- Epic 80 / FR142 已关闭；表面 crate：`bitloom` / `bitloom-prelude` / `bitloom-macro` / `bitloom-sim`。
- 政策路径：`docs/semver-1-0-policy.md`；交叉链 `docs/semver-0x-policy.md` 与表面清单。
- CI 工具：**`cargo-semver-checks`**（经 `scripts/semver-check.sh` / `just semver-check`）；缺工具 → **非零**；禁止 `continue-on-error`。
- Q5：MSRV 保持 **1.97.1**；上调另开并写入 1.0 政策。
- NFR15：结项期曾停 0.x；本 FR **授权**对钉死表面升 1.0；关闭 sprint ≠ 自动 major。
- NFR62：政策/CI 须先成文再宣称关闭。

### (b) 粗工期带

- Epic 81：约 **0.5–1.5 人周**。置信度：中（semver-checks 对 crates.io 基线误报风险）。

### (c) 禁止的静默降级清单

- 不得缺本记录将 **81.2–81.4** 标 `ready`。
- 不得用 `continue-on-error` 吞掉 semver CI。
- 不得缺工具时静默成功。
- 不得未成文政策即宣称 FR143 关闭。
- 不得改写 FR94–140 失败（NFR60）。
- 不得冒充 NFR14-crates。

### (d) 负责人

- Richard（Dev）— NFR14 / NFR62。

---

### Epic 81 关闭条件（Story 81.4 勾选）

- [ ] **SemVer 1.0 政策成文** — Story 81.2
- [ ] **semver CI / just 门禁** — Story 81.3
- [ ] **README / deferred 指针** — Story 81.4
- [ ] **禁止事项未触发**
- [ ] **品牌：** Bitloom
