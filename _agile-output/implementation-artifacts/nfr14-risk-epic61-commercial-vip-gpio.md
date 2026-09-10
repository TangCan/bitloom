# NFR14 风险记录 — Epic 61 商业 VIP GPIO 全家桶（FR120）

> **权威：** PRD NFR14；AD-28；Phase 14 **NFR48 / NFR49 / NFR51**；交付 **FR120**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic50-gpio-near-vip.md`；`nfr14-risk-epic57-phase14-nfr47-deferred-deepen.md`。  
> **前置：** Epic 57 **closed**（FR116）；Epic 43 **closed**（FR98 UART/SPI/I2C/AXI 近 VIP）；Epic 50 **closed**（FR108 GPIO 近 VIP P1–P4）。  
> **门禁：** 无本有效记录（或缺字段 a–d / 下方协议清单）⇒ **不得**将 **61.2–61.3** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR120 / Epic 61；NFR14、NFR48、NFR49、NFR51；对照 FR98 / FR108 / NFR47→Phase 14 升格 |
| 记录日期 | 2026-09-10 |
| 状态 | **open** — Story 61.1；关闭勾选 → Story 61.3 |

### (a) 上游约束

- **Epic 57 / FR116 已关闭：** Phase 14 加深合同已开闸；本 epic 将 NFR47「商业 VIP GPIO」升格为 **FR120** 必选。
- **FR108 / Epic 50（已关闭 · 隔离）：** `Gpio` 近 VIP **P1–P4**（方向 / 读写 / 掩码 / ATDD）**仍有效**（NFR48）；**不得**改写为失败，亦不得以 P1–P4 alone 冒充本 FR。
- **FR98 / Epic 43（已关闭 · 隔离）：** UART/SPI/I2C/AXI4-Lite 近 VIP **仍有效**；**不得**以四类近 VIP alone 冒充 GPIO 商业 VIP。
- **设计依赖：** 设计 crate 只依赖 `bitloom-prelude`；实现预计落在 `bitloom-prelude` IP 面（`ip.rs` 体积风险见下）。
- **品牌：** Bitloom / `bitloom-*`。

### (b) 粗工期带

- **预计：** Epic 61 整体约 **1–4 人周**（61.1 ≤0.25；61.2 商业 VIP GPIO 实现 0.75–3；61.3 收口 0.25–0.75）。置信度：**中**。
- **假设：** 不交付全 SoC pad 环 / 商业对拍记分板 / debounce·驱动强度（NFR51）；既有 FR98 四类与 FR108 `Gpio` 回归不破。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **61.2–61.3** 标 `ready`。
- **不得仅以 FR108 P1–P4 / `Gpio` 近 VIP alone 关闭 FR120。**
- **不得仅以 FR98 四类近 VIP alone 关闭 FR120。**
- **不得口头宣称「商业 VIP」而无 elaborate/emit/tick + ATDD 夹具。**
- **不得 docs-only / 仅改文档关闭。**
- 不得改写 FR98 / FR108「已关闭」为失败（NFR48）。
- 不得静默扩大超出下方商业 VIP 必选清单（NFR51）。
- 不得冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR51** 共同责任人；NFR48/NFR49 共同注意人。
- 备份 / 升级路径：缩回「仅近 VIP」口径或扩大至全 SoC pad / 商业对拍须升级至产品 / Correct Course 批准人。

---

### 相对 FR108 P1–P4 的商业 VIP 协议/模式清单（供 61.2 / FR120）

基线 **P1–P4**（FR108）仍须可回归；本 FR **增量**如下：

| # | 协议/模式 | 断言/计时深度 | elaborate/emit/tick + ATDD 义务 |
| --- | --- | --- | --- |
| **C1** | **上升沿 IRQ** | 每脚：`pad_in` 相对上一拍上升沿置位 pending；`irq_en` 门控；`irq_clear` 清除；`irq_status`=pending；**`irq_out`（1-bit）** = `(irq_status & irq_en) != 0`（bank OR）；至少 1 拍采样延迟可文档化 | 正向：边沿→pending→irq_out；负向：`irq_en=0` 抑制 `irq_out`；clear 无新边沿后 pending 清零 |
| **C2** | **开漏 + OE** | 每脚 `od`：开漏时仅在输出 0 时断言驱动；`pad_oe` 可读；push-pull（`od=0`）保持 FR108 `pad_out=out&dir` | 正向：`od=1,out=0` → OE 有效；`od=1,out=1` → OE 无效（Hi-Z 语义）；负向：`od=0` 不改变近 VIP pad_out 语义 |
| **C3** | **原子 set/clear** | 超出 `wr_mask`：`set_en`/`set_data` 置位；`clr_en`/`clr_data` 清位；与掩码写可共存（文档优先级：同拍 set/clr 相对 wr 的合并序） | 正向：set/clear 改变 `out`；负向：en=0 不改；与 FR108 掩码写回归夹具并存 |
| **C4** | **ATDD 深度** | 上表 C1–C3 各至少一类正向 + 一类负向/边界；公开品牌 Bitloom | `cargo test -p bitloom --test fr120_gpio_commercial_vip`（或等价）纳入 `just test` |

**回归边界（相对 UART/SPI/I2C/AXI 与 FR108）：**

- **FR108 `Gpio`：** 既有 P1–P4 elaborate/emit/tick / ATDD **不得回归失败**（可保留独立类型，或保证兼容端口子集）。
- **FR98：** `UartTx`/`UartRx`/`SpiMaster`/`I2cMaster`/`Axi4LiteSlave` smoke elaborate **不得回归失败**。
- 商业 VIP 面 **≠** 改写四类近 VIP 关闭证据。

### 明确非目标 / deferred（NFR51 · 诚实披露）

下列 **未**列入本 FR 关闭清单；**不得 silent 宣称已交付：**

- 全 SoC pad 环 / 多 bank 互联 / I/O 标准库全家桶
- 商业 VIP 对拍记分板（对照第三方 GPIO VIP）
- Debounce、驱动强度 / slew、模拟 / 差分配对
- 电平敏感 / 双沿 / 降沿 IRQ 模式全家桶（本记录仅钉 **上升沿**）
- GPIO↔AXI 寄存器窗一等互联夹具

### `ip.rs` 体积风险（评估 · 非关闭条件）

- 现状：`bitloom-prelude` `ip.rs` ~2975 LOC（Epic 50 后）。
- **评估（61.1）：** 61.2 预计继续同文件加深（与 UART/SPI/I2C/AXI/`Gpio` 一致）；**拆分独立模块文件为工程卫生可选项，不是 FR120 关闭条件。**
- 若 61.2 后体积显著膨胀，可另开卫生故事拆分；**不以拆分未完成阻塞 Epic 61 关闭。**

### Epic 61 关闭条件（Story 61.3 勾选）

- [ ] **61.2 / FR120：** C1–C4 + ATDD；FR98/FR108 回归不破
- [ ] **文档 / deferred / IP README / 未覆盖协议诚实披露**
- [ ] **禁止事项未触发**
- [ ] **品牌 / 依赖：** Bitloom；prelude 边界
- [ ] **FR98 / FR108 关闭仍有效**（NFR48）

---

## 门禁一句话

**缺 NFR14（或缺 a–d / C1–C4 清单）⇒ 不得将 61.2–61.3 标 `ready`。**  
**不得以 FR108 alone / FR98 alone / 口头商业 VIP 无夹具 / docs-only 关闭 FR120。**  
**未列入协议保持 deferred（NFR51）。**
