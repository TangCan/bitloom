# Story128.1 create-story 校验（2026-09-21）

结论：create-story 已完成，Story128.1 `ready-for-dev`、Epic128 `in-progress`；并非NFR14风险正文或FR197实现完成。创建基线 `b926629e93b0ad5e09d795b59ec2f1bdb47010b6`。故事入口：`../implementation-artifacts/128-1-外设-nfr14.md`。

## 激活与输入

已读取 `.agents/skills/bmad-create-story/SKILL.md`（用户七步明确调用弃用技能）、config、discover-inputs、template、checklist；resolver返回prepend/append/persistent_facts/on_complete均空。语言中文，用户Richard，精确目标128.1，无需自动挑选或新授权。

SELECTIVE_LOAD发现并审阅正式epics的Phase24共用条件、完整Epic128五故事及相关126/127/129依赖；PRD/addendum Phase24条款和当前M2状态；architecture脊柱与AD30/31；正式`docs/ip/phase24-contract.md`全部外设细则；研究implementation-plan §3–5（旧草案数值不覆盖正式合同）；NFR14模板、127.1故事、M2关闭、CSR接口及现有gate。无适用UX文件。最近五次提交为127.4、127.3、127.2、127.1、126.4。

sprint完整文件读入（3540行）并对全部development_status映射解析；仅修改两个精确状态和last_updated。主代理独立源码与前置审计见`128-1-root-prerequisite-audit.md`：旧UART8位分频、旧GPIO8位与无同步/事件、CSR候选值独立于reject/commit、M2仅测试peer边界均已融入故事。

## Checklist结果与修正

| 风险 | 已纳入的可执行约束 |
|---|---|
|误把create当build|所有任务未勾选；epic-128-nfr14正文与实际工具探针留给build；128.2–5保持backlog|
|复用旧IP却缩小合同|GPIO32、IRQ5、UART BAUD_DIV32及DIV+1最大2^32周期显式；保留旧8位入口行为|
|双状态owner/反馈|自主状态External owner，candidate/write_mask独立于commit；reject按候选和提交前状态，沿前commit同沿消费|
|事件反复中断|原始脉冲与粘滞EVENT分开、RAW不含TEST、W1C硬件/软件set优先|
|Timer优先错误|有效写抑制该拍自然计数/match，零WSTRB不抑制；COMPARE0回绕；EVENT清除不阻止match|
|UART副作用错误|提交前full/empty规则、无byte0不push、零mask绕过动态拒绝、busy合并值校验、错误帧丢弃|
|CDC宣称超界|RX/GPIO双级同步及采样预算、GPIO初始高/沿前DIR、reset同步前提；无板级/亚稳态/PPA证明冒称|
|错误关闭M3|128.5依赖与全128.1–5done关闭要求分开；FR198/201及外部核留后续|
|工具或支持谎报|固定Python闭包、真实现有RTL探针和版本记录；native/generated层级unsupported；formal/综合/ignored分别记录|
|门禁范围夸大|脚本仅M0/.1/epic关闭/deferred，跨故事边独立核验；临时变异不得污染真实sprint|
|不当范围升级|无新产品API、无工具钉修改/发布；FR142逐符号留功能故事，历史done/deferred保持|

无需用户新决策；已批准接口选择直接落实为故事约束。估算合计10.5–17有效人日，25%预留单列，不冒充交付期限。

## 当前实际验证

- `python3 scripts/check_phase24_gate.py`：退出0，6 epics / 22 stories、M0/NFR14 gates、FR189 deferred通过。
- 对HEAD与工作树完整状态映射差分：仅`epic-128: backlog→in-progress`、`128-1-外设-nfr14: backlog→ready-for-dev`；last_updated独立更新时间，其余状态保持。
- 所有故事内本地Markdown目标存在；任务均未勾选。
- 本阶段未运行产品RTL、形式、综合或工作区回归；故事仅要求后续build真实探针，不能用本次文档检查代替。

## 官方网页核验

2026-09-21读取[cocotb2.0.1 release notes](https://docs.cocotb.org/en/v2.0.1/release_notes.html)、[cocotbext-axi0.1.28维护者说明](https://pypi.org/project/cocotbext-axi/0.1.28/)、[OpenTitan reggen](https://opentitan.org/book/util/reggen/index.html)。保留仓库固定版本；2.0 API变动/高层BFM对齐边界作为工具风险背景；本项目W1C优先级仍来自正式合同。未声称最新稳定版本或升级必要性，无新UART测试依赖。

创建产物仅故事、本文、sprint目标状态及时间戳；无代码实现、commit、push或publish。
