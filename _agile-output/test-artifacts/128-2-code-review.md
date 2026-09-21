# Story128.2 独立代码审查

四层均完成：blind、edge、verification为fresh同能力子代理；第四层fresh spawn及首次followup被并发上限拒绝，待verification结束后复用未参与实现的backend_recon作acceptance。未跳层，未在全部结果返回前triage。verification明确报告No verification gaps found，不是执行失败。

范围：HEAD 1c4cf20基线至全部未提交文件，diff 518526 bytes / 9750行（包含历史证据）。spec-128-2-timer.md及其三个context；完整故事按用户既有授权连续审查，无需重复批准。

|ID|来源|裁定|证据及处理|
|---|---|---|---|
|1|blind|medium|多位selector旧HEAD即不支持，当前Timer1位；既有defer，与11同根因。|
|2|blind|medium|CI有FR194/196例子而无Timer例子/C编译；patch接现有runner窄入口。|
|3|blind|medium|Reset XOR只由Timer左操作数覆盖，右侧缺失；patch补实际JVM真值表。|
|4|blind|medium|trace只显式触发compare0回绕及1/2/17，未触发COMPARE最大值；patch双模式近边界定向轨迹。|
|5|blind|medium|内审改pair后两实例用了不同定义，当前未实际复用同一Timer定义两次；patch保留两种组合运行。|
|6|blind|medium|多后端只有固定名Timer单实例，双实例/重命名仅direct；patch对同一定向pair运行FIRRTL/JVM。|
|7|blind|medium|本故事observer尚无故障注入拒绝证据；patch代表性match抑制/事件优先/位宽故障，必须到达性质失败。|
|8|blind|low|runner默认复现命令缺构建证据中的numeric脚本；patch加入此相关emitter门禁。|
|9|blind|medium|identity验证均assert，Python优化模式会删除检查却输出身份记录；patch使用显式条件失败并验证优化模式。|
|10|blind|low|确实只有aggregate bits，没有命名enable/periodic字段；拒绝该改法：ATDD已明确固定字段bits且公开描述合同由spec引用，建议替换字段会改本次规格。文档已明确bit0/bit1，不据此宣称当前AC缺失。|
|11|edge|medium|同1的旧问题，未由此修改引入，defer合并记录。|
|12|acceptance|medium|新CI Timer formal直接cargo调用，只记录version；installer可跳过已有工具；违反故事明确身份前置，patch接实际身份核验后运行证明。|

分组后0 decision-needed、9 patch、1既有defer、1 rejected（逐项12行保留）。修补尚未执行；用户七步授权覆盖全部无歧义修补，不提前done/commit。

修补闭合：9 patch全部完成。详见128-2-code-fix-evidence.md、命令/原始归档，以及128-2-root-code-fix-audit.json独立621文件核验。代码审查步骤完成；故事七步尚未完成。
