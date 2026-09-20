---
stepsCompleted: ['step-01-preflight-and-context', 'step-02-generation-mode', 'step-03-test-strategy', 'step-04-generate-tests', 'step-04c-aggregate', 'step-05-validate-and-complete']
lastStep: 'step-05-validate-and-complete'
lastSaved: '2026-09-20'
storyId: '126.4'
storyKey: '126-4-参数-fifo-与-m1-关闭'
storyFile: '_agile-output/implementation-artifacts/126-4-参数-fifo-与-m1-关闭.md'
atddChecklistPath: '_agile-output/test-artifacts/atdd-checklist-126-4-参数-fifo-与-m1-关闭.md'
generatedTestFiles: ['crates/bitloom/tests/fr195_param_sync_fifo.rs', 'crates/bitloom/tests/fr195_param_sync_fifo_formal.rs']
inputDocuments: ['AGENTS.md', '_bmad/tea/config.yaml', '_agile-output/implementation-artifacts/126-4-参数-fifo-与-m1-关闭.md', '_agile-output/implementation-artifacts/epic-126-context.md', '_agile-output/implementation-artifacts/epic-126-nfr14.md', 'docs/ip/phase24-contract.md', 'crates/bitloom/tests/fr195_rv_reg_slice.rs', 'crates/bitloom/tests/fr195_rv_reg_slice_formal.rs', '_agile-output/test-artifacts/automation-126-3.md']
---
# Story126.4 ATDD

Richard，本轮Create。resolver前后置/持久事实均空。Rust Cargo backend，已有集成测试框架，AC1–10明确，前置126.1–126.3完成。用户连续七步授权涵盖输入确认。采用AI生成；无浏览器、HTTP、数据库、移动或独立消费提供方边界。Playwright/Pact flags不适用Rust runner，不引入JS框架。一次工具列表probe：pact_mcp_reachable=false，未调用broker。

知识读取：data-factories、component-tdd、test-quality、test-healing-patterns、test-levels-framework、test-priorities-matrix、ci-burn-in及Playwright/Pact mandates、pact-mcp。采用独立模型、固定种子、隔离产物、严格工具退出与风险优先分层。框架范式复用126.3，不复用容量2假设。

## 步骤3：策略

AC1/2/5→P0 Rust API、精确端口/参数身份及非法边界；AC3/4/6→P0独立VecDeque的24配置×3seed双native/真实RTL，预后沿检查、取消epoch、真实覆盖命中。AC7→P0三实例(两个64×3复用、一个8×7专门化)真实RTL隔离，P1 native层级明确拒绝。AC8→P0实际RTL W1 D1/2/3 SBY安全+cover及含64×16/非二次幂严格Yosys综合。AC9→P0旧FIFO已知RAM RTL向量及既有黄金回归；AC10为后续文档/七步/状态人工证据门禁，不以脆弱文本测试替代。首红预期仅缺失ParamSyncFifo产生E0432，工具故障不是产品红；既有兼容测试可保持绿。

## 步骤4：生成与聚合

能力probe：有subagent，无独立agent-team启动，auto→subagent。API/native/RTL worker由协调端执行，formal/synthesis worker并行隔离；两个JSON success及完整文件已核对。没有并行加速量化声明。用户实际首红要求优先于通用JS test.skip模板；Rust行为入口主动执行，formal六项为专用ignored入口，必须显式--ignored运行，不把忽略当通过。无额外fixture文件，队列数据工厂与超时runner封装于测试target内。

## 步骤5：实际红测与验证

2026-09-20，PATH=/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin:$PATH，CARGO_PROFILE_TEST_OPT_LEVEL=1：

```sh
cargo test -p bitloom --test fr195_param_sync_fifo -- --nocapture
cargo test -p bitloom --test fr195_param_sync_fifo_formal -- --ignored --nocapture
rustfmt --edition 2024 --check crates/bitloom/tests/fr195_param_sync_fifo.rs crates/bitloom/tests/fr195_param_sync_fifo_formal.rs
git diff --check
```

