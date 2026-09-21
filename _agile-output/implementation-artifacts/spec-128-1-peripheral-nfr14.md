---
title: 'Epic128 外设风险闸门'
type: 'chore'
created: '2026-09-21'
status: 'done'
route: 'dispatch'
review_loop_iteration: 0
baseline_commit: 'b926629e93b0ad5e09d795b59ec2f1bdb47010b6'
context:
  - '/nvme_data2/richard/2026/rhdl/_agile-output/implementation-artifacts/128-1-外设-nfr14.md'
  - '/nvme_data2/richard/2026/rhdl/docs/ip/phase24-contract.md'
---

<frozen-after-approval reason="沿用用户已批准的逐故事七步执行授权">

## Intent

完成128.1的NFR14记录及真实基座探针，为后续外设开工提供可审核依据。完整验收以context故事AC1–6为准；本次build只完成其中风险正文、工具探针和门禁证据，后续独立审查、automate、全量回归和提交由主流程执行。

## Boundaries & Constraints

严格沿用正式合同；GPIO32、IRQ5、Timer32、UART8N1与32位分频/双8×4 FIFO不得缩减。保留既有产品和工具钉。FR197/M3仍未交付，128.2–5保持backlog，FR189保持deferred。无产品API、推送、发布或新范围。

</frozen-after-approval>

## Code Map

- `128-1-外设-nfr14.md`：六项AC、精确接口与跨故事依赖。
- `nfr14-risk-record-template.md`：必填(a)–(d)及维护叠加。
- `../../scripts/phase24_axi_bfm_probe.py`：固定闭包断言及独立stub；产物在target/phase24-axi-bfm-probe。
- `../../crates/bitloom/tests/fr196_csr.rs`：`p0_same_session_real_peer_owns_counter_and_partial_write_wins_increment`；三模块真实RTL、单owner与候选拒绝，产物target/fr196-csr/peer-system-*。
- `../test-artifacts/128-1-atdd-gate.py`：35状态场景，精确诊断并保护sprint字节；不是全部跨故事依赖验证。
- `../test-artifacts/128-1-atdd-manual-acceptance.md`：15项人工证据映射。

## Tasks & Acceptance

- [x] 新建`epic-128-nfr14.md`：依模板完整填写风险、逐外设合同/验收、工期、owner、依赖、失败动作及停止条件。风险有效性与七步最终关闭分开。
- [x] 新建`../test-artifacts/128-1-build-*`证据：实际发现Rust/Cargo、Python固定包、Icarus/vvp、Yosys/sby/z3路径和版本；记录UTC、命令、退出码、原始日志。
- [x] 实跑固定BFM与上述单个CSR RTL探针，归档原始产物至test-artifacts，避免最终clean删除；重跑35项gate并另审跨故事依赖。
- [x] 更新人工清单和故事build记录，只勾已核验项；最终七步项保持待办。

验收：AC1–5全部可查证，AC6仅记录build范围。不得把旧基座、工具发现、未跑formal/综合或未来产品列为新外设PASS。无未解决阻塞后可建议风险accepted，但不得关闭故事或Epic。实现代理只编辑风险正文、build证据、人工清单和故事build记录；主代理管理spec、sprint、总账及后续步骤。发现无法表达的合同保留失败并报告，不自行降级。

## Implementation Notes

无待用户决策项，无不可逆操作；涉及风险正文与多种真实工具证据，采用dispatch。调查已完成。现有七步授权覆盖本spec；提交留第七步。

## Spec Change Log

## Review Triage Log

三层结果：blind提出10项（190.923KB→floor10），edge=[]，verification=No verification gaps found。逐项核验后均属本次文档/证据的直接修补，无intent gap或bad spec。

|ID|判定/路线|核实依据与处理|
|---|---|---|
|B1|low / patch|故事ready-for-dev与实际build状态不一致；改当前in-progress，保留历史。|
|B2|low / patch|File List仅三项，缺本次核心交付；补清单。|
|B3|low / patch|SHA manifest混用根目录路径和archive basename；统一仓库相对路径。|
|B4|medium / patch|35场景测试的实际gate源码未列hash；补来源身份。|
|B5|medium / patch|临时工具路径易失，缺可恢复安装引用；补现有脚本/历史来源，不虚构本轮安装。|
|B6|low / patch|风险指定sby完整pin但本轮仅版本发现；明确后续formal前精确来源核验前置。不是本次已执行proof失败。|
|B7|low / patch|当前35例只覆盖ready目标，未锁定更后活动状态；补12负例，不改变历史35结果。|
|B8|low / patch|当前35例未主动变异历史deferred；补3负例并保留准确诊断。|
|B9|medium / patch|UART中心采样未给奇数周期舍入/锚点决策责任；补128.5时序表及独立向量要求，不改既定协议。|
|B10|medium / patch|通用验证清单未逐外设区分独立核与层级；补方法适用表，禁止unsupported当PASS。|

实现代理续接B1–6、9–10；主代理补充B7–8独立15例脚本，保留原ATDD35及历史hash。修补后主代理按spec再验：BFM1、CSR1、gate35及补充15均通过，9个来源/归档SHA一致。无defer，详见128-1-build-root-recheck.json；不提前提交。

## Verification

工具PATH候选：`/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin`；每个入口重新探测。

- `/tmp/bitloom-phase24-bfm-py312/bin/python scripts/phase24_axi_bfm_probe.py`：固定五包、1用例无失败/跳过。
- `cargo test -p bitloom --test fr196_csr p0_same_session_real_peer_owns_counter_and_partial_write_wins_increment -- --exact --nocapture`：实际iverilog/vvp且FR196 PASS。沿用`CARGO_PROFILE_TEST_OPT_LEVEL=1`、`BITLOOM_REQUIRE_RTL=1`。
- `python3 _agile-output/test-artifacts/128-1-atdd-gate.py`：35通过，真实sprint不变。
- 人工逐项映射(a)–(d)、接口/停止条件、依赖；正式合同优先历史草案。
