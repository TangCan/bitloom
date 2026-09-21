# 127.1 主代理独立人工审阅

日期2026-09-21。实施者为csr_gate_implementation；本审阅由主代理逐条读取正式合同、风险全文、ATDD和真实输出后作出，不以字段出现替代内容判断。并参考已完成四层独立review；不是产品验收。

| 项目 | 结论 | 实质理由与证据 |
|---|---|---|
|M01|通过|完整状态中125.3/126.1–4 done，127.2–4 backlog，122.2/3 deferred；风险正文(a)分别回链M0/M1，FR196未交付。|
|M02|通过|正文(a)–(d)有owner Richard、Codex执行、0.5–1及12.5–19人日、各失败修复与停止/放行条件；维护面覆盖生成物/组合/旧bank。|
|M03|通过|对照builder define_module完整参数+非捕获fn签名；编码完整描述/名称，缺失编码诊断，独立golden/C消费者；动态状态明确单owner，Timer自主更新与GPIO别名写可通过wrapper合成，不要求第二份存储。尚未实现已明确。|
|M04|通过|唯一非reset提交，16/32/4/2位接口、提交快照、下一周期响应、背压不重交及reset优先均与合同一致。|
|M05|通过|WSTRB字节选择、RO/WO错误、失败读0、保留位、零mask与TX byte0例外、W1C set优先正确；错误无访问副作用不阻止自然硬件事件。|
|M06|通过|明确0x100窗口及0x0400–0xffff DECERR，窗内SLVERR，无高位别名；黄金边界与127.4责任清楚。|
|M07|通过|AW/W/AR独立槽，完整写资格，B/R预留/保持，轮转初始读优先、提交后翻转，满响应类排除；新增结构依赖锥检查确认寄存握手。|
|M08|通过|永久背压只承诺安全，活性列公平性；共同同步reset/取消矩阵、物理发送不回滚和有限leaf均明确。|
|M09|通过|ip/axi.rs旧ADDR8 bank保持无diff，风险正文单列兼容时序，prelude/HIR/PROT/层级unsupported边界正确。|
|M10|通过|build日志逐命令UTC/exit/path/版本，固定包metadata成功，pip失败保留；GPI编译版本与运行查询区别另证。SBY/Yosys/Z3仅发现，不冒称CSR proof。|
|M11|通过|两次真实BFM exit0，各自JUnit/stub已归档；主代理解析均恰1test无错误/失败/跳过，六项历史hash直接核对。raw驱动与高层BFM对齐限制明确。|
|M12|通过|工具缺失/表达冲突/RTL分歧/新范围都有责任和动作，pip查询问题已正确metadata替代；没有未解决架构/工具发现阻塞。|
|M13|待验|create/ATDD/build/code-review已完成；automate进行中，clean/fmt/just test与单故事提交尚未执行。不能提前判七步通过。|
|M14|通过（提交前再核）|当前产品/工具pin/API无diff；跨故事依赖人工核对epics一致，gate不覆盖全部依赖的限制明确，无FR196/M2完成宣称。|

风险内容本身有效，后续ready仍必须等待127.1七步done。最终回归/提交证据追加后才能关闭M13；本文件不将全部故事标done。

## 最终补验

2026-09-21：M13除提交记录本身外各阶段已核验，automate117场景及BFM真实通过，clean/fmt/just test退出0，1763/0/14。M14最终产品diff仍为空，127.2–4 backlog、Epic127未关、FR189延期保持；单故事commit在最终步骤生成，包含本文件的git提交完成M13。
