# Story128.3 build三路审查

三路fresh context同能力reviewer同时启动，全部返回后才triage。blind按886734字节计算floor10，edge提出1项，verification结论No verification gaps found。无跳过层。

|ID|发现|裁定/路由|核实依据|
|---|---|---|---|
|B1|archive无输入也成功|medium / patch|脚本members空时仍写tar、集合/hash比较空集均通过，可能误报clean前保护|
|B2|basename排除run可能排掉未来证据|low / reject|当前18个run均以vvp解释器头开始，都是明确可再生可执行；其他名称来自同一已知runner。没有当前证据丢失。为未出现的任意文件加分类机制超出直接修正，日常故障未示出|
|B3|启动异常不记commands|medium / patch|subprocess.run抛OSError先于records.append，真实缺工具会只有空/旧记录|
|B4|例子产物比较失败不记commands|medium / patch|example命令成功写入后require比较，可使整个gate失败却JSON全成功，需独立比较步骤|
|B5|SBY包/bytecode可绕过字节身份|medium / patch|根复制安装添加sby_core包，16文件核验通过而实际sby导入未检查包并报自定义标记；128-3-build-review-shadow-repro.json。需隔离缓存及拒绝未核验导入路径，保留真实安装不动|
|B6|直接formal测试命令绕过身份门|low / patch|CI/runner正确，但文档直接test入口未区分未检查调用；文档指向required runner并说明直接入口边界|
|B7|Yosys/Z3版本只记录不强制|low / patch|实际固定本机Yosys0.33/Z34.8.12已记录；产品钉为Rust/firtool/Chisel与SBY源，不新增升级。文档明确host版本是记录策略而不是runner锁定承诺|
|B8|重复后端helper|low / reject|当前两路径同正确命令/端口适配，无已发生divergence；提出抽共享框架是非平凡维护重构，不修当前用户缺陷|
|B9|其它Chisel面没有真实JVM|false / reject|新增单测明示仅名字拼写防回归，文档明确字符串不等于行为；实际机械JVM独立通过，三面共用同emit_expr全限定API，未声称所有面执行JVM|
|B10|故事../../docs链接坏，应../../../|false / reject|根Path.resolve实测../../docs正是项目/docs且存在；审查建议多退一层会出仓库。原路径正确|
|E1|空archive通过|medium / patch（与B1同因）|同B1，逐项保留后合并处理|

verification层无发现，不虚构补项。保留产品黄金、已验证行为和工具钉；patch只涉及证据工具/说明，无冻结意图更改。修补后根按spec重跑相关验证，七步尚未完成。


主代理修补后复验（2026-09-21）：新runner全部命令exit0，活动ATDD9项、formal3项（23 cover及3预期反例）、全向量与三组合后端、FIRRTL21项、numeric实际JVM、SemVer、原文例及C头通过。完整命令/源码摘要见128-3-root-review-recheck.json；新归档128-3-review-fix-raw.tar.gz，2228成员逐字节对照原target再次通过，SHA256 989b72dd2d932ebc0eccce8d69b151d5e71ef1c18a6dd9eb16de90145c419557。B1/B3/B4/B5/B6/B7及E1修补关闭；历史日志/归档不改。第六步clean/fmt/全workspace仍按七步顺序随后执行，不以本轮定向复验替代。
