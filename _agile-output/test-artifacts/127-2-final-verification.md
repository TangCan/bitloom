# Story127.2 最终验证与七步关闭

2026-09-21；基线 `77ea01392e5be74b9f2622009ad146edb2468ea9`。本记录随单故事提交关闭127.2，Epic127继续in-progress；只交付FR196的静态CSR描述、组合叶节点及软件产物子集，桥/译码/M2/Phase24未关闭。FR189/Epic122 deferred及NFR91不变；未push/publish，未改包版本或工具钉。

## 七步与实际结果

1. create-story：8条AC、single-owner/候选值无环/同沿commit/软件命名/真实工具矩阵落盘；127.1/126.2前置done。
2. ATDD：12入口；首次真实编译红仅缺CSR API的3项E0432。语法噪声与后续observer括号修复均留原始日志，未把编译红冒充硬件行为验证。
3. build：实现固定5类型/5方法、完整规范codec与无捕获共享定义体，RW/RO/WO/W1C、单响应槽/快照/动态拒绝/外部唯一owner；Markdown/C同源生成。三路内审全部逐项裁决、修复，另纠正一帧producer刺激，性质不削弱。
4. code-review：四层均完成；10修补、1 low拒绝（同大写include guard的不同定义已明确不支持；保留固定guard/API）。补多field硬件、更多read/write拒绝、实际producer monitor、单例综合、C消费者、文档边界；历史源码压缩且逐字节验证。无未解决high/medium、无延期。
5. automate：新增3个P0；21功能/config+3codec+4实际formal/综合=28项定向PASS。实际15个producer违约及1个RTL高位突变均vvp exit1+准确fatal，7正控制PASS；原版形式证明PASS，events_value置零突变SBY exit2/FAIL、第3步命名性质反例。缺工具/ERROR/UNKNOWN不会记为负例成功。
6. **完整回归**：`cargo clean && cargo fmt --all && just test` 实际exit0；清理28213文件/9.9GiB，463个结果块，**1787 passed / 0 failed / 18 ignored**。开始2026-09-21T02:41:10.135006+00:00，结束02:48:21.728700+00:00；test opt-level1（保留断言/溢出检查），BITLOOM_REQUIRE_RTL=1；格式化前后CSR源码摘要相同。
7. commit：本记录与本故事所有实现/验证/状态在同一独立提交，subject含 `Story 127.2`。git full SHA由实际提交后读取；不为自引用伪造hash。

## 真实证明与ignored边界

增强Probe采用initial reset及producer合法稳定假设，无DUT正确性假设或rsp_ready公平性；depth16基例/step14归纳成功，5covers在depth24范围全部达到。另对Probe及5种singleton原始无observer RTL真实Yosys综合/check；singleton仅结构/综合，不冒充形式证明。4个CSR专用ignored入口已在automate显式`--ignored`全部运行通过，并由CI原formal-sby job纳入。默认workspace的18 ignored=4个CSR专用+8个既有M1专用+6个既有doc-test；旧M1专用在126.3/126.4已有证据，本轮未重新执行，不当作本轮PASS。

Generated Rust standalone本故事未验；native/generated层级仍unsupported，组合由实际RTL验收。无PPA/板级/无条件活性结论；本地workspace通过不等于远程全部CI/JVM/CIRCT运行通过。C头使用完整宏集合不相交的集成约束，同大写namespace不同定义禁止。

## 可核验证据

- [ATDD清单](atdd-checklist-127-2-csr-描述-rtl-与软件地址产物.md)、[固定API](127-2-atdd-api-contract.md)。
- [build](127-2-build-evidence.md)、[build修补](127-2-review-fix-evidence.md)、[主代理核验](127-2-build-root-audit.md)。
- [独立review](127-2-independent-review.md)、[独立修补](127-2-independent-fix-evidence.md)。
- [automate报告](automation-127-2.md)、[实际命令/exit](127-2-automation-commands.json)、[源码/工具tar主代理复核](127-2-root-archive-verification.json)：19源码成员、397工具文件摘要全部通过。
- [完整回归原始日志](127-2-clean-workspace-regression.log)、[命令/时间/源码摘要](127-2-clean-workspace-regression.json)、[实际exit](127-2-clean-workspace-regression.exit)、[最终计数](127-2-workspace-results.json)。
- [历史归档迁移说明](127-2-source-archive-compaction.md)，旧review源为tar member，验证器实际确认13源码成员及26原证据摘要无损。历史结果不覆盖，target清理不丢专用证明与反例。
