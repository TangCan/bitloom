---
title: '126.4 参数 FIFO 与 M1 关闭'
type: 'feature'
created: '2026-09-20'
status: 'done'
route: 'dispatch'
review_loop_iteration: 0
baseline_commit: 'b248dd48a948a303de9aa6aebf656a5c356b52f2'
context:
  - '{project-root}/AGENTS.md'
  - '{project-root}/_agile-output/implementation-artifacts/126-4-参数-fifo-与-m1-关闭.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-126-context.md'
  - '{project-root}/_agile-output/test-artifacts/atdd-checklist-126-4-参数-fifo-与-m1-关闭.md'
---

<frozen-after-approval reason="用户已授权逐故事执行全部七步">

## Intent

实现 ParamSyncFifo，使小型参数化 FIFO 能在同一 session 组合；故事全部十条 AC 为完整合同。验收后关闭 M1。

## Boundaries & Constraints

WIDTH1..64、DEPTH1..16，默认32/4；共用唯一模块体、完整参数身份，先诊断后声明。沿用既有 HIR/builder，寄存器存储，保持旧 SyncFifo、native 层级拒绝及工具 pin。不推送或发布。M1 仅由主代理完成七步后关闭。

## I/O & Edge-Case Matrix

|状态|结果|
|---|---|
|空入队|下一拍有效，无直通|
|满并出队|当拍拒绝入队，下拍恢复|
|中间占用同时传输|有序守恒；DEPTH1 无此情形|
|背压|有效数据保持|
|reset/flush|reset优先，取消在途，取消沿不计传输|
|非法参数|带名称/值的 Diagnostics，无 panic 或半定义|

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip/rv_reg_slice.rs`：共享定义、同步 reset 和 builder 范式；新文件独立实现。
- `crates/bitloom-prelude/src/ip/sync_fifo.rs`：旧 RAM 与注册 dout，只读；未初始化 RTL RAM 不能强制为零。
- `crates/bitloom/tests/fr195_param_sync_fifo{,_formal}.rs`：真实 API 编译红已归档，独立队列与工具脚手架；保留合同，允许有证据的脚手架修复。
- `scripts/check_fr194_example.py`：新增第三个完整 prelude-only 文档例检查。

## Tasks & Acceptance

- [x] 新建 `crates/bitloom-prelude/src/ip/param_sync_fifo.rs`，在 `ip/mod.rs` 重导出。
- [x] 完成两套 fr195 FIFO 测试，保存真实执行、版本、种子和产物。
- [x] `.github/workflows/ci.yml` 加入严格 FIFO formal/综合门禁和失败产物上传；复用已有安装流程。
- [x] 新增 `docs/ip/param-sync-fifo.md`；更新文档例检查与 `docs/public-api-1-0-surface.md` 的 minor/API 登记。
- [x] 准备 `epic-126-closeout.md` 证据索引，明确七步最终核验未完成；主代理最终同步风险、合同、需求和 sprint。

Given 完整故事合同，When 运行 API 测试，Then 24配置×3种子双native及真实RTL、三实例隔离、参数诊断、旧FIFO兼容通过。

Given 实际生成RTL，When 运行专用工具入口，Then W1×D1/2/3 安全归纳与非空cover通过，1×1/8×3/64×16 原始RTL综合无latch/多驱动且记录cells。缺工具、UNKNOWN、超时均失败。

Given 新文档与现有接口，When 检查完整示例和兼容测试，Then prelude-only示例可执行且旧行为保持。

## Implementation Notes

无意图缺口、无不可逆操作；新增API及验证属于同一交付，采用dispatch。现有脏文件均为本故事已授权ATDD/状态产物。实施代理负责产品、测试、CI、文档与证据，不改 sprint/story 状态、不提交、不 cargo clean；七步末由主代理提交。

## Design Notes

移位寄存器队列可避免非二次幂指针：count宽度须表达DEPTH；pop后移位，push写入count-pop位置。此为可选实现，不改变端口合同。formal独立端口队列可能需针对实际寄存器布局追加纯assert归纳不变量；不得新增内部一致性assume、删除合同断言或把有界检查称为证明。原始RTL与observer分开保存。旧FIFO实测仅比较已写入已知RAM后的定义值。

## Spec Change Log

## Review Triage Log

## Verification

PATH可加入 `/tmp/bitloom-maintenance-tools/bin`、`/tmp/bitloom-1263-sby-installed/bin`；实际版本写入证据，临时路径不能成为产品依赖。使用 `CARGO_PROFILE_TEST_OPT_LEVEL=1` 保留断言。

- `cargo test -p bitloom --test fr195_param_sync_fifo -- --nocapture`
- `cargo test -p bitloom --test fr195_param_sync_fifo_formal -- --ignored --nocapture`
- `cargo test -p bitloom --test fr82_fifo_uart_baseline --test fr103_ip_dual_model`
- `cargo test -p bitloom-prelude`
- `python3 scripts/check_fr194_example.py`

真实日志与必要proof/synthesis产物保存至 `_agile-output/test-artifacts/126-4-*`，避免后续clean丢失。主代理随后执行build内审、独立code-review、automate、clean/fmt/just test；此前不得声称M1关闭。


## 首轮 build 审查裁定

Blind 10 条；Edge 0 条；Verification 0 条。主代理检查代码与已运行证据后逐条裁定：

|编号|裁定/路由|证据与处理|
|---|---|---|
|B1|low / patch|默认深度两侧均省略，不能锁定4；改为显式32,4参考。|
|B2|medium / patch|73拍重叠紧接72拍reset，实际为空；增加在途重叠取消并断言非空命中。|
|B3|low / patch|组合仅初始化共享reset，后续转换为独立flush；补多实例在途共享reset及恢复向量。|
|B4|low / patch|双非法参数仅检查WIDTH；产品已报告两项，补逐参数诊断验证。|
|B5|low / patch|冲突后未核验保留定义；补finish与原定义一致性断言。|
|B6|low / reject|要求额外深度行为脚手架超出已完整运行的指定24矩阵，需增加测试分支；现有通用移位结构与1/2/3/4/7/16已覆盖计数宽度及非二次幂，不为可选冗余引入复杂度。|
|B7|low / reject|新增跨恢复接收到交付cover需额外观察状态；当前安全归纳已证明每个有效槽与独立队列相等，cover非空恢复接收，72组实际取消后交付通过。无证据表明现有明确验收未满足。|
|B8|low / patch|Rust类型文档遗漏取消沿表面握手不计成功；直接补充说明。|
|B9|low / patch|IP目录未索引新FIFO且状态过时；补入口与真实范围，保持M1待最终验收。|
|B10|low / reject|重复版本探针确实存在，但全部31测试仅2.86秒；缓存引入共享状态与路径依赖，不为未证实的实际开销增加复杂度。|

主代理矩阵审计：72个唯一配置/种子实测日志齐全，31 API/native/RTL、6专用formal/synthesis、12旧接口和17prelude通过；全部归档SHA256核对一致。以下修补完成后重跑适用验证，再进入独立code-review。

B5修补前进一步核验：builder::define_module 160–161/194–215在冲突时保留原定义但将E0244存入session，finish按合同失败，无公开快照。原“finish取得原定义”建议不适用；最终裁定low/patch仅补冲突阻止freeze断言，不改变builder、不宣称已从错误session取回HIR。

Build内审修复完成；主代理按Verification五条命令复跑全部通过，见126-4-build-review-regression.log。无延期项。用户七步顺序优先，当前不提前commit；独立code-review、automate与clean回归尚待完成。
