# Story 125.2：旧 AXI bank 的真实 RTL 红测证据

2026-09-20，产品基线 **`0e2a873961988f3c76222d193c402017ec4d8253`**，新增测试尚未提交时执行。没有修改产品 RTL、FL 或旧 FR98 测试。Bitloom 与 `samitbasu/rhdl` 无关。

## 可复现测试与实测结果

驱动为 `crates/bitloom/tests/fr193_axi_protocol.rs`。三个手写定向帧序列分别运行 Sim Interpreter、Sim Compiled 和 direct emitted Verilog（`bitloom_vlog::emit` → Icarus → vvp）。黄金值不是从 FL、模拟器或 RTL 输出推导；三种后端消费同一输入及断言列表。

```bash
cargo test -p bitloom --test fr193_axi_protocol
PATH=/tmp/bitloom-maintenance-tools/bin:$PATH BITLOOM_REQUIRE_RTL=1 \
  cargo test -p bitloom --test fr193_axi_protocol -- --ignored --nocapture
```

第一个命令退出 **0：1 passed / 9 ignored**；普通项生成设计和测试台、检查旧端口。第二个退出 **101：0 passed / 9 failed**。本机工具路径只是复现实例；测试代码仅通过 PATH 查找工具，不包含机器专属路径。显式 RTL 测试始终要求 `iverilog`、`vvp`，严格程度不低于 `BITLOOM_REQUIRE_RTL=1`；缺工具或编译失败会以 `TOOL ERROR` 失败，不能视为协议复现成功。

| 场景 | Interpreter | Compiled | 真实 RTL |
|---|---|---|---|
| `aw_first` | frame 3 post BVALID=0，期望1 | 同左 | 同左；vvp退出1 |
| `w_first` | frame 3 post BVALID=0，期望1 | 同左 | 同左；vvp退出1 |
| `concurrent` | frame 3 post RVALID=0，期望1 | 同左 | 同左；BVALID=1；vvp退出1 |

三个 RTL 场景的 `iverilog -g2012 -s tb -o simulation design.v tb.sv` 均退出 **0**，没有编译错误。`vvp simulation` 在 Time 55、Scope tb 报具体 `PROTOCOL` 断言：

```text
aw_first: delayed W must produce B post s_axi_bvalid: expected 1, got 0
w_first: delayed AW must produce B post s_axi_bvalid: expected 1, got 0
concurrent: concurrent same-address read must produce R and B post s_axi_rvalid: expected 1, got 0
```

工具实测版本：`iverilog -V` 为 Icarus Verilog **12.0 (stable)**；`vvp -V` 为 runtime **12.0 (stable)**；`rustc --version` 为 **1.97.1 (8bab26f4f 2026-07-14)**；`cargo --version` 为 **1.97.1 (c980f4866 2026-06-30)**。

## 输入、采样与产物

每帧先驱动全部输入（未列项显式归零），settle / #5 后检查 pre-edge READY；只产生一次上升沿，tick / #5 后检查响应。已经握手的 VALID 下一帧撤销，避免多 tick 意外重复接受。响应 READY 默认低，直到指定消费帧才拉高。

- AW早：地址0x04先握手，然后撤AWVALID并将地址变成0x0c；空一拍后W发送0x11223344/WSTRB=0xf，应有B并写原地址。
- W早：先握手0x11223344/WSTRB=0x5；撤WVALID后将payload变成0xdeadbeef/WSTRB=0xa；延迟AW至0x04，应有B，读取应为0x00220044。
- 并发：先向0x04写0xaabbccdd并消费B；再同拍AW/W/AR握手，写0x11223344、读0x04，应同时有B/R，R返回旧值0xaabbccdd；之后读取新值。

固定后续帧还检查响应受阻保持、消费后撤销、无重复响应和地址0x0c未被误写。**基线在第一个缺失响应处终止，这些后续断言尚未执行通过，不能用本页证明缓存payload或read-before-write已实现。** 同步reset初始化后再驱动事务。

产物保存至 `target/fr193-axi/<aw_first|w_first|concurrent>/<interpreter|compiled|rtl>/`：`design.v`、`tb.sv`、`frames.txt`，以及 native 的 `native.log` 或 RTL 的 `compile.log`、`rtl.log`、`simulation`。普通emit项写入同场景的 `emit/`。每后端独立目录避免并行覆盖；再次运行会更新同目录。产物不入版本库，测试源与本页足以重新生成，不能仅凭现存日志判定后续版本结果。

## 未交付范围与125.3接续

九项测试以明确 `#[ignore = "125.2 reproduces unfixed AW/W or concurrent AR transaction loss; enable in 125.3"]` 隔离，没有 `should_panic`、没有把工具错误转为成功。普通测试绿不等于 AXI 修复交付；125.3需要修复硬件和手写FL、解除ignore，并补齐随机交错、背压、reset及旧映射兼容回归。

本驱动是 **raw-channel testbench**，没有使用高层 BFM。主代理独立并行验证的 cocotbext-axi API 探针见 [phase24-axi-bfm-probe.md](phase24-axi-bfm-probe.md)；stub探针成功不等于产品 RTL 通过。

本页仅复现旧8-bit地址/32-bit数据/四寄存器bank缺陷，不引入未来16-bit CSR桥规则，不声称协议完备、综合、formal或整个工作区CI通过。旧 unmapped读0、写忽略并OKAY的语义不变；本故事未重新证明其完整矩阵。Phase24未交付，Epic126–130仍backlog，FR189 deferred与NFR91不变。

原始失败输出已归档：[phase24-axi-red.log](evidence/phase24-axi-red.log)，不随后续绿色运行覆盖。
