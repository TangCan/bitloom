# AXI-Lite BFM API与IRQ helper：第二轮定向证据

访问：2026-09-20。本轮4次web调用、5个实际源码文件；初次test文件返回不足内容，重读同URL成功。未读项目文件，未安装依赖、未执行仿真，不宣称版本兼容或测试通过。第一轮的疑问在此收敛，未知项不继续扩搜。

## 来源及元数据

各claim继承引用source的publisher、pub_date、accessed。版权年份不等于发布日期；master是移动引用，不是版本锁。

| ID | source | publisher | pub_date | accessed |
|---|---|---|---|---|
| R2S1 | [axil_master.py](https://raw.githubusercontent.com/alexforencich/cocotbext-axi/master/cocotbext/axi/axil_master.py) | Alex Forencich/cocotbext-axi维护者 | 未注明 | 2026-09-20 |
| R2S2 | [axil_channels.py](https://raw.githubusercontent.com/alexforencich/cocotbext-axi/master/cocotbext/axi/axil_channels.py) | Alex Forencich/cocotbext-axi维护者 | 未注明 | 2026-09-20 |
| R2S3 | [stream.py](https://raw.githubusercontent.com/alexforencich/cocotbext-axi/master/cocotbext/axi/stream.py) | Alex Forencich/cocotbext-axi维护者 | 未注明 | 2026-09-20 |
| R2S4 | [tests/axil/test_axil.py](https://raw.githubusercontent.com/alexforencich/cocotbext-axi/master/tests/axil/test_axil.py) | Alex Forencich/cocotbext-axi维护者 | 未注明 | 2026-09-20 |
| R2S5 | [prim_intr_hw.sv](https://raw.githubusercontent.com/lowRISC/opentitan/master/hw/ip/prim/rtl/prim_intr_hw.sv) | lowRISC/OpenTitan | 未注明 | 2026-09-20 |

## Claims

| Claim | class | confidence | source / 支持摘意 |
|---|---|---|---|
| J1：Lite AW/W/AR是独立source，B/R是独立sink；都由公共stream工厂构造，具备pause控制。 | API源码事实 | 高（静态阅读） | R2S1构造器；R2S2五个define_stream声明；R2S3工厂与StreamSource/StreamSink继承关系。 |
| J2：source的send只把事务排队，不等待总线握手；AW入队后再W入队不强制AW先握手。 | API语义 | 高（静态阅读） | R2S3 send先检查队列空间再put；R2S1 _process_write依次send。AW暂停、W不暂停可构造W先到，但精确拍数要采样确认。 |
| J3：source pause阻止发起新载荷，不撤销已VALID且尚未READY的载荷；sink pause控制READY，有按时钟采样的生效点。 | API语义 | 高（静态阅读） | R2S3 _run：source只在握手或原VALID低时换载荷；sink以pause_sample更新ready。 |
| J4：master复位清空通道及命令队列、终止工作协程，相关待完成event赋None，在途计数归零；释放后重启。 | API复位语义 | 高（静态阅读）；端到端恢复unverified | R2S1两类_handle_reset。R2S3 source reset拉低VALID、sink reset拉低READY。不能把None当OKAY响应。 |
| J5：高级API首拍保留未对齐原地址，后续拍按总线字节宽度推进；写strobe与读返回切片自动生成，跨拍响应合并。 | API事务转换 | 高（静态阅读） | R2S1 _process_write/_process_read及响应处理；纠正“高层API必把首地址对齐”的潜在误解。 |
| J6：上游Lite测试确实使用逐通道pause API并测试地址偏移和读写长度；所读测试不证明待响应复位或独立随机AW/W相位已覆盖。 | 上游测试结构 | 高（所读文件） | R2S4 set_idle_generator/set_backpressure_generator；cycle_pause固定周期，各通道由同一个工厂分别构造同相序列；reset在各测试开始；stress随机地址/长度/等待。 |
| J7：Event IRQ helper把新事件及test事件合成，并向CSR给出de和d=新事件OR旧state；软件clear并不是helper输入。 | RTL事实 | 高（静态阅读） | R2S5 g_intr_event；IRQ取当前state，再按enable及FlopOutput输出。 |
| J8：把J7接到第一轮S2所描述的通用RW1C，逻辑合成为先OR新事件后施加软件clear，因此同bit同拍clear胜出；helper本身不能被称为set-wins实现。 | 跨来源逻辑推断 | 高（前提为该标准RW1C连接）；具体外设接线unverified | R2S5 + 第一轮interfaces-r1-1.md的S2/I6。未检查所有生成CSR或实际外设连接，不能外推为全部OpenTitan IRQ必然相同。 |

## 可使用的精确API候选

以下是从R2S4调用点确认的候选路径，`master`为AxiLiteMaster实例；不是本环境已运行示例。随机生成器应每通道分别实例化，使用不同相位/种子并记录seed。

| 意图 | 候选调用 |
|---|---|
| AW发起暂停 | `master.write_if.aw_channel.set_pause_generator(aw_gen)` |
| W发起暂停 | `master.write_if.w_channel.set_pause_generator(w_gen)` |
| AR发起暂停 | `master.read_if.ar_channel.set_pause_generator(ar_gen)` |
| 写响应背压 | `master.write_if.b_channel.set_pause_generator(b_gen)` |
| 读响应背压 | `master.read_if.r_channel.set_pause_generator(r_gen)` |

R2S3提供`pause`属性及`clear_pause_generator()`。停止生成器不等于保证pause恢复False，测试收尾应显式恢复控制状态并观察READY/VALID。R2S1构造器`reset_active_level`默认True；AXI低有效reset接入需显式配置False。这只是所读签名事实，不是复位时序保证。

## raw-channel补充的准确边界（建议/推断）

- **不能说“误对齐只能raw测试”**：J5说明高层调用能保留首拍误对齐。高层调用适合正常按字节访问、自然strobe和返回值检查。
- **仍建议raw测试**：精确只发一笔AW/W、zero或稀疏WSTRB、地址低位和strobe特殊组合、只发AW后reset、只发W后reset、逐拍检查错误返回及副作用，需要避免高层拆分和汇总隐藏中间行为。R2S2可直接导入`AxiLiteAWTransaction`、`AxiLiteWTransaction`及相应Source，字段分别是awaddr/awprot与wdata/wstrb；Source.send的排队语义见J2。raw B/R Sink单独收集每笔响应。不得让高层master和raw driver同时驱动相同针脚。
- **复位scoreboard**：待完成API返回None只表示BFM丢弃操作；DUT是否已发生一次不可逆副作用需单独观察并按产品合同记录。已提交UART TX不能因BFM flush就声称被撤销。
- **独立oracle**：上游S4主要是master与同项目RAM模型互测；候选项目必须把BFM连接到实际发出的RTL，并单独检查CSR/IRQ合同。文件被读到或上游有测试均不是目标环境兼容通过。

## 剩余限制

- 5个来源虽覆盖源码和测试，只有2个publisher；同项目源码和测试不是独立兼容确认。未锁定提交、Python/cocotb/仿真器组合，版本兼容、性能和目标产品故障结论均unverified。
- 未取得pub_date，模式≤2年窗口不能靠版权年份或抓取时间确认。实现前固定revision、运行最小AW先/W先/响应背压/复位测试，保存实际RTL与波形及版本。
- IRQ helper的Status路径与FlopOutput组合还存在不同取值路径；本轮只对Event同拍优先级给出条件结论。若选Status或向量事件，按实际CSR连接额外验证。
