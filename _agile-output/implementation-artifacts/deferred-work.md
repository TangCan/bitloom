# Deferred work

## Contract-green pointer (FR87 / NFR38)

阶段五–七「绿 / 产品做完」验收以 **合同绿** 为准，定义见
[`docs/requirements/19. 实施路线图.md`](../../docs/requirements/19.%20实施路线图.md) §19.7–19.9
与 README「状态与 deferred」。**禁止**用字面未交付项勾选全绿。

## 永久非目标（FR93）

公开合同锁定（与 README「状态与 deferred」§永久非目标（FR93）同源）。**须新 PRD 才能推翻**；不得把下列项标成 done 或静默交付交差。

1. 树内 / 自研 HLS 调度器
2. FIRRTL→idiomatic Scala
3. 默认 TLM≡CA 形式证明
4. VIP 级全协议 IP
5. 按键全设计 elaborate 的 netlist LSP

PRD 指针：`planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md`（Phase 11 / FR93）。

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
  summary: 五类一级 IP 历史为端口语义 stub；Epic 34/FR82 加深为非 stub 文档最小子集
  evidence: |
    epic-22-retro-item-48；更新（2026-09-08 / Story 34.3）：
    - SyncFifo：**FR82 非 stub** depth-4 + full/empty；非异步跨域 FIFO
    - UartTx：**FR82 非 stub** 8N1；**FR89 / Epic 38.2：** 可编程 `baud_div` 子集已交付；仍非 RX / 全双工 / VIP / 全协议 / 小数分频
    - SpiMaster：**FR82 非 stub** Mode-0-ish MSB byte shifter；非多 CPOL/CPHA / 多 CS
    - I2cMaster：**FR82 非 stub** START+8data+STOP；非 ACK/伸展/多主
    - Axi4LiteSlave：**FR82 非 stub** 单寄存器握手玩具；非 Full AXI / VIP
  status: done — 五类 FR82 基线已交付；全协议仍见下条；边界 `docs/ip/README.md`

- source_spec: `_agile-output/implementation-artifacts/epic-22-retro-2026-08-21.md`
  summary: UART/SPI/I2C「全协议」仍非默认交付；Epic 34 仅合同化最小可综合基线（非 VIP 级）
  evidence: |
    epic-22-retro-item-49 + Epic 34 / FR82 / nfr14-risk-epic34-ip-baseline.md；
    34.2–34.3 已交付五类文档最小子集；Epic 38.2 合同化 UartTx **可编程波特率子集**（`baud_div`；非 RX/VIP）；
    Story 38.3 已收口交叉引用并勾选 NFR14 Epic 38 关闭条件；
    全协议 / RX / 多模式 SPI / I2C ACK / VIP 等**仍须新合同**（FR89 子集 ≠ 全家桶）
  status: locked — 全协议加深仍须显式改合同；FR82 基线 ≠ 全协议；Epic 38 / FR89 子集已关闭

- source_spec: `_agile-output/implementation-artifacts/epic-22-retro-2026-08-21.md`
  summary: 可选：AXI4-Lite 与 UART/FIFO 简易连接夹具
  evidence: epic-22-retro-item-50；本批次不实现；现有 `examples/ip_box` 仅再导出单 IP，非互联夹具
  status: deferred — 可选后续故事

- source_spec: `_agile-output/implementation-artifacts/epic-23-retro-2026-08-21.md`
  summary: 交互式/更丰富波形浏览（超出静态 `timing.html` Value table）作为后续 epic
  evidence: epic-23-retro-item-51；当前产品面 = `cargo bitloom wave` → 静态 HTML + VCD；见 `docs/fr38-wave.md`
  status: deferred — 未来 epic；不宣称交互式波形已交付

- source_spec: `_agile-output/implementation-artifacts/epic-23-retro-2026-08-21.md`
  summary: 完整 / 自研 Bitloom LSP hover/goto（FR91 Path B 显式 defer；已声明 deferred；本条加固）
  evidence: epic-23-retro-item-52；**Story 39.3** 合同化 FR91 Path B（`docs/fr38-viz-lsp.md` / README；NFR14 FR91 已勾）；无 language-server 二进制；Epic 35 / Story 35.4 再次声明 LSP **不是**本 epic 完成条件；**FR90 宿主 rust-analyzer 路径**见 `docs/fr90-host-ide-rust-analyzer.md`（≠ 自研 Bitloom LSP）
  status: deferred — FR91 Path B；层次/时序 HTML 不声称 LSP 完成；Epic 35 不交付 LSP；宿主 IDE（FR90）另文档；改选浅层 MVP（分支 A）须新 NFR14

