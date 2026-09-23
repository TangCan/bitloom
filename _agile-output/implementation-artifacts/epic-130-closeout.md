# Epic130 外部 IP 试点关闭（2026-09-23）

Story130.1–130.3 已各自完成。130.2 在本地修正为单一提交 `4152610`；130.3 由本提交独立归档。仅关闭 FR199、FR200 与 FR201 外部试点对应行，不把外部结果替代核心系统矩阵或宣称更广子集。

| 合同 | 交付及证据 |
|---|---|
| 130.1 / NFR14 | 既有风险、来源、维护/许可证准入合同；原关闭有效 |
| 130.2 / FR199 | 三仓库210文件完整不可变身份与许可证；来源-only空缓存获取、复制禁网重放、只读原输入、精确负测；`130-2-final-verification.md` |
| 130.3 / FR200 | 实际 Bitloom HIR父模块链接锁定fifo_v3；32-bit/depth8/fall-through0独立队列模型104步；原上游六组各100000比较；`130-3-final-verification.md` |
| FR201外部行 | 来源/编译/行为证据与required CI入口、owner Richard、升级/弃用策略齐备，单一试点maintained；`docs/ip/phase24-support-matrix.md` |

两份独立clean/fmt/workspace：130.2为1889 passed / 0 failed / 50 ignored；130.3为1893 passed / 0 failed / 55 ignored。真实外部工具门禁另行显式运行；ignored不计PASS。130.3后续继承的FR199单测单独复跑11 passed / 1 ignored，未重复加到workspace总数。四层23条发现全部修复，无未解决阻断项。

canonical lock `49b22842deda1d09bb86baaa5c8c3474f013a52c7d587f624f827705b5141d6e` 绑定CLI `c40e36535ba32de8c285161c508aa06d1581809762dd493bcb9a2097acffd0c4` 与真实编译/仿真helper。FR199关闭时的工具快照单独保存；来源记录未变。上游六进程保留原driver/oracle/参数/时钟/reset、启用断言，每组seed1303，随机轨迹不同于原顶层交错运行；不是逐刺激等价或形式证明。原900秒超时、1000次诊断及冗余单进程取消均留档且不计PASS。

required CI job与Just入口已配置并在本机实际执行对应命令，不声称远程CI或发布完成。设计crate只依赖prelude，第三方源码不进入HIR，无第二IR；未新增公共library符号，CLI逐项登记FR142/minor。无native/generated层级模型、任意参数、多时钟、复杂协议或物理签核承诺。没有改产品工具钉/包版本，没有push/publish；FR189/Epic122 deferred、NFR91保持，不由本Epic宣称整个Phase24完成。
