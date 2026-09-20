# Epic126 / M1 关闭证据索引

状态：Epic126 / M1 已完成，FR194/FR195关闭。本记录随Story126.4单独提交；全部七步证据如下，历史build阶段证据保持原样。

| 交付/边界 | 证据 |
|---|---|
| 126.1 NFR14闸门 | `epic-126-nfr14.md` |
| 126.2共享session模块组合 FR194 | `docs/ip/module-composition.md`、`docs/ip/module-composition-evidence.md` |
| 126.3两槽注册切片 FR195 | `docs/ip/rv-reg-slice.md`、`_agile-output/test-artifacts/126-3-final-verification.md` |
| 126.4参数FIFO FR195本阶段实现/实测 | `docs/ip/param-sync-fifo.md`、`_agile-output/test-artifacts/126-4-build-evidence.md` |
| API真实红→绿 | `126-4-atdd-api-red.log`、`126-4-atdd-formal-red.log`、`126-4-build-api.log`、`126-4-build-formal.log`（均在test-artifacts） |
| 安全归纳+cover及原始RTL综合 | `126-4-build-evidence/fr195-fifo-formal.tar.gz`；首轮UNKNOWN和最终PASS分开保存 |
| 24配置×3种子native/RTL与三实例隔离 | `126-4-build-evidence/fr195-fifo.tar.gz`；详细命中见`126-4-build-api.log` |
| 旧SyncFifo与完整prelude兼容 | `126-4-build-legacy.log`、`126-4-build-prelude.log` |
| 设计仅依赖prelude与显式API登记 | `126-4-build-doc-example.log`；`docs/public-api-1-0-surface.md` Story126.4追加 |

支持矩阵：单时钟寄存器FIFO WIDTH1..64 / DEPTH1..16；行为实测WIDTH1/8/32/64×DEPTH1/2/3/4/7/16；形式W1×D1/2/3；原始RTL综合1×1、8×3、64×16。native/generated层级仍unsupported；组合行为由真实RTL验收。不承诺异步/深RAM FIFO、BRAM推断或PPA。旧SyncFifo原API/行为保留。

新增类型/helper/Elaboratable入口显式登记FR142，属SemVer minor，代码未发布。M1之后Epic127–130仍须各自前置和NFR14；整个Phase24尚未完成。FR189 deferred保持、NFR91未清空，不push、不publish。

## 最终关闭与需求映射

2026-09-20 Story126.4七步完成，随本故事单独提交关闭Epic126/M1：126.1–126.4均done，FR194模块组合与FR195两槽注册切片/参数FIFO交付。clean + fmt + just test退出0，460个实际结果块汇总1763 passed、0 failed、14 ignored；8个专用形式测试另有真实运行证据，6个既有doc-test保持ignored。最终证据 `_agile-output/test-artifacts/126-4-final-verification.md`，关闭映射 `_agile-output/implementation-artifacts/epic-126-closeout.md`。Epic127–130 / FR196–201尚待各自七步与NFR14，整个Phase24未完成；FR189/Epic122 deferred及NFR91保持，不push、不publish。

|需求|交付与前置|最终证据|
|---|---|---|
|FR194|126.1风险闸门、126.2共享session与实例校验|module-composition-evidence.md；126.2提交62832eab1915faf46a1050e44120f44f100b33db|
|FR195 注册切片|126.3两槽、WIDTH1..64、取消与隔离|126-3-final-verification.md；提交b248dd48a948a303de9aa6aebf656a5c356b52f2|
|FR195 参数FIFO|126.4 WIDTH1..64/DEPTH1..16、无空直通/满拒绝、旧行为兼容|126-4-final-verification.md、automation-126-4.md；本文件所在Story126.4提交|
|NFR14/M1门禁|126.1–126.4全部完成，风险/支持矩阵/需求一致|epic-126-nfr14.md；check_phase24_gate.py|

独立review逐项裁定在126-4-independent-review.md；automation未发现需新增的覆盖，37项重新执行通过。最终clean回归1763 passed/0 failed/14 ignored；默认ignored不当作证明成功，证明依据为专用归档。审查后最终取消矩阵向量与源绑定以126-4-automation-*为准，首轮ATDD/build归档为明确历史快照。既有实例名合法性和Instance.params覆盖遗留仍保留，不宣称无限支持或账本清空。
