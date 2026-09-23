---
title: 'Bitloom 1.2.0 发布候选准备'
type: 'chore'
created: '2026-09-23'
status: 'done'
route: 'oneshot'
review_loop_iteration: 0
context: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

在Phase24及M1–M4维护完成后，准备可审阅的Bitloom 1.2.0发布候选。统一发布家族版本与内部依赖，记录增量功能、修复与既有限制，完成SemVer、打包及仓库外候选使用验证。用户已接受上一轮明确提出的准备范围。

本轮不上传crates.io、不推送、不创建正式发布标签，不下载板卡工具或改变工具钉。保留FR189 deferred/NFR91及物理/native/generated层级边界；不将本地候选重放称为已发布安装。正式发布前应提交可审阅的结果与剩余操作。

</frozen-after-approval>

## Implementation Notes

- 基线：0638f0b131bb56a3a910b7cb23c3ad43c9b75cea；初始工作区干净，main。
- 意图无未决选择；本轮没有不可逆动作。改动限版本清单/锁文件、发布说明及验证记录，不引入产品API。采用oneshot。
- 复用Cargo.toml workspace.package/dependencies；sim恢复version.workspace。历史版本说明不批量替换。
- 复用scripts/semver-check.sh，新增发布检查记录；优先使用Cargo原生多包打包以验证未发布家族间依赖。仓库外验证使用候选包的本地源，不冒充crates.io安装。
- 调查发现生成器旧依赖字面量（cycle hir/builder=1.1.1、functional hir=1.1.0）；同步为当前同版家族CARGO_PKG_VERSION并更新既有断言，不改变模拟语义/API。AD-2/5/6/14保持。完整候选包仓库外构建覆盖路径优先掩盖的问题。
- 首次workspace打包实际通过，但包含非发布示例/内部包；最终验收改为显式十个发布包，避免误上传清单。两次日志分别保留。
- 独立审查后修正上述实现选择：hir/builder最低版本固定为实际依赖1.2.0，避免未来sim独立补丁自动要求不存在的同号hir/builder；sim自身仍使用CARGO_PKG_VERSION。生成物真实registry清单与运行归档，包已重新打包并重装验证。
- 根Cargo.lock按项目约定被忽略，本轮不改变忽略策略；另存workspace-Cargo.lock.txt并核对hash，发布清单规定恢复锁后--locked。正式上传后的生成器复验已加入清单。
- 首次全工作区回归在FR149联网publish dry-run因offline终止，非产品测试失败；日志保留。候选回归明确跳过五个执行publish dry-run的测试，不能将过滤项/ignored算PASS；真实registry检查留到上传依赖后。

- 最终结果：十包package/联合publish dry-run、仓库外安装及两生成器运行、LSP握手、三包SemVer通过；候选回归1898 passed / 0 failed / 56 ignored / 5 filtered；最终sim库另跑38 passed。fmt/diff、依赖/锁/归档hash检查通过。没有上传、推送或标签。

## Review Triage Log

- medium / patch：sim版本隐含传播给hir/builder；改为实际最低依赖1.2.0，保留sim独立补丁能力。
- medium / patch：helper断言不足；最终解包安装生成真实registry清单并编译运行两种生成物。
- medium / patch：根锁文件未追踪；归档锁与摘要，清单明确恢复和--locked。
- medium / patch：正式发布后漏检生成器；清单加入无patch的新环境复验。
- Blind Hunter修正后复核无剩余实质问题；完整证据见bitloom-1-2-0-release-verification.md及bitloom-1-2-0-release/review.md。
