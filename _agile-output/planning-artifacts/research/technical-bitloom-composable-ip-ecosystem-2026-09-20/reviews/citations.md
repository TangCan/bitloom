# 引用终审：来源与相邻主张

审查日期：2026-09-20。范围仅为 `research.md` 与 `digests/*.md`；未读取产品代码、项目审计、实施计划或其他规划。已逐项映射报告全部26个引用。外部复读4个既有URL、共4次web调用；其余22个引用明确为 **digest-based**，不声称原文全部重新阅读。复读同一上游不会增加独立publisher数量。

**结果：未发现必须修正的实质不支持、错误归因或版本混淆。** 26项均有对应digest支持报告所限定的主张；其中产品排期、拟定CSR合同与成本估算只是建议/推断，不能升级为来源证实的结论。没有新增兼容性、性能、执行通过或完整依赖锁结论。

## 映射索引

缩写：L=`landscape-r1-1.md`；I1=`interfaces-r1-1.md`；I2=`interfaces-r2-1.md`；E1=`ecosystem-r1-1.md`；E2=`ecosystem-r2-1.md`；V=`verification-r1.md`。各文件均位于 `../digests/`。

“支持”只表示与报告实际限定相符，不表示上游功能已经在Bitloom中运行通过。

| 引用 | digest定位 | 审查依据 | 相邻主张及结论 |
|---|---|---|---|
| [1] LiteX | L:S1/L1；V:V2/VL1 | digest-based | 支持基础连接、简单核、复杂生态核分层和混合RTL路径；不证明外设排期或维护收益。报告已明确限定。 |
| [2] Comportability | L:S4/L4；I1:S3/I7；V:V1/VL1 | digest-based | 支持时钟、复位、总线、寄存器及模块间约定；报告正确标明TL-UL，未当作AXI规范。 |
| [3] reggen | L:S5/L5；I1:S2/I6；V:V3/VL2/VC1 | 原URL复读 | 支持寄存器生成及通用RW1C软件清除优先。不能外推HWExt或所有外设，报告没有这样做。 |
| [4] LiteX CSR Bus | V:V4/VL1/VL2 | digest-based | 支持CSR对象映射及软件访问头文件生成；未声称与reggen全部输出对等。编辑日期与首次发表日期已分开。 |
| [5] Arm IHI0022H | I1:S1/I1–I5；V:V5/VI1/VI2 | 原URL复读关键节 | 支持握手依赖、受阻稳定、允许同拍接收。分别捕获通道、内部CSR结构与容量是设计选择，不是该规范要求。 |
| [6] ZipCPU握手分析 | V:V8/VI1/VI2 | digest-based | 支持独立作者实现分析及缓冲模式；不能替代Arm，也没有逐项独立确认所有到达顺序。报告只称“佐证这些握手模式”，可接受。 |
| [7] CIP testbench | I1:S4/I8 | digest-based | 支持CSR错误和随机复位场景；报告正确保留can_reset_with_csr_accesses配置边界。不能据此证明Bitloom实际覆盖或指定副作用提交实现。 |
| [8] LiteDRAM | L:S2/L2 | digest-based | 支持独立生成Verilog的上游自述；未证明工具兼容或运行通过。 |
| [9] LiteEth | L:S3/L3 | digest-based | 同上；与[8]同publisher，报告没有把它们当双独立兼容证据。 |
| [10] CAPI2 | E1:S1/E1–E2 | digest-based | 支持文件集、依赖、target、参数、provider和生成器；报告明确latest开发文档，不冒充稳定版或完整离线锁。 |
| [11] Bender dependencies | E1:S3/E5；V:V6/VE1 | digest-based | 支持版本约束与精确Git解析身份分离；报告未外推本地路径及全部下载也被锁定。 |
| [12] Hardware Methodology | L:S6/L6 | digest-based | 支持按开发阶段判断成熟度，不能把目录存在等同交付；没有量化维护收益。 |
| [13] Assertions | I1:S7/I11 | digest-based | 支持独立协议检查模块及初始化/复位/结束检查。与[12]合引支持逐IP证据的建议；该页本身不是成熟度分级标准。 |
| [14] IRQ helper | I2:R2S5/J7–J8 | 原URL复读 | 支持Event分支合成新事件与旧state，无软件clear输入；接通用RW1C后的优先级为有前提推断，报告前提完整。 |
| [15] axil_master.py | I2:R2S1/J1/J4/J5 | digest-based | 支持各通道实例、reset取消返回None、首拍未对齐地址保留及拆分/汇总；没有安装兼容结论。 |
| [16] stream.py | I2:R2S3/J1–J4 | digest-based | 支持pause及发送排队/受阻保持语义；五通道工厂关联另有I2:R2S2，虽未列入报告26项附录，但证据链保留在digest。 |
| [17] test_axil.py | I2:R2S4/J6 | digest-based | 实际调用支持逐通道pause入口。不能声称上游已经独立随机化所有通道或覆盖全部在途reset；报告未作该扩张。 |
| [18] AMD CDC | I1:S8/I12 | digest-based | 支持该CDC报告不提供timing slack。报告表述限于AMD方法，不冒充跨工具运行结论。 |
| [19] Cargo manifest/lock | V:V7/VE1 | digest-based | 支持跨生态的依赖意图/解析身份区分；不证明HDL工具兼容、离线或逐位复现。 |
| [20] common_cells releases | E2:C5/维护事件时间线 | 原URL复读 | 支持2026-07-02/16/29的发布事件、beta与v1.40.0区分、FIFO更名及端口变化。未将修复说明当独立通过证据。 |
| [21] cc_fifo.sv | E2:C3/P1–P2/P4–P5 | digest-based | 支持master平坦端口、宏/包引用。报告没有把这些接口归给stable fifo_v3。 |
| [22] cc_fifo_tb.sv | E2:C4/P3–P4 | digest-based | 支持测试源码及部分参数覆盖的存在；不是执行结果，也不证明clear/status全部覆盖。报告没有夸大。 |
| [23] taxi_axis_register.sv | E2:T1/T3/T5 | digest-based | 支持taxi_axis_if modport与文件SPDX；不是已flatten或工具兼容证据。 |
| [24] Taxi测试 | E2:T2/T4/T5 | digest-based | 支持reg_type/data_w组合与Verilator/Cocotb入口；报告正确保留wrapper/接口仍需读取及运行。 |
| [25] common_cells LICENSE | E2:C2/P5 | digest-based | 支持所读许可证标题0.51；不是完整依赖或法律效果结论。 |
| [26] Taxi LICENSE | E1:S7/E9；E2继承说明/T5 | digest-based | 支持CERN-OHL-S-2.0标识与许可证标题；没有沿用其他同作者仓库许可。 |

