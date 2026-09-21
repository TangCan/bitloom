# Story128.1 独立人工验收清单（ATDD历史 + build自审）

本文件不是E2E、产品测试或风险正文。ATDD时全部项初始 **unreviewed**；2026-09-21 build实施者Codex完成M01–M14自审，独立review及最终主代理复核已完成（见末节）；未来产品行为预期不算已观测结论。不能以文件名、章节标题、字段存在或历史绿色代替内容有效性。验收者须逐项记录实际正文段落、源码/命令/日志、结论及剩余unknown；失败与未执行分开。父流程负责后续build与最终复核。

输入：`_agile-output/implementation-artifacts/128-1-外设-nfr14.md`（六AC）、`docs/ip/phase24-contract.md`（正式合同，历史状态按后续更新解释）、`_agile-output/implementation-artifacts/nfr14-risk-record-template.md`、`_agile-output/test-artifacts/atdd-checklist-128-1-外设-nfr14.md`（步骤1–3）。预期审阅目标为 `_agile-output/implementation-artifacts/epic-128-nfr14.md`。

ATDD阶段历史观测（本轮build观测另列下表）：2026-09-21T06:53:19.546131+00:00执行 `test -f _agile-output/implementation-artifacts/epic-128-nfr14.md`，退出1，stdout/stderr均空。原始记录 `128-1-atdd-presence.json` / `.log`。RED仅指风险正文缺失，不表示RTL失败；此观测不充当任何人工项PASS。ATDD阶段未运行工具版本、BFM、RTL、formal、综合或产品探针；build已运行的项目见128-1-build-evidence.md，formal/综合/新外设仍未运行。

后端为Rust/Python，无UI、HTTP或provider边界，浏览器/E2E不适用；utils/pact配置虽为true，JS runner条件不满足。沿用父流程 `pact_mcp_reachable=false`（能力不可用），本worker未重探测；provider states=N/A。没有生成假Playwright测试、API测试、fixture或文档字符串测试。

## 人工判定记录

每行证据栏均需替换为**实际位置及摘要**；unknown须说明如何消除。默认复核人和日期未填，不能自动判PASS。

