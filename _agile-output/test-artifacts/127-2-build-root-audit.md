# 127.2 build 主代理核验

2026-09-21；基线77ea01392e5be74b9f2622009ad146edb2468ea9。首次实现已由主代理核对代码/测试/文档与原始日志，不以实现代理报告替代检查。三路review统一裁决在[spec](../implementation-artifacts/spec-127-2-csr-product.md)；后续修补尚在执行。

- 合法性入口先canonical/validate后调用builder；prevalidation可恢复与E0244 poison测试分开。编码完整字符串/字段/owner/event/reject，不使用摘要或捕获闭包。
- RTL仅现有builder；私有寄存器pending/data/error与leaf-owned状态，external无副本；候选值独立于拒绝/握手。single-slot消费沿不再提交；读值锁存提交前值；W1C自然事件每拍累计。
- 默认测试实际双native引擎与真实Icarus，沿前先settle、沿后再比较；peer是实际第三模块，软件提交与自主递增在唯一状态更新处选择。第一reset沿前不比较未初始化RTL，之后全部比较。
- 独立软件黄金+C11真实编译；重排相同HIR/RTL/Markdown/header；文档原文prelude-only执行。
- formal observer只读端口，初始reset和合法producer稳定假设，无DUT输出正确假设或ready公平性。首轮拼接括号语法错误保留，修补未改属性；归纳和cover分列，无observer综合单列。
- 初始工具归档SHA256 086136a54e9f867de5bc9dc3c503e0f1801136710b2cfe1ba59eec7e26ebd1bb；158成员。主代理读取首次proof ERROR16及最终prove/cover PASS状态，并于修补前复核25个build manifest条目均OK。后续修改后这些摘要仅证明初始快照，不能冒作最终源摘要。
- 首次实现实跑：功能/配置10、codec3、真实formal/综合2、旧M0/M1回归62均通过；旧回归5个result blocks合计62/0/0。每条命令/exit在build-commands.json。未重复不受修改影响的回归。

## 审查范围与决定

三路按skill以无先前对话子代理完成：csr_build_blind、csr_build_edge、csr_build_verification；全部启动后收集，全部结果后统一裁决。blind10条、edge1条、verification1条，每条在spec有独立判定；B1/E1同因合并，B3/V1同因合并，其余不丢弃。全部可用小幅实现/测试/文档修补，不新增公开API。

C宏固定拼写来自已批准ATDD契约。组合边界采用“完整生成宏集合必须互不重叠”，而非任意不同block字符串即可；在既定拼写内增加编译时显式诊断与碰撞负例。不会悄悄改变宏编码或宣称单块validator能看到其他头文件。

当前尚未完成独立code-review、automate、最终clean/fmt/just test与提交；story/sprint保持未done，M2/FR196整体不关闭。
