# AXI4-Lite 到 CSR 桥（FR196 / Story127.3）

Bitloom 与 [samitbasu/rhdl](https://github.com/samitbasu/rhdl) 无关。设计 crate 只依赖 `bitloom-prelude`。`ip::AxiLiteCsrBridge` 把独立的 AW/W/AR 通道转换成 [CSR 叶节点](csr.md)的单请求接口，B/R 各自保存响应。地址为16位字节地址，数据32位，WSTRB4；不支持burst、ID、多主、异步域或外部无限等待设备。四窗译码由Story127.4交付，当前不据此关闭FR196/M2。

## 入口与端口

`AxiLiteCsrBridge::elaborate()`（需导入 `Elaboratable`）返回独立 `FrozenHir`，电路与模块名均为 `AxiLiteCsrBridge`。`AxiLiteCsrBridge::define_module(&mut ElaborateSession, impl Into<String>) -> Result<String, Diagnostics>` 在调用者session中定义模块并返回模块名；相同完整定义可复用。两入口共用无捕获定义体。必须在顶层模块开始前完成各定义，最后只对设计session调用一次 `finish()`。非法模块名、定义冲突和连接错误沿用现有Diagnostics；helper失败会使session无效，不能忽略错误继续使用。

以下宽度单位为bit，除clk/rst外均为UInt；没有未列出的配置或可变宽度。

| 方向 | 端口 |
|---|---|
| 输入 | `clk:Clock`, `rst:Reset`（同步高有效） |
| 输入 | `s_axi_awaddr:16`, `s_axi_awprot:3`, `s_axi_awvalid:1` |
| 输入 | `s_axi_wdata:32`, `s_axi_wstrb:4`, `s_axi_wvalid:1`, `s_axi_bready:1` |
| 输入 | `s_axi_araddr:16`, `s_axi_arprot:3`, `s_axi_arvalid:1`, `s_axi_rready:1` |
| 输出 | `s_axi_awready:1`, `s_axi_wready:1`, `s_axi_bvalid:1`, `s_axi_bresp:2` |
| 输出 | `s_axi_arready:1`, `s_axi_rvalid:1`, `s_axi_rdata:32`, `s_axi_rresp:2` |
| CSR输出 | `csr_req_valid:1`, `csr_write:1`, `csr_addr:16`, `csr_wdata:32`, `csr_wstrb:4`, `csr_rsp_ready:1` |
| CSR输入 | `csr_req_ready:1`, `csr_rsp_valid:1`, `csr_rdata:32`, `csr_error:2` |

CSR端口连接leaf去掉 `csr_` 前缀的同名端口。AWPROT/ARPROT保留但忽略，不实现权限域。桥不对齐、不截断地址，不修改WSTRB；未对齐/空洞/权限错误由leaf返回SLVERR。DECERR可由后续decoder提供，桥原样路由00/10/11到对应响应；合规下游不得返回01，所有失败读须提供rdata0；写响应中的rdata不使用，错误写也不要求该字段为0。零WSTRB写仍提交并响应，leaf决定是否产生访问副作用。

## 捕获、提交与响应

| 阶段 | 容量与动作 |
|---|---|
| AXI捕获 | AW/W/AR各一槽；AW和W分别按接受顺序配对，未收齐写请求不阻塞读 |
| CSR请求提出 | 全局空闲且目标B/R槽可预留时选择；一旦valid，类别和全部payload保持到接受或reset |
| CSR提交 | 非reset上升沿req_valid且req_ready；消耗对应捕获槽，记录唯一执行owner |
| CSR响应消费 | rsp_valid且rsp_ready；按owner填B或R槽，释放全局CSR执行容量 |
| AXI响应消费 | 对应VALID且READY；仅释放该类响应槽 |

reset后同时合格先读，此后每次成功CSR提交优先另一类。捕获、提出请求及消费响应本身不改变优先。目标响应槽满的类别不参加仲裁；B受阻时仍可完成有空间的读，R受阻时仍可完成有空间的写。CSR响应已交给桥后，即使AXI主机未接收，另一类也可使用CSR执行槽。

外部AWREADY/WREADY/ARREADY/BVALID/RVALID仅依赖寄存状态，无输入组合路径，包括rst。当前实现采用保守气泡：满捕获槽不在消费沿补入新payload，CSR响应消费沿不同时提交新请求，AXI响应消费沿不直接补入同类响应。在空闲且目标响应槽有空间时，捕获沿之后一沿锁定选择，再下一沿最早提交CSR；若等待配对或下游ready则更晚。CSR响应消费后，下一沿才可选择新请求。无每拍吞吐承诺。CSR提交沿由leaf执行一次副作用并形成读快照，之后响应背压不重新执行读写。

B/R可无限期受阻，valid与payload须稳定。安全性不依赖主机最终ready；请求完成时间只有在明确下游响应与主机ready等待上界时才有界。首版只面向固定有限响应的片上CSR；不提供超时丢弃迟到响应机制。单CSR执行容量是此实现约束，不是AXI4-Lite协议本身的限制。

## 复位

首次使用前至少施加一个同步reset沿。reset优先捕获、提交和响应消费，清AW/W/AR、锁定请求、执行owner、响应和仲裁状态；该沿即使寄存ready/valid仍高，也按取消计数，不按传输计数。非reset时producer不得撤回未接受valid或改变受阻payload。内部CSR提交可用rst屏蔽，外部AXI五个ready/valid不组合门控rst。

桥和全部leaf必须同域共同reset。系统使用aresetn时，外部复位控制器须先把断言和释放都同步到ACLK，再反相为内部rst；反相不是同步器。此例直接接受满足约束的同步高有效rst，不构建板级同步器。不支持只复位桥、让外设继续执行；已经发出的UART物理位也不承诺回滚。

## 仅依赖 prelude 的完整组合例

例中一个RW寄存器位于0x0000，全32位可写，reset为0；通过 `control_value` 观察状态。其余地址由leaf返回SLVERR，没有四窗decoder。连线是同一session中的实例，没有拼接FrozenHir。

```rust
use bitloom_prelude::{Diagnostics, ElaborateSession, GroundType, Span};
use bitloom_prelude::ip::{AxiLiteCsrBridge, CsrAccess, CsrBlock, CsrField, CsrOwner, CsrRegister};

fn main() -> Result<(), Diagnostics> {
    let block = CsrBlock {
        name: "Device".into(),
        registers: vec![CsrRegister {
            name: "control".into(), offset: 0, reset: 0,
            access: CsrAccess::Rw, owner: CsrOwner::Leaf, event: None,
            read_reject: false, write_reject: false,
            fields: vec![CsrField {
                name: "bits".into(), mask: 0xffff_ffff, reset: 0, access: CsrAccess::Rw,
            }],
        }],
    };
    let mut s = ElaborateSession::new("ExampleAxiCsr");
    AxiLiteCsrBridge::define_module(&mut s, "Bridge")?;
    block.define_module(&mut s, "Leaf")?;
    let sp = Span::default();
    s.begin_module("ExampleAxiCsr", sp);
    s.add_input("clk", GroundType::Clock, sp);
    s.add_input("rst", GroundType::Reset, sp);
    let mut bridge = vec![("clk".to_string(), "clk".to_string()), ("rst".into(), "rst".into())];
    let mut leaf = bridge.clone();
    for (name, width, input) in [
        ("s_axi_awaddr",16,true), ("s_axi_awprot",3,true), ("s_axi_awvalid",1,true),
        ("s_axi_wdata",32,true), ("s_axi_wstrb",4,true), ("s_axi_wvalid",1,true),
        ("s_axi_bready",1,true), ("s_axi_araddr",16,true), ("s_axi_arprot",3,true),
        ("s_axi_arvalid",1,true), ("s_axi_rready",1,true),
        ("s_axi_awready",1,false), ("s_axi_wready",1,false), ("s_axi_bvalid",1,false),
        ("s_axi_bresp",2,false), ("s_axi_arready",1,false), ("s_axi_rvalid",1,false),
        ("s_axi_rdata",32,false), ("s_axi_rresp",2,false),
    ] {
        if input { s.add_input(name, GroundType::UInt { width }, sp); }
        else { s.add_output(name, GroundType::UInt { width }, sp); }
        bridge.push((name.into(), name.into()));
    }
    for (name, width) in [
        ("req_valid",1), ("write",1), ("addr",16), ("wdata",32), ("wstrb",4),
        ("rsp_ready",1), ("req_ready",1), ("rsp_valid",1), ("rdata",32), ("error",2),
    ] {
        let wire = format!("csr_{name}");
        s.declare_wire(&wire, GroundType::UInt { width }, sp);
        bridge.push((wire.clone(), wire.clone()));
        leaf.push((name.into(), wire));
    }
    s.add_output("control_value", GroundType::UInt { width: 32 }, sp);
    leaf.push(("control_value".into(), "control_value".into()));
    for (name, width) in [
        ("control_read_commit",1), ("control_write_commit",1),
        ("control_candidate",32), ("control_write_mask",32),
    ] {
        s.declare_wire(name, GroundType::UInt { width }, sp);
        leaf.push((name.into(), name.into()));
    }
    s.add_instance("bridge", "Bridge", bridge, vec![], sp);
    s.add_instance("leaf", "Leaf", leaf, vec![], sp);
    s.end_module();
    let hir = s.finish()?;
    assert_eq!(hir.circuit().modules.len(), 3);
    println!("{}: {} modules", hir.circuit().name, hir.circuit().modules.len());
    Ok(())
}
```

原文校验：`python3 scripts/check_fr196_bridge_example.py`。脚本创建仅有prelude依赖的临时设计crate并编译运行；它验证elaboration，不把运行Rust生成器当作硬件仿真。真实组合行为由以下RTL测试验收。

## 验证入口与支持边界

```text
cargo test -p bitloom --test fr196_axi_lite_csr --test fr196_axi_lite_csr_formal -- --nocapture
cargo test -p bitloom --test fr196_axi_lite_csr_formal -- --ignored --nocapture
```

普通入口包含独立事件记账、native两引擎、真实RTL重放、桥与真实CSR组合、双桥实例隔离及结构检测器负例；专用ignored入口负责实际sby prove/cover、原始RTL综合和五个输出的输入依赖锥；结构检测还核对每个FF连接专用clk，拒绝错误或缺失时钟。CI显式运行专用入口并保留失败产物，默认ignored不是证明通过。

随机预算16seed×1000事务按逐事务排空执行；写偏斜/WSTRB覆盖只按实际写计数。另有固定seed的独立AW/W/AR并发生产者测试，每种native引擎完成256写和256读，并将轨迹交给真实RTL重放；断言三输入及B/R均经历背压，不称随机穷举。安全证明的假设、深度、工具状态和实际结果以故事验证记录为准；测试入口存在不代表已经运行通过。native/generated层级仍unsupported，层级行为只以真实RTL验收；Generated Rust独立桥在本故事未验证。不宣称PPA、板级可靠性或商业VIP全协议认证。

审查修补后的安全证明使用初始reset和合法producer/因果CSR响应假设，已移除原先7拍响应上界；不假设CSR响应最终到达或BREADY/RREADY公平性，失败写响应的无关rdata也不加零值假设，因此安全结论不代表整体活性。独立端口观察器另断言：空闲且已有合格请求和响应容量时，下一沿必须提出offer。prove配置depth8，基例与归纳均PASS；这是归纳安全证明，不是仅8拍BMC。cover配置depth64，七个目标在第6、6、6、8、8、9、10步达到；其中新增两个目标分别见证SLVERR/DECERR写响应携带非零无关rdata后，该事务的正确B响应被消费。各目标证明场景可达，不代替安全结论。辅助状态对应关系均为assert并随整体证明通过，没有把DUT正确性改成assume。另以不带内部对应引理的端口观察器运行depth10变异检查：原版PASS，配对、响应路由和offer保持三种RTL突变均产生真实断言反例。独立审查专用运行见[形式日志](../../_agile-output/test-artifacts/127-3-independent-fix-formal.log)，新增cover与同一安全证明的重跑见[automate形式日志](../../_agile-output/test-artifacts/127-3-automate-formal.log)。最初depth32归纳失败/超时及原7拍假设版本保留在[初始build证据](../../_agile-output/test-artifacts/127-3-build-evidence.md)，不把历史快照当作当前证明配置。

旧`Axi4LiteSlave`的ADDR8、四寄存器和read-before-write行为保持；新桥按CSR提交次序串行访问，不继承旧bank的同拍一tick响应承诺。新增公开符号逐项列入[FR142](../public-api-1-0-surface.md)，属于SemVer minor追加；未改版本或发布，不代表crates.io已有。FR189/Epic122仍deferred，NFR91保留。