- source_spec: `_agile-output/implementation-artifacts/epic-24-retro-2026-08-21.md`
  summary: CI 默认 Bambu stub（`bambu-ci-stub.sh`）验证接线与非零覆盖，非真实 HLS 调度质量
  evidence: epic-24-retro-item-53；真机入口 `BITLOOM_HLS_USE_REAL=1`（+ `BITLOOM_BAMBU_PATH` 或缓存 AppImage）；见 `docs/fr35-hls.md`；Epic 37 / FR88 Path B
  status: closed — 文档锁定 stub≠质量；真机为显式入口而非 CI 默认（Epic 37 本阶段选 B）
  resolved: '2026-09-09'
  resolution: Story 37.3 Path B；FR88 HLS 诚实条勾选；不冒充真机质量

- source_spec: `_agile-output/implementation-artifacts/epic-24-retro-2026-08-21.md`
  summary: 可选：CI optional/夜间 job 跑真实 Bambu 2024.10（缓存 AppImage；失败不 ignore）
  evidence: epic-24-retro-item-54；Epic 37 / FR88 **本阶段选 B**（显式保持 stub 默认；真机仍为 `BITLOOM_HLS_USE_REAL=1`）；见 `docs/fr35-hls.md`
  status: closed — 本阶段选 B（不落地夜间真机 job）
  resolved: '2026-09-09'
  resolution: Story 37.3 显式 Path B；可选夜间 A 路径未实现；若未来重开须失败不 ignore / 无 continue-on-error

- source_spec: `_agile-output/implementation-artifacts/epic-24-retro-2026-08-21.md`
  summary: 可选：发布烟测夹具扩到第二算法函数或负向调度质量断言
  evidence: epic-24-retro-item-55；当前烟测以 `add` 等最小夹具为主；扩夹具另开可选故事
  status: deferred — 可选后续

- source_spec: `_agile-output/implementation-artifacts/epic-25-retro-2026-08-21.md`
  summary: 首次 push 后记录 GHA fr28-chisel-jvm 冷/热墙钟并收紧 timeout-minutes（现行 20）
  evidence: epic-25-retro-item-56；流程与空表已写入 `docs/fr28-chisel-compilable.md`「GHA 墙钟记录」；ci.yml 注释指向该表。在实测样本填入前不得盲目下调 timeout
  status: deferred — 等待首次 CI 样本；文档门禁已就位
  resolved: '2026-08-21'
  resolution: 文档化测量流程 + 占位表；关闭 action item（实测填表为后续运维，非阻塞代码）

- source_spec: `_agile-output/implementation-artifacts/epic-25-retro-2026-08-21.md`
  summary: 维护者合并 FR28/emit_chisel 前本机 just chisel-fr28-jvm 检查清单
  evidence: epic-25-retro-item-57
  status: closed — `docs/fr28-chisel-compilable.md`「维护者合并前检查清单」
  resolved: '2026-08-21'
  resolution: 勾选清单已写入 FR28 文档并链 README

<!-- action-items-sweep-2026-09-09: epics 26–35 open retro items disposition -->

- source_spec: `_agile-output/implementation-artifacts/epic-26-retro-2026-09-09.md`
  summary: Epic 27–35 ready 前门禁（NFR14 + PRD §5.9 / epics.md Gate）— 历史 gate 已兑现
  evidence: |
    epic-26-retro-item-58 + items 60/63/66/69/72/75/78/81；
    sprint-status：epic-27…35 与各 `*-1-*nfr14*` 均为 done；
    记录：`nfr14-risk-phase9-closures.md`（26–28/Phase9）、`nfr14-risk-epic29…35-*.md`
  status: closed — gates satisfied; sweep 2026-09-09
  resolved: '2026-09-09'
  resolution: 全部后续 epic 已 done；NFR14 故事与风险记录在库；无需再挡 ready

