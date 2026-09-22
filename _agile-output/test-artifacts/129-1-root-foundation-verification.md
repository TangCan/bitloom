# Story129.1：主代理独立基座核验

2026-09-21，主代理实际执行 `129-1-root-foundation.py`；输入工具通过环境指定，脚本按仓库位置发现源码。35 个命令全部 exit0，570 个产品/构建输入 SHA 起止一致。详细命令、路径、UTC、耗时及输出见 `129-1-root-foundation-results.json` 和同名前缀归档；不是 FR198 系统测试或形式证明。

工具实际身份：Rust/Cargo1.97.1，Python3.12.3，Icarus/vvp12.0，firtool1.159.0，Java17.0.20，Yosys0.33 git2584903a060，Z3 4.8.12。BFM 安装闭包由原脚本核验 cocotb2.0.1、cocotb-bus0.3.0、cocotbext-axi0.1.28、find-libpython0.5.1、scapy2.7.0。Scala/Chisel 的实际编译身份由单独的复位探针记录；Java 发现不替代该门。

SBY 本轮从官方 origin/tag 对象及 peeled commit 读取 Git 原件，重新应用官方安装替换，逐字比较16个实际安装文件；还拒绝覆盖已核文件的影子模块。独立 `python3 -I -B`、核验 support 路径优先加载，实际 `--version` 为 SBY yosys-0.47。完整记录在归档 `run/sby-identity.json`，此处未运行 prove/cover。

BFM stub 实际执行1项读写/五通道暂停接口/背压/复位取消测试，日志含 `BFM_API_PROBE_PASS`，结果 XML 无失败/跳过。旧桥译码 exact 测试实际1 passed、0 failed、0 ignored；本次目录 `target/fr196-decoder-integration/rtl-1681109` 的仿真报告31提交/31响应、26叶子提交、7B/24R，包含真实生成RTL/TB/编译与运行日志。它使用旧 CsrBlock peers，不是四个真实外设系统。

归档保留341个文件；`129-1-root-foundation-manifest.json` 给每个成员 SHA，并明确当前运行目录。归档中其余 decoder 目录是已有快照，不合并为本次成功证据。原始归档 SHA 为 `b19a66446c10d381d2091816c1033367deb57118887358fcaef85b41cb1c42a0`；最终 clean 后须再核。

人工按正式 epics 的 Given 逐边核对结果见 `129-1-root-dependency-audit.json`：129.1 的 M0 前置已完成；129.2 的127.4/128.2/128.4/128.5均done，但129.1尚未done；129.3依赖129.2仍backlog。不能把过程gate称为这些边的自动检查。IRQ128.3已done且是实际组合依赖，正式129.2的Given虽未单列它，不应省略其真实模块。

源码独立核对：UartCsr `events_lo={launch,receive}`、`events_hi={framing,overflow}`，组合后 raw_events[0] RX成功入队、[1]TX取队首、[2]overflow、[3]framing。因此五路IRQ接线必须是 `{gpio_raw, uart[3]|uart[2], uart[1], uart[0], timer_match}`，不能简单连接四位UART加Timer而漏GPIO；所有事件均按非sticky源验收。AD-30明确 aresetn 的外部同步条件，薄无状态边界可负责转换，不能声称反相实现同步。
