# 全部Story七步执行记录

用户目标：对sprint-status中的未完成Story逐个执行create-story、ATDD、build、code-review、automate、clean/fmt/regression、commit，直到全部完成。历史done故事不重复制造提交；不能把尚未具备上游条件的deferred标done。

初始基线：62832ea；455 stories，436 done，17 backlog，2 deferred。上一目标触发前回合完成126.1/126.2，属于progress；当前目标active，无预算限制。

执行顺序：126.3、126.4、127.1–127.4、128.1–128.5、129.1–129.3、130.1–130.3；122.2/122.3实时核验前置后恢复。始终保存整目标，不把单故事完成当总目标完成。

| Story | create | ATDD | build | review | automate | regression | commit |
|---|---|---|---|---|---|---|---|
|126.3|完成|完成（E0432首红）|完成（内审修复27项通过）|完成（4层，5项修补）|完成（取消恢复cover，20项通过）|完成（1732通过，8忽略）|b248dd48a948a303de9aa6aebf656a5c356b52f2|
|126.4|完成|完成|完成|完成|完成|完成|e88a22bef266b52174be51f831d7ef510b65f108|
|127.1|完成|完成（27场景GREEN，正文缺失RED）|完成（3路审查，7项修补）|完成（4层，7项修补）|完成（117场景+BFM）|完成（1763通过，14忽略）|77ea01392e5be74b9f2622009ad146edb2468ea9|
|127.2|完成|完成（12测试，E0432首红）|完成（三路内审修复，15+2测试）|完成（四层，10修补/1拒绝）|完成（28通过，真实RTL/formal突变）|完成（1787通过，18忽略）|9393c261cdfb2ca3699edb959efd1986bbe5275e|
|127.3|完成（8项AC，独立checklist）|完成（11入口，2项E0432首红）|完成（三路内审10项修补，11+3+78验证通过）|完成（四层，10项验证补强，13+3通过）|完成（7cover、24负例+8正例、真实组合）|完成（1801通过，21忽略）|91828619f66f787c63483dda4b0ee558bb7501e8|
|127.4|完成（8项AC，固定接口与独立checklist）|完成（9入口，3目标E0432首红）|完成（三路内审8修补，14+4+91+7通过）|完成（四层，9修补/1驳回）|完成（8 P0，512完成/36取消，8 WO反例）|完成（1823通过，25忽略）|b926629e93b0ad5e09d795b59ec2f1bdb47010b6|
|128.1|完成（6 AC、接口/工具/停止条件）|完成（正文缺失RED，35门禁GREEN，15人工项）|完成（三路内审10修补，BFM/CSR/50状态通过）|完成（四层，10补强；55状态场景）|完成（33门禁调用/8定向变异+CSR RTL）|完成（1823通过，25忽略）|1c4cf20c4f8e8fdcb038d57a7b3f03f9f80b5c00|
|128.2|完成（7 AC、Timer32单owner/边界）|完成（8入口，2目标E0432首红）|完成（三路内审11修补/1既有defer，主代理复核通过）|完成（四层，9修补/1既有defer/1驳回）|完成（1 P0直接RTL检错控制/2真实变异）|完成（1833通过，30忽略）|随本故事提交|

## 上游阻塞实时核验

2026-09-20T11:06:06.665610Z，官方GitHub API llvm/circt releases全部223条（分页100+100+23），正式非draft/non-prerelease firtool最大仍1.159.0，>1.159.0为空；latest一致。官方release：https://github.com/llvm/circt/releases/tag/firtool-1.159.0 ，发布2026-09-09T20:45:25Z。FR189必须严格大于该版本，所以122.2/122.3仍deferred；这是外部条件，不能用live-tip代替。官方Chisel v7.15.0的etc/circt.json配对1.158.0，未猜测升钉。其余17故事仍可推进，当前不构成整体目标无进展阻塞。

## 执行纪律

每故事按七步留真实产物与命令。用户要求的提交安排覆盖build技能内部默认提交：推迟到第七步，不能在review/automate/regression之前提前提交。技能常规确认由当前完整授权覆盖；若有无法推断的实质需求再澄清。外部工具/网络失败如实报告，不能以skip或mock视为产品验证通过。

126.3收尾后：437 done、16 backlog、2 deferred；整体目标active，下一故事126.4。忽略的2个新formal测试已由真实专用运行覆盖，其余6个为既有doc-test。

126.4收尾：438 done、15 backlog、2 deferred；M1关闭，整体目标仍active。下一故事127.1，仍按七步单故事提交；FR189前置未满足，不假交付。

127.1关闭：439 done、14 backlog、2 deferred；Epic127继续in-progress，FR196尚未交付。下一故事127.2，整体目标active。

127.2关闭：440 done、13 backlog、2 deferred；CSR叶子集交付，FR196/M2整体仍开放。下一故事127.3，继续七步；当前整体目标active。

127.3关闭：441 done、12 backlog、2 deferred；仅桥子集交付，127.4四窗译码/M2待验收。下一故事127.4，整体目标active。

127.4关闭：442 done、11 backlog、2 deferred；Epic127/FR196/M2完成，整体目标仍active。下一故事128.1；实际完整回归及单故事提交见127-4-final-verification.md，FR189条件未由本故事重新核验，保持deferred。

128.1关闭：443 done、10 backlog、2 deferred；Epic128风险门有效但FR197/M3未交付。下一故事128.2 Timer，整体目标仍active；FR189未由本故事重查或假交付。

2026-09-21T08:45:25Z 再核验官方 llvm/circt releases 全部223条（100+100+23），正式firtool最大仍1.159.0，无严格更高版本；122.2/122.3前置仍不满足。结果见`../test-artifacts/128-2-fr189-upstream-recheck.json`；web访问API失败后通过官方API直接分页获取，不用旧数据冒充实时结果。其余故事继续推进。

128.2关闭：444 done、9 backlog、2 deferred；Timer子集已交付，FR197/M3仍开放。下一128.3事件IRQ，整体目标active。FR189本故事已实时复核，仍无严格更高版本，保持deferred。
