# Story128.3 独立代码审查

范围：相对9d2aff720025d5b084ca746de4b1c9938757b350完整45文件，+19329/-4，full模式，diff /tmp/128-3-code-review-yu34evv8.diff。spec及story/ATDD/Epic128上下文已读。四层同能力fresh审查，无跳过/失败；线程上限使acceptance在verification完成后启动，未在全部启动/报告前triage。verification无缺口，acceptance无具体AC违例；盲审10项、edge2项逐条如下。

|ID|裁定|路由|发现|核实|
|---|---|---|---|---|
|B1|medium|patch|归档遗漏example-artifacts|gates集合不含新增比较gate；example退出0而比较失败仍满足集合。|
|B2|medium|patch|历史PASS可替代当前源码结果|passed跨所有commands.json并集，没有选定run或源码匹配，改变产品后旧PASS仍有效。|
|B3|medium|patch|formal状态只检查非空|require_paths仅检查size，ERROR/UNKNOWN与负控制PASS都可满足该层检查；须关联本轮并核对期望。|
|B4|medium|patch|自定义runner目录被归档漏掉|runner接受BITLOOM_IRQ_RERUN_DIR任意不存在路径；archive仅遍历五个固定target根，实际支持路径可能丢失。|
|B5|low|patch|孤立manifest被覆盖|只保护archive.exists，最终manifest.write_text直接覆盖；直接补对两个目标的拒绝检查。|
|B6|medium|patch|TEST字节矩阵被全1状态遮蔽|三层循环每次先TEST31，addr8时pending已经31，selected-byte不作用也不可见；应空/混合pending分别起步。|
|B7|medium|patch|harness回归未接CI|CI三个IRQ入口均未执行128-3-build-harness-tests.py；身份和证据错误场景不会受持续回归保护。|
|B8|false|reject|Concat注释无法证明位序|实际high=0 low=非零仍可区分high/low反转，反转使低位IRQ事件消失；丢弃恒零high是该向量的等价变换而非位序错误。变更只是Cat名字限定，字符串检查与实际JVM范围已明示，不承诺一般Concat全部值域。|
|B9|medium|patch|精确过滤零测试也成功|runner仅依cargo退出0；Cargo过滤无匹配时合法返回0，缺少目标执行数检查。|
|B10|medium|patch|运行期间改源码不被识别|source-sha256仅在gate之前写，没有结束比较，不能确认长门禁期间被编译的相关源码保持相同。|
|E1|medium|patch|归档遗漏example-artifacts|独立同B1；先单列裁定后按同根因合并。|
|E2|medium|patch|历史PASS可替代当前源码结果|独立同B2；先单列裁定后按同根因合并。|

B1/E1及B2/E2分别同因合并：9组patch、0 decision-needed、0 defer、1 rejected。用户已授权七步持续执行，实施全部patch，不重复确认。未改变冻结意图或公开表面。

## 运行边界

主代理已读取工作树IRQ完整定义，TEST矩阵初始化循环，Concat共用emit与新增回归，runner全部控制流，archive全部控制流，CI各IRQ调用。盲审的floor计算2,559,527 / 1000 = 2,559.527 kB；min(floor(sqrt(kB)+1),10)=10。验证缺口层原文：No verification gaps found. 验收层原文：No concrete acceptance-criteria violations found in the reviewed diff. Implementation and verification match AC1–AC7; final automate, clean/fmt, workspace regression and story commit remain explicitly pending, without premature closeout claims.


修补完成复核：9组patch全部落实并经主代理审阅。direct扩展为97,738帧，普通/-O工具回归各12项通过；新默认完整runner 20260921T103117.009451Z-237066全命令PASS，活动9项、formal3项、完整两后端及三组合、FIRRTL21项、numeric/semver/example/header全部通过，两exact目标各1次真实PASS。起止和当前566源文件SHA一致。原始归档128-3-code-review-raw.tar.gz由root独立逐字节复核3681成员，SHA256 7ef194ce9e68a300cd4ad42475f26349a9eeae051a108adbe9761630247ded08；详见128-3-root-code-review-recheck.json。代码审查阶段完成，0 decision-needed、9 fixed、0 defer、1 rejected。用户七步顺序优先：Story/sprint在automate及clean/fmt/workspace/commit之前仍review，不提前done。


最终回归后指纹边界修复：cargo clean/fmt/just test全部exit0（1844通过/0失败/35忽略），格式化前后输入完全一致。唯一运行中变化是gitignore保护的crates/rhdl-formal/fixtures/fr119/fr119_pass/logfile.txt，由旧SBY测试生成，非产品源码。source_fingerprint错误包含该.txt，已排除SBY运行logfile.txt并扩充原有动态日志不干扰回归。仅runner/harness两Python文件更改；普通/-O各12项通过，详见128-3-final-source-audit.json与128-3-final-fingerprint-validation.json。原始回归JSON保留这次差异，未回写历史摘要为假一致；Rust产品/测试与CI均未在回归后更改。新完整runner再次绑定修正后的证据脚本及最终源码，旧最终归档保留其历史阶段。
