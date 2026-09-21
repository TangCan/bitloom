# Story128.2 review修补

2026-09-21。本轮仅运行修改路径的专用测试，完整回归由主代理执行。历史`128-2-build-*`日志/JSON/归档保持不变；runner和C检查源按审查修订。原始历史archive SHA仍为a1da2bd35a47a8a9c15129beca80a21c2939601a1445e054b3f52462c4c05423。

- CI：现有钉死Chisel job追加精确Timer后端入口（含本轮新emitter fixture）；formal job追加Timer专用安全/cover/原始综合，增加各类失败产物路径。
- Runner：从脚本位置定位checkout，使用调用方PATH、BITLOOM_SBY_SOURCE、RHDL_FIRTOOL_PATH；输出仅到新rerun目录，拒绝覆盖。独立核验官方origin、固定tag对象/剥离commit/HEAD/clean，按固定git blob和上游安装变换逐字比对16个已安装文件；不读取/tmp旧identity。自动创建只依赖prelude的例子crate，C头普通include及明确-I；`--only`支持窄验证。环境和使用说明在runner docstring。
- API：非法名字、不同体、私有CSR名碰撞、错误宽/型/方向均核对具体诊断类别及可提供的模块/端口/网名。双实例改为两种公开模块名共享单个私有CSR定义，新增同时提交、不同快照、lane0持续背压而lane1继续写/读成功的真实RTL场景。
- 后端：由现有真实emitter生成ChiselTypeRegression，实际Scala/JVM→RTL检查u64高位/MAX常量及两个初始化Mem字、Reset AND/OR左右两种操作数顺序；硬编码黄金与所有reset/bit/address组合。已纳入同一persistent dedicated backend入口，未改生成Scala。
- 形式：原先CTRL/COUNT/COMPARE合并cover拆成3个独立cover；目前10个cover全部到达，prove和原始614cells综合仍通过。

实际结果：API 7 passed；formal 2 passed（prove/10covers/原始综合）；专用后端1 passed（新增类型fixture及原有三seedTimer FIRRTL/Chisel全部通过）；runner重建例子与C compile/execute均exit0。独立runner验证通过真实身份、搬迁安装身份、篡改单个支持文件拒绝；CI YAML解析与精确命令/失败路径核对通过。修改Rust文件rustfmt --check与git diff --check通过。

本轮命令UTC/exit见`128-2-review-fix-commands.json`；API输出`128-2-review-fix-api.log`，身份`128-2-review-fix-sby-identity.json`，runner/YAML验证`128-2-review-fix-runner-check.json`。新原始产物`128-2-review-fix-raw.tar.gz`及manifest独立于历史archive，供clean后审计。没有额外workspace/SemVer重复运行，没有commit、push、publish或范围关闭。
