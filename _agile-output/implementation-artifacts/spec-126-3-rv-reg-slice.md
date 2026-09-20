---
title: '126.3 两槽 ready/valid 注册切片'
type: 'feature'
created: '2026-09-20'
status: 'done'
route: 'dispatch'
review_loop_iteration: 0
baseline_commit: '62832eab1915faf46a1050e44120f44f100b33db'
context:
  - '{project-root}/AGENTS.md'
  - '{project-root}/_agile-output/implementation-artifacts/126-3-两槽-ready-valid-注册切片.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-126-context.md'
  - '{project-root}/_agile-output/test-artifacts/atdd-checklist-126-3-两槽-ready-valid-注册切片.md'
  - '{project-root}/_agile-output/test-artifacts/126-3-formal-tool-probe.md'
---

<frozen-after-approval reason="用户已授权全部故事依次七步实施">

## Intent

补齐可组合的两槽注册切片，隔离输入到输出组合路径。实现故事126.3全部AC并让独立ATDD在native、真实RTL和形式工具上通过。

## Boundaries & Constraints

始终遵守故事端口、WIDTH1..64、reset优先flush、取消记账、寄存输出和模块身份合同。使用既有builder和HIR，设计只依赖prelude。无发布或push；不改变旧SyncFifo、工具钉或native层级unsupported。

## I/O & Edge-Case Matrix