前两条实际exit=101，均唯一编译错误E0432：bitloom_prelude::ip::ParamSyncFifo未定义。日志与独立.exit见126-4-atdd-api-red及126-4-atdd-formal-red。这是真实缺API首红；未执行到产品行为、Icarus、SBY或综合，不能称这些阶段失败或通过。formal入口另有因缺API连带的unused-import warning。后两条检查exit=0。

独立抽取VecDeque trace的Rust probe实际exit=0，72组(24配置×3seed)全部覆盖断言通过，保存126-4-atdd-vector-probe.rs/.log/.exit；只是测试向量质量检查，不是产品或形式证明。每组420拍含末尾20拍排空、至少两次满→空、每个占用命中、W64高位传出、在途reset/flush/重叠、取消epoch守恒和生产者保持。D1同时push/pop严格0，其余必须大于0。三个固定seed为0x12641950a551/0xdeadbeef8012/0x73592401ffff。

### 验收检查

- [x] AC1–9映射到31项Rust API/native/RTL与6项专用formal/synthesis测试；默认32×4、9端口、WIDTH/DEPTH冲突、非法0/65/17/MAX、失败共享定义后合法恢复。
- [x] 实例顶层唯一Clock/Reset，独立flush，空参数实例、两个定义三个实例；native层级必须明确unsupported。
- [x] 原始RTL与observer分离；综合只读原始design.v，严格Yosys check -assert和cell白名单，不称PPA/BRAM。形式仅初始化reset及合法生产者协议assume，无ready公平性或内部一致性assume。
- [x] 工具runner有限timeout+kill-after，非零/UNKNOWN/timeout不通过，产物目录带进程/配置，失败保留。
- [x] 旧RAM测试先写已知四字再检查读延迟/满阻写/reset保留RAM，首个未知读取不强制0。
- [x] 测试无顺序依赖，无工具替身，无产品实现；保持故事/sprint状态，不提交/clean。
- [x] 浏览器/HTTP/JS/Pact/faker/selector/数据库mock及页面fixture均N/A，Playwright/Pact deviations无(不在作用域)。无浏览器会话需关闭；工具日志保留属于验收证据。

### GREEN实施清单与入口

- [ ] 新建ParamSyncFifo产品模块并重导出，先校验WIDTH/DEPTH，再注册模块参数和唯一寄存器实现体；运行API target至绿。
- [ ] 在产品存在后实际跑上述formal专用命令；纯端口队列归纳若不足，按实际寄存器布局增加可证明的辅助assert，不得改assume内部状态或删除合同断言。首红阶段未声称形式 harness 已通过工具前端。
- [ ] 保存24配置native/RTL实际命中日志、真实原始RTL、SBY prove/cover见证、版本、synthesis cell统计；把target内结果归档后才clean。
- [ ] 运行cargo test -p bitloom --test fr82_fifo_uart_baseline --test fr103_ip_dual_model及cargo test -p bitloom-prelude；旧黄金向量维持。
- [ ] AC10中文使用页、prelude-only组合例、FR142明确表面登记、SemVer minor/未发布、文档门禁、独立review、automate、clean/fmt/just test、主代理一故事提交及真实M1收口。估算沿用Epic126 3–5有效人日，不是运行墙钟承诺。

下一步骤为已授权build；无需重建测试框架或重新ATDD。没有暂定UI端点或需用户决定的接口。API首红解除后仍需全部行为与工具实际验收，当前不关闭FR195/M1。FR189 deferred/NFR91保持。

完成钩子：resolver --key workflow.on_complete 返回空字符串，按技能跳过hook并正常结束。

最终GREEN回链：全部实现与验证已完成，见126-4-final-verification.md及automation-126-4.md；上文首红阶段记录和probe保持历史事实，非当前待办。
