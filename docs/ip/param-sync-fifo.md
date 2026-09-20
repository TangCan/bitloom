# 参数化同步寄存器 FIFO

Bitloom 与 samitbasu/rhdl 无关。设计 crate 只依赖 `bitloom-prelude`。

`ParamSyncFifo<const WIDTH: u32 = 32, const DEPTH: u32 = 4>` 支持 WIDTH=1..64、DEPTH=1..16，包括非二次幂深度。非法参数在声明硬件前返回带名称和值的 Diagnostics，不裁剪、不 panic。相同名字和完整参数复用定义；同名不同 WIDTH 或 DEPTH 失败，不同名专门化可并存。

以下完整程序创建同一 session 的两个独立实例、复用同一 8×3 定义，最终只 finish 一次：

```rust
use bitloom_prelude::{Diagnostics, ElaborateSession, GroundType, Span, ip::ParamSyncFifo};
fn main() -> Result<(), Diagnostics> {
    let mut session = ElaborateSession::new("Top");
    let leaf = ParamSyncFifo::<8, 3>::define_module(&mut session, "Fifo8x3")?;
    assert_eq!(leaf, ParamSyncFifo::<8, 3>::define_module(&mut session, "Fifo8x3")?);
    let sp = Span::default();
    session.begin_module("Top", sp);
    session.add_input("clk", GroundType::Clock, sp);
    session.add_input("rst", GroundType::Reset, sp);
    for prefix in ["a", "b"] {
        for name in ["flush", "input_valid", "output_ready"] {
            session.add_input(format!("{prefix}_{name}"), GroundType::UInt { width: 1 }, sp);
        }
        session.add_input(format!("{prefix}_input_data"), GroundType::UInt { width: 8 }, sp);
        for name in ["input_ready", "output_valid"] {
            session.add_output(format!("{prefix}_{name}"), GroundType::UInt { width: 1 }, sp);
        }
        session.add_output(format!("{prefix}_output_data"), GroundType::UInt { width: 8 }, sp);
        let mut ports = vec![("clk".to_owned(), "clk".to_owned()),
            ("rst".to_owned(), "rst".to_owned())];
        ports.extend(["flush", "input_valid", "input_data", "output_ready",
            "input_ready", "output_valid", "output_data"].into_iter()
            .map(|name| (name.to_owned(), format!("{prefix}_{name}"))));
        session.add_instance(prefix, leaf.clone(), ports, vec![], sp);
    }
    session.end_module();
    let hir = session.finish()?;
    assert_eq!(hir.circuit().modules.len(), 2);
    Ok(())
}
```

| 方向 | 端口 | 类型 |
|---|---|---|
| 输入 | clk / rst | Clock / Reset |
| 输入 | flush / input_valid / output_ready | UInt1 |
| 输入 | input_data | UIntWIDTH |
| 输出 | input_ready / output_valid | UInt1 |
| 输出 | output_data | UIntWIDTH |

使用前须施加 rst=1 的有效同步上升沿。正常上升沿前 valid && ready 才计一次传输；空态没有直通，入队最早下一周期可输出。满态 input_ready=0，即使同拍出队也拒绝入队，下一周期恢复空间。非空非满可同拍进出；DEPTH=1 不存在该状态，不承诺每拍吞吐。受阻时保持 output_valid 和有效 payload。

同步高有效 reset 优先于 flush，二者清占用并取消全部在途事务，取消沿的表面握手不计成功。已接收 = 已输出 + 已取消 + 驻留，最终排空后驻留为零。无效 payload 不作功能承诺，flush 不保证清零存储。生产者受阻时也应保持 valid/data，直至接收或取消。

| 支持面 | 范围 |
|---|---|
| 存储、时钟 | 单时钟寄存器存储；不引入 Mem，不承诺 BRAM、异步 FIFO 或 PPA |
| 单模块执行 | Interpreter、Compiled 和实际生成 Verilog→Icarus→vvp |
| 实例组合 | 同 session、空实例参数；真实层级 RTL；native/generated 层级仍明确 unsupported |
| 行为矩阵 | WIDTH 1/8/32/64 × DEPTH 1/2/3/4/7/16；3 固定种子、独立端口队列 |
| RTL 安全与 cover | WIDTH1 × DEPTH1/2/3；初始化 reset 与合法生产者协议，不假定最终 ready |
| 原始 RTL 综合 | 1×1、8×3、64×16；严格无 latch/多驱动检查及 cell 记录，不是面积/时序结果 |
| 旧接口 | `SyncFifo` 的 8×4 RAM、注册 dout、读延迟、reset 不变 |

测试前置：本仓库钉死的 Rust 工具链，以及 PATH 中可执行的 GNU `timeout`（支持 `--kill-after`，通常来自 GNU coreutils）、Icarus Verilog 的 `iverilog`/`vvp`；专用形式/综合入口还需要 SymbiYosys `sby`、Yosys、Z3。Ubuntu 可安装 `coreutils iverilog`；SBY/Yosys/Z3 复用仓库的 `bash scripts/ci-install-sby.sh`（安装时需要 sudo 和网络，读取 `scripts/ci-sby-pins.env`，保留现有钉）。若已有工具，脚本会跳过安装，实际版本仍以测试产物中的探针日志为准。CI 使用同一路径，不依赖本地临时工具目录。

验证入口如下，工具缺失、UNKNOWN、超时或失败都不视为通过：

```sh
cargo test -p bitloom --test fr195_param_sync_fifo -- --nocapture
cargo test -p bitloom --test fr195_param_sync_fifo_formal -- --ignored --nocapture
python3 scripts/check_fr194_example.py
```

三个种子为 `0x12641950a551`、`0xdeadbeef8012`、`0x73592401ffff`，每组先执行原有420拍（末20拍排空），再追加 reset-only、flush-only、reset/flush 重叠三类取消×空/部分/满占用的定向序列；DEPTH1不存在部分占用。每次取消同时提出输入valid和输出ready，明确覆盖非空flush取消表面输出握手，并逐类记录实际占用命中。最后重新入队再以20拍排空，保持独立队列、生产者协议和取消记账。深度1/2/3/4/7/16的总拍数分别为450/459/462/468/480/522。定向命中和随机扰动共同检查边界，而非只凭随机概率。专用 formal 在默认 workspace 中 ignored，必须显式运行。证据索引见 `_agile-output/implementation-artifacts/epic-126-closeout.md`；该索引记录完整七步验收，Epic126/M1 已关闭。M1 不代表整个 Phase24 完成，FR189 仍 deferred、NFR91 未清空。

本故事将类型、define_module 与 Elaboratable 入口显式登记至 [FR142 表面](../public-api-1-0-surface.md)，新增 API 按 SemVer minor；未修改版本号、未发布。
