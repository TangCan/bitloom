# Story128.2 独立代码审查修补证据

2026-09-21。9项修补完成，Timer公开API、描述字段bits、产品行为与工具钉保持既定合同。实施子任务未改spec/sprint/总账，未clean/workspace/commit。历史build/review-fix归档保留。

|项|改动及实际验证|
|---|---|
|1|CI test job调用runner --only example --only header，从仓库原文重建并通过。|
|2|ChiselTypeRegression增加Reset XOR双操作数顺序，全部reset/input/address组合实际JVM→RTL通过，无手改Scala。|
|3|独立Trace增加COMPARE=ffffffff、COUNT=fffffffd的one-shot/periodic场景；断言恰增1个match、one-shot保留MAX并关闭、periodic归0且使能，再经CSR读取。direct/FIRRTL/Chisel同向量均通过。|
|4|私有pair参数选择同一SharedTimer的两个实例或不同公开名字共享CSR的两个实例。两图在direct RTL运行相同状态隔离、同时请求和非对称背压黄金。|
|5|专用pair后端入口将两图实际降FIRRTL和Scala/JVM，再用同一tb执行；仅转换Chisel端口名。CI及runner明确调用该ignored入口。|
|6|formal先证明原始RTL，再仅改DUT的配置抑制、EVENT更新优先或COUNT bit31。唯一替换点与实际变化必须成立；相同带标签observer拒绝各变异，要求SBY exit2/FAIL、指定可达basecase断言及VCD，不能用解析/求解错误或UNKNOWN代替。|
|7|runner默认矩阵加入chisel-numeric-check.sh，并提供--only numeric；实际窄调用另存命令记录。|
|8|身份/结果验证全部显式require/RuntimeError，AST确认无Python assert；python -O与继承PYTHONOPTIMIZE=1均接受真实安装，拒绝复制后篡改一个支持模块的安装。|
|9|CI Timer formal将installer源码路径传给runner --only formal，来源/pin/安装绑定通过后才证明。已有工具无法绑定或源码缺失必须失败，保存invocation/env及identity-failure JSON，version/hygiene不替代。|

## 实际结果

完整命令、UTC、环境与exit见128-2-code-fix-commands.json。主矩阵8个active通过、3个专用formal通过、Timer后端与pair后端各1通过、例子/C编译/C执行exit0；默认ignored不计产品通过。numeric窄入口13个Rust矩阵和18个真实JVM RTL case通过，命令另见numeric-commands.json。优化模式记录见optimization-checks.json。修改Rust文件rustfmt检查、git diff --check、CI YAML及门禁连线检查通过。

三个seed仍为128219705511/deadbeef1282/73592401ffff，目前帧数6610/6610/6611，每seed 470个raw match；提交/消费/取消分别2754/2749/5、2727/2723/4、2739/2733/6。明确使用软件设置近边界值，不声称经历2^32个周期。

原始安全/control均PASS、10个独立cover到达、原始综合614cells。三个真实变异分别触发timer_match、timer_event、timer_count的basecase失败，均exit2/FAIL并有非空trace；observer方程不变，只给原断言加诊断标签。

128-2-code-fix-raw.tar.gz独立保存621个原始生成源码、tb、日志、JVM输入、RTL、形式脚本/模型/SMT/trace、runner及身份负例；manifest记录归档和8个相关源码SHA，排除可重建编译缓存/二进制。主代理已逐文件核对，见128-2-root-code-fix-audit.json。最终全量回归及故事关闭仍待主流程完成。
