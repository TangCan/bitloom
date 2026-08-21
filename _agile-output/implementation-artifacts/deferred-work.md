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
  status: closed — OUT OF SCOPE + trybuild `nested_hwvec_bundle`（epic-19-retro-item-38）
  resolved: '2026-08-21'
  resolution: 文档锁定嵌套为 OUT OF SCOPE；负向编译测失败清晰

- source_spec: `_agile-output/implementation-artifacts/19-4-实现-bundle-vec-可综合路径-fr51.md`
  summary: 未提供 #[derive(Bundle)]；仅手写 Bundle::leaves
  evidence: Design Notes 允许手写；夹具已用手写；宏 derive 非 AC
  status: closed — documented defer + trybuild `derive_bundle_unavailable`（epic-19-retro-item-41）
  resolved: '2026-08-21'
  resolution: language-surface/prelude 声明 derive 不可用；负向测

- source_spec: `_agile-output/implementation-artifacts/19-4-实现-bundle-vec-可综合路径-fr51.md`
  summary: 展平叶名可能碰撞（field_a + member b_c vs field_a_b + member c）
  evidence: 命名约定 {field}_{member}/{field}_{i}；无去重门禁；评审指出
  status: closed — E0152 fail-before-emit（epic-19-retro-item-39）
  resolved: '2026-08-21'
  resolution: `ensure_fresh_signal_name` + `flatten_leaf_name_collision_fails_before_emit`

- source_spec: `_agile-output/implementation-artifacts/19-4-实现-bundle-vec-可综合路径-fr51.md`
  summary: check_connect 只比位宽、不比 GroundType kind（Bool↔UInt 等同宽仍可能过）
  evidence: 预存宽合同；edge-case hunter；非 FR51 引入

- source_spec: `_agile-output/implementation-artifacts/19-5-clockdomain-产品叙事与夹具-fr52.md`
  summary: assign_reg_d_* 不做跨域检查（仅 assign_net）
  evidence: 预存 builder 行为；language-surface 已披露；夹具合法路径用 assign_net

- source_spec: `_agile-output/implementation-artifacts/19-5-clockdomain-产品叙事与夹具-fr52.md`
  summary: ARCHITECTURE-SPINE AD-22 仍写 Clash Signal&lt;D,T&gt;，与 session 域标签实现不一致
  evidence: 本故事可选对齐未改脊柱；产品面已在 language-surface/prelude 钉死
  status: closed — AD-22 修订对齐产品面（epic-19-retro-item-40）
  resolved: '2026-08-21'
  resolution: ARCHITECTURE-SPINE AD-22 + walkthrough 改为 ClockDomain::&lt;ID&gt; + bind_domain

- source_spec: `_agile-output/implementation-artifacts/19-5-clockdomain-产品叙事与夹具-fr52.md`
  summary: 跨域诊断码命名空间仍为 rhdl::E0220（Bitloom 品牌下预存）
  evidence: builder 既有码；夹具沿用

- source_spec: `_agile-output/implementation-artifacts/20-3-firrtl-frozenhir-可编译-chisel-fr28.md`
  summary: 文档路径仍为 `docs/fr28-chisel-best-effort.md`（内容已改写为可编译合同）
  evidence: 避免断链；README 已改显示名；正式更名可另故事
  status: closed — 更名为 `docs/fr28-chisel-compilable.md`（epic-20-retro-item-42）
  resolved: '2026-08-21'
  resolution: git mv + 全仓链接更新

- source_spec: `_agile-output/implementation-artifacts/20-3-firrtl-frozenhir-可编译-chisel-fr28.md`
  summary: InOut 端口发射为 `Analog(Analog())`；实例 InOut 连接仅注释
  evidence: 预存尽力路径；AD-3 导入子集本就排除 Analog/InOut；FR28 夹具不覆盖

- source_spec: `_agile-output/implementation-artifacts/20-3-firrtl-frozenhir-可编译-chisel-fr28.md`
  summary: 仅精确名 `clk`/`rst` 从 IO 剥离；其他 Clock/Reset 端口名仍进入 Bundle
  evidence: 机械约定；多时钟命名留给后续 epic

- source_spec: `_agile-output/implementation-artifacts/20-4-反向导入-chisel-fir-bitloom-fr46-腿-2.md`
  summary: 未知 `parent <= inst.port`（rhs 有点但无匹配实例）仍落入普通 assign
  evidence: 评审；合法夹具均有匹配实例；坏 fir 由 seal 或语义外失败
  status: closed — E0403 显式拒绝（epic-20-retro-item-43）
  resolved: '2026-08-21'
  resolution: 未知 lhs/rhs 点号实例连接 → E0403；往返谓词对 dangling 失配文案加强

- source_spec: `_agile-output/implementation-artifacts/20-4-反向导入-chisel-fir-bitloom-fr46-腿-2.md`
  summary: dangling connect 经 emit 丢失，往返谓词不覆盖 dangling
  evidence: emit 跳过 dangling；FR46 夹具无 dangling
  status: closed — 往返谓词明确拒绝 dangling 失配（epic-20-retro-item-43）
  resolved: '2026-08-21'
  resolution: `fr46_dangling_connect_roundtrip_predicate_fails_clearly`

- source_spec: `_agile-output/implementation-artifacts/20-5-import-cli-混合夹具-fr40-fr46-腿-3.md`
  summary: `import` CLI 默认只写 `.v`（可选 `--also-fir`），不发射 Chisel Scala
  evidence: 对齐 `build` Verilog 后端；Scala 仍用库 `emit_chisel`
  status: closed — `import --also-chisel` 经 `emit_chisel` 可选写出 `.scala`（epic-20-retro-item-44）
  resolved: '2026-08-21'
  resolution: `cargo bitloom import --also-chisel` + `import_also_chisel_writes_scala`

