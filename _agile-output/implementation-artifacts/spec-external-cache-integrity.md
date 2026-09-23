---
title: '修复外部IP缓存事务与路径边界'
type: bugfix
created: '2026-09-23'
status: done
route: oneshot
review_loop_iteration: 0
baseline_commit: '063991d2274ed0e5e75c7bd74ebdc949a8598ead'
context: []
---

<frozen-after-approval reason="用户明确要求修复复盘M1/M2">

## Intent

**Problem:** FR199锁定发布失败会删除复用的既有闭包；离线验证只检查末级目录，缓存祖先符号链接可越界。用户要求修复两个已核实问题，并复评Epic130。

**Approach:** 以本次是否创建闭包作为回滚依据，逐层拒绝缓存路径中的符号链接；通过本地确定性失败注入、实际CLI反例及正常禁网重放确认旧缓存保持和合法流程可用。保持AD-31、锁格式、公开API、工具钉及原历史证据；不下载厂商软件、不push/publish。只处理M1/M2，文档/追踪M3/M4保持提议。

</frozen-after-approval>

## Implementation Notes

- 无意图缺口、无不可逆外部操作；改动external_ip.rs及external_ip_tests.rs私有实现、回归与本轮证据/复评。采用oneshot，无新公共接口；未创建新Epic故事，不改旧22个故事提交。
- 独立调查确认两处回滚都必须用闭包创建所有权；新锁也可能共享旧闭包。提取现有发布/许可证/验证收尾为私有函数供真实文件系统测试，禁止测试另造一份清理实现。
- 测试既有/新建闭包许可证失败、复用闭包验证失败、正常提交；逐层链接、末级链接和正常树。Given已有有效缓存，when失败重锁，then已有字节不变；Given祖先链接，when verify，then明确path错误；Given合法缓存，when禁网重放，then实际编译/行为通过。
- 不宣称防御恶意并发改写文件系统的TOCTOU；缓存由本地调用者控制，现有锁格式与非并发使用约定保持。

## Review Triage Log

独立Blind Hunter：11426字节，floor=4；四项逐一核对后均为medium/patch，无缺层（oneshot仅配置该层），无延期项。

- R1：publication的`?`绕过暂存清理，静态链接拒绝可实际触发。改为错误分支只移除本次staging_paths及暂存许可证，保留旧闭包；增加回归。
- R2：cache anchor检查晚于fetch写入，链接可提前重定向写。入口在fetch前验证cache/closures，测试调用实际lock并确认链接目标仍为空。
- R3：新增锁写失败回滚未覆盖。以非空目录作锁目标，验证new/reused两种所有权和目标内容保持。
- R4：链接负测只经过verify，不能保护publication。补直接publish_closure_cache的cache/closures/digest及悬空链接负测，确认不动外部目录。

初轮5测试2通过/3失败确认旧实现缺口；修复后5/5，审查补齐后9/9。初次正常CLI与禁网编译/binding/行为重放通过，但审查后需以最终源码再次验证，不复用旧二进制结果。

审查后独立复核确认原四项全部解决（review-followup.json）；最终源码CLI重建后再次复现祖先链接明确拒绝，正常verify及bubblewrap禁网compile/binding/独立FIFO104周期行为通过，原缓存/复制缓存哈希一致。用于重放的测试锁仅重绑定本地CLI内容hash，来源身份及原锁/缓存保持；非在线重锁或上游升级。

最终工作区回归（CARGO_PROFILE_TEST_OPT_LEVEL=1）：487个结果块，1903 passed / 0 failed / 56 ignored；fmt检查通过。初次默认配置长随机测试主动中断并归档，不计PASS。用户明确修复授权覆盖M1/M2实施与状态同步，脚本将这两项标done；M3/M4保持open，Epic130复评accepted、Phase24复评accepted-with-open-items。
