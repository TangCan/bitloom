# Story127.2 automate 完成清单

依据 `.agents/skills/bmad-testarch-automate/checklist.md` 全部适用项逐类核对。

- [x] Create/BMad-Integrated；resolver/config/context/现有ATDD及review已读取，framework=Cargo存在。
- [x] AC3/4/7风险映射；19既有功能/config、3codec、3formal/综合基础上补3项负例敏感性，不镜像产品实现。
- [x] runtime能力探测auto→subagent；独立API/backend workers，唯一文件所有权，完整JSON成功后聚合落盘。
- [x] P0前缀、描述性命名、固定刺激、正控制、准确fatal/FAIL/exit与反例；编译失败/工具缺失不接受。
- [x] 既有factory/monitor/oracle复用；新增2个helper限定测试文件；目录label/PID隔离，无共享DUT状态。
- [x] 28项真实定向PASS，其中4项专用formal实际执行；0非预期失败、0healing；完整命令/版本/退出码与时间已记录。
- [x] 既有CI --ignored全formal target会包含新测试；默认workspace无需SBY；无新tool pin或依赖。
- [x] 源码与工具tar/成员摘要回读核验；无普通历史源代码副本；历史证据不变。
- [x] scoped rustfmt check、git diff --check成功；全workspace/clean交下一步。
- [x] 范围诚实：Probe限定、singleton仅综合、输出mutation非完备score、standalone未验。
- [x] 输出汇总、执行指令、文件列表和下一步骤齐全；状态/提交由主代理负责。
- [x] UI/HTTP/TS/package.json/Pact/data-testid/auth/network-first/MCP browser全部N/A；无浏览器进程。
- [x] 硬件适配：固定地址/mask黄金优先于faker，精确模拟周期不是墙钟hard wait，保留证据优先于删除工具产物。

最终on_complete结果另见127-2-automation-on-complete.log。清单完成不宣称story/M2/Phase24 done。