- source_spec: `_agile-output/implementation-artifacts/epic-26-retro-2026-09-09.md`
  summary: 合同故事 commit 保持 diff 纯净（无无关示例/格式化/工具脚本）
  evidence: epic-26-retro-item-59；续 76/79/82/85
  status: deferred — standing process
  resolved: '2026-09-09'
  resolution: 写入 `process-one-story-one-commit.md`「Diff purity」；后续故事触及时遵守

- source_spec: `_agile-output/implementation-artifacts/epic-27-retro-2026-09-09.md`
  summary: Typed Wire/Reg handle / typed inline macros 落地后，跟进自动硬件捕获与 rustc 闭包体·借用分析（增强/替代手工 HwCaptureRef 与 Cap-R-60/70 token）；保持 E0141–E0146 与 D1/host 路径分立（NFR35）
  evidence: |
    epic-27-retro-item-61；epic-28-retro-item-64；epic-29-retro-item-67；epic-30-retro-item-70；
    当前仍为 token/手工路径；typed language-surface 未交付
  status: deferred — future trigger（typed Wire/Reg / inline macros）
  resolved: '2026-09-09'
  resolution: 阻塞于 typed surface；出现后新开故事，禁止静默替换诊断码分立合同

- source_spec: `_agile-output/implementation-artifacts/epic-27-retro-2026-09-09.md`
  summary: bitloom-builder 按域拆分（closures / inline / hw_capture / session）— 体量卫生
  evidence: |
    epic-27-retro-item-62；epic-28-retro-item-65；复现于 68/71/74/77/80/83/86；
    2026-09-09 首步：`crates/bitloom-builder/src/closures.rs`（~379 行）抽出 FR73–75
    自由函数与类型；`lib.rs` ~2731（原 ~3101）；crate root `pub use` 不变；
    `cargo test -p bitloom-builder --lib` 45 passed
  status: deferred — further domain splits
  resolved: '2026-09-09'
  resolution: |
    首步 scaffolding 已落地。后续预算：仅当单文件再增 ≥~800 行或新 FR 域进入时，
    再拆 `inline`/`hw_capture` 或 session 子模块；禁止无故事的大 refactor。
    评估完成 → action items 关闭；剩余拆分为可选 hygiene 故事。

- source_spec: `_agile-output/implementation-artifacts/epic-29-retro-2026-09-09.md`
  summary: bitloom-prelude `ip.rs` 体量监视与拆分预算（FR77 overlay + FR82 五类）
  evidence: |
    epic-29-retro-item-68；续 83/86；现行 ~1357 行单文件（SyncFifo/UartTx/SpiMaster/
    I2cMaster/Axi4LiteSlave/ExtBlackBox/Crc8Lut + tests）
  status: deferred — size watch + split budget
  resolved: '2026-09-09'
  resolution: |
    本 sweep **不**拆 `ip.rs`（行为风险高于收益；五类 Elaboratable 紧耦合测试）。
    触发拆分：单文件 ≥~1800 行，或新增第六类一级 IP / 大块 FR77 生成器时，
    按 IP 类型 `mod` 拆（`ip/{sync_fifo,uart_tx,…}.rs`）且保持 `bitloom_prelude::ip::*` 再导出。
    在此之前仅监视；禁止 silent 大 refactor。

- source_spec: `_agile-output/implementation-artifacts/epic-31-retro-2026-09-09.md`
  summary: Standing NFR37 诚实边界（触及对应面时遵守；非立即实现）
  evidence: |
    - item-73：SyncFIFO（FR79 CDC）≠ ip::SyncFifo（FR82 单时钟）— docs/ip/README.md
    - item-76：一层 Bundle 嵌套合同；HwVec&lt;Bundle,_&gt; / ≥2 层不冒充 FR80
    - item-79：emit_chisel Mem Path A + E0901；NFR12 版本钉死；FR71≠历史 E0901
    - item-82：FR82 五类非 stub ≠ Epic 22 stub；无生成器闭包（overlay=FR77）
    - item-84：加深 FR83/84/85 或新 epic 前复用 NFR14 选型表；禁历史 Partial 冒充深度
    - item-85：C ABI / SoftF16 / formal / LSP 合同边界（FR83–85）；LSP 仍 deferred
  status: deferred — standing contract when touching area
  resolved: '2026-09-09'
  resolution: 编入本 ledger；触碰相关 crate/文档时对照；合同故事仍遵守 diff purity（item-59）

