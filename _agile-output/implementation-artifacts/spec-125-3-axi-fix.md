---
title: 'Story 125.3 AXI 修复、双模型与 M0 关闭'
type: bugfix
created: 2026-09-20
status: done
route: dispatch
baseline_commit: ac6e4804578b6c53a4eddbd164f54fad57554766
review_loop_iteration: 0
context:
  - '{project-root}/AGENTS.md'
  - '{project-root}/docs/ip/phase24-contract.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-125-context.md'
---
<frozen-after-approval>
## Intent
修复旧Axi4LiteSlave接受AW/W/AR后丢失事务，使合法分拍/并发/背压/reset在native和实际RTL均正确；同步FL并保留旧接口/寄存器映射和同拍1tick响应。M0已获用户批准。
## Boundaries & Constraints
沿用8位addr、32位data、4寄存器与原clk/rst；unmapped/unaligned读0写忽略且OKAY。旧单拍响应不可变慢；分拍AW/W必须保持被接受的payload，不采样后续针脚。保持单一HIR现有builder。新CSR/16位桥不在此故事。主代理负责状态/提交。
## I/O & Edge-Case Matrix
| 场景 | 输入/状态 | 期望 |
|---|---|---|
| 分拍 | AW早/W早，间隔0/1/7/31，接受后针脚变化 | 各一槽；凑齐提交一次；B保持到ready |
| 并发 | AW/W/AR同拍接受，同址 | B/R各一次；读旧值 |
| 背压 | B/R ready为0，输入valid/payload继续尝试 | 响应稳定，不覆盖；无重复副作用/配对 |
| 字节 | 全16种WSTRB，含0，四地址及洞/未对齐 | 字节merge；无效地址保持旧规则 |
| reset | 仅AW/仅W/待B/待R/双响应停顿 | 同步清bank与所有暂存/valid，无幽灵响应；reset后新事务可完成 |
</frozen-after-approval>
## Code Map
- `crates/bitloom-prelude/src/ip/axi.rs`: 增加AW/W缓存有效位和payload，ready只依赖寄存状态；优先缓存/否则live接受的payload。保留同拍直通以维持旧延迟；读独立执行/锁存，可在B阻塞时继续读。同址读为寄存器旧值。
- `crates/bitloom-sim/src/ip_dual.rs`: Axi4LiteSlaveFunctional同样支持缓存，但用手写算法；读快照在修改bank前计算。公开stimulus可加独立合法序列。
- `crates/bitloom/tests/fr193_axi_protocol.rs`: 125.2红测转required；扩展矩阵并以独立事务队列/byte-array scoreboard验证。两native引擎、FL和actual directRTL不得只互相对拍而无黄金。
- `crates/bitloom/tests/fr98_axi_near_vip.rs`, `fr82_spi_i2c_axi_baseline.rs`, `fr168_spi_i2c_axi_handwritten_fl.rs`: 旧验收必须保持通过，不修改期望回避缺陷。
- `.github/workflows/ci.yml`: 已有BITLOOM_REQUIRE_RTL=1 cargo test --workspace，纳入新矩阵；追加phase24 gate检查。
## Tasks & Acceptance
- [x] 修电路和手写FL，注释按真实能力更新。
- [x] 解除125.2忽略，将矩阵全部执行；加入固定seed随机独立AW/W/AR延迟、B/R背压的scoreboard，至少16seed×1000事务，公平ready下有超时；记录接受/写提交/响应/复位取消，验证不丢不重。RTL由独立黄金向量驱动，不能把DUT结果当黄金。
- [x] README AXI能力段、docs/fr168-spi-i2c-axi-handwritten-fl.md与docs/ip/phase24-axi-green-evidence.md记录修复边界、直接RTL/FL实测和后续未交付；保留红测历史证据。不要声称已跑Chisel/FIRRTL本故事未执行矩阵或形式证明。
- [x] CI gate检查加入现有测试job，严格RTL必须运行/缺工具fail。不引入新的包或公开字段。
**Acceptance Criteria:** Given125.2已真实失败，When修复后执行相同测试及矩阵，Then两native引擎/FL/实际RTL均满足黄金结果；Given旧API，When运行既有AXI/FR168测试，Then兼容通过；Given复位，When半笔/阻塞事务被取消，Then无幽灵响应且新事务可用。
## Implementation Notes
实现代理只负责以上代码测试文档，不能git commit，不能修改sprint/spec状态。主代理负责最终workspace/SemVer验证及M0 gate关账。
## Spec Change Log
## Review Triage Log
- B1 false：review快照形成时green evidence尚未写入；实际 `docs/ip/phase24-axi-green-evidence.md` 已存在，完整实测结论等待运行结束后填写，不是最终缺失交付。
- B2 medium / patch：随机读可能早于最后写结束，末尾bank缺乏必达观察；追加四寄存器读回并单列验证读数量。
- B3 medium / patch：native断言可能先于vector/TB落盘，影响失败复现；将独立工件生成前移。native失败后强制继续RTL非本故事完成条件，不添加吞错。
- B4 medium / patch：随机交叉场景只有总计数，补小型覆盖计数与必达断言，不能将随机预算冒称穷尽。
- B5 medium / patch：补连续reset与首次释放边沿事务，明确恢复行为。
- B6 medium / patch：补reset与响应READY/非零新payload同时出现，确保reset优先；与B5共用最小场景。
- B7 maybe-false / reject：新增RTL子进程无单独deadline属事实，但未找到可达的无限delta循环，CI test job已有25分钟timeout；局部诊断改进为未证实低风险，当前不引入进程管理复杂度。
- E：edge-case-hunter返回空列表，无新边界缺陷。
- V：verification-gap层未发现验证缺口。
- 主审 medium / patch：docs/ip/README仍写125.3待修复，更新当前状态并保留历史红测链接。

## Verification
`PATH=/tmp/bitloom-maintenance-tools/bin:$PATH BITLOOM_REQUIRE_RTL=1 cargo test -p bitloom --test fr193_axi_protocol -- --nocapture`
`cargo test -p bitloom-prelude axi`
`cargo test -p bitloom --test fr82_spi_i2c_axi_baseline --test fr98_axi_near_vip --test fr168_spi_i2c_axi_handwritten_fl`
只格式化改动Rust文件，避免全库既有风格噪声。

## 完成验证与交接

原9项缺陷回归及两类矩阵通过；最终定向24,119拍，随机16seed/288,245拍，合计16,000写+16,000工作负载读+64验证读。最后reset-ready交叉补充单独定向复跑通过。Workspace 453 target结果：1690 passed/0 failed/6既有ignored/2 filtered（两个长矩阵已独立通过），合并唯一Rust测试1692通过。fmt通过；SemVer三库各196通过，58适用性skip。三路review按上表处置，未遗留已确认的产品缺陷。完整结果见 `../../docs/ip/evidence/phase24-verification.json`。

M0获准范围完成，Epic126–130保持backlog；不自动启动后续、push或publish。
