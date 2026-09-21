# GPIO32 CSR

Bitloom `bitloom_prelude::ip::GpioCsr` 提供32位GPIO的软件控制、双级同步实际针脚读取和原始上升沿事件（Story128.4 / FR197 GPIO子集）。Bitloom与samitbasu/rhdl无关。设计crate只依赖`bitloom-prelude`；旧8位`Gpio`和`GpioFunctional`行为保持。

`GpioCsr::define_module(&mut ElaborateSession, impl Into<String>) -> Result<String, Diagnostics>` 在调用者session定义GPIO和共享CSR叶；同名同定义复用、不同名共享叶定义，冲突/宽向错误沿用现有诊断。完成所有定义后仅一次`finish()`。`Elaboratable for GpioCsr` 的 `elaborate() -> Result<FrozenHir, Diagnostics>` 使用同一定义体，顶名`GpioCsr`。`registers() -> CsrBlock` 返回固定静态描述，修改返回值不会修改GPIO硬件。

16端口：输入`clk: Clock`、`rst: Reset`、`req_valid: UInt1`、`write: UInt1`、`addr: UInt16`、`wdata: UInt32`、`wstrb: UInt4`、`rsp_ready: UInt1`、`pad_in: UInt32`；输出`req_ready: UInt1`、`rsp_valid: UInt1`、`rdata: UInt32`、`error: UInt2`、`pad_out: UInt32`、`pad_oe: UInt32`、`raw_event: UInt1`。`pad_out=OUT & DIR`，`pad_oe=DIR`，这是逻辑数据和使能，不是物理三态pad。

|local字节地址|系统地址（调用者加base）|名称/访问|唯一owner|
|---|---|---|---|
|00|0100|DIR RW|CSR leaf|
|04|0104|OUT RW|wrapper|
|08|0108|IN RO|wrapper同步第二级|
|0c|010c|SET WO|无状态，对OUT置1|
|10|0110|CLEAR WO|无状态，对OUT清0|
|14|0114|RISE_EVENT W1C|CSR leaf|

全32位有效、reset0，DIR=1输出/0输入。RW按小端WSTRB逐字节合并，SET/CLEAR/W1C只作用于选中字节的写1位；SET/CLEAR没有另存副本。合法零WSTRB写OKAY且无软件作用；IN写（包括WSTRB0）、SET/CLEAR读、洞/未对齐/高位地址均SLVERR，失败读与所有写响应rdata0。wrapper不截高位，0x0100直接输入不会别名local0；系统窗外DECERR由decoder处理。

非reset上升沿`req_valid && req_ready`是唯一软件提交。读返回提交沿前快照，下一周期响应有效；背压保持响应，消费沿无refill，受阻请求保持valid/payload直至提交或共同reset取消。单请求保证OUT/SET/CLEAR三个软件写互斥。自然事件在背压期间继续更新，安全性无需ready公平性。

每个非reset沿：`sync1'=pad_in; sync2'=sync1; history'=sync2`。`IN=sync2`始终读实际针脚，包括DIR输出位，不以OUT代替。沿前`rise=sync2 & ~history & ~DIR`；`event_next=(event_old & ~committed_clear)|rise`，set胜clear，DIR同沿写按旧DIR筛选。改变方向本身不产生边沿。

初始全零，针脚自e沿前保持高：e后sync1=1/sync2=0/history=0；e+1后sync1=1/sync2=1/history=0，此时raw_event=1；e+2沿采样该事件到本地RISE_EVENT和IRQ，随后history=1、raw_event=0。e+1沿提交IN读返回0，e+2沿返回1。初始高会生成一次事件；持续高/下降不重复，低后再高可再次触发。

`raw_event=(!rst) && |rise`只代表当拍新沿，接IRQ bit4，其他四源映射不变；禁止把粘滞RISE_EVENT接IRQ。真实GPIO→IRQ夹具验证本地事件仍高时清IRQ不重触发，下一新沿仍置IRQ；mask不清pending，新沿胜同拍本地和IRQ双W1C，live raw期间reset立即抑制事件。同步高有效reset清DIR/OUT/同步链/history/本地事件/待响应，优先取消提交与新沿；raw_event在rst高时立即无效，其余寄存状态直到reset沿才清除。leaf/wrapper/IRQ须同域共同reset；系统aresetn须外部控制器同步断言和释放后转换，单纯反相不是同步器。

[原文prelude-only例](gpio-csr-example.rs)实际编译并展开共享定义与独立入口，运行参数为软件输出目录。同一描述生成[C头](gpio-csr-registers.h)和[地址表](gpio-csr-registers.md)，与独立手写黄金地址比较并由C11消费者编译。

`fr197_gpio`三个seed共55151帧的独立逐拍参考覆盖32位、16种WSTRB×五个可写寄存器、初始高/同步/方向竞争、事件清除碰撞、权限/响应/取消账本。专用FIRRTL/Chisel入口执行相同向量；`fr197_gpio_api`另实际运行共享/重命名双实例及GPIO→IRQ。独立形式ghost从原始pad_in推导状态，状态对应是assert，不假设observer等于DUT或ready公平性；prove/cover、原始无observer综合与DUT变异反例分列。

复现使用`python3 _agile-output/test-artifacts/128-4-build-runner.py`，依调用者PATH、RHDL_FIRTOOL_PATH和BITLOOM_SBY_SOURCE；固定SBY身份核验并隔离缓存后才运行形式工具。普通入口核对11个具名测试及两个专用ignored，形式入口核对3项实际执行，两exact后端入口各核对一次；例子比较是独立gate。一次完整run起止/当前源码SHA须一致；归档`python3 _agile-output/test-artifacts/128-4-build-archive.py --run <run目录>`逐成员校验且不覆盖tar或manifest。另以不改黄金的IN=OUT实际RTL变异验证逐拍scoreboard能检出指定读值错误；形式三种变异及其指定反例分别留存。具体实测见[构建证据](../../_agile-output/test-artifacts/128-4-build-evidence.md)，ignored不算PASS，远端CI未运行不称通过。

独立入口也包含CSR实例，native Interpreter/Compiled与GeneratedFunctional层级仍明确unsupported；不为测试新增单模块产品路径。双级同步不提供去抖、滤毛刺、多位相干采样、MTBF或pad/电气/板级签核保证，综合不等于PPA。新增四符号显式登记FR142，按SemVer minor；工具钉/包版本/发布不变。仅GPIO子集，UART128.5、M3/FR197整体、Epic129–130和整个Phase24仍未交付；FR189 deferred/NFR91保持。七步最终状态以故事记录为准。
