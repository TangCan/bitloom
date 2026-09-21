---
title: '缓冲UART CSR与M3关闭'
type: feature
created: '2026-09-21'
status: done
route: dispatch
baseline_commit: '2f52d9969f60d08cc633e30e0c263f05854b39fe'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/128-5-缓冲-uart-与-m3-关闭.md'
  - '{project-root}/_agile-output/test-artifacts/atdd-checklist-128-5-缓冲-uart-与-m3-关闭.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-128-nfr14.md'
---

<frozen-after-approval reason="用户已授权全部未完故事连续七步">

## Intent

交付独立可组合UartCsr，双FIFO支持软件缓冲收发和真实事件IRQ；验证后关闭128.5及M3。详尽合同和固定API以故事及ATDD为准，不降级。

## Boundaries & Constraints

prelude-only、同session一次finish；复用CsrBlock和两ParamSyncFifo<8,4>。旧UART8/VIP/FL不变。32位DIV、8N1、15端口及六CSR固定，不扩协议/原生层级，不改工具钉/版本、不push/publish。M3不吞129/130，FR189仍deferred；七步最后才提交。

## I/O & Edge-Case Matrix

|场景|预期|
|---|---|
|CSR|CTRL/DIV/STATUS/TX/RX/EVENT local0/4/8/c/10/14；WSTRB、权限/洞/错齐按独立黄金；一次commit、快照/背压/无refill|
|配置|DIV>=3方可enable；忙时有效DIV写或改变enable拒绝；零mask不拒绝。成功空闲配置该沿生效并锁存新帧分频|
|TX|idle高，启动沿start，P=DIV+1；全宽不溢出。队列满同拍pop仍拒写|
|RX|两级/历史复位0；沿前s2/history下降为e，H=floor(P/2)，start e+H、data e+H+(k+1)P、stop e+H+9P|
|错误/队列|假start取消无事件；坏stop丢弃/framing；满即使同沿pop仍丢新/overflow；空同沿arrival读错，新字节可自然入队|
|事件|四raw：RX入队/TX取队首/overflow/framing；本地W1C set优先，实际IRQ1/2/3非sticky|
|reset|清CSR/双FIFO/FSM/同步/响应；优先提交/事件；物理已发bit不回滚，账本区分活动帧|
|验收|三后端实际串行、回环/独立解码、双实例/IRQ；有限formal、综合、行为变异；全宽极值证据诚实标注|

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip/uart.rs`：旧UART独立入口，不改。
- `crates/bitloom-prelude/src/ip/csr/{mod,rtl}.rs`：Leaf配置/W1C、External RO、candidate/reject/commit，无需新HIR。
- `crates/bitloom-prelude/src/ip/param_sync_fifo.rs`：满空前态、共享定义、reset优先，flush绑0。
- `crates/bitloom/tests/fr197_uart{,_api,_formal}.rs`：14项RED及明确未完夹具，独立deadline/队列黄金。
- `128-4-build-{runner,archive,harness-tests}.py`：借证据规则，逐项映射新测试/目录/性质，不能只替换字符串。

## Tasks & Acceptance

- [x] `crates/bitloom-prelude/src/ip/uart_csr.rs`与`ip/mod.rs`：实现固定API/唯一状态及安全全宽计时。
- [x] `crates/bitloom/tests/fr197_uart*.rs`：激活GREEN，补真实回环/组合/极值/取消账本/三类DUT负控制；保留黄金独立性。
- [x] `docs/ip/uart-csr*`与`docs/public-api-1-0-surface.md`：原文示例、同源软件产物、四符号/minor/边界。
- [x] `.github/workflows/ci.yml`与`_agile-output/test-artifacts/128-5-build-*`：真实门禁、不可空过runner/归档及普通/-O检查。
- [x] `_agile-output/implementation-artifacts/epic-128-closeout.md`与当前状态文件：最终七步完成才映射128.1–5关闭FR197/M3。

Given故事七AC，When实际三后端执行，Then逐拍独立参考一致且必达场景齐；Given代表串行/FIFO/事件配置故障，When同黄金验证，Then控制通过而故障在指定断言失败并有波形。Given所有前置故事完成，When本故事最终回归通过，Then诚实关闭M3并保留后续范围。

## Implementation Notes

意图无缺口，无不可逆操作；当前改动均本故事create/ATDD和前故事SHA登记，连续授权覆盖脏树与常规checkpoint。一个UART交付目标，采用dispatch。主代理已读完整架构/缓存Epic128上下文和128.4连续性；独立只读调查无阻塞。

Builder assign_ult目标必须Bool而非UInt1；CSR数据32/FIFO8、event_bits32/raw_events4必须显式slice/concat。TX reset用idle组合高，普通reg复位0。旧产品保持；若修改额外既有文件先全文读并记录。

本机执行环境：PATH=/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin:/tmp/bitloom-jvm-tools/bin:$PATH；RHDL_FIRTOOL_PATH=/home/richard/.cache/rhdl/firtool/1.159.0/bin；BITLOOM_SBY_SOURCE=/tmp/bitloom-1263-sby-src；CARGO_PROFILE_TEST_OPT_LEVEL=1、BITLOOM_REQUIRE_RTL=1、PYTHONDONTWRITEBYTECODE=1。产品/runner不能写死主机路径。

实现代理负责实现、补齐测试/文档/CI/runner、真实专用验证与原始归档；不clean、不commit、不改sprint/总账为done。M3 closeout先准备可审草稿，主代理最终回归后关闭当前状态；此最后任务由主代理完成。主代理独立完成build内审、外部code-review、automate及最终全workspace七步。不要自行调用bmad-build递归或停在技能额外确认。

沿用128.4最终证据质量：exact测试名/数、全部固定seed/账本、工具身份/源起终绑定，缺cover/VCD/故障指定断言或返回码必须拒绝；normal/-O harness实际跑。单次完整run与归档源一致，历史失败日志保留，不改生成Scala，不以emit/ignored/发现工具作为PASS。Build交接说明未完项；不能为赶时间静默删合同场景。

## Spec Change Log

## Review Triage Log

|ID / layer|Verdict / route|核对证据与处理|
|---|---|---|
|B1 idle timers|low / patch|uart_csr.rs每个空闲沿仍推进32位timer；在常见disabled/idle使用中产生无用翻转。最小修改为idle保持，仅launch/active更新，不改周期或API。|
|B2 serial cover|false / reject|formal只声明有限安全性质与三项短总线cover；真实TX/RX/错误到达由独立三后端逐拍向量和非零计数验证。未声称形式覆盖完整串行协议，缺额外cover不使这些实际证据无效。|
|B3 dynamic response formal|false / reject|动态响应错误已由direct/FIRRTL/Chisel独立黄金逐拍比较；formal配置ghost核配置及commit，未声称独立证明每种动态response。无未验收行为或失实证明声明。|
|B4 raw-event ghost|false / reject|formal明确由raw统计串行副作用，是有限occupancy/accounting性质；事件正确性有独立deadline模型、四事件覆盖和实际raw故障负控制。未将相关ghost冒充完整协议独立等价。|
|B5 symbolic byte proof|false / reject|payload/order由独立VecDeque参考、读响应和TX解码三后端检查；复用既有参数FIFO。合同未要求新增完整符号字节形式证明，现有证明声明不含它。|
|B6 endpoint backends|false / reject|合同明确允许全宽极值独立近终点/算术/形式证据并单列边界；文档已准确标注direct注入、三后端bounded launch，不声称全后端极值终点。代表全串行三后端已实跑。|
|B7 extra mutations|false / reject|要求的串行采样或计时、FIFO副作用、事件/配置关键类别已由TX countdown、FIFO push、raw输出三种真实DUT变异覆盖；不是要求每个子类别都突变。|
|B8 concurrent artifact capture|low / reject|共享artifact根对同workspace并发runner会导致重复类而拒绝归档；本流程串行且CI作业隔离，不会错误PASS。罕见并发使用的修复需新增锁/根参数复杂度，依技能低影响规则不加入。|
|B9 outer timeout|low / reject|内部RTL/JVM/formal/消费者有timeout，CI亦有作业终止；外层Cargo等极端挂死可人为终止且attempt已落盘。未复现正常用例挂死，新增全门时间预算/策略超过直接修正，依低影响规则不加入。|
|B10 nested launch records|low / reject|内部spawn失败确实来不及写内部JSON，但外层runner在launch前保存命令并记录失败，整体无法认证成功。缺工具为异常安装，补每个内层包装器的新错误分支不带来日常实质收益，依低影响规则不加入。|
|E1 missing gate logs|medium / patch|主代理实际调用独立synthetic complete_run，七种门禁log均缺失而validate_run通过。补必需原始日志存在性检查并缺失回归；header-execute成功stdout可以为空，不能错误要求nonempty。|
|G0 verification-gap|false / no finding|独立verification-gap reviewer报告No verification gaps found；无发现需补测。|


## Design Notes

候选→沿前busy拒绝→commit→有效配置→启动单向无环；FIFO flags用前态。32位倒计时装DIV形成P，H=(DIV>>1)+(DIV&1)避免溢出。原始事件沿前组合，两接收端同沿捕获。

## Verification

三个UART测试目标、两个后端入口实际运行；旧UART/VIP/FL、C11/原文例、SemVer，有限prove/cover及无observer综合。源码/原始工件归档独立校验后，实际cargo clean→cargo fmt --all→just test，再单故事commit。

Implementation handoff 2026-09-21: first four implementation tasks complete; final M3/status task remains main-agent work after reviews/automate/clean-regression/commit. Selected consolidated run `20260921T133203.691419Z-1329506`:39commands PASS,570 matching sources; raw3196-member archive SHA256 `14360ff18ea57e3c1719733d81459f58efc7985064e7590e69ee951bc2bc1cc0`, stored as three≤64MiB parts. See `../test-artifacts/128-5-build-evidence.md` and `128-5-build-contract-matrix-audit.md`.

最终M3关闭：482个结果块、1870 passed / 0 failed / 47 ignored；见`../test-artifacts/128-5-final-verification.md`。最后当前源绑定run为20260921T135428.448283Z-1415681，631成员归档；此前handoff run为修补前历史。
