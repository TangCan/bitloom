# NFR14 风险记录 — Epic 33 Chisel Mem / FR28 深度（FR81 / AD-27）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；**AD-27**（Bitloom↔Chisel 可编译 Scala 产品路径）；**AD-9** / Stack（Chisel↔firtool 钉死对）；Phase 10 **NFR37**（规划 done ≠ 深度 done）。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **历史合同：** Epic 20 / **FR28** + **FR46** — FrozenHir → 可编译 Chisel Scala（机械风格可接受）；Mem 不在 FR28 最小完成面内时可结构化失败。Epic 25 / **FR71** — 默认 CI required JVM 真编译门禁（`fr28-chisel-jvm` / `just chisel-fr28-jvm`）。  
> **体例对照：** `nfr14-risk-chisel-bidirectional.md`；`nfr14-risk-epic32-nested-bundle.md`；`nfr14-risk-epic31-cdc-true-rtl.md`。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 33 后续故事 **33.2–33.4** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR81（加深 FR28）/ Epic 33；NFR12、NFR14、NFR37；衔接 FR71 / AD-27 |
| 记录日期 | 2026-09-09 |
| 状态 | accepted — 门禁有效；Epic 33 关闭条件已由 Story 33.4 勾选 |

### (a) 上游约束

- **NFR12 钉死对（强制继承，本 epic 不得漂移）：** Chisel **7.14.0** ↔ CIRCT firtool **1.155.0**（ARCHITECTURE-SPINE Stack / AD-9）。升钉须等 Chisel 正式配对并更新校验表后再改 AD-9 / Stack；**不得**为「让 Mem 编过」私自升 firtool-1.156.0 或换未配对 Chisel。
- **AD-27 / FR28 产品路径：** FrozenHir/`.fir` → **可编译** Chisel Scala；验收=编译通过 + 端口/层次谓词；机械风格可接受；**不**要求 Scala `Parser.parse`。AD-3 FIRRTL 文本契约仍独立。
- **E0901 现状（诚实基线 → Path A 收敛）：** 历史全量 `MemDecl`→`rhdl::E0901`；Story 33.3 后 **Path A 子集内** `emit_chisel` 降级为可编译机械 Scala，**子集外仍 E0901**（不得批发删除诊断）。FIRRTL / Verilog / tick 路径上的 Mem（AD-21）独立；缺口收口专指 **Chisel Scala 降级腿**。
- **Epic 25 / FR71 JVM 门禁（不得破坏）：** GHA required job `fr28-chisel-jvm` 与 `just chisel-fr28-jvm` 对文档化黄金夹具在钉死 Chisel 下真 `sbt`/`scalac` 编译；缺 JDK/sbt 或编译失败必须红。本 epic 任何 Mem 实现或非目标合同化**不得**使既有无 Mem 黄金夹具变红，亦不得把 FR71 改成 `continue-on-error` / skip-on-missing-Java。
- **FR81 合同目标（Story 33.2 已选定 Path A）：** 支持文档化 Mem 子集使 `emit_chisel` 可编译；子集外保留 E0901。决策页 `fr81-mem-chisel-contract-decision-2026-09-09.md`。正向腿仍服务 FR46 / AD-27。
- **NFR37：** Epic 20 / FR28「done」+ E0901 清单存在 ≠ FR81 深度关闭；不得用「Mem 本来就 E0901」话术永久回避合同收敛。

### (b) 粗工期带

- **预计：** Epic 33 整体约 **1–2.5 人周**（33.1 本风险记录 ≤0.25 人周；33.2 合同决策+文档 0.25–0.5 人周；33.3 实现所选路径 0.5–1.5 人周；33.4 ATDD + FR71 回归 + 文档 0.25–0.5 人周）。
- **置信度 / 假设：** 中；假设选定路径后：
  - **支持子集：** 仅有限 `Mem`/`SyncReadMem` 形态映射到 Chisel `Mem`/`SyncReadMem`（或文档等价），且黄金夹具仍在 NFR12 钉死对下通过 FR71。
  - **永久非目标：** 保留稳定 E0901（或文档化等价诊断）+ 书面替代验收，**不**删除诊断冒充支持。
  - 若要求 CHIRRTL 全表面、双时钟 mem、或绑定 Chisel 内部 API，工期显著上修，须改本记录而非静默扩/缩范围。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **33.2–33.4** 标 `ready` 或开工实现。
- **不得在未做出 FR81 决策（Story 33.2）时删除或绕过 E0901**，使含 Mem 的 HIR「碰巧」emit 出不可编译/未验收 Scala，并宣称已支持 Mem→Chisel。
- **不得破坏 FR71：** 不得削弱 `fr28-chisel-jvm` / `just chisel-fr28-jvm`（`continue-on-error`、缺工具链 skip、换成仅 Rust 谓词冒充 JVM 编译、私自换未钉死 Chisel 版本）。
- 不得私自漂移 **NFR12** 钉死对（Chisel 7.14.0 ↔ firtool 1.155.0）来「修好」Mem。
- 不得把 Epic 20 / FR28「done」+ 现状 E0901 冒充 FR81 深度关闭（**NFR37**）。
- 不得同时声称「支持子集」与「永久非目标」而不选定唯一路径（33.2）。
- 若选非目标：不得省略替代验收（须可测/可文档核验）；若选支持子集：不得省略支持的 Mem 形态与版本约束清单。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR37** 深度诚实度共同责任人
- 备份 / 升级路径：Mem 子集 vs 永久非目标争议升级至 AD-27 / FR81 产品决策；钉死升版跟 **NFR12**；JVM CI 回归红升级至 FR71 / NFR34 维护者。

