# NFR14 风险记录 — 一级 IP（Epic 22 / FR37 + FR48）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（Phase 7 风险门禁）。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 22 后续故事 **22.2–22.6** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR37 + FR48 / Epic 22 |
| 记录日期 | 2026-08-21 |
| 状态 | accepted |

### (a) 上游约束

- **五类一级 IP（FR48）：** UART、SPI、I2C、FIFO、AXI 均须可官方依赖例化；各至少一 smoke：elaborate → emit → tick（或文档等价）。
- **AXI 范围（PRD Open Q7 已关闭）：** AXI 类达标 = **AXI4-Lite 最小从接口**；非 Full AXI、非完整互联即验收线。
- **FR37 起步条：** 树内至少 **FIFO + UART** + 一黑盒 wrapper；黑盒保持不透明（不内联子 HIR）。
- **设计依赖边界：** 设计 crate 只依赖 `bitloom-prelude`（及文档化的官方 IP 包名）；IP 实现本身亦只经 prelude 语言表面 elaborate。
- **治理偏好（树内 vs 组织发布）：** 本阶段五类以 **树内 / workspace 官方包**（如 `bitloom-ip` 或 `examples/` 黄金夹具）交付，便于 smoke 与 CI；**新鲜 IP 宜 API/测试稳定后再深绑进 `bitloom-prelude` 发布表面**（调研：避免未稳定 IP 永久耦合核心）。组织下 crates.io 发布可作为后续稳定收编路径，不豁免本 epic 树内可演示义务。
- **维护现实：** 全线一级 IP 为多年维护面（addendum）；本 epic 交付最小可综合 stub + 文档限制即可，不得用「同业多树外」永久砍类。

### (b) 粗工期带

- **预计：** Epic 22 整体约 **2–4 人周**（22.1 风险记录 ≤0.25 人周；22.2 FIFO+UART+黑盒 0.5–1 人周；22.3 SPI / 22.4 I2C 各 0.25–0.5 人周；22.5 AXI4-Lite 最小从 0.5–1 人周；22.6 索引与例化文档 0.25–0.5 人周；缓冲含 CI 与对抗审查）。
- **置信度 / 假设：** 中高；假设沿用既有 `ElaborateSession` / `emit` / `Sim::tick`，IP 为最小端口语义 stub。若要求全协议栈或与外部 VIP 对拍，工期显著上修。

### (c) 禁止的静默降级清单

- 不得将五类缩成「**仅 FIFO**」（或仅一类）并宣称 FR48 完成，而不改 PRD。
- 不得把 **AXI** 范围从「AXI4-Lite 最小从」偷换成 Full AXI / 完整互联验收，或反过来用「只有文档无模块」冒充 AXI 类达标。
- 不得删除 / 绕过 **黑盒**路径并宣称 FR37 完成。
- 不得交付无端口语义的空壳（跳过 elaborate / 无 emit / 无 tick）冒充 smoke。
- 不得在无本记录（或缺 a–d）时将 **22.2–22.6** 标 `ready` 或开工实现。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）
- 备份 / 升级路径：AXI 范围或五类砍改须先改本记录与 PRD Open Q7 / FR48；治理深绑 prelude 争议升级至 AD-28 / 产品维护者。

### 树内 vs 官方组织发布（治理）

| 策略 | 说明 | 本记录 |
| --- | --- | --- |
| 树内 / workspace 官方包 | 可演示、进 `just test`；新鲜 stub 默认落点 | **本 epic 默认** |
| 组织 crates.io 发布 | 稳定后再发；避免未稳定 API 深绑 | **后续收编可选** |
| 深绑 `bitloom-prelude` 表面 | 仅 API/测试稳定后 | **禁止过早深绑** |

### 并行 / 维护叠加（Chipyard 式）

- 与 Epic 20/21/23/24 并行时：IP stub + 语言表面（Bundle/Vec）+ 模拟器生成回归面叠加；升钉 / 端口约定变更须同步五类 smoke，避免「只修一类」。
- 禁止把 FR48 失败归因到「FR37 起步未做满」而永久砍 SPI/I2C/AXI。

### 引用

- AD-28 — Phase 7 风险门禁（NFR14）
- PRD NFR14（`prd-rhdl-2026-08-19/prd.md` §6）
- PRD FR37 / FR48；Open Q7 已关闭（AXI4-Lite 最小从）
- Addendum：一级 IP 多年维护面
- 历史别名消歧：**NFR14-crates**（crates.io FCFS）≠ 本门禁 **NFR14**

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 22 故事 22.2–22.6 标 `ready`。**
