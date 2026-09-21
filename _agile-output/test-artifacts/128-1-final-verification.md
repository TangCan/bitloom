# Story128.1 最终验证与关闭

2026-09-21，基线`b926629e93b0ad5e09d795b59ec2f1bdb47010b6`。本故事交付Epic128 NFR14风险闸门；Timer/IRQ/GPIO/UART仍分别待128.2–5实施，FR197/M3和Phase24整体未交付。

## 七步与AC映射

|步骤/AC|实际完成与证据|
|---|---|
|create / AC1–3|128-1-外设-nfr14.md、create-story-validation、root-prerequisite-audit；正式合同和M0/M1/M2依赖核验|
|ATDD / AC5|正文缺失真实exit1历史RED；35原有过程场景GREEN，15人工项不伪装新外设测试|
|build / AC1–4|epic-128-nfr14.md(a)–(d)、精确参数/风险/停止条件/维护叠加；工具实际发现、固定BFM1通过、CSR peer2311帧通过；三路内审10补强|
|独立review / AC1–5|四层审查10补强，0未解/0defer；code-review.md。门禁扩为55场景（35原有+20补充），真实sprint未被fixture修改|
|automate / AC4–5|33个CLI调用覆盖8定向mutation运行（7种唯一源码变体），另1个runner复用CSR leaf native/RTL测试；34调用/集成案例全通过，0新产品测试；automation-summary.md|
|完整回归 / AC6|实际cargo clean → cargo fmt --all → just test，全部exit0，原始日志/JSON见128-1-final-*|
|commit / AC6|随本Story128.1单独本地提交闭合，提交SHA见Git；不推送、不发布|

## 实际全量结果

- clean：4.128s，移除26,751文件、12.2GiB。
- fmt：1.136s，产品Rust文件无变化。
- just test（实际cargo test --workspace）：895.897s，470结果块、1823 passed / 0 failed / 25 ignored。
- 总计901.163s。日志哈希、UTC和环境在final-regression.json，精确ignored清单在final-test-summary.json。

25 ignored为19个既有专用形式/综合测试和6个既有doc-test，不计PASS；本风险故事没有重新声称专用formal或综合通过。M1/M2历史专用证据保持历史属性；未来产品formal前还须按风险记录核验实际sby安装身份。

定向build与automate的原始SV/XML/compile/run/工具日志已各自归档；本轮workspace生成的CSR产物另存`128-1-final-csr-artifacts.tar.gz`（264文件，8836345字节，SHA256 `9369131703f3167741ca06a1a994b346b776f5186f92f4d7397aa7683e105517`），逐文件hash见final-artifact-manifest.json。所有持久证据位于test-artifacts，后续clean不会删除。

## 关闭边界

只将128.1置done，Epic128仍in-progress，128.2–5及129/130待各自故事；既有done与FR189/Epic122 deferred/NFR91保留。没有产品代码、API、包版本或工具钉改动。完整文件清单见128-1-file-manifest.txt；raw vvp版本日志的尾部空行保留原貌，未为通过whitespace检查改写真实证据。

下一故事128.2 Timer，前置128.1/127.2完成；其它外设仍按正式跨故事边推进。风险闸门有效不等于未来算法、CDC/板级、PPA、FR198系统或外部核已经交付。
