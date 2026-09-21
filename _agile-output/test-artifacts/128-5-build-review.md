# 128.5 build 内审

按本故事已渲染的 bmad-build step04，三名全新同能力 reviewer 同时启动，全部返回后才逐项核对和分流。Diff：`/tmp/bitloom-1285-build-review-yox3baa2.diff`，包含基线2f52d9969f60d08cc633e30e0c263f05854b39fe之后产品、测试、CI、文档及原始证据索引；二进制以Git binary条目表示。主代理重读产品/测试/工具与矩阵，并独立逐字节校验3196归档成员和570源码；大型生成哈希索引采用逐项机器校验，不声称逐行人工阅读全部哈希。

Blind reviewer提出10项；edge reviewer提出1项；verification-gap报告无缺口。每项独立结论、证据与路线完整保留在spec的Review Triage Log。两项patch：空闲timer保持、必需门禁原始日志存在性检查；其余9项按证据驳回，包括准确标注的有限形式/极值证据范围，以及异常环境下收益低且需增加复杂度的扩展。没有intent_gap或bad_spec，无loopback，无新增defer。

修复已交回原implementation代理，不能先声称修复验证完成。修复后主代理运行完整Verification并生成新的源码绑定归档；旧consolidated归档不可覆盖，保留为审查前历史。M3最后任务由用户七步顺序安排在最终回归后，不提前关闭或提交。

## 定向修复验证与记录限制

原实现代理完成4文件最小修复：两idle保持mux、新idle保持形式断言、必需command log存在性检查（JSON-only comparison例外）、逐一删除14日志的回归。其定向direct三seed与safety/cover通过；原始子工具工件位于`target/fr197-uart/direct-1413850-1789998813657840925`和`target/fr197-uart-formal/proof-1413142-1789998786399830082`。外层Cargo/harness stdout当时未重定向，仅保留会话工具输出，不能声称存在外层原始文件。

首次harness因PATH缺sby而10 tests/7 errors（两个身份/缓存测试多子场景，Path(None) TypeError）；恢复约定PATH后normal与-O均10/10通过。这是环境失败，不改写成产品失败或抹去。主代理随后重新运行两模式并将完整输出保存为`128-5-build-review-harness-{normal,opt}.log`，最终结果以主代理实际退出记录为准。主代理完整39命令复验运行目录`target/fr197-uart-reruns/20260921T135428.448283Z-1415681`，尚在执行。

主代理留存的normal与-O检查均退出0，分别10 tests/3.906s和10 tests/3.855s。修复后完整run已通过普通15、formal3、全部四个后端入口；原始综合1431 cells，新增idle-hold断言归纳通过。完整run最后维护门仍待结束，不提前认证。

## Build内审关闭

修复后完整run39命令全部PASS、570源码起止一致；普通15（4后端入口另实际执行）、formal3、四后端入口、legacy24、FIRRTL21、numeric、SemVer、原文例/软件产物/C11全部通过。631成员新归档SHA256 `394efe3aa88688493b26f0581a59d326e5aa66211701e2e3cb56b25d4428813a`。两项patch均完成，9项reject，无未决与defer。spec done仅为build技能完成，sprint/story进入review；用户七步顺序覆盖默认提前commit，M3最终任务仍保留未勾选。
