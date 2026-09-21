# Story127.4 ATDD 固定API与周期契约

2026-09-21；经主代理故事审阅固化。本文件是验收输入，不是产品实现声明。公共名Bitloom；只依赖bitloom-prelude；无版本升级/发布。

## API

```rust
use bitloom_prelude::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir};
use bitloom_prelude::ip::CsrDecoder;
// pub struct CsrDecoder;
let standalone: Result<FrozenHir, Diagnostics> = CsrDecoder::elaborate();
let mut session = ElaborateSession::new("Top");
let module: Result<String, Diagnostics> = CsrDecoder::define_module(&mut session, "Decoder");
```

`Elaboratable for CsrDecoder`独立入口电路/模块名固定`CsrDecoder`，自身finish一次。`define_module(&mut ElaborateSession, impl Into<String>) -> Result<String,Diagnostics>`与独立入口共享无捕获body，返回注册模块名，相同完整定义复用；非法名/冲突与既有helper诊断一致，helper失败poison session，不忽略错误继续使用。调用者先全部define，再顶层begin/instances，最后一次finish。没有构造参数、window配置、运行时状态对象或额外公共常量要求。

## 精确52端口

除clk/rst外均UInt。没有调试owner/选择端口；通过实际下游valid/ready和响应观察。

| 名称 | 方向 | 类型/宽 |
|---|---|---|
|clk|in|Clock|
|rst|in|Reset，同步高有效|
|req_valid|in|1|
|write|in|1|
|addr|in|16|
|wdata|in|32|
|wstrb|in|4|
|rsp_ready|in|1|
|req_ready|out|1|
|rsp_valid|out|1|
|rdata|out|32|
|error|out|2|

每个P严格取`uart`、`gpio`、`timer`、`irq`，各10端口：

| 名称 | 方向 | 宽 |
|---|---|---|
|P_req_ready|in|1|
|P_rsp_valid|in|1|
|P_rdata|in|32|
|P_error|in|2|
|P_req_valid|out|1|
|P_write|out|1|
|P_addr|out|16|
|P_wdata|out|32|
|P_wstrb|out|4|
|P_rsp_ready|out|1|

## 映射与响应

完整16位addr命中`0000–00ff`→uart、`0100–01ff`→gpio、`0200–02ff`→timer、`0300–03ff`→irq；各size100(hex)。命中后local=addr−base，以16位表达，保留低两位。固定map无重叠/越界，不增加不存在的可配置map错误表面。未选叶payload无效，不要求其地址归零，oracle只比较被选叶有效payload；所有叶req_valid onehot-or-zero。

空闲且!rst：命中时唯一叶req_valid=req_valid，上游req_ready=该叶req_ready；write/wdata/wstrb与局部addr透明路由。上游握手与叶提交同一非reset上升沿，不能添加请求队列拆成两次提交。该沿锁定owner，响应来自owner且仅owner收到rsp_ready；其他叶输入不参与本响应。直到上游rsp_valid&&rsp_ready消费前禁止下一提交，消费沿仍不接受新请求。未接受的上游请求必须保持全部payload，直到接受或reset；提交后可切换到下一请求，但decoder忙时不接受。

未命中req_ready在空闲!rst时为1，所有下游req_valid=0；上游提交后锁存本地DECERR pending，下一周期rsp_valid=1/rdata=0/error=3，持续到消费。不能从当前addr组合生成响应。只有合法leaf错误00/10/11进入路由，不接收保留01为合法peer；失败读rdata0，写的无关rdata不限。窗内hole/未对齐/权限由leaf给SLVERR，窗外包括未对齐均DECERR。

rst=1时上游req_ready/所有下游req_valid/下游rsp_ready为0。up_rsp_valid不组合rst门控：已有pending响应在沿前可见，reset沿清状态后为0；reset优先取消，不能计消费或副作用。所有模块共同同步reset，首次使用前施加有效沿。保持义务跨无reset周期，reset清producer/response monitor历史；没有桥独立reset支持。decoder内部组合路径允许，桥外部5 ready/valid仍寄存。

合法片上CsrBlock响应在提交沿锁存、下一周期可见；单decoder形式安全允许无界等待以证明安全不依赖完成性，不能由此宣称支持外部无限等待产品或无条件活性。

## 验收与范围

- 单模块两native引擎及真实RTL；65536全地址比较实际DUT与独立range/local oracle。
- 真实层级bridge+decoder+4 CsrBlock采用正式寄存器布局；动态state/event/reject为测试peer，只测总线/CSR连接，不称Epic128外设算法交付。
- 层级native/generated明确unsupported；不新增flatten模拟。独立生成Rust decoder本故事没有默认已验证宣称。
- 普通行为测试build需解除ATDD ignore；sby prove/cover、原始decoder/组合综合显式另跑，ignored≠PASS。
- 只新增FR196译码子集；M2需后续全部七步真实闭合。旧bank/CSR/bridge/M1保持。
