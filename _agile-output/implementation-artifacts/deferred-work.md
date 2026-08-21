# Deferred work

- source_spec: `_agile-output/implementation-artifacts/17-4-五级流水-转发-分支-flush.md`
  summary: LW/SW 在 EpisodeIIPipe 已接线但无 tick 黄金；load_q 时序与 load-use stall 留给 17.5
  evidence: 17.4 AC 明确不要求 load-use；PIPE.md 标明 LW 依赖未定义直至 17.5
  status: closed — 17.5 已交付 load-use ATDD 与 LW/SW 黄金路径

- source_spec: `_agile-output/implementation-artifacts/17-4-五级流水-转发-分支-flush.md`
  summary: SW 到 LED MMIO 时仍可能写入 DMEM[0x100&0xf]（与 Episode I 相同）
  evidence: `assign_mem_write_en` 仅看 `ex_mem_is_sw`，未排除 `is_mmio`；预存于 rv32_core
  status: closed — `dmem_we = is_sw && !is_mmio`（pipe + core）；`tick_sw_mmio_excludes_dmem_bypass_golden`

- source_spec: `_agile-output/implementation-artifacts/17-4-五级流水-转发-分支-flush.md`
  summary: ADDI/BEQ/LW/SW 判定仍为 opcode-only（无 funct3），非子集编码可能被误认
  evidence: 与 `rv32_core` Episode I 译码合同一致；教学子集假定合法编码

- source_spec: `_agile-output/implementation-artifacts/17-5-load-use-停顿-atdd-验证阶梯.md`
  summary: Load-use ATDD 仅覆盖 rs1 消费者（ADDI）；未单独测 ADD/BEQ/SW 的 rs2 依赖路径
  evidence: 评审；硬件已对 use_rs2 门控，但无第二黄金
  status: closed — `tick_load_use_rs2_consumer_atdd_golden`（ADD 作 rs2 消费者）

- source_spec: `_agile-output/implementation-artifacts/17-5-load-use-停顿-atdd-验证阶梯.md`
  summary: ATDD 不断言 stall 周期数 / PC 冻结拍，只断言最终 RF
  evidence: 无停顿时 x4=1 已证明依赖 stall；更强观测可后补

- source_spec: `_agile-output/implementation-artifacts/18-3-可选-zicsr-m-mode-trap-教学最小集.md`
  summary: 可选 Zicsr + M-mode trap RTL（mstatus/mtvec/mepc/mcause/mscratch + mret + CSR 写后 flush）未实现；仅教程 stub
  evidence: Story 18.3 / NFR32 允许延期；优先 stub 而非半成品 CSR；见 `docs/tutorials/rv32-episode-ii/06-csr-m-trap-deferred.md`
  resolved: '2026-08-21'
  resolution: 由 epic-18-retro-item-35 交付 `examples/rv32_priv` + `06-csr-m-trap.md`；教学最小集 tick 黄金绿；不宣称 Privileged 合规

- source_spec: `_agile-output/implementation-artifacts/19-4-实现-bundle-vec-可综合路径-fr51.md`
  summary: Bundle 叶仅 GroundType；不支持嵌套 Bundle 或 HwVec&lt;Bundle,_&gt;
  evidence: 最小展平实现；AsGround 门控 HwVec；评审指出嵌套路径未交付

- source_spec: `_agile-output/implementation-artifacts/19-4-实现-bundle-vec-可综合路径-fr51.md`
  summary: 未提供 #[derive(Bundle)]；仅手写 Bundle::leaves
  evidence: Design Notes 允许手写；夹具已用手写；宏 derive 非 AC

- source_spec: `_agile-output/implementation-artifacts/19-4-实现-bundle-vec-可综合路径-fr51.md`
  summary: 展平叶名可能碰撞（field_a + member b_c vs field_a_b + member c）
  evidence: 命名约定 {field}_{member}/{field}_{i}；无去重门禁；评审指出

- source_spec: `_agile-output/implementation-artifacts/19-4-实现-bundle-vec-可综合路径-fr51.md`
  summary: check_connect 只比位宽、不比 GroundType kind（Bool↔UInt 等同宽仍可能过）
  evidence: 预存宽合同；edge-case hunter；非 FR51 引入

- source_spec: `_agile-output/implementation-artifacts/19-5-clockdomain-产品叙事与夹具-fr52.md`
  summary: assign_reg_d_* 不做跨域检查（仅 assign_net）
  evidence: 预存 builder 行为；language-surface 已披露；夹具合法路径用 assign_net

- source_spec: `_agile-output/implementation-artifacts/19-5-clockdomain-产品叙事与夹具-fr52.md`
  summary: ARCHITECTURE-SPINE AD-22 仍写 Clash Signal&lt;D,T&gt;，与 session 域标签实现不一致
  evidence: 本故事可选对齐未改脊柱；产品面已在 language-surface/prelude 钉死

- source_spec: `_agile-output/implementation-artifacts/19-5-clockdomain-产品叙事与夹具-fr52.md`
  summary: 跨域诊断码命名空间仍为 rhdl::E0220（Bitloom 品牌下预存）
  evidence: builder 既有码；夹具沿用

- source_spec: `_agile-output/implementation-artifacts/20-3-firrtl-frozenhir-可编译-chisel-fr28.md`
  summary: 文档路径仍为 `docs/fr28-chisel-best-effort.md`（内容已改写为可编译合同）
  evidence: 避免断链；README 已改显示名；正式更名可另故事

- source_spec: `_agile-output/implementation-artifacts/20-3-firrtl-frozenhir-可编译-chisel-fr28.md`
  summary: InOut 端口发射为 `Analog(Analog())`；实例 InOut 连接仅注释
  evidence: 预存尽力路径；AD-3 导入子集本就排除 Analog/InOut；FR28 夹具不覆盖

- source_spec: `_agile-output/implementation-artifacts/20-3-firrtl-frozenhir-可编译-chisel-fr28.md`
  summary: 仅精确名 `clk`/`rst` 从 IO 剥离；其他 Clock/Reset 端口名仍进入 Bundle
  evidence: 机械约定；多时钟命名留给后续 epic
