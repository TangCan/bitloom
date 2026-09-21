# Story127.4 最终验收

2026-09-21。固定四窗CsrDecoder及真实七模块CSR连接完成；随本Story单独本地提交关闭Epic127 / FR196 / M2。Bitloom与samitbasu/rhdl无关；没有改包版本、工具钉、推送或发布。

| 步骤 | 真实证据 |
|---|---|
| create-story | 固定52端口/时序、8项AC、独立接口审计与checklist；127-4-create-story-validation.md |
| ATDD | 三目标缺CsrDecoder的E0432首红，exit101；只证明编译失败，9入口行为尚未执行；原始源码与日志归档 |
| build | 共享无捕获定义体、完整16位地址命中、单owner与持久DECERR；三路内审8修补；14普通+4专用+91兼容+7兼容专用及4原文例实际通过 |
| code-review | 独立四层，9修补/1驳回/0待决/0defer；根重验6普通+4专用+文档例，17cover、6port-only突变反例和256次双类有资格仲裁决策 |
| automate | 8新增P0通过；4seed共512完成/36取消，Interpreter/Compiled/真实RTL逐周期对照；4 WO control及8实际RTL输出突变 |
| regression | 实际cargo clean、cargo fmt --all、just test依次exit0；470结果块，1823 passed / 0 failed / 25 ignored；格式化无源码变化 |
| commit | 全部实现、测试、文档和证据归入一个包含Story 127.4的本地提交；完整SHA以Git记录为准，不推送 |

完整回归开始UTC `2026-09-21T06:27:28.046063+00:00`，结束UTC `2026-09-21T06:42:12.300994+00:00`，总耗时884.255s。clean 7.591s（删除34,386文件/19.3GiB）、fmt 1.111s、just test 875.484s。精确环境/命令/退出码/日志SHA见`127-4-final-regression.json`及各step JSON/log，结果汇总`127-4-final-test-summary.json`。

25 ignored为decoder4专用、既有CSR4/bridge3/M1流基础8专用以及6既有doc-test。本故事decoder4专用及CSR/bridge7专用已实际另跑通过，M1专用历史证据保持；工作区ignored不算PASS，不宣称完整远程CI或所有外部后端已执行。

## AC逐项映射

| AC | 实现与检查 |
|---|---|
|1|固定52端口、独立/共享helper同一body、双实例真实RTL隔离、定义复用/冲突/宽度方向诊断；prelude-only文档例、FR142逐符号/minor |
|2|两native引擎各65536地址，完整高位命中、低位不对齐保持；组合手写黄金区分窗外DECERR与窗内hole/权限SLVERR，无alias或非法副作用 |
|3|提交同沿选唯一leaf并锁owner、无第二提交、消费沿气泡；owner数据与非owner噪声隔离、miss背压保持；新长延迟三后端回放 |
|4|桥+decoder+4真实CsrBlock同session七模块RTL，20正式寄存器布局；16seed×1000完成，另4096并发事务；全部WSTRB、0/1/7/31实际AW/W接受间隔、两类容量与轮转 |
|5|共同同步reset，各槽/owner/miss/叶和B/R取消、复位边界不记传输，恢复与尾部重复检查；独立接受/提交/副作用/响应/取消账本 |
|6|depth8基例+归纳prove通过，辅助状态对应均assert；17cover分列；port-only BMC10六突变实际assert FAIL/2且control PASS；原始decoder/七模块综合、FF时钟与桥5输出寄存边界 |
|7|C11多头消费者、20独立全局地址常量与base，Markdown名称/offset/mask/权限/reset/owner黄金；旧bank/M0/M1/CSR/桥兼容及原文例实跑 |
|8|review/automate/clean/fmt/justtest及专用证据齐，单故事本地提交与Epic127关闭映射；未来范围和历史deferred保持 |

七模块peer只提供value/event/reject，不交付Epic128的UART/GPIO/Timer/IRQ算法。native/generated层级仍明确unsupported；独立generated Rust decoder未声明验证。形式安全不假设叶响应上界或上游最终ready；cover只证明可达，mutation BMC不冒充归纳证明。无板级、商业VIP或PPA/性能收益承诺。

## 证据完整性

ATDD、build及修补、独立审阅修补、automate均保留源码与工具归档/逐成员SHA；初始UNKNOWN/编译刺激错误保留，不能把历史失败改写为初次全绿。Automate生成审阅发现的非法peer响应在首次运行前按合同修正，无产品改动、测试重试或skip。

最终干净构建产物归档`127-4-final-artifacts.tar.gz`共238成员、1,444,492字节，SHA256 `c130bb76e7fdaff37707fc22e88d5915a1e9061dd395974d98dc7f2bb3069eb5`，逐成员回读成功；manifest保留源码/生成RTL/trace/日志。专用形式在clean前独立归档，最终workspace没有冒充重跑ignored证明。原始工具失败与预期突变失败均保留，日志不为格式检查而修剪。

关闭映射：`_agile-output/implementation-artifacts/epic-127-closeout.md`。Epic128–130/FR197–201及整个Phase24未交付；FR189/Epic122仍deferred，NFR91未清空。整体全部Story目标active，下一故事128.1按七步执行。
