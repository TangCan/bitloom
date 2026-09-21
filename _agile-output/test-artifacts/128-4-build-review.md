# Story 128.4 构建审查

三个独立层均已返回后开始核实；blind共10项，edge共1项，verification无发现。

|编号|判定|证据与处理|
|---|---|---|
|B1|low|runner以目录差集关联工件，并发运行确会混入目录；本故事受控串行且CI工作区分离，日常不会遇到。加入锁或路径参数超过直接修正，按技能规则拒绝。|
|B2|medium|archive只读comparison记录和nonempty，没有重算生成文件SHA；归档前文件改变可误认证。patch：重算两产物并对照记录及当前金样。|
|B3|medium|archive仅检查16文件和runtime_command，未检查记录的pin或sby-runtime成功。patch：核对既有pins及运行状态。|
|B4|medium|archive positive formal只检查formal.v和PASS，缺失cover见证仍可通过。patch：要求10个cover目标关联的见证及证明日志。|
|B5|medium|formal使用--ignored但不核对3个测试实际执行，移除ignore可零测试通过。patch：核对既有3目标名称和计数。|
|B6|medium|gpio_irq_tb未在pending=16时关闭ENABLE；故事集成矩阵明确mask不清pending。patch：增加屏蔽、读pending、重使能无新沿断言。|
|B7|medium|reset前raw已在第三tick消失，当前断言不能验证有效raw被reset压低。patch：在sync2有效窗口复位，检查立即压低及接收端取消。|
|B8|medium|3个故障只过形式observer，故事明确同时检验scoreboard。patch：代表IN误接OUT变异复用原direct黄金，要求指定行为失败与VCD且控制PASS。|
|B9|low|Rust.status启动失败在写metadata前panic；外层runner仍记录失败，不会误计PASS。timeout缺失非日常环境，增加多处错误分支超过直接修正，按技能规则拒绝。|
|B10|low|两测试文件确有JVM harness复制，未来版本更新可能漏改其中之一；现有两个入口均实际运行且pin一致。假设更新失误非当前日常缺陷，共享抽取非直接修正，按技能规则拒绝。|
|E1|low|与B1相同的并发目录差集缺陷，独立判定仍为low；受控串行/隔离CI下不常见，锁引入复杂度，拒绝。|

verification-gap返回No verification gaps found。以上逐条判定后，B2/B3/B4/B5/B6/B7/B8分别作为patch处理；无冻结意图或公开API变化，不需要重新批准。历史归档保持原字节，修补后重新完整验证和新归档。

修补交接：原实现代理完成B2–B8。harness普通/-O各14通过；新scoreboard exact、GPIOIRQ direct exact、组合FIRRTL/Chisel exact各1通过。实际10个cover目标共9份VCD，校验按日志逐目标关联。主代理已审修补diff，完整runner复验进行中；历史归档不覆盖。
