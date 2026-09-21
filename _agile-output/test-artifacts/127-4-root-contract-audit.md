# Story127.4 主代理合同核对

2026-09-21。仅设计与验证策略审计，不是行为、形式或综合通过证据。基线91828619f66f787c63483dda4b0ee558bb7501e8。

已完整核对127.4故事、创建清单、Epic127四故事、phase24合同CSR段、epic127 context和实际bridge/CSR接口。固定CsrDecoder API与52个端口符合现有单session接口；不增加可配置map，避免无依据扩大维护面。

| 风险 | 独立验收要点 |
|---|---|
| 高地址别名 | 全65536地址驱动DUT，并以完整范围比较的手写golden核验。0x8104不能转成GPIO0x0004；未命中检查所有叶valid均0。 |
| 低位被对齐 | 0x01ff仍传local0x00ff，由叶SLVERR；0x0401是DECERR。先判断窗口，再形成local地址。 |
| 副作用提交被移拍 | 命中上游握手与唯一叶握手必须同沿；不可新加队列后仍沿用提交时点声明。 |
| 实时地址劫持响应 | 提交后改变上游addr/write及非owner输入，响应只能来自锁存owner。命中与miss pending都要背压。 |
| 非法producer制造伪缺陷 | valid受阻时保持全部payload，READY升高的接受沿也检查；reset清保持历史。至少一个真实monitor负例。 |
| reset与消费竞争 | req_ready/down_req_valid/down_rsp_ready在rst时0；up_rsp_valid沿前可以仍高，不能在reset沿计入消费；共同reset后恢复另一窗口。 |
| 测试peer冒充外设 | 四CsrBlock使用正式地址/权限/mask，peer只提供动态状态/event/reject，不声称UART串行、Timer计数等已实现。 |
| 对AXI背压的虚假覆盖 | B受阻区间内实际完成读、R受阻区间内实际完成写；需独立AW/W/AR并发补充串行随机预算。 |
| 形式过约束 | 不假设叶响应等待上界或上游最终ready；失败写无关rdata可非0；内部对应关系只能assert，不能assume。 |
| 结构门禁逃逸 | 原始无observer组合网表检查5个外部握手输出无输入组合路径；FF时钟必须clk；未知/latch/未驱动不能作寄存截止。 |

无发现需用户新增决策的合同冲突。create完成，ATDD进行中，127.4及M2未交付。

独立工具预检见127-4-root-tool-preflight.json：8条版本命令实际退出0。该证据只证明工具可调用，不替代后续RTL、prove/cover或综合。

## 实现中独立核对

已读取core CsrDecoder初版：52端口、page[15:8]完整比较、16位local减法、busy/miss/owner对应及共同reset门控与固定合同一致；静态核对不替代实际运行。基础真实组合日志31提交/31响应/26叶提交/7B/24R已保存。

首轮formal基础8步通过、归纳失败，整体UNKNOWN/exit4，保留原始日志。加入ghost↔busy/miss/owner的assert归纳辅助后，实际prove基础与归纳PASS、7cover PASS；没有添加响应时限或ready公平性假设。完整证据在127-4-root-inductive-evidence.json。尚待后续mutant敏感性及全合同验收。

实现中反馈F1：f_stalled在响应消费后应清除，以将stall→consume cover绑定同笔响应，避免前一笔历史stall使后一笔cover成立。已发给实现代理，待代码/轨迹核验，当前不宣称已解决。

实现中反馈F2：组合scoreboard先消费响应并清pending，再处理新提交，可能容忍合同禁止的同沿CSR响应+请求交接。要求在事件处理前检查两个握手互斥，并用负例/正例检验observer；已发送实现代理，待实际修补和结果核验。

实现中反馈F3：组合读快照需增加AR捕获后、CSR提交前peer值改变，现有只在提交后改变值不能排除错误的AR快照。F4：随机/并发阶段分别记录gap、AW/W先后、各握手stall与并发命中，防止定向阶段填满总计数后冒充随机覆盖。均已发送实现代理；尚未作为已解决或通过宣称。

F1/F2阶段核验：routing-1922639实际prove/cover均PASS，observer消费时清f_stalled；monitor-bubble-1925020实际exit1且精确触发CSR consume/refill bubble，control为PASS。相关中间证据50成员已独立归档回读，见127-4-root-intermediate-checks.json。stress-1913628的16种子每种1000随机+256并发全exit0，但该快照早于F3修改，不能冒称最终代码已验证。

## Step03最终核验

F1–F4已在最终差异和实际通过日志中核对解决：cover停顿标志消费清除；禁止同沿消费/重填的断言及负例；AR之后、CSR提交前改变读值且提交后再变；随机覆盖重置独立统计，并发通道真实停顿均非零。最终14普通与4专用入口、91普通兼容及7专用兼容均通过。内部审阅、独立code-review、automate和最终clean/fmt/workspace仍待执行，不关闭M2。