- source_spec: `_agile-output/implementation-artifacts/epic-30-retro-2026-09-09.md`
  summary: 可选产品加深（「若产品需要…」）— 超出已满足 AC 的 MVP；单独立项
  evidence: |
    - item-71：DUT 真 busy × FR47 深度夹具（30.3 Counter+host busy；30.2 UartTx 已覆盖真实 busy）
    - item-74：可配置 DEPTH/WIDTH 全家桶或 wr_clk/rd_clk 双物理时钟（超出 4×8 / phantom 双域）
    - item-77：≥2 层嵌套递归或 HwVec&lt;Bundle,_&gt;（超出一层 MVP）
    - item-80：双时钟裸 mem / 多口掩码 / 翻转 Path B（永久非目标除非新合同）
    - item-83：全协议 / VIP / Full AXI / 可编程 baud **全家桶**（小数分频/表）/ depth·width 全家桶；注：UartTx **分频子集**已由 Epic 38.2 / FR89 合同化并由 Story 38.3 收口，不等于全家桶
    - item-86：任意 FrozenHir C 加载器 / SoftF16→HIR→emit Path A / 商用 sby 全证明 / LSP 二进制
  status: deferred — optional product scope（explicit new contract required）
  resolved: '2026-09-09'
  resolution: 非缺陷；当前 epic AC 已满足。需要时开新故事+改文档/NFR14，禁止静默扩大 MVP。

<!-- action-items-sweep-2026-09-09: epics 36–39 Phase-11 open retro items disposition -->

- source_spec: `_agile-output/implementation-artifacts/epic-36-retro-2026-09-09.md`
  summary: Standing Phase-11 诚实 / 过程边界（Epic 36–39；触及对应面时遵守；非立即实现）
  evidence: |
    - item-87：对外「阶段五–七全绿 / 产品做完」须引 FR87 / NFR38（docs/requirements/19 §19.7–19.9；README「状态与 deferred」；本文件 Contract-green pointer）；合同绿 ≠ 字面七阶段
    - item-88：FR93 五条永久非目标公开锁定（本文件「永久非目标」+ README + PRD addendum）；须新 PRD 才能推翻；不得标 done / 静默交付
    - item-90：NFR12 升钉纪律 — 禁止私自升 Chisel/firtool；须上游正式配对并同步 ARCHITECTURE-SPINE Stack / AD-9 / addendum / docs/fr28-chisel-compilable.md 运维清单
    - item-91：FR88 HLS Path B — stub≠质量；真机仅 BITLOOM_HLS_USE_REAL=1；未来夜间真机 job 失败不得 continue-on-error（docs/fr35-hls.md）
    - item-93：FR89 / NFR39 — 可编程 baud_div 子集 ≠ RX / 全双工 / VIP / 全协议 / 小数分频·波特率表全家桶；未选分支 B 不得声称已交付（docs/ip/README.md）
    - item-94：UartTx ABI — 新增 baud_div 为合同加深；未驱动→0 ≡ FR82；改分频语义/默认值须回归 fr82_* / fr89_* 并改文档
    - item-96：FR91 Path B / NFR39 — 宿主 rust-analyzer ≠ 自研 LSP；HTML/时序可视化 ≠ LSP 完成；半成品 language-server 不得冒充完成（docs/fr38-viz-lsp.md / fr90-host-ide-rust-analyzer.md）
    - item-97：FR92 / AD-5 — 同刺激记分板 + adapter 模板 ≠ 自动 FL≡RTL 形式证明 ≠ SystemC TLM-2.0；不得引入第二套无对照仿真语义（docs/fr92-shared-stimulus-adapter.md）
    - item-89/92/95/98：epics.md frontmatter phase11Epic36–39Status 与 phase11Status 对齐为 complete（编排元数据；非产品加深）
  status: deferred — standing contract when touching area
  resolved: '2026-09-09'
  resolution: |
    编入本 ledger；公开文档已含对应诚实条。触碰相关 crate/文档/CI 叙事时对照；
    加深子集或改选分支须新合同 + NFR14，禁止静默扩大。
