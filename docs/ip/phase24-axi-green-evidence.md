# Story125.3 / FR193：既有AXI bank修复证据

2026-09-20；Bitloom，与 `samitbasu/rhdl` 无关。[125.2红测](phase24-axi-red-evidence.md)及其原始失败日志保留，不用本次绿色产物覆盖历史证据。

## 修复边界

旧 `Axi4LiteSlave` 的AW、W各有一个捕获槽。READY仅由寄存的槽/响应占用决定；写选择已捕获payload或本拍实际握手payload，收齐后只提交一次。同拍AW/W保持一tick BVALID；B消费拍保持原有等待拍，不承诺无气泡吞吐。AR独立接受，即使B受阻也能读；同拍同址AR/写返回写前值。B/R及读快照保持至消费。同步高有效reset清四个bank寄存器、捕获payload/valid及响应，不产生幽灵配对。

接口仍为8-bit地址、32-bit数据、4-bit WSTRB、0x00/04/08/0c四寄存器，reset值0；WSTRB=0无副作用。未映射/未对齐读0、写忽略，均OKAY。这不是未来16-bit CSR桥的SLVERR/DECERR合同。手写FL采用私有Option捕获与更新前读快照；未增加公开字段、包、端口或稳定API。

## 独立黄金与覆盖

`crates/bitloom/tests/fr193_axi_protocol.rs`保留125.2原始九项断言并解除ignore。扩展黄金以16字节数组保存bank，事务队列保存已接受AW/W及待消费B/R；读在写前取快照。每拍验证接受、提交、消费、复位取消及仍在途数量守恒。所有结果来自黄金模型，禁止从DUT输出推导期望；同一输入/期望分别检查Interpreter、Compiled、手写FL和实际`bitloom_vlog::emit`→Icarus→vvp。

- 定向矩阵：四合法地址及1/3/13/16/252/255未对齐/空洞，全部16种WSTRB，AW早/W早/同拍，间隔0/1/7/31；捕获后针脚变化、并发同址读旧值、B/R背压时继续尝试、消费后无重复。
- reset矩阵：仅AW、仅W、待B、待R、双响应停顿；优先于本拍VALID，检查bank归零、连续空拍无幽灵响应，再先发送相反半笔，确认不能与已取消旧半笔配对，补齐新事务并验证读回。待响应reset帧同时拉高B/RREADY及非零payload/VALID，确认reset优先于消费和提交；另覆盖半笔补齐期间保持reset三拍，随后释放首拍立即同拍新AW/W/AR，并读全bank检查旧地址未写入。
- 随机：固定seed `0x1253193000000001` 至 `0x1253193000000010`，每seed **1000笔写+1000笔工作负载读**，全部排空后额外读四个bank寄存器（`verification_reads=4`），捕获随机读先结束后晚到写入的最终状态，AW/W/AR分别按0..31拍延迟发起；VALID/payload受阻保持，握手后可变化。BREADY/RREADY独立随机，分别至少每17/19拍置1；每seed上限100000拍，超时失败。随机阶段不注入reset；取消由上述定向矩阵覆盖。每seed明确计数并要求AW-first、W-first、同拍AW/W、AR与写提交同拍、R受阻时写提交五种交集均大于0。
- RTL初始reset边沿之前寄存器未初始化，不断言首帧pre；该帧post及后续全部pre/post都检查。native首帧pre检查零初始化状态，FL检查每帧post。

## 命令与产物

```bash
PATH=/tmp/bitloom-maintenance-tools/bin:$PATH BITLOOM_REQUIRE_RTL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1 \
  cargo test -p bitloom --test fr193_axi_protocol -- --nocapture
cargo test -p bitloom-prelude axi
cargo test -p bitloom --test fr82_spi_i2c_axi_baseline \
  --test fr98_axi_near_vip --test fr168_spi_i2c_axi_handwritten_fl
python3 scripts/check_phase24_gate.py
```

机器专属PATH仅用于本次执行；测试只查PATH。另以空工具PATH直接运行`aw_first_rtl`，实测退出101并报告`TOOL ERROR: iverilog required on PATH`，验证缺工具不会变成skip/pass。实际RTL始终required（不依赖env才强制），缺`iverilog`/`vvp`、编译失败、断言失败、没有PASS标记均失败。CI现有test job已安装Icarus并执行`BITLOOM_REQUIRE_RTL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1 cargo test --workspace`，新矩阵无ignore；同job加入Phase24 scope/dependency gate。

独立vectors、design和testbench均在执行native/FL断言之前落盘，失败也保留完整复现输入。产物在`target/fr193-axi/directed/golden/`及`seed-<hex>/golden/`：独立`vectors.hex`、`accounting.txt`、真实`design.v`、常量大小`tb.sv`、`iverilog.log`、`vvp.log`、`simulation`。原九项仍按各自场景/后端写入。产物可重生成且不入版本库。

工具：Rust/Cargo 1.97.1，Icarus/vvp 12.0 stable，与125.2红测相同。旧兼容回归：prelude axi **1 passed**，FR82 **5 passed**，FR98 **5 passed**，FR168 **10 passed**；Phase24 gate PASS。FR193修订完整测试 **12 passed / 0 failed / 0 ignored**，测试执行 **243.99秒**；`CARGO_PROFILE_TEST_OPT_LEVEL=1`仅提高test优化等级，仍为test profile、保留debug assertions/overflow checks，不改Cargo.toml或产品profile。最后仅增补定向reset帧的READY/payload交集，使用相同环境命令加`required_directed_protocol_matrix`过滤单独复跑 **1 passed**（20.19秒）；随机函数未再变化。

| 实测矩阵 | 拍数 | 写提交 | AR接受 / 说明 |
|---|---:|---:|---|
| 最终定向矩阵 | 24,119 | 2,569 | 3,877；AW/W/B/R复位取消各2，最终无在途事务 |
| 16个随机seed合计 | 288,245 | 16,000 | 16,064 = 16,000工作负载读 + 64末尾验证读 |

五类随机交集（AW-first、W-first、同拍AW/W、AR+写提交、写提交时R受阻）合计分别 **7,124 / 7,178 / 1,698 / 932 / 2,117**；各seed对应最小值 **409 / 390 / 87 / 43 / 103**，全部非零。所有矩阵均完成Interpreter、Compiled、手写FL、实际direct RTL黄金对照。原九项缺陷回归全部转绿，期望未更改。

完整运行及最后定向复跑输出归档：[phase24-axi-green.log](evidence/phase24-axi-green.log)。此处记录实际执行边界，工作区剩余验证已由主代理独立完成。

## 不作的宣称

本故事不证明Chisel/FIRRTL路径上的该矩阵、综合/PPA、形式证明、商业VIP或协议完备性；raw-channel driver不是cocotbext-axi BFM覆盖。工作区/SemVer总体验证及M0状态关闭由主代理另记。Phase24整体未交付；Epic126–130保持backlog，FR189/Epic122 deferred、NFR91不变，不新增FR142稳定表面。

## M0 关闭与工作区验证

2026-09-20：工作区1690 passed/0 failed/6既有ignored；两个长矩阵已独立通过，合并唯一Rust测试1692通过。三个稳定库各196个SemVer检查通过；fmt及Phase24 gate通过。完整命令与范围见[验证汇总](evidence/phase24-verification.json)。Epic125/FR192/FR193关闭，后续Epic126–130仍backlog，Phase24整体未完成。
