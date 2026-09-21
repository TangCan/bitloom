# Story128.2 Timer 最终验收

2026-09-21，七步已执行至最终单故事提交。交付固定32位可组合Timer、同源C/Markdown和prelude-only示例；为实际Timer Chisel路径修复大UInt literal/初始化Mem、Reset位运算和1位Mux选择器类型转换。只关闭128.2子集，FR197/M3/Epic128/Phase24仍开放。

## 七步与证据

|步骤|实际结果|记录|
|---|---|---|
|create-story|7项AC、单owner、字段/时序/工具合同固定|128-2-create-story-validation.md|
|ATDD|8入口、两个target实际E0432 RED；未冒充行为全部红测|atdd-checklist-128-2-timer.md、128-2-atdd-red.json|
|build|实现与实际三后端/形式/综合；三路内审11项修补、1既有defer；主代理复核全绿|128-2-build-evidence.md、128-2-review-fixes.md、128-2-root-recheck-*|
|code-review|四层9项修补、1既有defer、1修改规格建议驳回；实际源码/安装/CI及边界证据|128-2-code-review.md、128-2-code-fix-evidence.md；主代理621文件/8源码SHA审计|
|automate|新增1 P0直接RTL检查器负控制：原始PASS、2个DUT变异被准确字段FATAL检出|128-2-automation-summary.md、128-2-automate-validation.json|
|regression|实际cargo clean → cargo fmt --all → just test全部exit0；473块、1833 passed、0 failed、30 ignored|128-2-final-regression.json、128-2-final-test-summary.json及各log|
|commit|本故事以单独feat(ip)提交，哈希由git log记录|故事128-2-timer与总账|

完整回归耗时917.75秒，just test为910.61秒。clean实际删除32853文件/13.7GiB；fmt前后7个相关源码/CI SHA一致。30个ignored不计通过：本故事5个专用入口（Timer后端、双实例后端、formal安全cover/原始综合/故障注入）均在专用命令实际通过，另外25个为既有忽略项，不能据此声称本轮重跑了全部旧专用验证。

## AC与行为验证

|AC|证据结论|
|---|---|
|1 组合/API|13端口、独立与共享实现一致、同/不同定义双实例、私有名冲突、宽/型/方向明确诊断；prelude-only原文例实际编译|
|2 地址/描述|4个局部地址、WSTRB全部16种、保留位/高位/未对齐/错误响应；C/MD同源与独立关键地址黄金、C11实际编译|
|3 完整32位|周期/一次模式、关闭/再使能、COMPARE0回绕、COMPARE最大值近边界命中、比较值低于当前值；不缩位宽|
|4 软件优先|CTRL/COUNT/COMPARE有效写独立抑制、同值/零值写、零WSTRB与CTRL保留字节不抑制；formal三类独立cover|
|5 事件|沿前raw事件、粘滞EVENT、set胜clear、连续periodic COMPARE1；formal及独立黄金|
|6 背压/reset|提交前读快照、消费不refill、持续背压、取消账本、两实例并发/非对称背压；安全证明无最终ready假设|
|7 真实验证|三后端同黄金、两pair图跨后端、SBY来源安装绑定、原始综合、SemVer minor及全workspace兼容|

三个seed为128219705511/deadbeef1282/73592401ffff，每个后端6610/6610/6611帧，共19831帧；每seed470次raw match。提交/消费/取消分别2754/2749/5、2727/2723/4、2739/2733/6。边界用软件配置接近上限后自然计数，不声称经历2^32周期。

形式原始归纳安全/control PASS、10 cover可达；3个只改DUT的变异分别触发timer_match/timer_event/timer_count指定basecase失败并有VCD，工具/语法错误不算检出。原始无observer综合614cells且有DFF无LATCH，不是PPA或板级时序结论。Chisel类型夹具实际检查u64高位/MAX及初始化Mem，Reset AND/OR/XOR左右顺序；numeric矩阵13 Rust入口/18 JVM RTL case通过。just semver-check三个surface crate的minor模式通过。

CI已持久接入Timer专用后端/pair/形式/文档/C门禁，形式前必须实际核验来源和安装文件；python -O及PYTHONOPTIMIZE下真实安装通过、篡改拒绝。此处报告本机等价命令和YAML检查，不冒充远端CI已经运行。

## 审计、范围与下一步

五阶段原始归档在cargo clean后再次核对SHA与成员数，见128-2-post-clean-archive-audit.json；历史失败、不同源码阶段及主代理独立复核保留，不覆盖旧结果。最终源码SHA见final-regression.json，后续只更新文档/状态。

native Interpreter/Compiled/generated对Timer内含CSR层级明确unsupported，已验证拒绝，不新增仅供测试的无层级产品路径。Chisel多位UInt Mux选择器为旧HEAD已有缺口，已记deferred；当前Timer使用1位。CTRL描述保持已固定的bits字段，bit0/bit1含义已文档化。新增公开面逐符号FR142/minor，未改版本或发布。

09-21T08:45:25Z官方223条release复核仍无严格高于firtool1.159.0的正式版本，FR189/122.2/122.3继续deferred，记录128-2-fr189-upstream-recheck.json；不将其计交付。下一故事128.3事件IRQ继续七步，整个目标保持active。