| 情景 | 预期 |
|---|---|
| 空态接收 | 下一周期有效，无直通 |
| 单槽同时出入 | 顺序守恒，每拍一笔 |
| 满态释放 | 当拍不接收，下一拍恢复ready |
| 背压 | valid/data保持至传输或取消 |
| reset/flush重叠 | reset优先，取消在途；无组合影响 |
| 非法宽度/同名不同宽度 | Diagnostics，不panic |

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip/gpio/base.rs`：共享define_module与单独elaborate的范式；params完整登记。
- `crates/bitloom-prelude/src/ip/mod.rs`：协议模块和pub use。
- `crates/bitloom-builder/src/lib.rs`：既有assign、reg、同步reset，复用不另建IR。
- `crates/bitloom/tests/fr195_rv_reg_slice{,_formal}.rs`：已归档缺API红测；保留独立队列期望，实际运行后可修复有证据的脚手架问题，不能削弱AC。
- `scripts/ci-install-sby.sh`、`scripts/formal-sby-check.sh`及FR119 fixtures：探针已复现阻塞，修复安装和status语义，保持pin及expect fail。

## Tasks & Acceptance

**Execution:**
- [x] 新建 `crates/bitloom-prelude/src/ip/rv_reg_slice.rs`，重导出公开泛型API；共享唯一模块体。
- [x] 完成两套fr195验收测试；保存native/RTL/formal真实通过证据。
- [x] 修复上述脚本及 `scripts/ci-sby-pins.env` 注释；验证annotated tag严格解引用和上游make安装，无PYTHONPATH依赖。FR119正例PASS，负例实际FAIL并返回非零。
- [x] `.github/workflows/ci.yml`加入强制RV formal命令；保留已有hygiene/install顺序。
- [x] `docs/ip/rv-reg-slice.md`中文使用说明、`docs/public-api-1-0-surface.md`显式追加API及minor说明、phase24合同更新实际交付范围。

**Acceptance Criteria:**
- Given故事AC1–5，When运行fr195非formal测试，Then四宽、三种子、双实例、真实RTL全部通过。
- Given真实生成RTL，When专用formal入口运行，Then WIDTH1/8归纳prove和cover各PASS且无组合路径；缺工具/UNKNOWN/超时非零。
- Given既有formal CI，When修复前置工具与fixtures，Then正例通过且负例由真实反例失败。

## Implementation Notes

无意图缺口或不可逆操作；新增API和跨层验证，采用dispatch。现有脏文件均为本故事先行合同与ATDD产物，用户七步授权覆盖继续工作。完整故事AC是合同。
实施代理负责上述产品、脚本、CI与文档；不改sprint/story状态，不提交。主代理在实施后执行build内审、独立code-review、automate、clean回归，第七步才提交。用户顺序优先于技能内部提前commit。

实施记录：产品使用front/back/count寄存器，所有端口输出仅依赖寄存态。ATDD双实例因两个顶层Reset触发既有E0121，改用共享rst及独立flush；未放宽HIR。formal追加观察器与纯assert归纳不变量，原始RTL另存并做结构检查；最终smtbmc --unroll z3的basecase、induction和cover均PASS，早期UNKNOWN如实归档。安装前置修复保持原SBY对象钉，不宣称完整sudo/apt环境部署已验证。

主代理矩阵审计：空入队、单槽同拍、满释放、背压、取消由四宽测试的独立队列和命中断言覆盖；非法参数与模块身份由专门API测试覆盖。8项native/RTL、2项实际formal及15项前置回归均已执行通过，见126-3-build-final.log。全workspace回归尚待步骤六。

## Spec Change Log

## Review Triage Log

首次build内审：Blind 10条；Edge 0条；Verification 1条。逐项裁定如下，同根因仅在执行修补时合并。

|编号|裁定/路由|证据与处理|
|---|---|---|
|B1|medium / patch|formal CI只跑FR119正例和FR195；workspace无sby可跳过旧负例。强制工具存在后运行现有真实负例。|
|B2|medium / patch|FR195失败日志只留runner target，CI无upload。增加失败产物上传以保留可诊断证据。|
|B3|medium / patch|timeout仅TERM没有kill-after，无法保证已声明的有限退出。对已有超时补强制终止期限。|
|B4|medium / patch|Rust runner child.kill只杀driver，Icarus子进程可能残留。使用同一timeout进程组机制及kill-after。|
|B5|medium / patch|FR119测试继承fixture override却读取默认trace路径，可验证错位。移除该测试的override使运行与证据对应。|
|B6|medium / patch|组合路径检查器只有正例，删除递归将不被检测。增加直接和多级输入路径负例及寄存边界正例。|
|B7|medium / patch|drivers.get(None)返回false，未驱动输出可误过结构隔离检查。遇到非输入/常量/驱动的位必须失败，并加负例。|
|B8|low / patch|已有测试覆盖同宽复用与冲突，缺不同名异宽共存。追加共session专门化检查是直接覆盖补充。|
|B9|low / patch|formal与RTL均先reset，用户文档未明确首次使用前要求。直接补初始化合同说明，避免native零初始化误导。|
|B10|low / patch|唯一组合示例的top和finish留在注释，不能完整演示已公开用途。补完整prelude-only例并编译核验。|
|V1|medium / patch|验证层已核实真实负例可在默认CI跳过，与B1同根因，合并修补但保留本行。|


内审修复核验：B1–B10/V1均完成，主代理联合定向复跑27项通过（8+6+9+3+1），见126-3-build-review-regression.log；无延期项。build阶段完成，用户要求的独立code-review、automate、全量clean回归与commit仍待执行，故不在此提前提交。

## Verification

环境：PATH加入 `/tmp/bitloom-maintenance-tools/bin` 和 `/tmp/bitloom-1263-sby-installed/bin`；`CARGO_PROFILE_TEST_OPT_LEVEL=1`保留断言/溢出检查。

- `cargo test -p bitloom --test fr195_rv_reg_slice -- --nocapture`
- `cargo test -p bitloom --test fr195_rv_reg_slice_formal -- --ignored --nocapture`
- `bash scripts/formal-sby-check.sh`及 `BITLOOM_SBY_MODE=fail`负例：核验实际status，不能把语法错误当反例。
- 相关FR161/FR119回归；保留真实日志至 `_agile-output/test-artifacts/`。不清理用户或并发代理文件。
- 全workspace clean回归由主代理在步骤六执行；实施代理不提前cargo clean。
