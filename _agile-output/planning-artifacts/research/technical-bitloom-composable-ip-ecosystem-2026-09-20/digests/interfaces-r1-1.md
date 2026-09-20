# 接口、CSR、FIFO与RTL验证：第一轮证据摘要

访问日期：2026-09-20。独立研究；未读取任何本地项目文件。7次web调用，实际打开8个来源，达到本轮来源上限。下列来源ID的publisher/pub_date/accessed元数据被各claim按引用继承。搜索抓取时间不当作发表日期。

## 来源表

| ID | source | publisher | pub_date | accessed | 适用性 |
|---|---|---|---|---|---|
| S1 | [AMBA AXI and ACE IHI0022H](https://developer.arm.com/-/media/Arm%20Developer%20Community/PDF/IHI0022H_amba_axi_protocol_spec.pdf?hash=6325311012DDADF238C35A6C0FD734E520754F82&la=en&revision=71bd7c57-2ed7-487b-bc3e-68c4ab56fa5f) | Arm | 2020-03-31，PDF变更表H行 | 2026-09-20 | 历史版稳定规范；仅引用AXI4/Lite规则，不宣称最新版 |
| S2 | [reggen & regtool](https://opentitan.org/book/util/reggen/index.html) | lowRISC/OpenTitan | 未注明 | 2026-09-20 | 本次读取在线文档；未固定提交，模式来源时效未完全确认 |
| S3 | [Comportability](https://opentitan.org/book/doc/contributing/hw/comportability/) | lowRISC/OpenTitan | 未注明 | 2026-09-20 | 同上；TL-UL集成模式，不能当AXI规范 |
| S4 | [Comportable IP Testbench Architecture](https://opentitan.org/book/hw/dv/sv/cip_lib/index.html) | lowRISC/OpenTitan | 未注明 | 2026-09-20 | 同上；只描述文档中的验证结构 |
| S5 | [cocotbext-axi](https://github.com/alexforencich/cocotbext-axi) | Alex Forencich/项目维护者 | 未注明 | 2026-09-20 | 主分支README；具体版本兼容未验证 |
| S6 | [prim_fifo_sync.sv](https://github.com/lowRISC/opentitan/blob/master/hw/ip/prim/rtl/prim_fifo_sync.sv) | lowRISC/OpenTitan | 未注明 | 2026-09-20 | 本次可见实际RTL；主分支未固定提交 |
| S7 | [Assertions](https://opentitan.org/book/hw/formal/) | lowRISC/OpenTitan | 未注明 | 2026-09-20 | 方法参考；未执行其工具链 |
| S8 | [Report Clock Domain Crossings, UG906 2026.1](https://docs.amd.com/r/en-US/ug906-vivado-design-analysis/Report-Clock-Domain-Crossings) | AMD | 未注明；检索元数据约2个月前，不作正式发表日期 | 2026-09-20 | 方法说明；不作当前版本兼容承诺 |

## Claims与支持摘意

| Claim | class | confidence | source / 支持位置与摘意 |
|---|---|---|---|
| I1：AW/W不能要求同拍到达；写响应需等两个通道均完成握手。 | 规范 | 高 | S1 A3.3/A3.3.1，页A3-44至46：写数据可先于地址；AXI4写响应额外依赖AW与W握手。 |
| I2：VALID不能等待READY；受阻时VALID及载荷保持，传输仅发生于两者同时有效的采样边沿；AXI接口禁止输入至输出组合路径。 | 规范 | 高 | S1 A3.1/A3.2，页A3-40至43。 |
| I3：AXI复位可异步置位但同步释放；复位时master的AR/AW/W VALID和slave的R/B VALID均低。 | 规范 | 高 | S1 A3.1.2。该节不提供系统级断电或局部复位恢复合同。 |
| I4：Lite为单拍、32或64位；支持WSTRB；非存储slave可支持、忽略或拒绝不支持的strobe，不能宣称全部外设必须逐字节写。 | 规范 | 高 | S1 B1.1.1至3。 |
| I5：Lite允许多个outstanding，也允许slave靠握手限制；译码失败的interconnect须返回DECERR，错误不免除完成传输责任。 | 规范 | 高 | S1 B1.1.4及A3-60。 |
| I6：OpenTitan通用RW1C不是硬件set优先：同拍硬件更新后仍施加软件清除；hwext把实现责任移到外部。 | 实现合同反例 | 高（限文档） | S2 “Simultaneous SW and HW access” / “HWExt RW1C”：明确软件优先，并讨论仍活跃事件下一拍重新置位。 |
| I7：comportability把系统地址图、外设局部地址、时钟复位和中断语义都当作集成描述；event与status IRQ区分。 | 集成模式 | 高（文档内容）；时间未核实 | S3 “Bus Interfaces”“Clocking”“CIP Interrupt Types”：INTR_STATE未被enable掩蔽；event可W1C，status为RO且需解决原因。 |
| I8：CSR验证需专门错误、随机复位和并发访问场景；某些随机复位配置只在无在途访问时注入，不能自动等于已覆盖在途复位。 | 验证模式 | 高（文档内容） | S4 run_tl_errors_vseq检查错误且CSR不变；run_seq_with_rand_reset_vseq由can_reset_with_csr_accesses区分在途行为；有same-CSR outstanding场景。 |
| I9：现有独立Python BFM可驱动AXI-Lite读写并连接DUT。 | 工具能力 | 高（README）；兼容性unverified | S5 AxiLiteMaster / AxiLiteBus。README展示的set_pause_generator段属于AXI-Stream，不能凭该段断言Lite通道使用完全相同API。 |
| I10：可复用FIFO需要显式参数边界；参考RTL对Depth=0/1/一般深度有不同实现，Pass控制空FIFO直通，clear会忘记存储内容。 | RTL实现模式 | 高（所读代码） | S6参数声明及gen_passthru_fifo/gen_singleton_fifo；Depth=0要求Pass，直接连ready/valid/data；存在容量和已知值断言。 |
| I11：协议检查可作为独立模块绑定，而参数、时序、复位前/结束时守恒可以是不同断言。 | 验证模式 | 高（文档） | S7介绍独立tlul_assert、ASSERT_INIT、ASSERT_AT_RESET_AND_FINAL；宏可在复位期间disable。 |
| I12：CDC结构检查与timing slack不是同一证据。 | 工具方法边界 | 高（AMD说明）；跨工具泛化unverified | S8正文：CDC报告检查跨域结构及约束，标示潜在亚稳态/一致性问题，不提供slack。 |

上述没有性能、跨版本兼容或具体产品失效的已验证结论；这些类别未获得两个独立publisher的支持，不可提升为事实。OpenTitan各页共属一个publisher，不算独立复核。

## 建议实施合同（研究者推断/待产品决策，不冒充规范）

1. **最小纵向系统**：单时钟AXI4-Lite入口→AW/W分别捕获与配对→单一CSR请求/响应→静态无重叠地址译码→UART、GPIO、Timer→IRQ聚合。先限制outstanding容量并文档化；这利用I1/I5的许可，不是“Lite只能单笔”。CSR提交脉冲与AXI响应等待分离，避免BREADY/RREADY拉低重复触发副作用。
2. **Ready/valid primitive与FIFO**：写清延迟、吞吐、是否空直通、满时同拍pop+push、reset/flush优先级、非法Width/Depth、Depth=0是否支持。建议第一版注册切片用于AXI外部边界，内部通用FIFO可另有直通模式；I2/I10说明“普通FIFO可连线”不等于“AXI边界可任意组合穿透”。
3. **CSR精确定义**：每个寄存器列地址、宽度、reset、RW/RO/W1C/读清等类型、WSTRB处理、保留位、非法访问响应、读快照与副作用发生时点。地址窗口不命中按译码失败；窗口内空洞、RO写、未对齐读写分别作决定。建议未对齐CSR访问返回错误且无副作用，但这不是从S1推导出的通用AXI禁止未对齐命题。
4. **中断**：建议event pending采用`next=(pending & ~sw_clear_mask)|hw_event`使同拍新事件保留；这是建议的hw-set-wins合同，明确与I6参考实现不同。若要数清多次事件，单bit pending不作计数承诺。status/level IRQ另定语义；mask不得意外清pending；test寄存器、总IRQ与软件清除交互列真值表。
5. **UART/GPIO/Timer细节**：必须决策UART空读/满写是否报错、RX pop提交点、溢出事件与清除碰撞；GPIO输入同步、边沿/电平IRQ及输出reset；Timer计数宽度、回绕、compare相等/大于等于、使能与写计数/比较值同拍优先级。上述是需要规格化的设计问题，本轮没有为任一具体UART/Timer实现背书。
6. **复位**：推荐系统同一reset域、清理已收AW/已收W及待回响应；测试仅AW、仅W、完整提交待B、已AR待R、R/B受阻等切入点。要区分“复位丢弃未完成总线事务”与“已提交UART TX等外部动作可撤销”；不承诺后者。若保留外设而仅复位bridge，恢复/重试/重复副作用必须另立合同。

## 建议验证证据（推断，依据I1–I12）

- 实际发出的RTL接独立BFM和独立scoreboard；BFM候选S5，先用固定依赖验证Lite每通道pause控制。随机种子可复现；AW先/W先/同拍、任意间隔、B/R持续backpressure、读写交错、各byte lane及zero strobe、地址空洞、未对齐、CSR碰撞、复位都进入覆盖表。
- RTL属性：stall期间稳定；无无请求响应；写提交前恰好收齐一份AW/W；每对一次副作用；响应顺序与计数守恒；FIFO occupancy界限/顺序/不丢不重；译码onehot-or-zero；错误无副作用；W1C同拍set清除；reset不产生旧响应。复位期间禁用的属性另补复位前/后检查，避免空证明。
- 安全性质可不假设下游最终ready；活性/有界延迟证明必须显式公平性或延迟上界，不能通过“永远ready”假设伪装覆盖backpressure。保留cover轨迹证明边界可达。
- 功能仿真与RTL等价只说明两模型在既定条件下一致；建议把它作为一层，与外部协议属性、独立BFM、综合和物理约束检查分别报告。两模型共享错误不构成协议合规证明，这是证据逻辑而非工具性能事实。
- “单时钟内部”仍需列出UART RX/GPIO/外部IRQ等异步输入假设。CDC结构、复位释放、同步器实现约束和MTBF/物理时序需各自证据；本轮未证明任何同步器安全，也不以逻辑形式证明替代布局布线后的STA。

## 未解问题与跟进线索

- 尚未固定OpenTitan和cocotbext-axi提交；时间窗口≤2年未能凭页面发表日期确认。进入实现前固定revision并复查；工具版本兼容需≤1月证据及独立publisher支持，否则维持unverified。
- S5 README的pause API不能直接移植成Lite用法结论。下一轮读AxiLiteMasterWrite/Read和测试代码，验证分通道停顿、reset取消语义以及异常请求是否被BFM自动规范化；自动对齐BFM可能遮蔽原始误对齐针脚测试，需raw-channel补充。
- 查Arm官方更新版与历史版AXI4-Lite规则的差异；本轮只对IHI0022H的已读条款负责。
- S3中断实现图和S2通用subreg合同不可未经核验混同；下一轮读取prim_intr_hw实际RTL确认event路径。
- UART/Timer具体硬件合同、IRQ碰撞、非二次幂FIFO参数覆盖和外部BFM交叉互操作尚无运行证据；任何“已协议认证”“已综合可用”“满吞吐”声明均不受本摘要支持。