| ID / AC / P0 | 验收动作与通过条件 | 状态 | 实际证据 | Unknown / 失败动作 |
|---|---|---|---|---|
| M01 / AC1 | 逐读M0/M1/M2关闭文档、当前sprint及root前置审计。确认125.3、126.1–4、127.1–4 done，当前全部剩余故事七步授权覆盖128.1；历史M0-only/126.2-only不作当前限制。 | verified-risk | epic-125/126/127-closeout.md与sprint Phase24段：125.3及126/127全部done；当前整体七步授权有效 | 风险内容已由主代理及独立review核对 |
| M02 / AC1,6 | 风险故事只交付风险闸门；128.2–5保持backlog、Epic128 in-progress，FR197/M3未交付。已有done、FR189/Epic122 deferred、NFR91保留；无push/publish、无新外设或新稳定API。 | verified-risk | epic-128-nfr14.md (a)/(c)：范围与状态分开；build未编辑sprint/产品/API，128.2–5 backlog | 风险内容已由主代理及独立review核对 |
| M03 / AC2 | (a)写出上游约束、支持参数、工具钉和依赖；(b)128.1 0.5–1、128.2/3/4各2–3、128.5 4–7，合计10.5–17有效人日，不是墙钟或交付日期。若25%集成预留另列约13.1–21.3；不得套旧10–16基线。 | verified-risk | epic-128-nfr14.md (a)/(b)：精确参数、依赖、五故事10.5–17人日及另列25%预留 | 风险内容已由主代理及独立review核对 |
| M04 / AC2 | (c)禁止缩GPIO/IRQ/baud宽度、删错误场景、用工具发现当产品证明、skip当PASS、空壳或第二HIR；(d)Richard维护owner、Codex实施证据、主代理七步/提交责任清楚。维护叠加含旧API与CSR/FIFO/串行/CDC/多后端回归成本。 | verified-risk | epic-128-nfr14.md (c)/(d)及维护叠加：逐项禁止降级，Richard/Codex/主代理责任清楚 | 风险内容已由主代理及独立review核对 |
| M05 / AC2,6 | 每项风险有失败动作与停止条件：缺字段/owner、合同冲突、单owner或同步采样表达失败、缺工具/版本不兼容/探针失败、基座不符、新架构范围。保存原始失败、修复授权范围，未解不下游ready；不得把已批准合同重新当待定产品选择。 | verified-risk | epic-128-nfr14.md (d)风险表：每项责任/修复/停止条件；缺pip发现失败已由metadata修复 | 风险内容已由主代理及独立review核对 |
| M06 / AC3 | 审核同session只finish一次、唯一FrozenHir、设计仅bitloom-prelude、非捕获定义体及旧API兼容；native/generated层级仍unsupported，真实层级需实际RTL。Rust1.97.1/edition2024、firtool1.159.0、Chisel7.15.0不变。 | verified-risk | epic-128-nfr14.md (a)与组合方案；csr/mod.rs define_module、rtl.rs状态声明、既有peer hierarchy实际三模块RTL | 风险内容已由主代理及独立review核对 |
| M07 / AC3 | 逐项复核下列共同CSR与单owner检查表，正文提供可行策略与源码根据，不能只抄接口名。 | verified-risk | epic-128-nfr14.md 组合方案与共同CSR验收；csr/rtl.rs candidate→reject→commit及External无副本；csr-rtl.log实跑 | 风险内容已复核；未来产品不计PASS |
| M08 / AC2,3 | 逐项复核Timer合同及每个边界未来判据，责任128.2。 | verified-risk | epic-128-nfr14.md 128.2合同表及验证层次：写优先、回绕/COMPARE0、EVENT碰撞可判定计划 | 风险内容已复核；未来产品不计PASS |
| M09 / AC2,3 | 逐项复核IRQ合同及每个边界未来判据，责任128.3。 | verified-risk | epic-128-nfr14.md 128.3合同表及验证层次：严格五路、脉冲/粘滞、RAW/TEST/clear碰撞 | 风险内容已复核；未来产品不计PASS |
| M10 / AC2,3 | 逐项复核GPIO32合同及每个边界未来判据，责任128.4。 | verified-risk | epic-128-nfr14.md 128.4合同表及验证层次：32位bit31、沿前DIR、初始高与双同步、旧API回归 | 风险内容已复核；未来产品不计PASS |
| M11 / AC2,3 | 逐项复核UART配置、宽分频、串行/FIFO合同及未来判据，责任128.5。 | verified-risk | epic-128-nfr14.md 128.5两行及全宽baud段：DIV+1上界、配置拒绝、FIFO提交前满/空、串行独立解码 | 风险内容已复核；未来产品不计PASS |
| M12 / AC4 | 实际记录Rust/Cargo、Python3.12及固定闭包、iverilog/vvp、yosys/sby/z3：命令、解析路径、实际版本、UTC、退出码、stdout/stderr、来源及日志。历史/tmp候选路径须重验，不以PATH名证明可用。 | verified-risk | 128-1-build-commands.json与版本分项log：路径/UTC/退出/stdout/stderr齐备；五包metadata+BFM版本断言 | 风险内容已由主代理及独立review核对 |
| M13 / AC4 | 实跑固定BFM stub，再从实际源码选择既有CSR external-owner/event/reject或FIFO8×4真实RTL入口；保存精确调用、seed、工具及结果。明确只是旧基座接口可用性，不是FR197算法或FR198系统证明。 | verified-risk | 128-1-build-bfm.log（1/0/0，seed1281001）、csr-rtl.log（2311帧/1通过，seed12720a11ce55）；归档原始SV/XML/run.log | 风险内容已复核；未来产品不计PASS |
| M14 / AC1,2,6 | 人工独立核对下表跨故事边及M3五故事全done规则；脚本只证明M0/.1/epic-close/deferred，AC5过程集成产物另由worker A提供。 | verified-risk | epic-128-nfr14.md (a)逐边人工审计；epics.md:10043–10103；build-gate.log 35场景及sprint保护；脚本范围已限制 | 风险内容已由主代理及独立review核对 |
| M15 / AC6 | build后有效正文+实际探针+状态门禁齐备，无未解架构/工具阻塞。独立code-review与automate、clean/fmt/just test、单故事commit均有真实证据；ignored与未执行不记PASS。 | verified-precommit | 128-1-code-review.md、128-1-automation-summary.md、128-1-final-verification.md；完整回归1823/0/25 | 随本Story单独commit闭合；FR197/M3仍未交付 |

