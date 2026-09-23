# 模块组合（FR194 / Story126.2）

**当前状态（2026-09-23）：** Epic126/M1（FR194/FR195）及Phase24批准范围已交付，见[总验收](phase24-closeout.md)和[复盘与维护结论](../../_agile-output/implementation-artifacts/phase24-retro-2026-09-23.md)。下文“本故事”指Story126.2当时的交付范围；后续注册切片、FIFO、CSR和外设交付以总验收为准。复盘发现的外部缓存问题已[修复验证](../../_agile-output/test-artifacts/external-cache-integrity-verification.md)，不是未修复缺陷。FR189仍deferred，NFR91保留；结项不代表已发布或完成物理签核。

设计只依赖 `bitloom-prelude`。在同一个 `ElaborateSession` 中定义顶层与子模块，调用 `add_instance` 连接，最后调用一次 `finish`。不拼接已冻结电路，不引入另一套 IR。首批已有 IP 组合入口为 `Gpio`；其旧 `Elaboratable::elaborate` 与组合入口共用定义体，端口、8位数据宽度及同步复位行为保持。

## 模块身份与参数

模块名由设计者显式选择，helper接受 `[A-Za-z_][A-Za-z0-9_]*` 且排除HDL保留字；不提供转义标识符。参数键值按键排序后参与身份判断，键不能重复。同名、同参数、相同定义内容只保留一个模块定义；重复请求仍校验完整模块内容（包括 `Span`），不能靠名字或函数地址直接跳过检查。相同逻辑若使用不同 `Span`，也会保守地视为不同定义；需要复用时由同一函数使用一致的源码位置。相同配置可以使用不同名字分别定义。不同配置必须使用不同名字，例如 `Register8` 和 `Register16`；同名不同配置或不同定义返回 `Diagnostics`。

参数在展开时构造专门化模块，不是 Verilog 实例参数覆盖。现有 `add_instance` 的 `params` 字段不能作为后端参数化支持承诺；本例始终传空列表。回调使用非捕获函数指针，参数显式传入；它只定义模块体，不能开始、结束或嵌套定义模块。回调对象不进入冻结 HIR 或运行时 tick。

## 定义顺序与连接约束

模块定义 helper 只能在没有活动模块时调用。先结束顶层，再定义它引用的子模块也合法；顶层名应与电路名相同。旧调用若没有同名顶层，仅在实例图有唯一根时允许回退；多根必须明确顶层。尚未结束模块时再次开始模块，或直接 `finish`，均报错，不静默丢弃定义。

冻结前检查全部模块，而不只检查顶层可达部分：重复模块、重复端口或实例名、实例名与局部网名冲突、未知模块、循环实例图、重复或未知端口连接、未连接输入、未知父网、宽度/类型错误和多驱动均拒绝。显式 dangling 输入保留既有用途。

子模块输出只能连接父模块可驱动的 wire 或输出端口，不能驱动父输入或寄存器。子模块输入可以读取父输入、输出、wire 或寄存器。时钟和复位必须使用对应类型；`Bool` 与 `UInt<1>` 可以互连；其他不同类型不能仅凭等宽混用。

## 支持与验收边界

| 路径 | 本故事范围 |
|---|---|
| 同一 session 定义与实例连接 | 新增复用和冲突诊断，最终一次冻结 |
| 生成 Verilog、Icarus 编译与 vvp 执行 | 用独立参考检查8/8/16位实例及双GPIO隔离 |
| 旧 Gpio 单模块 elaborate | 共用定义体，保留旧API和行为 |
| native / GeneratedFunctional 层级执行 | 仍明确拒绝，不以扁平名称模拟层级 |
| 新 FIFO、注册切片、CSR及完整外设系统 | 后续故事，本故事不交付 |
| 综合、PPA、形式证明 | 本故事不据RTL回归宣称这些结果 |

本地验证需要 `iverilog` 和 `vvp` 位于 PATH。CI使用 `.github/workflows/ci.yml` 的系统包安装步骤；本机临时wrapper路径不是产品依赖。设置 `BITLOOM_REQUIRE_RTL=1` 时缺少工具必须失败。每个工具进程有60秒上限；实际命令、工具版本与结果见本故事证据。

新增设计API属于显式追加的 SemVer minor 表面，见[公开 API 清单](../public-api-1-0-surface.md)。本次不改变 crate 版本，不代表已经发布。**历史时点（Story126.2）：** 当时FR194完成尚未关闭M1，126.3/126.4及FR195还待交付；后续完成情况见页首当前状态。FR189仍deferred，NFR91未清空。

