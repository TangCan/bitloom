# Epic129 / FR198 / FR201 核心关闭

状态：已关闭。Story129.1–129.3 均按单 Story 提交完成；FR198 与 FR201 核心部分交付。

|故事|范围|提交|
|---|---|---|
|129.1|组合系统 NFR14、复位路线和禁止降级门|`46249478384360590e90be59b4709afef967d4ee`|
|129.2|AXI-Lite/direct CSR 两种真实四外设组合与独立行为配方|`218f0620e85ab438a3ef1351e253277552c8c410`|
|129.3|direct/FIRRTL/Chisel 完整行为矩阵、formal/综合、CI、SemVer 和贡献合同|包含本文件的 Story129.3 单提交；以 `git log -- _agile-output/implementation-artifacts/epic-129-closeout.md` 审计|

Story129.3 证据见 `../test-artifacts/129-3-build-evidence.md` 与 `../test-artifacts/129-3-latest-results.json`。三后端均实际执行同一完整 AXI 四外设系统，各 16,000 随机事务、62,416 断言和非空 VCD；direct CSR 第二拓扑另行执行。formal 将 IRQ 归纳证明、reset/response bounded check、cover 和真实失败 mutation 分列；原始三后端综合分列。SemVer 三库通过，未新增公开 API、未改包版本或工具 pin。

最终 `cargo clean && cargo fmt --all && just test` 退出 0：484 个结果块，1878 passed、0 failed、49 ignored；专用后端/formal 门由 Story129.3 runner 另行实际执行，未把 ignored 计为 PASS。code-review 的四个独立 reviewer 均超时，审查按技能规则明确记为不完整，未伪造 findings 或 clean-review 结论；用户确认继续后完成 automate、回归和本单提交。

该关闭只覆盖 FR198 和 FR201 **核心部分**。Epic130、FR199/FR200、FR201 外部试点行和 Phase24 整体保持开放；外部支持矩阵为 `not delivered`。native/GeneratedFunctional 层级仍 unsupported，formal 不是全 AXI/UART 证明，综合不是 PPA/时序/物理签核。FR189/Epic122 deferred 与 NFR91 保持，不 push、不 publish。


## 2026-09-23 补审与全阶段结项

2026-09-23：Phase24 已批准范围 FR192–FR201 / NFR93–NFR99 完成交付与结项，Epic125–130 共22故事done。129.3四层独立补审及修复、两拓扑六路线实际RTL/原始综合、有限请求/复位形式、耐清理的主/隔离原始证据和新鲜证据校验已完成；FR199/FR200外部试点按130.2/130.3最终记录交付。FR189/Epic122/122.2/122.3继续deferred、未交付；NFR91及既有明确遗留保留。无产品工具钉/包版本变更，不push、不publish，不声称远端CI已执行或物理签核。

[总验收与FR/NFR映射](../../docs/ip/phase24-closeout.md)。先前M0-only、126.1/126.2-only、Epic130开放或Phase24未完成的分日期段落保留为历史，本条为当前状态。
