# Story 129.2 code review

日期：2026-09-22。范围：相对`46249478384360590e90be59b4709afef967d4ee`的Story 129.2暂存实现。

BMAD Blind Hunter、Edge Case Hunter、Verification Gap与Acceptance Auditor三轮启动后均基础设施超时，没有返回可采信finding；未把超时记为clean review。主线程按spec、Epic129 context和NFR14逐项fallback审查，并实际运行目标RTL门。

## Triage

1. **patch / medium / fixed**：初始实现遗漏NFR14冻结的16 seed x 1000完成事务预算。两种拓扑现各自执行16个固定seed、每seed 1000笔完成事务；evidence保存seed、随机计数和总计数。
2. **patch / medium / fixed**：初始配方只在当前checkout/default Cargo target运行，未证明隔离重放。新增`129-2-isolated-replay.sh`，从HEAD+Story diff建立独立checkout，使用独立Cargo/RTL产物目录，并保存checkout、patch、源与工具指纹。实际空target重放通过。
3. **defer / contract ownership**：Story AC7把FIRRTL/Chisel四外设系统复核留给129.3，Epic129 NFR14表仍写129.2责任。冻结spec不在code review中修改；该矩阵登记到deferred ledger并由129.3实际关闭。
4. **rejected / false**：direct图缺Timer periodic/wrap或UART overflow。复核当前direct testbench已包含这些定向向量。

## Reverification

- 隔离checkout：AXI 16102事务/62416断言；direct 16098事务/60406断言；每图随机事务16000。
- 两图Icarus/VVP exit 0、VCD非空、Yosys `check -assert` 0问题。
- review未更改公开API、工具pin、包版本或既有外设算法。
