# Epic 125 / M0 关闭记录

2026-09-20，FR192/FR193关闭；仅本次获准的M0完成。Phase24整体尚未交付，FR194–201/Epic126–130 backlog；FR189/Epic122 deferred和NFR91保留。

## 交付与证据

- Story125.1：正式合同、22故事/6Epic、AD30/31、CSR接口与NFR14、gate；提交 `0e2a873961988f3c76222d193c402017ec4d8253`。
- Story125.2：旧设计3场景×两native/真实RTL共9个预期失败，BFM API独立stub探针；红测日志永久归档。提交见下文版本记录。
- Story125.3：AW/W独立缓存、并发读写及read-before-write、FL修复；原9项回归转绿。旧8位地址/四寄存器映射、WSTRB/OKAY与同拍一tick响应保持。

[绿色行为证据](../../docs/ip/phase24-axi-green-evidence.md)：定向24,119拍；随机16组各1000写+1000读，另各4笔最终读回，共288,245拍。Interpreter/Compiled/手写FL/direct Verilog实际执行均通过。三路review补齐末尾读回、覆盖计数、held reset/首拍恢复及失败向量保存；未发现待修复产品缺陷。

[验证汇总](../../docs/ip/evidence/phase24-verification.json)：工作区1690 passed/0 failed/6既有ignored，2个长矩阵单独运行通过；合并唯一Rust测试1692通过。fmt通过；prelude/sim/firrtl各196个SemVer检查通过。没有将适用性skip或现有ignored称为已执行。真实RTL缺工具的负向检查返回失败。

本次不证明该AXI矩阵在Chisel/FIRRTL路径执行、formal、综合/PPA或商业VIP完备性；不交付新CSR系统/外部核，不发布、不推送。

## 下一执行窗口与重估

建议下一窗口为126.1 NFR14 → 126.2共享模块定义，再推进注册切片/FIFO；尚未启动。M0已验证Icarus、原始通道驱动与固定BFM API兼容；同session组合、CSR生成和稳定版外部核准入仍未实现或估测。

剩余M1–M5基础估算：43.5–69有效人日，其中核心38–60、外部5.5–9；加25%预留约55–87人日。此区间沿用逐故事估算，不能把代理本次墙钟时间换算成人工生产率。126.1进一步验证模块生命周期/参数复用后再收窄估算。后续保持backlog，需后续安排；各Epic自身NFR14 gate仍有效。

版本记录：Story125.2 `ac6e4804578b6c53a4eddbd164f54fad57554766`；Story125.3 commit由本文件所在提交及subject定位，避免写入自引用hash。