## M07：共同CSR与单owner精确检查表

- addr16字节地址/data32/WSTRB4、小端4字节对齐；UART/GPIO/Timer/IRQ四个0x100窗口基址0000/0100/0200/0300。窗口洞、未对齐、访问类型错误SLVERR，0400–ffff窗外DECERR，不截断高地址；无ID/version。
- 非reset的`req_valid && req_ready`唯一提交；读响应在该沿快照，下一周期valid，背压保持且无新提交，消费沿无refill；RX pop/TX push/W1C/软件写仅一次。reset优先，共同清bridge/leaf/wrapper及在途槽；aresetn断言和释放均须外部同步到ACLK，取反不是同步器。
- RW先按字节合并，保留位读0写忽略；RO写/WO读SLVERR，失败读rdata0；合法零有效mask写OKAY无副作用且不触发动态busy拒绝，访问错误仍优先；所有CSR reset0、CTRL关闭。
- 审阅`ip/csr/{mod,codec,rtl}.rs`和`docs/ip/csr.md`的RW External、动态RO、WO提交、W1C Leaf/event、动态reject。自主COUNT/CTRL等外设唯一拥有，CSR不可另存副本。`R_candidate/R_write_mask`不依赖commit，reject仅据候选值和提交前状态；`R_*_commit`是沿前组合脉冲，同一上升沿消费，绝非响应后下一拍。仅合法成功且有效mask非零产生write_commit。要求画出或写出candidate→reject→commit→状态的方向，证实无commit→candidate/reject组合反馈。

## M08：Timer32

0200 CTRL RW(enable bit0/periodic bit1)、0204 COUNT RW32、0208 COMPARE RW32、020c EVENT W1C bit0。关闭保持；有效写CTRL/COUNT/COMPARE时保持未被写的计数状态、抑制match，COUNT写优先。零WSTRB不抑制自然计数。普通enable拍先模2^32得next_count再比较，periodic命中COUNT归0，one-shot清enable；COMPARE0仅回绕命中。EVENT清除不抑制计数/match，同拍set优先。风险正文须给出回绕、比较值越过当前值、部分写、reset、写/事件碰撞的可判定未来验收。

## M09：IRQ5

0300 PENDING W1C、0304 ENABLE RW、0308 TEST WO、030c RAW RO，mask均0x1f。五路严格0 Timer match、1成功RX到达、2 TX取走队首腾空间、3 overflow或framing、4 GPIO新沿聚合；无额外外部IRQ针脚。pending接新事件脉冲，不接本地粘滞EVENT。TEST依WSTRB写1加入pending，RAW只取提交点硬件事件快照且不含TEST；硬件/软件set遇clear时set优先。`irq=|(pending & enable)`，mask不清pending、重复事件不计数。须安排局部EVENT未清但IRQ已清不自重触发、TEST/RAW分离与碰撞验收。

## M10：GPIO32

0100 DIR RW、0104 OUT RW、0108 IN RO、010c SET WO、0110 CLEAR WO、0114 RISE_EVENT W1C，均32位。SET/CLEAR按选中字节写1更新OUT；reset方向输入、OUT0且清同步链/历史。针脚双级同步，前一拍与当前同步值判断上升，按沿前DIR选择输入位，初始高同步后产生事件。聚合IRQ只为当拍新沿，不是RISE_EVENT电平。覆盖bit31、部分写、方向同拍改、同步延迟、reset与clear/set碰撞、旧GPIO端口/延迟/FL回归。双级同步不滤毛刺，不证明pad/驱动强度/电气/亚稳态。

## M11：UART32分频、串行与FIFO