- source_spec: `_agile-output/implementation-artifacts/20-5-import-cli-混合夹具-fr40-fr46-腿-3.md`
  summary: 混合夹具 `include_str` 耦合 monorepo 相对路径
  evidence: 文档夹具；standalone 用户经 `cargo bitloom import --input`

- source_spec: `_agile-output/implementation-artifacts/epic-20-retro-2026-08-21.md`
  summary: 可选卫生：拆分/模块化膨胀的 `rhdl-firrtl` import/emit 面（`lib.rs` 等）
  evidence: epic-20-retro-item-45；不阻塞 FR28/46；大重构另开 hygiene 故事，勿 silently 大 refactor
  status: deferred — 文档锁定；不做本批次大重构

- source_spec: `_agile-output/implementation-artifacts/epic-21-retro-2026-08-21.md`
  summary: FR47 cycle/functional 生成器 MVP = 扁平单模块子集（无层次实例、无 MemDecl 周期精确 emit）
  evidence: epic-21-retro-item-46；`bitloom-sim` `cycle.rs` 显式拒绝 instances/memories；`generate.rs` 取 `modules.first()`；见 language-surface / `docs/fr47-dual-sim-generation.md`
  status: deferred — 子集已文档锁定；扩子集须改文档+故事，禁止静默扩大

- source_spec: `_agile-output/implementation-artifacts/epic-21-retro-2026-08-21.md`
  summary: 层次模块 / Mem 的周期精确生成（及对应 functional 语义）作为后续故事
  evidence: epic-21-retro-item-47；当前 MVP 故意不含；扩子集前必须更新 deferred-work + language-surface + fr47 文档
  status: deferred — 未来故事；forbid silent subset expansion

- source_spec: `_agile-output/implementation-artifacts/epic-22-retro-2026-08-21.md`
  summary: 五类一级 IP 均为端口语义 stub，非全协议实现
  evidence: |
    epic-22-retro-item-48；边界：
    - SyncFifo：depth-1 skid；非异步跨域 FIFO
    - UartTx：字节保持寄存器；非波特率移位 / 全双工
    - SpiMaster：主设备字节缓冲；非 CPOL/CPHA / 多 CS / 从模式
    - I2cMaster：主设备字节缓冲；非多主仲裁 / clock stretch / 从模式
    - Axi4LiteSlave：最小从握手 stub（ADDR=8, DATA=32）；非 Full AXI / 非互联
  status: deferred — 边界写入本文件 + `docs/ip/README.md`；勿误读为全协议

- source_spec: `_agile-output/implementation-artifacts/epic-22-retro-2026-08-21.md`
  summary: UART/SPI/I2C 长期保持 stub MVP（产品锁）；不深化协议语义除非新 epic 显式改合同
  evidence: epic-22-retro-item-49；优先锁 MVP 而非加深协议；见 `docs/ip/README.md`「长期 stub MVP」
  status: locked — 产品文档显式锁定；加深协议 = 新 epic

- source_spec: `_agile-output/implementation-artifacts/epic-22-retro-2026-08-21.md`
  summary: 可选：AXI4-Lite 与 UART/FIFO 简易连接夹具
  evidence: epic-22-retro-item-50；本批次不实现；现有 `examples/ip_box` 仅再导出单 IP，非互联夹具
  status: deferred — 可选后续故事

- source_spec: `_agile-output/implementation-artifacts/epic-23-retro-2026-08-21.md`
  summary: 交互式/更丰富波形浏览（超出静态 `timing.html` Value table）作为后续 epic
  evidence: epic-23-retro-item-51；当前产品面 = `cargo bitloom wave` → 静态 HTML + VCD；见 `docs/fr38-wave.md`
  status: deferred — 未来 epic；不宣称交互式波形已交付

- source_spec: `_agile-output/implementation-artifacts/epic-23-retro-2026-08-21.md`
  summary: 完整 LSP hover/goto 作为后续 epic（已声明 deferred；本条加固）
  evidence: epic-23-retro-item-52；`docs/fr38-viz-lsp.md` / README 已声明；无 language-server 二进制
  status: deferred — 后续 epic；层次/时序 HTML 不声称 LSP 完成

- source_spec: `_agile-output/implementation-artifacts/epic-24-retro-2026-08-21.md`
  summary: CI 默认 Bambu stub（`bambu-ci-stub.sh`）验证接线与非零覆盖，非真实 HLS 调度质量
  evidence: epic-24-retro-item-53；真机入口 `BITLOOM_HLS_USE_REAL=1`（+ `BITLOOM_BAMBU_PATH` 或缓存 AppImage）；见 `docs/fr35-hls.md`
  status: deferred — 文档锁定 stub≠质量；真机为显式入口而非 CI 默认

- source_spec: `_agile-output/implementation-artifacts/epic-24-retro-2026-08-21.md`
  summary: 可选：CI optional/夜间 job 跑真实 Bambu 2024.10（缓存 AppImage；失败不 ignore）
  evidence: epic-24-retro-item-54；本批次不实现完整 CI；默认路径保持 stub
  status: deferred — 可选夜间真机 job

- source_spec: `_agile-output/implementation-artifacts/epic-24-retro-2026-08-21.md`
  summary: 可选：发布烟测夹具扩到第二算法函数或负向调度质量断言
  evidence: epic-24-retro-item-55；当前烟测以 `add` 等最小夹具为主；扩夹具另开可选故事
  status: deferred — 可选后续
