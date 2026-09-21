# Story127.1 最终验证

2026-09-21；基线 e88a22bef266b52174be51f831d7ef510b65f108。交付CSR/总线NFR14风险门禁，不交付CSR产品。

1. create-story完成；故事包含完整AC、正式合同/依赖与估算。
2. ATDD：原27个进程场景GREEN，缺风险正文真实exit1；14项人工清单。
3. build：风险正文与真实工具探针完成；三路审查10项，7修补、3反证拒绝。完整裁定在spec Review Triage Log。
4. 独立code-review：四层全部返回，7修补、3反证拒绝、0延期；详见127-1-independent-review.md。人工独立核对见127-1-manual-review.md。
5. automate：117/117门禁，新增90；优化模式正确拒绝，BFM1/1通过、312ns。保留fixture首次失败并修复Epic键误匹配；原始日志/JUnit/stub与18项hash均在target外，clean后核验一致。
6. 完整回归：2026-09-21T01:21:20Z至01:28:03Z，`cargo clean && cargo fmt --all && just test`退出0。clean移除25043文件/8.5GiB；460个结果块共1763 passed、0 failed、14 ignored。日志127-1-clean-workspace-regression.log，元数据同名json/.exit，统计127-1-workspace-results.json。环境PATH前置Icarus/SBY安装，CARGO_PROFILE_TEST_OPT_LEVEL=1，Rust1.97.1；不是所有远端CI/JVM gate均重跑。
7. 单故事提交：本记录随Story127.1提交；git log -1 --本文件可得完整SHA。不push/publish。

14 ignored中8个为既有专用形式测试，真实专用执行见126.3/126.4历史证据；本故事未重跑它们，不能把ignored称本次证明。其余6个既有doc-test仍ignored。无新CSR RTL/formal/综合/PPA交付。

历史与当前产物分开：初次27例脚本归档127-1-atdd-gate-initial.py.txt，首次automation失败源码另归档；当前117例脚本hash在127-1-automation-sha256.json。BFM build/review/automation各自有对应日志与JUnit，不混用seed。

状态：439 done、14 backlog、2 deferred；Epic127 in-progress，127.2–4 backlog，FR196/M2未交付。FR189/Epic122 deferred/NFR91保持。整体七步目标仍active，下一故事127.2。仅文档/测试工件变化，无产品代码/API/工具pin变化。