- 0000 CTRL RW bit0 enable；0004 BAUD_DIV RW32；0008 STATUS RO bits0 rx_nonempty/1 tx_full/2 tx_busy/3 rx_busy；000c TX_DATA WO低8；0010 RX_DATA RO低8成功读pop；0014 EVENT W1C bits0 RX到达/1 TX腾位/2 overflow/3 framing。
- 8N1、LSB first、TX idle高，TX/RX各8×4寄存器FIFO。enable0不启动新收发但可预填TX/读取RX；关闭不清FIFO、reset清FIFO/状态。忙时有效写分频或更改enable返回SLVERR；空闲关闭允许保存0..2，启用须>=3且enable状态不得切无效分频。全部检查以WSTRB合并值为准；零有效mask不触发busy错误。
- 每bit=DIV+1个ACLK；0xffffffff对应2^32周期，不能32位helper溢出为0或复用旧8位端口。代表分频实跑串行；全宽极值采用标明的近终点状态、算术定向或形式证据，不声称已逐拍模拟2^32周期。
- RX双级同步后下降沿检测，半bit确认start，中心采8 data及stop；stop低丢帧并framing。合法帧未满push并RX事件，满丢新byte并overflow不覆盖；TX队首实际进入发送器才产生腾位，不能用busy结束代替。
- 提交前RX空，即使同拍到达也SLVERR且无pop；TX满即使同拍取走仍拒有效写。TX未选byte0始终无push且满时OKAY。普通非空非满同拍push/pop。覆盖背压不重复副作用、部分写、有效/无效分频忙配置、相位、错误帧、reset、独立解码与真实串行loopback；DIV>=3仅同步预算起点，须128.5边界实测，逻辑仿真不证明板级可靠性，物理发送不承诺回滚。

## M12/M13：探针与证据分层

固定闭包来源`scripts/phase24-axi-bfm-requirements.txt`：cocotb2.0.1、cocotbext-axi0.1.28、cocotb-bus0.3.0、find-libpython0.5.1、scapy2.7.0；入口`scripts/phase24_axi_bfm_probe.py`。formal版本来源`scripts/ci-sby-pins.env`。历史 `/tmp/bitloom-phase24-bfm-py312/bin/python`、`/tmp/bitloom-maintenance-tools/bin`只是候选。高层BFM拆分/对齐，非法对齐/特殊WSTRB未来须原始通道，不能同时驱动同一针脚；reset返回None记取消。

探针候选`cargo test -p bitloom --test fr196_csr -- --nocapture`须先读源码确定实际RTL入口。formal若调用ignored专用入口须显式运行并独立记录。工具存在、BFM stub、旧基座RTL、未来FR197行为、formal证明/cover、综合、板级结果分别报告；ATDD时均未执行；build已实际执行BFM/CSR RTL，见128-1-build-evidence.md。未来验证采用独立手写关键地址/期望scoreboard，不复用DUT next-state；安全性不依赖ready公平性，活性另列公平性及上界，综合不等于PPA。

## M14：人工跨故事依赖矩阵

| 目标 | 必须独立核验的前置 | 初始状态 / 证据 |
|---|---|---|
|128.1|125.3/M0；当前M1/M2可用基座|已审计 / 见epic-128-nfr14.md (a)逐边状态；未完成前置不准ready|
|128.2 Timer|128.1、127.2 done|已审计 / 见epic-128-nfr14.md (a)逐边状态；未完成前置不准ready|
|128.3 IRQ|128.1、127.2 done|已审计 / 见epic-128-nfr14.md (a)逐边状态；未完成前置不准ready|
|128.4 GPIO|128.3、126.2 done，且有效128.1门禁|已审计 / 见epic-128-nfr14.md (a)逐边状态；未完成前置不准ready|
|128.5 UART|128.3、126.4 done，且有效128.1门禁|已审计 / 见epic-128-nfr14.md (a)逐边状态；未完成前置不准ready|
|M3/Epic128关闭|128.1、128.2、128.3、128.4、128.5全部done；128.5单独完成不够|已审计 / 见epic-128-nfr14.md (a)逐边状态；未完成前置不准ready|
|129.2系统|129.1、127.4、128.2、128.4、128.5 done|已审计 / 见epic-128-nfr14.md (a)逐边状态；未完成前置不准ready|

七步create-story→ATDD→build→独立code-review→automate→clean/fmt/workspace regression→commit的最终闭合尚待父流程执行与审阅。此清单无状态修改授权结果、无产品交付结论；FR198系统/Epic130外部核均不属本风险故事交付。

## 主代理最终复核

2026-09-21逐项复核M01–M14为有效风险/接口计划及真实基座证据（verified-risk不是未来外设行为PASS）；M15的review/automate/clean/fmt/workspace证据齐备，随本故事单独commit闭合。上文阶段性“待主流程”保留为build时快照，此节为最终判定；独立code-review无未解问题，未来专用工具身份/产品行为仍按风险矩阵验证。
