# Story128.4 独立四层code-review

四层均已返回后判定。verification-gap无发现；acceptance-auditor无AC违背，最终automate/全回归/commit仍待。用户连续七步授权涵盖全部无歧义patch，不再请求重复许可。

|ID|来源|判定|问题|核实与处理|
|---|---|---|---|---|
|1|blind|low|并发工件归属|目录差集并发可混入；串行受控运行/CI隔离下罕见，新增锁或路径参数复杂，拒绝。|
|2|blind|medium|ATDD必须执行清单|atdd仅exit0；误ignore可绕过所需测试。patch核对11个普通目标及计数。|
|3|blind|medium|三seed证据完整性|archive direct-只要求非空类，删除两seed仍可通过。patch要求三个seed/accounting覆盖与成功日志。|
|4|blind|medium|scoreboard反例归档|archive未要求scoreboard-mutation目录，遗漏后仍认证。patch校验control和指定行为FAIL及VCD。|
|5|blind|medium|组合与后端运行记录|archive只要求源码/VCD，不要求日志/执行退出。patch要求已有成功marker及命令记录。|
|6|blind|medium|形式反例的实际失败原因|archive仅FAIL/非空VCD可丢失指定性质和退出证据。patch复核已有Rust要求的exit2/性质/VCD。|
|7|blind|medium|GPIOIRQ同沿清除和新事件|组合夹具未覆盖两接收端同时清除与真实rise。patch同一三后端夹具增加新事件胜双clear。|
|8|blind|low|增加时序变异种类|既有三种代表故障满足代表性负控制合同，时序由独立逐拍/形式断言覆盖；未展示盲点，额外变异和分支非直接修正，拒绝。|
|9|blind|low|内部launcher失败元数据|内层启动失败在metadata前panic，外层仍记录失败；timeout缺失罕见，新增多处分支超过直接修正，拒绝。|
|10|blind|low|JVM harness抽取|复制可能让未来修改漏同步，当前两实际入口同pin均执行；尚无日常分歧，抽取复杂，拒绝。|
|11|edge|low|并发工件归属|独立判定同1：共享目录差集确可受并发影响，但当前受控串行/隔离CI，锁引入复杂度，拒绝。|

逐条判定后：6 patch（2–7），5 rejected，0 defer，0 decision-needed；分别根因独立，不合并。

6项patch均已修补；主代理差分核实，针对性harness普通/-O各17通过，组合direct与FIRRTL/Chisel exact各1通过，原日志128-4-code-review-fix-*保留。完整runner复验已启动，结果另记。code-review动作完成，故事/sprint仍review至第六步最终回归，七步未完不提前done；用户既有授权覆盖技能例行确认及默认状态流转。

最终完整复验：20260921T121311.335406Z-902189全部gate通过且源绑定一致。678成员新归档SHA95f6b137604dd5064e4536c9519a532f355871a2dce07e9dcb2df8a56df852d9，主代理独立原字节复核见128-4-code-review-root-audit.json。6修补完成，无未决项。
