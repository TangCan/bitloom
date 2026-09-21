# Story128.5有限形式与负控制设计

状态：ATDD期间的验证设计；未运行DUT，无PASS声明。UART API/线端行为以atdd-checklist和故事为准。

## 安全性质分组

1. CSR：独立pending/response ghost，初始reset唯一初始化假设；submit=valid&&!pending&&!rst，消费沿不refill，stall保持，reset取消。响应error与快照由六地址、独立byte mask、沿前配置/忙/队列flags决定。禁止把DUT ready等于模型ready写成假设。
2. 配置：独立32位CTRL/DIV recurrence；effective配置先于同沿启动，busy使用沿前状态。有效busy BAUD同值仍错，CTRL同值允许；无有效mask旁路。可用状态对应断言强化归纳，但不能用assume强制对应。
3. 事件：独立flags=(flags&~clear)|new_events，原始事件在reset为0；四来源需连接实际接收提交/发送取队首/overflow/framing，不以自证raw接sticky作为唯一性质。IRQ局部组合由RTL覆盖，形式不冒完整SoC证明。
4. FIFO：占用ghost范围0..4、前态满拒push/空拒pop、同时push/pop及reset；软件副作用仅成功commit。复用FIFO已有proof不能替代UART接线和提交门控。观察内部wire允许，但独立黄金不得从DUT next-state复制。
5. 串行：start/data/stop周期与RX半周期从独立数学契约导出；针对DIV>=3代表值做有界可达/采样性质，全DIV保留32位减计数近终点和H算术。需在报告明确哪些归纳prove，哪些仅bounded，不能以几拍证明整帧或2^32周期运行。

## 必达cover

至少实际到达：成功TX启动、RX成功字节、低stop framing、RX满overflow、busy拒绝、有效配置同沿启动、EVENT同位clear+set、长stall中事件、reset取消后恢复。长串行cover若成本过高可由真实RTL覆盖该项并明确形式子集，但不得标未运行cover为PASS。

## 原始综合与负控制

原始无observer design.v 单独hierarchy/proc/check/synth/check/stat，记录状态元件、无latch/多驱动。不得把observer综合结果说成IP面积。

至少三类实际DUT变异：串行采样/周期偏一；TX或RX的提交/FIFO门控错误；配置拒绝或事件clear优先错误。保持observer/黄金不变。每项原控制PASS，变异需命中特定行为断言、非零退出且有效波形；formal故障严格exit2+FAIL+指定property+反例VCD，编译错误/超时/UNKNOWN均失败。若采用RTL断言负控制需同样核对指定断言而非任意失败。

## 完整性与归档

runner绑定唯一运行、全部固定seed及执行数、起终源码指纹、实际工具身份；SBY固定官方标签对象/commit/安装模块字节，不能只相信--version。每命令记UTC/退出/原始log，launch失败亦记录。清理前保存RTL/TB/trace/证明/综合/JVM产物且独立逐成员hash复核；最终clean后重新核archive SHA。普通与python -O防空过用例覆盖缺测试/缺seed/缺故障证据。
