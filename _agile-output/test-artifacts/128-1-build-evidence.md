# Story128.1 build证据（2026-09-21）

本轮只交付[风险记录](../implementation-artifacts/epic-128-nfr14.md)、工具发现与旧基座探针。FR197/M3未交付。无产品源码/API/工具钉变更，无push/publish。128.2–5保持backlog；风险记录accepted不等于七步结束。

## 可重放命令与原始证据

所有命令的argv、cwd、PATH、解析路径/realpath、UTC开始/结束、耗时、退出码、完整stdout/stderr见`128-1-build-commands.json`；每条另有`128-1-build-<label>.log`。执行环境：

```sh
export PATH=/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin:$PATH
export CARGO_PROFILE_TEST_OPT_LEVEL=1
export BITLOOM_REQUIRE_RTL=1
export RANDOM_SEED=1281001
/tmp/bitloom-phase24-bfm-py312/bin/python scripts/phase24_axi_bfm_probe.py
cargo test -p bitloom --test fr196_csr p0_same_session_real_peer_owns_counter_and_partial_write_wins_increment -- --exact --nocapture
python3 _agile-output/test-artifacts/128-1-atdd-gate.py
```

本机/tmp只是本轮重新实际探测通过的路径，不承诺其他机器存在。源码与配置hash及归档hash在`128-1-build-sha256.json`；归档`128-1-build-probe-artifacts.tar.gz`保留BFM stub、XML、仿真文件及CSR design.v、tb.sv、工具版本/编译/运行日志，成员为规范化target相对路径。原始日志永久在test-artifacts，不会随cargo clean消失。

|发现项|实际路径与版本|退出/解释|
|---|---|---|
|Rust/Cargo|/home/richard/.cargo/bin/rustc、cargo（rustup代理）；rustc1.97.1 8bab26f4f，cargo1.97.1 c980f4866|0/0；rust-toolchain.toml与workspace固定版本不变|
|Python|/tmp/bitloom-phase24-bfm-py312/bin/python → /usr/bin/python3.12；3.12.3|0|
|五包闭包|cocotb2.0.1、cocotb-bus0.3.0、cocotbext-axi0.1.28、find_libpython0.5.1、scapy2.7.0|标准库metadata列出完整五包，BFM逐包比对requirements成功；包名下划线规范化与requirements连字符同一distribution|
|Icarus/vvp|版本与解析路径见iverilog/vvp分项log；Icarus12.0 stable|0/0；实际编译/运行亦成功|
|Yosys|/usr/bin/yosys；0.33 git2584903a060|0；仅版本发现|
|sby|/tmp/bitloom-1263-sby-installed/bin/sby；SBY yosys-0.47|0；版本与ci-sby-pins.env标签一致，不声称本轮重新核验安装源码完整SHA|
|z3|/usr/bin/z3；4.8.12 64-bit|0；仅版本发现|

版本探测发生2026-09-21T07:05:44Z。`python -m pip freeze`退出1（No module named pip）保留`128-1-build-packages.log`；没有隐藏失败或安装新包。07:06:08Z以标准库importlib.metadata完整列举distribution退出0（packages-metadata.log），固定BFM运行已逐包断言准确版本。这消除运行闭包发现阻塞，不要求pip为BFM运行依赖。

## 本轮真实结果与限制

|层次|实际结果|证据/不推出的结论|
|---|---|---|
|BFM独立stub|exit0，1 PASS/0 FAIL/0 SKIP，312ns，seed1281001；五通道pause、读写、reset取消=None|bfm.log及归档results.xml；不是Bitloom产品RTL|
|旧CSR三模块peer实际RTL|exit0，1 passed/0 failed/0 ignored；seed0x12720a11ce55，2311帧，接受1016/消费1013/取消3，17次peer reject、83次部分写、9次高位允许|csr-rtl.log；Icarus/vvp实际调用且run.log含FR196 PASS；11 filtered out不计通过|
|过程门禁|exit0，35场景通过，sprint字节/SHA保持|gate.log；28个128.1未done负例、4合法未来正例、当前/M0失败/提前Epic关闭；完整诊断逐场景保存|
|人工跨故事依赖|epics.md Epic128/129与sprint逐边核对，M0/M1/M2关闭文档已读|风险正文(a)矩阵；不声称gate脚本覆盖跨故事依赖|

CSR源码选择依据：`fr196_csr.rs`中的peer_body/hierarchy及精确测试，三模块同session只finish一次，计数状态由peer唯一拥有，候选高位与提交前奇偶产生reject，commit同沿优先软件部分写。对应CSR rtl External无副本，mask/candidate不依赖commit；counter自然增加提供单owner组合接口证据。该peer不是Timer，更不是未来UART/GPIO/IRQ或FR198系统。

未来产品逐项验收与停止条件见风险正文。未运行本故事的新外设产品（尚不存在）、形式prove/cover、综合/PPA、Chisel/FIRRTL产品矩阵或板级验证；Yosys/sby/z3存在不能证明这些项目。M1/M2历史证明仅作为基座历史引用。native/generated层级仍unsupported。

build自审映射M01–M14已记录，独立review可据源码和归档复核；M15保持pending。独立code-review、automate、clean/fmt/just test及单故事提交留主流程，不把当前局部探针当完整回归。风险记录与故事最终状态分开。

## 工具恢复来源与形式身份门槛（review修正）

临时目录消失后不能靠相同路径名恢复身份。[既有BFM安装配方](../../docs/ip/phase24-axi-bfm-probe.md)给出显式Python3.12解释器创建venv及`uv pip sync --python <venv>/bin/python scripts/phase24-axi-bfm-requirements.txt`；按固定五包恢复后重新运行版本断言和stub，不使用默认Python选择。Icarus安装来源可沿用[CI](../../.github/workflows/ci.yml)的apt安装入口，随后核验iverilog/vvp实际版本（本轮12.0）并实跑探针；发行版包漂移不自动等于已验证版本。原本/tmp/maintenance-tools是本机包装路径，不是可分发工具锁或本轮验证过的安装源码身份。

Yosys/z3及sby恢复入口为[ci-install-sby.sh](../../scripts/ci-install-sby.sh)，钉来自[ci-sby-pins.env](../../scripts/ci-sby-pins.env)。历史[126-3-fresh-install.log](126-3-fresh-install.log)及[126.3证据](126-3-build-evidence.md)记录fresh clone、标签对象bfc1c47eb786496fe794481ff88e75728f0529a6、剥离commit daed0e1544fd96ee7dab843e5a891d92784c6230和独立前缀上游make安装；这是历史来源证据，不证明当前临时入口未改变。

**后续任何formal产品声明前，必须重新核验所用sby源码标签对象等于SBY_GIT_SHA、HEAD等于该对象剥离commit，并把所用入口及支持模块绑定到该受验源码的安装记录。** 如无法绑定当前安装，就在受控fresh前缀从已验源码重新make install并记录入口/支持模块与版本，再执行证明。安装脚本已有工具时会skip clone，因此成功退出或`SBY yosys-0.47`字符串都不足以完成这项前置；ci-sby-hygiene-check只检查配置/脚本遵从，也不替代安装身份。本轮仅发现版本，未完成当前安装精确源码SHA核验、不宣称formal运行。未核验不得把后续证明标为符合固定工具合同。