## 关键原文复读

- **AXI同拍握手：通过。** [Arm历史规范](https://developer.arm.com/-/media/Arm%20Developer%20Community/PDF/IHI0022H_amba_axi_protocol_spec.pdf?hash=6325311012DDADF238C35A6C0FD734E520754F82&la=en&revision=71bd7c57-2ed7-487b-bc3e-68c4ab56fa5f) A3.3.1，A3-46明确允许slave等AWVALID、WVALID或两者再给AWREADY/WREADY，并规定写响应等待相应握手。报告的“同拍握手本身不违规”受到支持。未重新逐页阅读500页PDF。
- **RW1C与IRQ helper：通过。** [reggen](https://opentitan.org/book/util/reggen/index.html) “Simultaneous SW and HW access”提供软件清除覆盖硬件更新的公式；[prim_intr_hw.sv](https://raw.githubusercontent.com/lowRISC/opentitan/master/hw/ip/prim/rtl/prim_intr_hw.sv) Event分支57–65行将新事件并入旧state，没有软件clear输入。报告将二者组合后的结果写为有条件结论，未把Bitloom拟定set-wins合同冒充OpenTitan通则。
- **外部FIFO尚未准入：通过。** [发布页](https://github.com/pulp-platform/common_cells/releases)明确列出v2 beta与v1.40.0 Latest，日期均早于本轮2026-09-20访问日。报告未把master接口或beta修复归到stable，明确完整SHA、stable源码、依赖闭包和运行仍缺。复读发布页没有补齐这些缺口。

## 不能由引用证明的部分与非阻断提示

1. **优先级与效益不是研究事实。** [1]–[4]不直接支持“UART/GPIO/Timer/IRQ是Bitloom用户最优先需要的组合”，[8]–[9]不证明接入会节省多少工时。报告已将排序写为工程建议并承认用户数据缺失，因此不是待修正的无支持事实。
2. **内部CSR提交合同没有外部规范背书。** 第2节的“CSR成功接受请求时才提交副作用”等内容是上一句“首版建议”的实现合同延续；[5][7]只能支持相关协议约束/验证动机，不能证明这一具体接口设计已经正确。当前上下文能区分；下游摘抄时应保留建议性质。
3. **独立核对不等于独立功能验证。** 多个OpenTitan页仍只有一个publisher；源码与同仓库测试也不是第二个独立兼容证据。报告两处verified仅用于跨生态文档模式，候选兼容、BFM运行和全闭包保持unverified，等级合理。Arm与ZipCPU不是两份独立标准。
4. **日期无冒充。** 已注明的2020/2021/2023资料不属于近两年发表；common_cells的2026年7月事件不是未来日期。其他23项的精确初始pubdate未取得（其中CSR页仅有编辑日期），报告明确未知，没有将访问日期当发布日期。此审查未尝试补造日期。
5. **本地事实不在此审查背书范围。** 项目审计L1–L12、两行为探针、41–64/46–72人日估算、实施计划及staleness.json未被此终审读取或独立确认；它们也没有被用作行业证据。

无阻断引用问题；无需为本审查新增研究来源或扩大研究范围。