## API 与完整设计示例

新增两个入口（均通过 `bitloom-prelude` 使用）：

```rust,ignore
ElaborateSession::define_module(
    &mut self,
    name: impl Into<String>,
    params: Vec<(String, u32)>,
    body: fn(&mut ElaborateSession, &[(String, u32)]) -> Result<(), Diagnostics>,
) -> Result<String, Diagnostics>

Gpio::define_module(
    session: &mut ElaborateSession,
    name: impl Into<String>,
) -> Result<String, Diagnostics>
```

返回值是可传给 `add_instance` 的模块名。回调收到按键排序的参数。参数名称、允许值与缺省值由具体定义函数负责检查并以 `Diagnostics` 报错；通用helper不替具体IP猜测参数含义。任何定义错误都会记录到 session，即使调用者忽略即时错误，最终 `finish` 仍失败。

`python3 scripts/check_fr194_example.py` 会提取下方示例并离线编译、执行，已接入CI；先运行workspace构建以准备依赖缓存。

下面是完整 `src/main.rs`；Cargo设计依赖只有 `bitloom-prelude`。开发本仓库时使用对应本地路径；新增接口尚未发布，不能假定既有已发布版本已有该接口。

```rust
use bitloom_prelude::{Diagnostics, ElaborateSession, GroundType, Span};
use bitloom_prelude::ip::Gpio;

fn main() -> Result<(), Diagnostics> {
    let mut s = ElaborateSession::new("TwoGpio");
    let gpio = Gpio::define_module(&mut s, "Gpio8")?;
    // 重复请求核验后复用，不向HIR插入第二份定义。
    assert_eq!(gpio, Gpio::define_module(&mut s, "Gpio8")?);
    let span = Span::default();
    s.begin_module("TwoGpio", span);
    s.add_input("clk", GroundType::Clock, span);
    s.add_input("rst", GroundType::Reset, span);
    for lane in 0..2 {
        let mut connects = vec![
            ("clk".to_owned(), "clk".to_owned()),
            ("rst".to_owned(), "rst".to_owned()),
        ];
        for (port, width) in [
            ("dir", 8), ("wr_en", 1), ("wr_data", 8),
            ("wr_mask", 8), ("pad_in", 8),
        ] {
            let net = format!("g{lane}_{port}");
            s.add_input(net.clone(), GroundType::UInt { width }, span);
            connects.push((port.to_owned(), net));
        }
        for port in ["pad_out", "rd_data"] {
            let net = format!("g{lane}_{port}");
            s.add_output(net.clone(), GroundType::UInt { width: 8 }, span);
            connects.push((port.to_owned(), net));
        }
        s.add_instance(format!("gpio{lane}"), gpio.clone(), connects, vec![], span);
    }
    s.end_module();
    let frozen = s.finish()?;
    assert_eq!(frozen.circuit().modules.len(), 2); // 顶层 + 复用的GPIO定义
    Ok(())
}
```

两个实例各有自己的寄存状态，内部都可以沿用 `out_r` 等局部名字。顶层信号的 `g0_`/`g1_` 前缀只用于区分外部连接，不替代真实模块层级。

## 常见诊断

| 诊断 | 含义 |
|---|---|
| `E0240` | 活动模块生命周期错误 |
| `E0241` / `E0242` | helper模块名非法或保留字 / 参数键重复 |
| `E0243` / `E0244` | 定义回调失败或违规 / 同名定义冲突 |
| `E0002` / `E0255` | 无法唯一确定顶层 / 实例图存在环 |
| `E0250`–`E0254` | 重复模块、端口/局部网、实例名、连接，或未知子端口 |
| `E0201`–`E0204` | 缺模块、未驱动输入、宽度不匹配、未知父网 |
| `E0256` / `E0257` / `E0140` | 类型不匹配 / 输出目标非法 / 多驱动 |

完整诊断代码保留既有 `rhdl::` 前缀，不改变公开产品名 Bitloom。验证命令与实测结果见[本故事证据](module-composition-evidence.md)。

既有实例名和局部网名仍由调用者保证为合法HDL标识符，当前未统一提供转义/合法性诊断；本故事只为新helper的模块名增加此检查。该旧路径缺口与实例参数覆盖限制均记录在延后工作清单，不将它们算作本故事交付。
