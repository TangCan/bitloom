# Story128.4 implementation decisions

2026-09-21。按spec全部context读取，先读架构脊柱与CSR rtl、旧GPIO base/FL、IRQ/formal/runner/archive/harness，再新增独立GPIO模块；旧GPIO/CSR/后端均未改。修改前读FR142、Phase24合同与CI，追加保持历史门禁和历史状态。

唯一OUT用out_r保存，OUT candidate来自沿前out_r；SET/CLEAR candidate已由CSR执行WSTRB掩码，三个commit单请求互斥。DIR/RISE_EVENT保持CSR唯一owner。同步链sync1/sync2/history各32位、共同同步reset；rise_bits只用沿前sync2/history/DIR，raw_event另做reset门控。基座W1C本身set优先，不添加影子EVENT状态。

首次核心编译失败（多余括号/字面量类型）保留128-4-build-first.log和atdd-first.log，修复后direct三seed通过；API首次仅软件产物缺失，生成原文prelude-only例后修复。首次formal证明/cover/综合通过，clear-priority变异修改共享rise_bits导致先触发gpio_raw而非预定gpio_events；保留formal-first.log，改为仅变异CSR实例rise_bits输入，原始DUT与观察器不变。后续完整runner结果单列，不能并集这些初次日志冒充一次完整run。

形式ghost只从原始端口演算同步级、DIR/OUT/EVENT与响应，无DUT next-state复用。只假设初始reset与合法受阻请求保持；响应ready/pad任意，无最终ready公平性。状态对应均assert。10个cover包括bit31、双向DIR竞争、同位/异位clear-rise、五拍背压自然事件、reset取消后提交、IN/事件快照和初始高传播。三个真实DUT故障：IN接OUT、clear胜set、SET绕过WSTRB；同观察器要求原始control归纳PASS，变异为指定行为标签FAIL并保存VCD，编译错误/超时不计。

证据工具从128.3逐项映射：GPIO三组合目录（含gpio-irq）、两exact入口、gpio-csr软件产物、gpio_prove/gpio_cover、三故障目录及综合顶GpioCsr。原IRQ工具和CI入口保留。新runner与archive使用调用者环境、selected run和相关源码指纹；日志不进入指纹。harness普通与-O各12场景检验选定run、完整门禁、exact执行数、工具影子/缓存、软件差异和覆盖保护。

本记录不代替主代理独立review、automate和最终clean/fmt/just test；不改sprint/总账、不clean、不commit/push/publish。GPIO之外范围未交付，FR189仍deferred。
