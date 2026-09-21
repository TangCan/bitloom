# Timer32

Bitloom `bitloom_prelude::ip::Timer` 是固定32位、单时钟、可组合的CSR计时器（Story128.2 / FR197 Timer子集）。Bitloom与samitbasu/rhdl无关。设计crate只依赖`bitloom-prelude`。

`Timer::define_module(&mut ElaborateSession, impl Into<String>) -> Result<String, Diagnostics>` 在调用方session注册Timer和私有CSR定义；相同名字/完整定义复用，冲突沿用builder诊断。先结束所有模块定义，再一次`finish()`。`Elaboratable for Timer` 的 `elaborate() -> Result<FrozenHir, Diagnostics>` 使用相同实现，独立入口名为`Timer`。`Timer::registers() -> CsrBlock` 返回同源软件描述；修改返回的描述不改变固定Timer硬件。

## 端口与寄存器

13端口：输入`clk: Clock`、`rst: Reset`、`req_valid: UInt1`、`write: UInt1`、`addr: UInt16`、`wdata: UInt32`、`wstrb: UInt4`、`rsp_ready: UInt1`；输出`req_ready: UInt1`、`rsp_valid: UInt1`、`rdata: UInt32`、`error: UInt2`、`match_event: UInt1`。内部复位同步高有效；与上游bridge/decoder共同复位。异步reset须在外部同步，反相不等于同步。

| 局部字节地址 | 后续系统地址 | 寄存器 | 访问 / mask / 唯一owner |
|---|---|---|---|
| 0x0000 | 0x0200 | CTRL | RW / 3 / Timer；enable bit0、periodic bit1 |
| 0x0004 | 0x0204 | COUNT | RW / 0xffffffff / Timer |
| 0x0008 | 0x0208 | COMPARE | RW / 0xffffffff / CSR leaf |
| 0x000c | 0x020c | EVENT | W1C / 1 / CSR leaf |

全部reset0。WSTRB按小端逐字节合并，保留位读0写忽略。Timer只接局部地址，不截高位；直接送0x0200会SLVERR，不是CTRL。洞、未对齐、高位地址返回SLVERR(10)，失败读0；合法访问OKAY(00)。系统窗外DECERR由decoder负责。

## 沿与优先级

普通使能沿先计算`next_count=(COUNT+1) mod 2^32`，再与沿前COMPARE比较。命中时periodic使COUNT归0；one-shot保留递增后的COUNT、清enable且保留periodic位。COMPARE0仅自然回绕命中，关闭时保持。写COMPARE越过COUNT不立即触发。

成功且有效mask非零的CTRL/COUNT/COMPARE写，在提交沿抑制自然计数与match。COUNT采用合并候选值，其余此类写保持COUNT；相同值和写0也有效。零WSTRB，以及CTRL仅选择保留字节的非零WSTRB，不暂停计数。读、错误访问和EVENT写也不暂停。

`match_event` 是**沿前组合事件，在该上升沿采样**；供后续IRQ0连接，与粘滞EVENT不同。periodic COMPARE1可每沿为1，不插空拍；未清EVENT不阻止后续事件。`EVENT_next=(EVENT_old & ~clear)|match`，同拍set优先；clear只含所选字节的写1位。reset门控raw事件并最高优先清全部状态与响应。

非reset的`req_valid && req_ready`是唯一提交。读返回提交前状态快照，下一周期rsp_valid可见。背压保持valid/data/error，期间计数仍继续，不重复副作用；响应消费沿不接新请求。请求受阻须保持valid/payload，直到握手或共同reset取消。安全性不要求rsp_ready最终为1；未承诺无条件活性。

## 可编译原文例与软件产物

完整源码：[timer-example.rs](timer-example.rs)。将其作为仅依赖`bitloom-prelude`的设计crate的`src/main.rs`，运行时传入输出目录。独立临时crate在build中实际编译运行，未新增CLI依赖。同一描述生成并核对：[C头](timer-registers.h)、[地址表](timer-registers.md)；宏是局部offset，软件自行加系统base 0x0200。

## 验证与范围

`fr197_timer`以三个固定seed的独立黄金实际执行direct RTL；专用`--ignored`后端门运行同向量FIRRTL→firtool与Chisel→JVM→firtool→RTL。`fr197_timer_api`验证共享定义、错误连接及双实例独立状态。`fr197_timer_formal -- --ignored`分别执行归纳安全证明、独立cover和未插observer的原始综合。最新命令/工具/结果见[最终验收](../../_agile-output/test-artifacts/128-2-final-verification.md)，历史构建过程见[build证据](../../_agile-output/test-artifacts/128-2-build-evidence.md)。ignored本身不计PASS。

Timer独立入口内仍含CSR实例，native Interpreter/Compiled与GeneratedFunctional均明确拒绝层级；不提供仅供测试的无实例产品路径。综合结果不是PPA或板级时序保证。FR142逐符号追加属SemVer minor；没有修改版本或发布。仅Timer子集，IRQ/GPIO/UART、M3/Epic128、FR197整体和Phase24未由本故事关闭；FR189 deferred与NFR91保留。
