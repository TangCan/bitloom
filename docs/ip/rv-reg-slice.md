# 两槽 ready/valid 注册切片

Bitloom 与 samitbasu/rhdl 无关。设计 crate 只依赖 `bitloom-prelude`。

```rust
use bitloom_prelude::{Diagnostics, ElaborateSession, GroundType, Span, ip::RvRegSlice};
fn main() -> Result<(), Diagnostics> {
    let mut session = ElaborateSession::new("Top");
    let leaf = RvRegSlice::<8>::define_module(&mut session, "Slice8")?;
    let sp = Span::default();
    session.begin_module("Top", sp);
    session.add_input("clk", GroundType::Clock, sp);
    session.add_input("rst", GroundType::Reset, sp);
    for name in ["flush", "input_valid", "output_ready"] {
        session.add_input(name, GroundType::UInt { width: 1 }, sp);
    }
    session.add_input("input_data", GroundType::UInt { width: 8 }, sp);
    for name in ["input_ready", "output_valid"] {
        session.add_output(name, GroundType::UInt { width: 1 }, sp);
    }
    session.add_output("output_data", GroundType::UInt { width: 8 }, sp);
    let connections = ["clk", "rst", "flush", "input_valid", "input_data",
        "output_ready", "input_ready", "output_valid", "output_data"]
        .into_iter().map(|name| (name.to_owned(), name.to_owned())).collect();
    session.add_instance("slice", leaf, connections, vec![], sp);
    session.end_module();
    let hir = session.finish()?;
    assert_eq!(hir.circuit().modules.len(), 2);
    Ok(())
}
```

`RvRegSlice<const WIDTH: u32 = 32>` 支持 WIDTH=1..64；非法值返回 Diagnostics，不裁剪、不 panic。共享定义登记完整 WIDTH 参数；同名同参数可复用，同名不同参数失败。名称与实例约束见[模块组合](module-composition.md)。

| 方向 | 端口 | 类型 |
|---|---|---|
| 输入 | clk / rst | Clock / Reset |
| 输入 | flush / input_valid / output_ready | UInt1 |
| 输入 | input_data | UIntWIDTH |
| 输出 | input_ready / output_valid | UInt1 |
| 输出 | output_data | UIntWIDTH |

首次依赖 ready/valid 前，必须先经历 rst=1 的有效同步上升沿；上电未复位状态不作承诺。同步高有效 reset 优先于 flush，二者均清除占用并取消全部在途事务；该拍表面握手不计入 accepted 或 delivered。payload 无效时不得当作结果。通常仅在上升沿 valid && ready 成立时传输。受阻时输出 valid/payload 保持，直到实际出队或复位/flush 取消。

两个数据寄存器与占用寄存器隔离所有输入到输出的组合路径，包括 reset、flush 和下游 ready。空态无直通，新输入最早下一周期有效。满态 input_ready=0，即使同拍下游释放也不接受输入；下一拍恢复 ready。占用为一时可同时出入，连续上下游有效时稳态每拍一笔；不承诺所有状态零气泡。

验证入口（工具缺失或失败不得视为通过）：

```sh
cargo test -p bitloom --test fr195_rv_reg_slice -- --nocapture
cargo test -p bitloom --test fr195_rv_reg_slice_formal -- --ignored --nocapture
```

前者覆盖 Interpreter、Compiled 和实际 Icarus RTL 的 WIDTH=1/8/32/64、三个固定种子、取消守恒、完整排空、长背压及双实例隔离；后者专门执行真实 Yosys/SymbiYosys/Z3、安全证明、cover 与组合锥追踪。默认 workspace 测试的 ignored 不表示形式证明通过。证明范围仅安全性，不声明无限活性、PPA或异步/多时钟支持。native 层级仍明确 unsupported。

本 API 是 FR195 的注册切片子集；参数 FIFO 属 Story126.4，M1 与整个 Phase24 尚未关闭。新增公开接口按 SemVer minor 发布，本次未改包版本、未发布。
