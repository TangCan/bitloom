# 第一轮关键主张独立核对

accessed：2026-09-20。仅导入 `landscape-r1-1.md`、`interfaces-r1-1.md`、`ecosystem-r1-1.md`；没有读取项目代码、规划或其他项目上下文。外部核对共6次web调用、8个实际来源，停止条件为来源上限。全部是规范制定者、工具维护者或实现作者的一手材料；未执行工具、RTL或兼容性实验。

`verified` 限于所写的文档/模式主张；并不表示产品集成通过。独立publisher不等于彼此毫无引用关系：AXI规范的权威仍为Arm；ZipCPU提供实现作者的代码/握手分析，不能算第二份独立协议标准。上游自述功能或许可不因本表重新打开而提升为独立验证。

## 本轮来源

各claim继承本表的完整URL、publisher、pub_date和accessed。

| ID | source | publisher | pub_date | accessed |
|---|---|---|---|---|
| V1 | https://opentitan.org/book/doc/contributing/hw/comportability/ | lowRISC / OpenTitan | 未知 | 2026-09-20 |
| V2 | https://github.com/enjoy-digital/litex | Enjoy-Digital / LiteX维护者 | 未知 | 2026-09-20 |
| V3 | https://opentitan.org/book/util/reggen/index.html | lowRISC / OpenTitan | 未知 | 2026-09-20 |
| V4 | https://github.com/enjoy-digital/litex/wiki/CSR-Bus | Enjoy-Digital / LiteX维护者；页面署名enjoy-digital | 初始发表未知；页面最后编辑2023-07-28 | 2026-09-20 |
| V5 | https://developer.arm.com/-/media/Arm%20Developer%20Community/PDF/IHI0022H_amba_axi_protocol_spec.pdf?hash=6325311012DDADF238C35A6C0FD734E520754F82&la=en&revision=71bd7c57-2ed7-487b-bc3e-68c4ab56fa5f | Arm | 2020-03-31，变更表H项 | 2026-09-20 |
| V6 | https://pulp-platform.github.io/bender/dependencies.html | PULP Platform / Bender维护者 | 未知 | 2026-09-20 |
| V7 | https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html | Rust项目 / Cargo维护者 | 未知 | 2026-09-20 |
| V8 | https://zipcpu.com/blog/2021/08/28/axi-rules.html | ZipCPU / 实现作者 | 2021-08-28，页面日期 | 2026-09-20 |

两个OpenTitan页面只计一个publisher，两个LiteX页面亦然。未知日期保持未知；2023年页面和2020/2021年协议资料不能算近两年发布证据。

## Landscape：两个模式主张

### VL1 — verified；文档模式，medium

主张：至少OpenTitan与LiteX这两个独立生态使用共同的外围连接约定；“先约定接口再组合IP”有多个生态实例。

- V1 Feature List / Bus Interfaces / Configuration description：把时钟、总线和寄存器列为必需项，TL-UL为框架接口；顶层负责系统地址分配。
- V4 Introduction / Primary Bus to CSR：描述共同CSR本地总线与主总线桥接，CSR类型被收集并映射；V2 README另描述连接既有RTL核的使用方式。
- 支持范围：证明存在不同生态的共同接口组织实践。两生态协议、时序和元数据格式并不相同，不能升级成跨生态已有统一标准，也不证明某种排期收益最大。

### VL2 — verified；文档模式，medium

主张：寄存器描述用于连接硬件和软件产物，有两个独立生态实例。

- V3开头及Configuration and Register Definition：Hjson描述寄存器，生成RTL、头文件和文档。
- V4 CSRs及CSR Accessors：CSR对象参与地址映射；`get_csr_header()`生成`generated/csr.h`中的访问方法。
- 支持范围：可写“寄存器描述/对象驱动生成是多生态实践”。不可写两者采用同一schema、相同副作用语义，或已测得降低维护成本；LiteX来源并未在本轮证明其输出与OpenTitan全部产物对等。

## Interfaces：两个协议主张

### VI1 — verified（规范主张）；独立实现佐证范围有限

主张：AXI4/Lite的AW与W不能被假设为同拍首次有效；写响应必须等地址与所需写数据被接收。

