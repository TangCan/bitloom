# 静态 CSR 叶节点（FR196 / Story127.2）

Bitloom 与 [samitbasu/rhdl](https://github.com/samitbasu/rhdl) 无关。设计 crate 只依赖 `bitloom-prelude`。一份 `CsrBlock` 静态描述生成同源 RTL、Markdown 地址表与 C 头；配置在 elaboration 时消解，不是 tick 内的 Rust 对象。此入口只包含 CSR 叶节点；AXI 桥、四窗译码与外设产品分别由后续故事验收，FR196/M2 和整个 Phase24 尚未关闭。

## 完整设计例

以下独立程序只依赖 `bitloom-prelude`，展示五种访问/owner组合、独立生成与同 session 复用。`name` 是软件命名空间，`define_module`/`elaborate` 的参数是独立 RTL 模块名。集成系统先定义全部叶节点，再由调用者声明顶层与实例连线，最后调用一次 `finish`，见[模块组合](module-composition.md)。

```rust
use bitloom_prelude::{Diagnostics, ElaborateSession};
use bitloom_prelude::ip::{CsrAccess, CsrBlock, CsrField, CsrOwner, CsrRegister};

fn main() -> Result<(), Diagnostics> {
    let block = CsrBlock {
        name: "Example".into(),
        registers: [
            ("control", 0, CsrAccess::Rw, CsrOwner::Leaf, 0xff, None, false, false),
            ("status", 4, CsrAccess::Ro, CsrOwner::External, 0xff, None, true, false),
            ("tx", 8, CsrAccess::Wo, CsrOwner::None, 0xff, None, false, true),
            ("events", 12, CsrAccess::W1c, CsrOwner::Leaf, 0x8000000f,
                Some("hw_events"), false, false),
            ("counter", 16, CsrAccess::Rw, CsrOwner::External, 0xffffffff,
                None, false, true),
        ].into_iter().map(|(name, offset, access, owner, mask, event, read_reject, write_reject)| {
            CsrRegister {
                name: name.into(), offset, reset: 0, access, owner,
                event: event.map(str::to_owned), read_reject, write_reject,
                fields: vec![CsrField { name: "bits".into(), mask, reset: 0, access }],
            }
        }).collect(),
    };
    block.validate()?;
    let standalone = block.elaborate("ExampleCsr")?;
    let mut session = ElaborateSession::new("ExampleCsr");
    assert_eq!(block.define_module(&mut session, "ExampleCsr")?, "ExampleCsr");
    block.define_module(&mut session, "ExampleCsr")?; // identical specialization reuses
    let shared = session.finish()?;
    assert_eq!(standalone, shared);
    assert_eq!(shared.circuit().modules.len(), 1);
    let header = block.emit_c_header()?;
    assert!(header.contains("#define EXAMPLE_EVENTS_MASK UINT32_C(0x8000000f)"));
    assert!(header.contains("#define EXAMPLE_COUNTER_OFFSET UINT32_C(0x00000010)"));
    println!("{header}");
    println!("{}", block.emit_markdown()?);
    Ok(())
}
```

验证文档原文：`python3 scripts/check_fr196_example.py`。生产设计不依赖 CLI/sim/vlog；工具层在设计返回 FrozenHir 后负责 emit/仿真。

## 配置、诊断与确定性

固定 data32、addr16（local byte address）、WSTRB4、小端、四字节对齐、同步高有效 reset0。offset 为 host `u32`，合法范围 `0..=0xfffc`；field mask 为 host `u64`，合法非零32位值。这些较宽 host 类型用于报告越界错误，不会静默截断。块/寄存器字段列表非空，同一寄存器全部字段必须与寄存器访问类型一致；mask不重叠、reset均0且字段reset并集等于寄存器reset。没有混合字段权限、隐式地址别名或自动重命名。

| Access | Owner | 状态/事件 |
|---|---|---|
| RW | Leaf | leaf 保存状态 |
| RW | External | 同域 peer 唯一保存状态，leaf没有副本 |
| RO | External | peer 提供当前状态 |
| WO | None | 无存储，成功写产生提交脉冲 |
| W1C | Leaf | leaf保存状态；必须提供唯一合法 `event` 输入名 |

其他owner组合均拒绝；非W1C不得设置event。`read_reject`仅用于可读寄存器，`write_reject`仅用于可写寄存器。标识符为ASCII字母起始，后续字母/数字/单下划线；拒绝双下划线、HDL/C保留字、重复寄存器/字段名、大写宏碰撞以及事件/生成端口碰撞。私有RTL名以`_csr_`起始，公共名称不可能进入该空间。

所有入口首先用相同 `validate` 返回 `Diagnostics`；非法配置不会向session留下半模块。进入builder helper后发生的错误仍按既有契约poison session：同模块名不同完整配置报E0244，最终finish失败，不能继续复用该session。相同配置重排按offset、字段mask/name规范化；名字、字段划分、mask、owner、事件、reject开关全部进入完整参数身份，不以摘要替代。私有schema codec不属于公开API。emit不含时间、路径或随机数据。

## 端口及提交时序

| 方向 | 固定端口 |
|---|---|
| 输入 | `clk:Clock`, `rst:Reset`, `req_valid:1`, `write:1`, `addr:16`, `wdata:32`, `wstrb:4`, `rsp_ready:1` |
| 输出 | `req_ready:1`, `rsp_valid:1`, `rdata:32`, `error:2` |

下面R为寄存器原名，E为配置event原名；未列出的端口不存在。

| 条件 | 输入 | 输出 |
|---|---|---|
| RW External / RO | `R_value:32` | — |
| RW Leaf / W1C Leaf | — | `R_value:32` |
| 全部访问类型 | — | `R_read_commit:1`, `R_write_commit:1` |
| RW / WO / W1C | — | `R_candidate:32`, `R_write_mask:32` |
| read_reject | `R_read_reject:1` | — |
| write_reject | `R_write_reject:1` | — |
| W1C | `E:32` | — |

第一次使用前施加reset沿。非reset上升沿 `req_valid && req_ready` 是唯一提交点；同沿锁存提交前读快照/error，下一周期rsp_valid可见。仅一个响应槽；pending时req_ready恒0，响应消费沿不接新请求。背压期间rsp_valid/rdata/error保持。reset沿优先取消响应、清leaf状态，rst时req_ready和commit脉冲均0；响应valid是寄存状态，在reset沿后清零。peer必须同域共同复位。读值在提交沿采样，之后peer或事件变化不改变已排队响应。

`R_read_commit`/`R_write_commit` 是**沿前组合脉冲**，peer必须在同一个上升沿采样；不是响应拍pulse。输入改变后先settle再采样脉冲/tick。地址/权限合法且动态允许才产生pulse；write另外要求有效mask非零，不可用权限的pulse恒0。连续req_valid不能在背压期间重放。失败访问没有访问副作用；自然硬件事件仍可更新W1C。

`R_write_mask = expand(WSTRB) & valid_mask`；每个选中字节为0xff。candidate和mask不依赖地址、valid、write、reject、commit、reset，因此peer可用candidate与提交前状态组合产生reject而不构成反馈环：

- RW：`((current & ~write_mask) | (wdata & write_mask)) & valid_mask`。
- WO：`wdata & write_mask`；可以给FIFO push使用。
- W1C：`wdata & write_mask`（clear请求，不含event）；成功提交后 `next = (old & ~clear) | (event & valid_mask)`，硬件set胜clear。

RO/external current先屏蔽保留位，leaf输出状态也只有有效位。external RW写在peer处消费candidate/commit；有自主更新时，peer按自己的单个next-state方程决定软件写与自主更新优先级。leaf没有第二份存储。

合法RW/WO/W1C零有效mask写返回OKAY，忽略动态write_reject且不产生pulse；非零mask写入0仍产生pulse。低8位WO若未选byte0不push。RO写（即使零WSTRB）、WO读、未定义/未对齐地址均SLVERR。动态拒绝在提交前判断，并产生有限延迟SLVERR；无无限等待。所有写响应以及失败读rdata为0；叶只产生00 OKAY/10 SLVERR，不产生01或系统DECERR。W1C读同拍新事件在下一次读取才可见。

## 软件产物与验证边界

C头是local offset，调用者提供物理base；无隐式全局地址。include guard为 `BITLOOM_<BLOCK>_CSR_H`，宏 `<BLOCK>_<REG>_{OFFSET,MASK,RESET,ACCESS}` 和 `<BLOCK>_<REG>_<FIELD>_{MASK,RESET,ACCESS}` 全部大写；数值用 `UINT32_C(0x........)`，访问为字符串。共同include的头文件必须具有互不重叠的**完整宏集合**；仅block名字不同不足以保证：block `A_B` 的寄存器 `C` 与block `A` 的寄存器 `B_C` 都生成 `A_B_C_MASK`。固定宏拼写保持不变；每个生成宏定义前有 `#ifdef`/`#error`，即使重定义值相同也明确拒绝碰撞。同一头重复include仍由include guard正常保护。不同内容也不得复用同一**大写归一化后**的block名字/include guard：`Probe`与`PROBE`同属禁止组合。include guard不检测这种内容复用；它只保留标准重复include行为。Markdown包含寄存器/字段mask/reset/access、owner/event及read/write动态拒绝开关。

下面C消费者使用与上面Rust设计同源的`example.h`。`OFFSET`是字节数；使用`uintptr_t`地址相加后再转指针，避免对`uint32_t *`直接加OFFSET造成四倍缩放。调用者提供已映射、对齐的base；校验入口只严格C11编译，不执行MMIO。

```c
#include <stdint.h>
#include "example.h"

uint32_t example_control_read(uintptr_t base) {
    return *(volatile uint32_t *)(base + (uintptr_t)EXAMPLE_CONTROL_OFFSET);
}
void example_control_write(uintptr_t base, uint32_t value) {
    *(volatile uint32_t *)(base + (uintptr_t)EXAMPLE_CONTROL_OFFSET) =
        value & EXAMPLE_CONTROL_MASK;
}
```

运行入口：

```text
cargo test -p bitloom --test fr196_csr_config --test fr196_csr -- --nocapture
cargo test -p bitloom-prelude csr
cargo test -p bitloom --test fr196_csr_formal -- --ignored --nocapture
python3 scripts/check_fr196_example.py
```

默认workspace覆盖native两引擎、Icarus真实叶/层级peer、C11单/双头编译；formal-sby job显式运行专用ignored测试，不把ignored计作通过。安全proof使用初始reset与协议输入稳定假设，无rsp_ready公平性；prove深度16的基例/归纳与cover深度24分列，五个cover是成功提交、错误、背压恢复、W1C冲突，以及待响应被reset取消后重新成功提交。另对不含observer的原始RTL运行Yosys综合/check，不能据此宣称PPA或板级签核。层级native/generated仍明确unsupported；真实层级仅以实际RTL验收。Generated Rust standalone在本故事未验证，不能用层级拒绝或两个native引擎的结果替代该后端验收。实际命令、版本、seed、失败与通过日志见[build证据](../../_agile-output/test-artifacts/127-2-build-evidence.md)。

新增API逐符号登记[FR142表面](../public-api-1-0-surface.md)，后续发布属SemVer minor；本故事不改版本、不发布。旧Axi4LiteSlave/SyncFifo/Gpio/M1行为保留。FR189/Epic122仍deferred，NFR91未清空。
