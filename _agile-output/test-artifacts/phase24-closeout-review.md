# Phase24 结项补审（2026-09-23）

对象：Story129.3提交 `baf07e739af2b36f4e0befddda9826964d16118e`，父提交 `218f0620e85ab438a3ef1351e253277552c8c410`；20文件、1970新增/15删除。四个独立代理完成blind/edge/verification/acceptance层；受并发容量约束，verification在前两层返回后启动；没有缺层、超时或空结果。Blind按技能给定floor提出10项候选，主代理逐项核证，不将该数当质量指标。

历史129.3审查超时记录保留；本记录是后补审查，不倒填历史。当前结项基线 `4bdd680e1a6db7e9ca77e6d4fcd7caa4b825d2b1`。用户批准实施建议，范围内patch直接执行。

## 逐项裁定（先裁定，再按根因分组）

|ID|层|发现|裁定与证据|处置组|
|---|---|---|---|---|
|B1|blind|自定义证据输出不能接收隔离结果|medium：runner读args.evidence，shell写固定MAIN_RESULTS，确实不同|P1|
|B2|blind|成功隔离原始资料被清理|medium：仅合并三字段，trap删除所有隔离文件|P2|
|B3|blind|CI未上传隔离失败目录|medium：shell复制fr201-isolated-failure，workflow没有该路径|P3|
|B4|blind|一命令未校验新证据|high：producer不调用consumer，另一个workspace job只读提交快照，空backends可漏检|P4|
|B5|blind|形式检查缺请求保持|medium：observer只有response assertions，与原AC4不符|P5|
|B6|blind|API/版本字段固定声明|medium：公共表面及版本未实际比较，SemVer不能证明无增加|P6|
|B7|blind|缺工具负控制仅测分离探针|medium：require_tool不走真实preflight且不处理BITLOOM_SBY|P7|
|B8|blind|本地若干子进程无限等待|medium：runner无超时，Rust部分output也无deadline；本地命令可挂起|P8|
|B9|blind|源/seed/工具身份索引不完整|medium：tools仅三项，声明工具版本写常量，无generated FIRRTL/Scala hash和seed|P9|
|B10|blind|CI SemVer工具版本浮动|low：--locked不钉cargo-semver-checks自身版本；可直接固定本机已验收0.50.0|P10|
|E1|edge|自定义证据输出路径|medium：独立确认同B1|P1|
|E2|edge|成功隔离资料删除|medium：独立确认同B2|P2|
|E3|edge|隔离失败资料未上传|medium：独立确认同B3|P3|
|E4|edge|干净checkout空补丁失败|high：git apply空文件退出非零，违反干净checkout合同|P11|
|V1|verification|新证据未经过验收consumer|high：将序列化backends置空后所有旧执行仍成功，当前runner无后置验证|P4|
|V2|verification|自定义证据路径|medium：独立确认同B1|P1|
|A1|acceptance|direct CSR缺FIRRTL/Chisel执行|high：NFR14要求两图各三后端，当前只有system(true)矩阵|P12|
|A2|acceptance|请求保持及复位取消缺性质|medium：AC4与observer不符，需有限性质/可达/负控并明确界限|P5|
|A3|acceptance|clean后主/隔离原始证据缺失|medium：提交索引四条target路径现已不存在；只留摘要不满足归档|P2|
|A4|acceptance|CI失败归档遗漏|medium：独立确认同B3|P3|

共20项，12个根因组；decision-needed=0，patch=12，defer=0，rejected=0。没有修改原规格来消除发现。修复与实际验收结果将在下文追加。

## 修复复核的追加发现

|ID|独立层|裁定|修复|
|---|---|---|---|
|F1|verification|medium：CI仅失败上传使成功验收原始证据在runner销毁后丢失|core job改为always上传、使用中性evidence名称；本地不冒充远端CI执行|
|F2|blind|high：`direct-*`同样匹配`direct-csr-direct-*`，CSR后运行导致AXI聚合选错行|latest同时核对backend和topology，并加入较新CSR不能遮蔽AXI的回归|
|F3|blind|medium：新执行的untracked validator测试未进入源归档，git diff不含它|明确归档测试、replay shell及执行helper并纳入hashmanifest|

Rust修复独立复核无剩余阻断：两图六路保持原oracle/16seed预算；桥/叶请求保持、reset epoch和三个真实失败mutant；bounded depth8与四叶blackbox边界明确。Python独立复核已确认fresh consumer、自定义输出、缺工具、超时与hash拒绝，追加F2/F3修复后还须整体实跑。NFR99脚本和总验收草案经独立审阅，未将物理行/文件数当作生产率。

## 最终处置结果

P1–P12以及F1–F3全部修复；四层审查和Rust/Python/文档独立复核均已完成，没有未解决的范围内阻断项或本轮延期发现。F2/F3修复后，完整六路线、有限形式、兼容、8项普通/优化模式负控及隔离重放整体执行退出0；clean后独立consumer再次通过。最终完整工作区回归1894 passed / 0 failed / 56 ignored，见[原始验证记录](phase24-closeout-final-verification.md)。

FR189/Epic122延期与NFR91是原有明确范围边界，继续保留，不属于被本轮审查静默豁免的问题。CI归档配置已经修复，本轮未触发远端CI。
