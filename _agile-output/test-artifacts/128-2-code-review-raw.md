# Story128.2 四层审查返回记录

## Blind Hunter（保留完整问题含义）

finding floor: 518526 bytes / 1000 = 518.526 kB；min(floor(sqrt(518.526)+1),10)=10。

1. Chisel AssignExpr::Mux 对所有非Bool使用asBool，仍不支持多位选择器；应执行非零语义及多位实际回归；deferred条目仅承认问题。
2. CI缺Timer文档例子及C头编译门禁，已有FR194/FR196示例门禁；API/文档变化可能静默破坏例子。
3. 全局XOR emitter修改缺右操作数Reset回归；Timer只测试Reset在左，须UInt XOR Reset完整JVM真值表。
4. trace没有COMPARE=ffffffff的近边界真实命中；当前明确溢出只针对COMPARE0，随机难以击中最大值比较；须one-shot/periodic预置接近最大值COUNT。
5. pair两个实例当前是不同Timer定义；重复define SharedTimer只证明定义去重，未证明相同定义实例独立状态；增加相同定义双实例RTL。
6. FIRRTL/Chisel只执行standalone Timer，公开重命名与双Timer组合仅direct-Verilog；增加后端类引用和层级连线实际执行。
7. observer只对正确实现证明，缺可执行变异拒绝检查；对match抑制、事件优先、计数宽度作代表性变异，要求证明失败。
8. runner默认“original full build set”缺scripts/chisel-numeric-check.sh；该门禁属于已记录证据并验证全局emitter，须加入或收窄复现声称。
9. sby_identity用Python assert；-O或PYTHONOPTIMIZE移除检查仍能生成成功身份记录；改为显式失败。
10. Timer::registers及C头仅aggregate bits，没有enable/periodic命名mask，软件须自己表示位位置；建议独立命名字段。

## Edge Case Hunter

location: crates/rhdl-firrtl/src/chisel.rs:1108–1112。trigger: Mux selector是多位UInt。guard suggestion: integer selector用orR，1位和Reset用asBool。consequence: Chisel拒绝合法HIR选择器；此既有缺口已明确deferred。

## Verification Gap Reviewer

No verification gaps found.

## Acceptance Auditor

CI Timer形式入口绕过规定的SBY身份前置。违反AC7及故事工具验证约束：证明前核验tag对象、剥离commit、实际入口与支持模块绑定；installer skip/hygiene/version不能替代。CI新步骤直接cargo执行fr197_timer_formal，测试只记录工具版本；runner已实现sby_identity但CI未调用，ci-install-sby.sh已有工具可跳过。因此本地档案符合身份约束，不保证持久CI也符合；须在CI证明前接身份核验，无法绑定时失败或受控安装。

原始审查均通过collaboration返回；本文件是中文归档，裁定/去重/处理另见128-2-code-review.md。
