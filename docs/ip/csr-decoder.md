# 四窗口 CSR 译码器（FR196 / Story127.4）

Bitloom 与 [samitbasu/rhdl](https://github.com/samitbasu/rhdl) 无关。设计 crate 仅依赖 `bitloom-prelude`。`ip::CsrDecoder` 是固定单时钟、同步高有效 reset 的 addr16/data32/WSTRB4 译码器；没有构造参数、可编程窗口、额外队列或多主功能。

## 入口和完整端口

`CsrDecoder::elaborate()`（导入 `Elaboratable`）返回 `Result<FrozenHir, Diagnostics>`，独立电路/模块名为 `CsrDecoder`。`CsrDecoder::define_module(&mut ElaborateSession, impl Into<String>) -> Result<String, Diagnostics>` 返回已注册的模块名。两入口共用一个无捕获定义体；共享入口不 finish。相同完整定义复用，非法名/同名冲突会返回 Diagnostics 并使 session 无效；方向、宽度、缺端口等由已有 freeze 检查拒绝。先定义全部模块，再开始顶层例化，最后一次 finish，不拼接 FrozenHir。

准确52端口：公共12个，每叶10个。除 clk/rst 外均 UInt，宽度单位 bit。

| 方向 | 名称与宽度 |
|---|---|
| 输入 | `clk:Clock`、`rst:Reset` |
| 输入 | `req_valid:1`、`write:1`、`addr:16`、`wdata:32`、`wstrb:4`、`rsp_ready:1` |
| 输出 | `req_ready:1`、`rsp_valid:1`、`rdata:32`、`error:2` |
| 每叶输入 | `P_req_ready:1`、`P_rsp_valid:1`、`P_rdata:32`、`P_error:2` |
| 每叶输出 | `P_req_valid:1`、`P_write:1`、`P_addr:16`、`P_wdata:32`、`P_wstrb:4`、`P_rsp_ready:1` |

P 严格为 `uart`、`gpio`、`timer`、`irq`。上游与桥 `csr_*` 去前缀连接，下游与叶去 P 前缀连接。没有调试 owner 端口。

| 叶 | 全局字节窗口（含两端） | 局部地址 |
|---|---|---|
| uart | `0000–00ff` | 全局地址 − `0000` |
| gpio | `0100–01ff` | 全局地址 − `0100` |
| timer | `0200–02ff` | 全局地址 − `0200` |
| irq | `0300–03ff` | 全局地址 − `0300` |

完整16位先判定窗口，命中后才解释局部地址；局部以16位表达且保留低两位。例如 `0104→GPIO 0004`，`01ff→GPIO 00ff`，`8104` 不会别名。未选叶 payload 无效，无归零保证。窗内 hole/未对齐/权限错误由真实 `CsrBlock` 返回 SLVERR(10)，窗外 `0400–ffff`（包括未对齐）由译码器返回 DECERR(11)。成功00，合法 peer 不得返回保留01，失败读数据0，写响应无关数据不限。零 WSTRB 仍提交，由叶决定是否产生副作用。

## 提交、背压、复位和气泡

空闲且非 reset 时，上游 ready 等于命中叶 ready，唯一叶 valid 等于上游 valid；命中请求的上下游握手在**同一上升沿**提交，无先收后发缓存。等待期间合法上游必须保持 valid 和全部 payload（包括最终接受沿）。提交后锁住 owner，只有 owner 可接收 rsp_ready，其响应原样路由；上游地址/类别或其他叶变化不能切换旧响应。叶响应允许等待和背压。合法leaf只有在先前请求被接受的上升沿之后才可提出响应，不能在请求接受沿之前或同沿以组合响应代替；一旦提出，必须保持valid及payload直到消费或共同reset。

未命中在空闲时 ready=1，不向任何叶提出请求；提交后下一周期出现持久 DECERR/data0，直到消费。不能从当前地址组合生成错误响应。全局最多一笔已提交未消费请求，**响应消费沿不接收新请求**，下一周期才重新空闲。桥另有已文档化的捕获/选择/B/R槽气泡，因此不承诺每拍吞吐。桥外部五个 ready/valid 保持寄存边界；内部 CSR 允许组合路径。

首次使用前施加同步 reset 沿，桥、decoder、四叶及外部状态 owner 必须同域共同复位。rst=1 时 req_ready、全部下游 req_valid/rsp_ready 为0；上游 rsp_valid 不组合门控 rst，已有响应沿前可见，但 reset 沿仅计取消，清状态后消失，不能计消费或重放。aresetn 由外部控制器同步断言/释放后再反相；反相不是同步器，不支持只复位桥。

## Prelude-only 完整四叶连接例

下面采用正式寄存器偏移、权限和mask；UART/GPIO/Timer/IRQ只是CSR窗口。动态 value/event/reject 与提交输出暴露给调用者连接外部状态 owner，**不是 Epic128 外设算法**。一个 session 注册桥、译码与四个真实 CSR 模块，形成七模块 HIR。生成的四个C头继续使用局部offset；软件全局地址等于窗口base加局部offset，例如 `GPIO_BASE + GPIO_OUT_OFFSET = 0x0104`，不改变既有宏语义。

```rust
use bitloom_prelude::{Diagnostics, ElaborateSession, GroundType, Span};
use bitloom_prelude::ip::{AxiLiteCsrBridge, CsrDecoder, CsrBlock, CsrRegister, CsrField, CsrAccess, CsrOwner};
const LEAVES: [&str;4]=["uart","gpio","timer","irq"];
fn banks() -> Vec<CsrBlock> {
    use CsrAccess::*;
    let layouts: Vec<Vec<(&str, u32, CsrAccess, u32)>> = vec![
        vec![
            ("ctrl", 0, Rw, 1),
            ("baud_div", 4, Rw, 0xffffffff),
            ("status", 8, Ro, 15),
            ("tx_data", 12, Wo, 255),
            ("rx_data", 16, Ro, 255),
            ("EVENT", 20, W1c, 15),
        ],
        vec![
            ("dir", 0, Rw, 0xffffffff),
            ("out", 4, Rw, 0xffffffff),
            ("in", 8, Ro, 0xffffffff),
            ("set", 12, Wo, 0xffffffff),
            ("clear", 16, Wo, 0xffffffff),
            ("rise_event", 20, W1c, 0xffffffff),
        ],
        vec![
            ("ctrl", 0, Rw, 3),
            ("count", 4, Rw, 0xffffffff),
            ("compare", 8, Rw, 0xffffffff),
            ("EVENT", 12, W1c, 1),
        ],
        vec![
            ("pending", 0, W1c, 31),
            ("enable", 4, Rw, 31),
            ("test", 8, Wo, 31),
            ("raw", 12, Ro, 31),
        ],
    ];
    layouts
        .into_iter()
        .enumerate()
        .map(|(j, layout)| CsrBlock {
            name: LEAVES[j].into(),
            registers: layout
                .into_iter()
                .map(|(name, offset, access, mask)| CsrRegister {
                    name: name.into(),
                    offset,
                    reset: 0,
                    access,
                    owner: match access {
                        Wo => CsrOwner::None,
                        W1c => CsrOwner::Leaf,
                        _ => CsrOwner::External,
                    },
                    event: if access == W1c {
                        Some(format!("{name}_events"))
                    } else {
                        None
                    },
                    read_reject: access == Ro,
                    write_reject: access == Wo,
                    fields: vec![CsrField {
                        name: "bits".into(),
                        mask: mask as u64,
                        reset: 0,
                        access,
                    }],
                })
                .collect(),
        })
        .collect()
}

fn main() -> Result<(), Diagnostics> {
    let blocks=banks();
    let mut s=ElaborateSession::new("CsrSystem");
    AxiLiteCsrBridge::define_module(&mut s,"Bridge")?;
    CsrDecoder::define_module(&mut s,"Decoder")?;
    for (j,b) in blocks.iter().enumerate() { b.define_module(&mut s,format!("Leaf{j}"))?; }
    let sp=Span::default();s.begin_module("CsrSystem",sp);
    s.add_input("clk",GroundType::Clock,sp);s.add_input("rst",GroundType::Reset,sp);
    let clock=vec![("clk".to_string(),"clk".to_string()),("rst".into(),"rst".into())];
    let mut bridge=clock.clone();let mut decoder=clock.clone();
    for (name,width,input) in [
        ("s_axi_awaddr",16,true),("s_axi_awprot",3,true),("s_axi_awvalid",1,true),
        ("s_axi_wdata",32,true),("s_axi_wstrb",4,true),("s_axi_wvalid",1,true),
        ("s_axi_bready",1,true),("s_axi_araddr",16,true),("s_axi_arprot",3,true),
        ("s_axi_arvalid",1,true),("s_axi_rready",1,true),
        ("s_axi_awready",1,false),("s_axi_wready",1,false),("s_axi_bvalid",1,false),
        ("s_axi_bresp",2,false),("s_axi_arready",1,false),("s_axi_rvalid",1,false),
        ("s_axi_rdata",32,false),("s_axi_rresp",2,false),
    ] {
        if input {s.add_input(name,GroundType::UInt{width},sp);}
        else {s.add_output(name,GroundType::UInt{width},sp);}
        bridge.push((name.into(),name.into()));
    }
    let bus=[("req_valid",1),("write",1),("addr",16),("wdata",32),("wstrb",4),
             ("rsp_ready",1),("req_ready",1),("rsp_valid",1),("rdata",32),("error",2)];
    for (name,width) in bus {
        let wire=format!("csr_{name}");s.declare_wire(&wire,GroundType::UInt{width},sp);
        bridge.push((wire.clone(),wire.clone()));decoder.push((name.into(),wire));
    }
    for (j,b) in blocks.iter().enumerate() {
        let mut leaf=clock.clone();
        for (name,width) in bus {
            let wire=format!("{}_{name}",b.name);s.declare_wire(&wire,GroundType::UInt{width},sp);
            decoder.push((wire.clone(),wire.clone()));leaf.push((name.into(),wire));
        }
        // Side ports remain visible to the external owner/test peer.
        for r in &b.registers {
            let mut ports=vec![(format!("{}_read_commit",r.name),1,false),
                               (format!("{}_write_commit",r.name),1,false)];
            if r.access!=CsrAccess::Wo {ports.push((format!("{}_value",r.name),32,r.owner==CsrOwner::External));}
            if r.access!=CsrAccess::Ro {
                ports.push((format!("{}_candidate",r.name),32,false));
                ports.push((format!("{}_write_mask",r.name),32,false));
            }
            if r.read_reject {ports.push((format!("{}_read_reject",r.name),1,true));}
            if r.write_reject {ports.push((format!("{}_write_reject",r.name),1,true));}
            if let Some(event)=&r.event {ports.push((event.clone(),32,true));}
            for(name,width,input)in ports {
                let external=format!("{}_{name}",b.name);
                if input{s.add_input(&external,GroundType::UInt{width},sp);}
                else{s.add_output(&external,GroundType::UInt{width},sp);}
                leaf.push((name,external));
            }
        }
        s.add_instance(&b.name,format!("Leaf{j}"),leaf,vec![],sp);
        // Generation is deterministic and local; software adds these bases.
        let base=[0x0000u32,0x0100,0x0200,0x0300][j];
        assert_eq!(base+4,[4,0x104,0x204,0x304][j]);
        assert!(b.emit_c_header()?.contains("_OFFSET"));
        assert!(!b.emit_markdown()?.is_empty());
    }
    s.add_instance("bridge","Bridge",bridge,vec![],sp);
    s.add_instance("decoder","Decoder",decoder,vec![],sp);
    s.end_module();let hir=s.finish()?;
    assert_eq!(hir.circuit().modules.len(),7);
    println!("{}: 7 modules",hir.circuit().name);
    Ok(())
}
```

原文编译执行：`python3 scripts/check_fr196_decoder_example.py`。该命令验证设计 elaboration，不把 Rust 生成器运行当成硬件行为测试。四头文件共同include的C11消费者、完整局部mask/offset和全局base独立黄金由集成测试执行。

## 验证与支持矩阵

```text
cargo test -p bitloom --test fr196_csr_decoder --test fr196_csr_decoder_integration --test fr196_csr_decoder_formal -- --nocapture
cargo test -p bitloom --test fr196_csr_decoder_formal -- --ignored --nocapture
cargo test -p bitloom --test fr196_csr_decoder_automate --test fr196_csr_decoder_oracle -- --nocapture
```

| 路径 | 验收与边界 |
|---|---|
| 单decoder native | Interpreter/Compiled，两引擎各65536地址和owner/miss/背压/reset |
| 真实RTL层级 | 同session桥+decoder+四个CsrBlock；独立软件记账、正式权限/mask、16seed×1000完成事务，另每seed128写+128读独立通道并发 |
| 形式安全 | 初始reset、合法producer与因果保持leaf；不假设最终ready或叶等待上界，错误写无关rdata不限；辅助对应均assert；depth8基例+归纳，cover另列 |
| 形式敏感性 | 不含内部状态对应引理的port-only BMC depth10；control与串窗、双选、高位alias、ready门控valid、背压时miss丢失、reset残留pending共六种突变 |
| 延迟响应跨后端 | 4固定seed×128完成、各9取消，两native与实际单模块RTL逐周期对照独立oracle；合法长延迟leaf与请求/响应背压 |
| WO观察器敏感性 | UART TX_DATA、GPIO SET/CLEAR、IRQ TEST各有原版control及candidate/write_mask实际RTL突变，共8个目标断言反例 |
| 综合 | 原始decoder与七模块组合Yosys synth/check，无observer；拒绝latch/未知cell，所有FF须专用clk；桥五握手输出无输入组合依赖 |
| native/generated层级 | 仍unsupported，不提供隐式flatten仿真；独立generated Rust decoder本故事未声明验证 |

随机矩阵记录每seed、各窗/错误/WSTRB、AW/W间隔与先后、墙钟及退出码；它是有限验收，不是全协议穷举。观察器另实跑非法producer/同沿consume-refill负例。共同reset的取消接受、提交、CSR响应、B/R消费分别记账；排空后继续观察重复。独立decoder验证无界等待安全，产品仍面向有限响应片上CSR，不扩展到任意外部设备、超时恢复、商业VIP或PPA承诺。

CI显式运行专用ignored证明/综合及文档例，缺工具、UNKNOWN和timeout不是PASS。实际日志、保留失败与归档索引见[本轮build证据](../../_agile-output/test-artifacts/127-4-build-evidence.md)；[M2映射](../../_agile-output/implementation-artifacts/epic-127-closeout.md)已在完整七步验收后关闭，见[最终验收](../../_agile-output/test-artifacts/127-4-final-verification.md)。旧bank/CSR/桥保持；Epic128–130、FR197–201和整个Phase24仍未交付，FR189 deferred/NFR91保留。新增公开符号显式列入[FR142](../public-api-1-0-surface.md)，按SemVer minor，未改版本、未发布。