- V5 A3.3，A3-44：明确允许写数据先于地址或与地址同拍出现；A3.3.1，A3-46规定AXI4写响应依赖AW与W握手，突发还须末拍。
- V8 Example Slave logic：作者代码用分别缓冲后的`skid_awvalid`与`skid_wvalid`决定写提交，并受B通道状态约束。这独立佐证“先收集再配对”的实现模式；该页没有逐项给出W先/AW先的规范证明。
- 精确纠正：**不能把“AW/W独立”解释成禁止slave等到两个VALID均有效才同时拉READY。** V5 A3-46明确允许READY等待AWVALID、WVALID或两者；也不能把同拍握手写成违规。真正禁止的是要求master仅在同拍首次提出AW/W，或master等待READY才提出VALID而导致死锁。
- 若报告把“verified”定义为每个细节均须两个publisher逐字确认，则此处最强规范结论应降为`unverified / single-source-medium`，同时明确Arm一手规范已经确认；不得伪称独立实现页完整复核了所有到达顺序。

### VI2 — verified；规范与独立实现分析一致

主张：正常传输期间，VALID已有效而READY未有效时，VALID及其载荷必须保持；握手发生于两者有效的采样边沿。

- V5 A3.1.1 / A3.2.1，A3-40至41：上升沿采样、传输条件、载荷保持以及VALID保持至握手。
- V8 The rules / Example Master logic：规则与作者示例均只在非有效或对端可接收时更新VALID和载荷；页面明确规则也适用于AXI和AXI-Lite。
- 支持范围：协议义务及独立实现模式。复位置位是另行规定的边界，不应断言跨复位仍保持旧VALID。V8关于READY注册是实现建议/推导；直接可引用的Arm硬性要求是接口输入至输出无组合路径，不能任意扩大为所有内部ready/valid连接均禁止组合逻辑。

## Ecosystem：一个依赖身份主张

### VE1 — verified；跨生态文档模式，medium

主张：依赖意图与实际解析身份分开记录，有Bender与Cargo两个独立publisher的直接说明。

- V6 Revision-based / Version Resolution and the Lockfile：manifest可用版本或revision；update解析依赖树，将精确版本与Git commit hash写入`Bender.lock`。Path Dependencies另说明本地路径依赖不版本化。
- V7全文：`Cargo.toml`描述依赖，`Cargo.lock`记录准确解析信息；Git示例明确包含具体revision，更新命令再修改lock。
- 支持范围：manifest/lock职责分离。Cargo是跨领域佐证，不证明所有HDL工具都如此。两源不证明完整工具链、生成参数、本地路径内容及全部外部下载已经被锁定，也不证明离线或逐位可复现；不能从本轮推断FuseSoC具有完全相同lock语义。

## 特别纠正与保留项

### VC1 — verified（仅“OpenTitan文档如此规定”）；single-source-medium

V3 Simultaneous SW and HW access明确对通用RW1C给出先选硬件更新/旧值、再施加软件清除掩码的规则；同位同时set/clear时软件清除生效。文档另述持续硬件事件随后可重新置位，HWExt RW1C把具体行为交给外部实现。仅确认该来源声明，没有读取当前RTL或执行碰撞测试。

“W1C普遍硬件set优先”被该例反驳；“W1C普遍软件clear优先”同样没有依据，状态为`unverified`。建议采用set-wins可以保留为设计选择，须与OpenTitan通用subreg示例区分，不能当作规范通则或推断全部OpenTitan中断路径。

### VC2 — unverified；single-source-medium

三个导入digest中的LiteDRAM/LiteEth、Taxi等上游功能自述及OpenTitan/Taxi许可证记录，保持`single-source-medium`。原digest的high最多描述对“原文如此写”的把握，不能转写成独立验证的功能、兼容或全依赖许可结论。来源沿用各原digest具体URL、publisher、未知pub_date及2026-09-20 accessed；本轮没有重新打开这些项目的许可/实现来源，不冒充新增核验。

### VC3 — unverified

Bitloom与任一候选IP的可用参数矩阵、性能、综合结果、离线重建、许可证对下游设计的法律效果、选择小控制系统的量化收益，均不在本轮验证范围内。没有足够证据把这些升级为事实。没有发现需要推翻三个digest整体结论的冲突；主要修正是限制证据等级，以及消除“AW/W不可同拍握手”的潜在误读。
