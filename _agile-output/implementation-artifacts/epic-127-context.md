# Epic 127 Context: CSR 与总线连接

<!-- compiled 2026-09-21 from Phase24 approved contract, epics and AD-30/31 -->

## Goal

交付 FR196/M2：可组合 CSR 描述、AXI4-Lite 桥及四窗口静态译码。M0/125.3 和 M1/126.4 已关闭；整体七步执行已获授权。127.1 只建立风险记录，不能关闭 FR196。

## Stories

- 127.1 CSR/总线 NFR14：约束、负责人、估算、工具实测和禁止降级。
- 127.2 静态 Rust CSR 描述生成 RTL、地址文档和 C 头文件，确定性产物与独立黄金值。
- 127.3 AW/W/AR 独立捕获的 AXI-Lite→CSR 桥，背压、轮转与 reset。
- 127.4 UART/GPIO/Timer/IRQ 四窗口静态译码、组合验收及 M2 关闭。

## Requirements & Constraints

设计只依赖 bitloom-prelude；沿用单一 ElaborateSession/FrozenHir。Rust 1.97.1、edition 2024；工具钉不变。CSR 单时钟、addr16 字节地址、data32、WSTRB4；每寄存器四字节对齐、小端、reset 值0。全局仅一个已提交未消费的 CSR 请求。

唯一提交点是非 reset 沿 req_valid && req_ready。写、W1C、TX push、RX pop 在此发生一次；读取在此取快照，下一周期响应，受阻保持且不得新提交。错误读返回0，错误无副作用。RW 按字节合并，保留位读0写忽略；RO 写和 WO 读为 SLVERR。W1C 硬件 set 优先：next=(old & ~clear)|event。合法零 WSTRB 写无副作用，但 RO 写仍报错。动态状态、事件、提交脉冲及拒绝条件须通过明确组合端口连接，不能只提供 RW bank。

错误编码00 OKAY、10 SLVERR、11 DECERR，不产生01。四窗基址0x0000/0100/0200/0300，各0x100；窗外 DECERR，窗内空洞/未对齐/权限错误 SLVERR；高位不截断，无双选，无 ID/version 寄存器。TX_DATA 未选 byte0 时不 push，即便满也 OKAY；RX空/TX满有效请求必须报错而非无限等待。UART忙检查合并后值，零 WSTRB 不触发忙错误。

AW/W/AR 各一捕获槽，完整 AW+W 才具写资格，不完整写不能阻读；B/R 容量在提交前预留，受阻稳定、防覆盖。外部 ready/valid 只由寄存态控制。轮转初始读优先，成功提交后优先另一类；无响应空间的类不参与。背压可永久持续，安全性不依赖公平性；活性需显式公平性/最大等待条件。

外部控制器先将 aresetn 的断言和释放同步到 ACLK，再取反；取反不是同步器。桥与 leaf 同域共同 reset，优先于提交，清捕获及响应；不支持仅桥复位，不回滚 UART 已发物理位。

## Technical Decisions

采用静态配置→elaboration，不新增硬件 IR、运行时对象或插件。define_module 的非捕获函数与完整规范参数可编码描述全部名称/offset/mask/reset/access/event，并在 elaboration 解码；禁止摘要代替完整身份、全局可变表或捕获闭包。127.2 才实现，若无法表达则先记录最小扩展和停止条件。

旧 Axi4LiteSlave 的 ADDR8、四寄存器、恒 OKAY、同拍 AW/W 一 tick 响应及 read-before-write 保持。新 API 在功能故事明确登记 FR142 minor；风险故事无 API。原生/生成层级仿真仍不支持，层级组合用实际 RTL。固定 cocotb2.0.1/cocotbext-axi0.1.28/Python3.12 探针只证明工具 API；高层 BFM 会对齐，特殊访问须 raw channel 驱动，不能双重驱动针脚。

未来验收包括独立 oracle、全部16种 WSTRB、AW/W 间隔0/1/7/31、背压/reset/取消、旧 bank 回归；16seed×1000事务只是计划。RTL/formal/synthesis/PPA 分列证据。门禁脚本仅验证 M0、各 epic 的 .1 及关闭状态，跨故事依赖另审。

## Cross-Story Dependencies

127.1 依赖125.3；127.2依赖127.1+126.2；127.3依赖127.2+126.3；127.4依赖127.3。128外设与129系统是后续工作。估算依次0.5–1、4–6、5–7、3–5有效人日，共12.5–19；Richard owner，Codex实施，主代理七步与提交。不得提前关闭 Epic127；FR189/Epic122 deferred 与 NFR91 保留。
