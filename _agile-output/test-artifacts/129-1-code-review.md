# Story129.1 独立code-review（2026-09-22）

完整规格模式；基线0ab222203916b1b0583c2c673558d5333cb05552；四层均完成：blind 10、edge 2、verification 1、acceptance无actionable项。用户七步连续授权包含明确缺陷修补；暂不将Story标done，后续automate/回归/提交未执行。

|ID / 来源|发现|裁决 / 处理及依据|
|---|---|---|
|B1 blind|--locked依赖的lock被忽略|medium / patch；git check-ignore证实，force-add实际已验证harness lock。|
|B2 blind|Yosys路径空格|medium / patch；绝对路径字符串未引用，交修复并用空格路径实际复跑。|
|B3 blind|regression超时子孙残留|medium / patch；直接subprocess.run仅杀just，改新session/killpg/reap。|
|B4 blind|失败回归无法保留重试|medium / patch；增加--attempt唯一标签，不覆盖旧报告/日志。|
|B5 blind|隔离Rust未格式化|low / patch；独立两个manifest格式化，最终回归加各自--check。|
|B6 blind|foundation证据漏runner身份|medium / patch；纳入自身指纹并归档源码。|
|B7 blind|共享归档可含旧产物|low / patch；原verification已标历史快照，新JSON也显式标范围；仅本轮exact日志给PASS。|
|B8 blind|失败缺结束指纹|medium / patch；两runner在finally写结束指纹。|
|B9 blind|主reset证据最终指针过期|medium / patch；补追加当前复验索引，原r3仅历史。|
|B10 blind|typed任意失败不影响成功|medium / patch；typed direct/FIRRTL要求PASS，Chisel只允许明确既有诊断或真实PASS。|
|E1 edge|regression超时子孙残留|medium / patch；与B3同根因，采用同修复。|
|E2 edge|Yosys路径/分号解析|medium / patch；与B2同根因，文件名按Yosys语法引用。|
|V1 verification Other|lock未提交导致重放失败|medium / patch；与B1同根因，同修复。|

共13项裁决合并10个修补组；0 decision-needed、0 defer、0 rejected。验收层确认AC1–7风险/实证边界，AC8保持pending，无完整系统交付声明。修补定向结果待追加。

修补复验完成：26命令含空格输出路径探针exit0；typed准确诊断、适配三后端行为/综合全PASS；源起止一致。foundation最新35命令exit0，归档`129-1-root-foundation-20260922T011141254627Z.tar.gz`，包含runner源及SHA。回归runner的进程组清理/attempt/独立fmt/失败指纹将由automate检查，完整回归仍留第六步。10组修补已应用，Story仍review依用户七步和AC8暂不done。
