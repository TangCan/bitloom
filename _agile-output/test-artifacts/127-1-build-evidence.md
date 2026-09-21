# Story127.1 build 实测证据

2026-09-21 UTC。风险正文为[epic-127-nfr14.md](../implementation-artifacts/epic-127-nfr14.md)。本轮仅文档和工具/进程门禁；未实现新CSR、桥或译码，没有新产品RTL/formal/综合/PPA结果。

每个`127-1-build-{name}.log`头部保存UTC、完整命令argv、解析路径、PATH及实际exit_code，随后原样stdout/stderr；统一清单为[probes.json](127-1-build-probes.json)。包源码查询另在[package-source.log](127-1-build-package-source.log)。运行环境PATH前置`/tmp/bitloom-1263-sby-installed/bin:/tmp/bitloom-maintenance-tools/bin`，不是跨机器安装合同。

| 探测 | 实际结果 | exit / 原始日志 |
|---|---|---|
|rustc / cargo|1.97.1 / 1.97.1|各0；build-rustc.log / build-cargo.log|
|Python|3.12.3，`/tmp/bitloom-phase24-bfm-py312/bin/python`|0；build-python.log|
|初次包入口查询|`python -m pip --version`失败：No module named pip|1；[原失败](127-1-build-packages.log)，保留不覆盖|
|实际固定包查询|importlib.metadata：cocotb2.0.1、cocotb-bus0.3.0、cocotbext-axi0.1.28、find-libpython0.5.1、scapy2.7.0|0；[安装源码/版本](127-1-build-package-source.log)，探针再次逐版本断言|
|iverilog / vvp|Icarus12.0 / runtime12.0，maintenance-tools入口|各0；build-iverilog.log / build-vvp.log|
|sby|SBY yosys-0.47，`/tmp/bitloom-1263-sby-installed/bin/sby`|0；build-sby.log|
|yosys / z3|Yosys0.33 git2584903a060 / Z3 4.8.12|各0；build-yosys.log / build-z3.log|
|临时状态门禁|27场景符合预期，真实sprint字节/hash保持|0；[gate](127-1-build-gate.log)|
|独立BFM stub|五通道pause、合法读写、受阻写reset取消返回None，312ns；JUnit恰1用例、无fail/error/skip|0；[BFM](127-1-build-bfm.log)、[JUnit](127-1-build-bfm-artifacts/results.xml)、[stub](127-1-build-bfm-artifacts/stub.sv)|

BFM命令：`timeout --kill-after=10s 120s /tmp/bitloom-phase24-bfm-py312/bin/python scripts/phase24_axi_bfm_probe.py`。随机种子1789952427来自真实JUnit；刺激为既有确定性pause序列，不将seed称为随机产品覆盖。源码/requirements/gate/JUnit/stub的SHA256在[artifact清单](127-1-build-artifact-sha256.json)。stub与JUnit均已复制至target之外，可在未来cargo clean之后复核。脚本未修改；只移除本次生成的Python字节码副产物。

安装源码是此次固定版本API核验的直接依据：`stream.py:193 set_pause_generator`、`reset.py:40 assert_reset(val=None)`、`axil_master.py:227/478` reset向未完成命令事件返回None；文件完整路径与SHA256见源码日志。实际探针拉外部reset，并未声称逐一调用assert_reset方法。`axil_master.py:261–301`按byte_lanes对齐word_addr、计算cycles/strb并分发AW地址，原文摘录见[alignment日志](127-1-build-bfm-alignment.log)。因此非法对齐、特殊WSTRB和半事务需原始通道驱动，不能与高层BFM同时驱动针脚。

版本固定依据：`scripts/phase24-axi-bfm-requirements.txt`与`scripts/ci-sby-pins.env`（SBY ref yosys-0.47 / ref对象bfc1c47eb786496fe794481ff88e75728f0529a6）。本次仅发现sby/yosys/z3入口并读取版本，不把版本字符串当安装git对象身份验证或CSR证明。后续formal/synthesis沿用`ci-install-sby.sh`、`formal-sby-check.sh`并建立独立CSR harness。历史M1证明仅作为可复用环境证据。

来源索引沿用创建阶段已核验的[维护者0.1.28说明](https://pypi.org/project/cocotbext-axi/0.1.28/)和[cocotb2.0.1文档](https://docs.cocotb.org/en/v2.0.1/library_reference.html)；本轮未重新读取网页，以本机固定安装源码和真实运行交叉核验API。Arm IHI0022先前入口重定向、无可读正文的限制保留，不据此新增协议断言；产品行为依据批准合同。

pip缺失已以正确的metadata查询解决，无需安装pip/升级包。没有未解决工具发现或架构表达阻塞；后续产品行为仍未测。独立review、automate、clean/fmt/just test和单故事commit由主流程继续，人工M13保持待验。实际sprint没有因本轮工具变异写入，FR196/M2保持未交付。

审查修补后：gate拒绝Python优化模式，正常27场景仍通过；BFM再次exit0。review日志另存。原artifact清单是build时点快照，其中gate旧源码保存为127-1-atdd-gate-initial.py.txt，hash与清单一致；当前脚本在最终验证另存hash，不覆盖历史。

独立review补充：review重跑的JUnit/stub保存至`127-1-build-review-bfm-artifacts/`，种子1789952959，对应review日志；初次build种子1789952427另存，不能混用。原hash清单现在直接指向归档旧脚本，所有命名路径均可直接验证。

Python版本差异核查：启动器及`Py_GetVersion()`均返回3.12.3，find-libpython定位`/usr/lib/x86_64-linux-gnu/libpython3.12.so.1.0`（实际命令见127-1-python-runtime.log）。GPI日志3.12.4使用编译期`PY_VERSION`宏，见[cocotb v2.0.1官方源码](https://raw.githubusercontent.com/cocotb/cocotb/v2.0.1/src/cocotb/share/lib/embed/gpi_embed.cpp)第85行；这是wheel编译信息，不是运行时版本查询。本轮未在模拟器内额外插入Py_GetVersion探针，不把启动器查询冒称嵌入进程实测；真实BFM仍以该进程PASS为准。
