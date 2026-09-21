# Story127.4 build 初始失败与修复

本文件为实际终端结果索引，不冒充完整原始终端转储；工具自身完整日志/RTL已保存在归档中。

- 首批5 native/API行为通过，真实七模块和C11两个初始ATDD入口通过。这只是初始GREEN，不是完整验收。
- `routing-1901242`：SBY prove depth8 basecase PASS、induction FAIL，因此最终UNKNOWN/exit4。没有宣称PASS；无限叶等待下任意归纳初态无法仅靠有限端口历史同步ghost与实现。增加`busy==f_pending`、活动事务miss/owner对应**assert**后，基例及归纳均PASS，没有添加完成性/ready公平性assume。后续cover恢复位按同笔消费清除。
- `stress-1913013`：Icarus exit2，SV常量后紧接`?`被词法分析吞入常量；显式空格修复测试bench语法，未改产品。原失败compile.log与tb.sv保留。
- 初版dual-native测试生成DualDecoder含两个Clock/Reset，freeze如实返回E0120/E0121。按同域合同改为共用clk/rst，并以A消费时B继续DECERR保持验证独立状态；没有扩多时钟能力。首次诊断仅在本次工具终端输出，不声明具有单独原始日志。
- 首版源码有unused GroundType warning，移除无用import；最终批次无该warning。
- 预期负例不是产品失败：结构检测catch_unwind、非法producer、同沿consume/refill以及三种RTL突变须失败于明确断言，control须PASS。对应退出码和反例/日志分别归档。

每次工具重跑使用不同进程PID目录，保留UNKNOWN/compile失败及修复PASS。后续补强改变的是测试覆盖（AR接受之后、CSR提交之前的快照变化；随机独立coverage；合法接受沿payload monitor等），不降低合同。