### 支持子集 vs 永久非目标+替代验收（利弊，供 33.2 决策）

| 路径 | 利 | 弊 | 关闭时须具备 |
| --- | --- | --- | --- |
| **A. 支持文档化 Mem 子集** | 真正加深 FR28/AD-27；含 Mem 设计可走 Chisel 产品腿；E0901 清单可收敛为「子集外仍失败」 | 实现/回归成本；须钉死形态与 NFR12 版本；可能扩 FR71 夹具面 | 支持形态表 + 正例可编译 + 子集外稳定诊断 + FR71 仍绿 |
| **B. 永久非目标 + 替代验收** | 合同诚实、工期可控；保留 E0901（或等价）清晰边界；不扩大 JVM 夹具 | 含 Mem 设计不能经 `emit_chisel`；须明确替代（如 `.fir` mem 保留）避免「未做=永久豁免」话术滥用 | PRD/文档永久非目标声明 + 可测替代验收 + NFR37 引用 + 不得删 E0901 冒充支持 |

**Story 33.2 已选定 Path A** — 决策页：`architecture/architecture-rhdl-2026-08-18/fr81-mem-chisel-contract-decision-2026-09-09.md`（支持文档化 Mem 子集；拒绝 Path B）。本表仍保留利弊对照供审计。

### E0901 / FR71 / NFR12 对照

| 维度 | 今日现状 | Epic 33 / FR81 期望 |
| --- | --- | --- |
| `emit_chisel` + Mem | **Path A 子集可编译**；子集外 **E0901** | 已按 33.2 Path A + 33.3 实现；33.4 夹具/FR71 回归 |
| NFR12 | Chisel 7.14.0 ↔ firtool 1.155.0 | **不变**（不得私自升版交差） |
| FR71 | 无 Mem 黄金夹具 JVM 编译硬门禁 | **保持绿**；Mem 变更不得破坏；可选 Mem 夹具不替换 required 面 |
| 诚实度 | FR28 done 可与历史 E0901 并存 | **不得**用该并存关闭深度（NFR37）；Path A 深度已关闭 |

### 并行 / 维护叠加（Chipyard 式）

- 与 Epic 20/25 产物并行：每次改 `emit_chisel` Mem 路径须跑 FR71 回归；禁止「只修 Rust 单元测试、跳过 JVM job」。
- 与 Epic 31–35 Wave 3 并行：Chisel Mem 是互操作深度，不得与 CDC/Bundle/IP 完成话术混用。
- AD-21（Mem/SyncReadMem 表面 + firrtl.mem）仍约束语言/FIRRTL 腿；本 epic 专责 **Chisel Scala 腿** 合同收敛。

### 引用

- AD-28 — 风险门禁（NFR14）
- AD-27 — Bitloom ↔ Chisel 产品互操作（可编译 Scala）
- AD-9 / Stack — Chisel 7.14.0 ↔ firtool 1.155.0（NFR12）
- AD-21 — Mem/SyncReadMem 表面；interop 仍 firrtl.mem
- PRD NFR14 / FR28 / FR81；NFR12；NFR37；FR71 / NFR34（Epic 25）
- `crates/rhdl-firrtl/src/chisel.rs` — `emit_chisel` → `rhdl::E0901` on `MemDecl`
- 体例：`nfr14-risk-chisel-bidirectional.md`；`nfr14-risk-epic32-nested-bundle.md`
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 33 关闭条件（Story 33.4 勾选）

- [x] **FR81 决策：** 33.2 已选定唯一路径 **Path A（支持文档化 Mem 子集）** — `fr81-mem-chisel-contract-decision-2026-09-09.md`
- [x] **实现：** 33.3 行为满足决策条文；子集外或非目标路径有明确失败/边界（Path A emit + OOS E0901）
- [x] **NFR12：** Chisel/firtool 仍为钉死对（无私自升版交差；7.14.0 ↔ 1.155.0）
- [x] **FR71：** `just chisel-fr28-jvm` / GHA `fr28-chisel-jvm` 合同路径仍绿（counter 黄金未改；脚本 ATDD 回归）
- [x] **NFR37：** 相对 FR28 done + 历史 E0901 已文档化；不得用最小合同冒充深度关闭（Path A 深度 + 子集外 E0901）
- [x] **禁止事项未触发：** 无未决策删 E0901 冒充支持；无破坏 FR71；无静默漂移 NFR12

**证据（33.4 填写）：** Story 33.4 — ATDD `fr81_path_a_mem_chisel_emit` + `fr81_mem_chisel_atdd_fr71`；Mem 夹具 `testdata/fr81_path_a_sync_read_mem.scala` + 可选 `just chisel-fr81-mem-jvm`；FR71 required 仍为 `fr28_golden_counter.scala` / `just chisel-fr28-jvm` / GHA `fr28-chisel-jvm`；文档 `docs/fr28-chisel-compilable.md` Mem↔Chisel 边界；`bash scripts/test-just-chisel-fr28-jvm.sh` PASS。

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 33 故事 33.2–33.4 标 `ready`。**  
**Epic 33 关闭条件（上节）由 Story 33.4 勾选后方可将 epic 标 done。**
