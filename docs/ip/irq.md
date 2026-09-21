# 五路事件 IRQ

Bitloom `bitloom_prelude::ip::Irq` 提供固定五路同步事件、粘滞PENDING与软件ENABLE（Story128.3 / FR197 IRQ子集）。Bitloom与samitbasu/rhdl无关。设计crate只依赖`bitloom-prelude`。

`Irq::define_module(&mut ElaborateSession, impl Into<String>) -> Result<String, Diagnostics>` 在调用方session注册IRQ和共享CSR定义。相同名字/完整定义复用，冲突及端口宽向错误沿用现有诊断。结束全部模块后只调用一次`finish()`；`Elaboratable for Irq` 的 `elaborate() -> Result<FrozenHir, Diagnostics>` 使用同一模块体、独立顶名`Irq`。`Irq::registers() -> CsrBlock` 返回静态描述，修改返回值不修改固定IRQ硬件。

## 端口、事件与地址

14端口：输入`clk: Clock`、`rst: Reset`、`req_valid: UInt1`、`write: UInt1`、`addr: UInt16`、`wdata: UInt32`、`wstrb: UInt4`、`rsp_ready: UInt1`、`raw_events: UInt5`；输出`req_ready: UInt1`、`rsp_valid: UInt1`、`rdata: UInt32`、`error: UInt2`、`irq: UInt1`。

`raw_events` 按沿前值在上升沿采样：bit0 Timer match、bit1 UART成功RX到达、bit2 TX取走队首腾空间、bit3 overflow或framing、bit4 GPIO新上升沿聚合。输入是同域同步事件脉冲，不接外设粘滞EVENT。连续两拍高表示两拍事件，不增加沿检测；重复事件仅保持pending，不计数、不排队。没有第六源、外部异步IRQ针脚或内部CDC同步器。

| 局部字节地址 | 后续系统地址 | 名称 | 访问 | 唯一owner |
|---|---|---|---|---|
| 0x0000 | 0x0300 | PENDING | W1C | CSR leaf |
| 0x0004 | 0x0304 | ENABLE | RW | CSR leaf |
| 0x0008 | 0x0308 | TEST | WO | 无存储 |
| 0x000c | 0x030c | RAW | RO | 当前硬件事件 |

全mask0x1f/reset0；五字段名称为timer/uart_rx/uart_tx/uart_error/gpio。WSTRB小端逐字节选择，仅byte0含有效位；保留位读0写忽略。合法零有效mask写OKAY(00)且无软件作用；RO写（含WSTRB0）、WO读、洞/未对齐/高位地址返回SLVERR(10)，失败读0。IRQ不截断高地址或做四窗译码，直接送0x0300不别名本地0；系统窗外DECERR属于decoder。

## 提交、快照与竞争

`pending_next=(pending_old & ~clear) | hardware_events | committed_test_bits`，mask到五位。硬件set胜clear；mask关闭不阻止pending接收事件。TEST只在成功有效写提交沿按WSTRB所选写1位注入，闲置wdata、错误访问、背压不会触发软件事件。单端口无法同沿提交TEST和PENDING两写，不承诺不可达的双写。

`irq=|(pending & enable)`；ENABLE关闭不清pending，重新开启即可反映保留事件。RAW始终只含当前硬件事件，连TEST提交拍也不混软件位或sticky pending。读RAW在提交沿锁存硬件快照，读PENDING/ENABLE返回沿前状态；同沿新事件在后续读可见。

非reset上升沿`req_valid && req_ready`是唯一软件提交点，下一周期响应有效。背压保持valid/rdata/error，不接新提交，消费沿无refill；硬件事件在受阻期间仍采样。受阻请求的valid/payload须保持至提交或共同reset取消。安全性不假设ready最终为1，不承诺无条件活性。

同步高有效reset优先，清pending/enable与在途响应，不捕获reset沿事件；释放后新事件正常接收。上游/leaf/wrapper共同复位；异步reset须外部同步断言与释放，反相不是同步器。

## 原文例与软件产物

[irq-example.rs](irq-example.rs) 是仅依赖prelude的完整可编译设计，包含共享定义、独立入口与真实Timer→IRQ组合：只将`Timer.match_event`接bit0，其余四位0。运行参数为软件输出目录；同一描述生成[C头](irq-registers.h)与[地址表](irq-registers.md)，软件自行加系统base0x0300。返回描述的修改不会改变产品固定实现。

## 验证与范围

`fr197_irq` 的独立逐源参考运行三个固定seed，穷举32×32源/mask组合、全部16种WSTRB与三可写寄存器、定向竞争/快照/权限/共同reset并扩充随机。当前字节矩阵每seed320组：PENDING/ENABLE各80组，TEST从硬件播种的空pending与混合pending分别运行80组；通过读回检查注入与未选字节无作用，避免全1粘滞状态遮蔽错误。direct与专用FIRRTL/Chisel入口实际执行相同向量。 direct另有170帧检错控制：原始RTL通过，GPIO源别名与TEST遗漏字节选择两种编译成功的变异须触发指定读回断言；编译错误和超时不算检出。`fr197_irq_api` 实际运行同定义双实例、异名共享CSR双实例、Timer原始事件组合；Timer本地EVENT仍高时清IRQ不会重触发，重新产生真实match才置位。

`python3 _agile-output/test-artifacts/128-3-build-runner.py --only formal` 是带身份核验的形式复现入口（设置调用者PATH和BITLOOM_SBY_SOURCE）。它先核验固定SBY源码/安装16文件、拒绝未核验的包/独立字节码影子，再以隔离Python和全新缓存目录执行独立ghost安全证明、23个可达cover、三种DUT变异负控制与原始RTL综合。直接运行`cargo test -p bitloom --test fr197_irq_formal -- --ignored`会绕过这些身份检查，不能据此声称已核验工具链。Rust/Chisel/firtool产品钉与SBY源码/安装身份是强制条件；host Yosys/Z3版本仅实录，不由此runner强制锁定。状态对应是assert，无ready公平性假设。具体实测状态及原始证据见[build记录](../../_agile-output/test-artifacts/128-3-build-evidence.md)，未执行/ignored不计PASS。

独立入口含CSR实例，native Interpreter/Compiled和GeneratedFunctional层级明确unsupported；测试检查拒绝诊断，不增仅供测试的无实例路径。综合不是PPA或板级电气/亚稳态保证。逐符号FR142追加属SemVer minor；未修改工具钉、包版本或发布。仅IRQ子集，GPIO/UART、Epic128/M3、FR197整体与Phase24仍开放；FR189 deferred/NFR91保持。七步最终关闭由故事记录决定。


完整构建证据须由一次默认runner完成全部gate，并保持起止相关源码fingerprint一致；两exact过滤门检查指定测试确实执行并通过，零匹配不算成功。归档使用`python3 _agile-output/test-artifacts/128-3-build-archive.py --run <本次runner输出目录>`，可附目标tar路径和`--include-history`；自定义输出目录也包含在归档。归档核对该run的example-artifacts、关联prove/cover/control PASS与三负控制FAIL，以及当前相关源码；旧历史PASS不能替代当前运行。tar及同名manifest都禁止覆写。
