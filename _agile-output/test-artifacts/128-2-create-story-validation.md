# Story128.2 create-story 验证

日期2026-09-21；创建基线`1c4cf20c4f8e8fdcb038d57a7b3f03f9f80b5c00`。结果：ready-for-dev，仅第一步完成。用户明确指定deprecated `bmad-create-story`，本轮执行其流程；用户七步连续授权覆盖checklist可选交互选择，所有明确的上下文补全直接应用。

## 实际流程与输入

- 全文读取`.agents/skills/bmad-create-story/SKILL.md`，执行`uv run _bmad/scripts/resolve_customization.py --skill .agents/skills/bmad-create-story --project-root . --key workflow`；prepend/append/persistent_facts为空，on_complete空。读取`_bmad/bmm/config.yaml`、discover-inputs、template和checklist；Richard/Chinese/rhdl及产物根已解析。
- 明确目标`128-2-timer`；读取完整sprint文件（3540行，710个development_status条目由脚本完整解析），128.1与127.2 done，128.2 backlog，Epic128 in-progress。保留全部已有done、历史注释及FR189/Epic122 deferred。
- discover-inputs SELECTIVE_LOAD：默认一层prd/architecture/ux glob无匹配，epics匹配1个单文件；依据项目路径解析实际深层PRD addendum与architecture脊柱。提取完整Epic128五故事、129依赖及Phase24合同；UX无适用输入。
- 全文读取前故事128-1-外设-nfr14.md、epic-128-nfr14.md（含RX补充/适用性矩阵/独立审查追加），正式phase24-contract.md；读取PRD Phase24、架构AD和研究实施计划§5（历史草案不能覆盖正式合同）。
- 实际读取UPDATE候选`ip/mod.rs`、`lib.rs`、`docs/public-api-1-0-surface.md`与SemVer政策，完整读取CSR mod.rs/rtl.rs和CSR文档；记录current/change/preserve，新增其他UPDATE文件要求实施前全文读。私有CSR基座默认无需修改。
- 最近五提交审计：128.1风险门、127.4译码/M2、127.3桥、127.2 CSR、127.1风险门；沿用独立协议文件、显式导出、专用RTL/formal及保留历史证据模式。主代理并行接口审计确认无需TimerConfig、单owner可用、local与全局地址分立。
- 官方web实际核验：[SBY reference](https://yosyshq.readthedocs.io/projects/sby/en/latest/reference.html)的bmc/prove/cover模式；[cocotb2.0.1 notes](https://docs.cocotb.org/en/v2.0.1/release_notes.html)的固定测试环境兼容说明。未扩展到其它IP研究、不新增依赖、不升级或宣称仓库钉为当前最新。

## Checklist检查与已纳入防错项

|检查面|结论|
|---|---|
|完整story基础|7条可判定AC、四组任务、开发约束、文件计划、参考与Dev Agent Record；2–3有效人日含owner|
|复用与架构|现有CsrBlock external CTRL/COUNT、leaf COMPARE/EVENT；无第二存储/HIR，单session一次freeze|
|精确计数|模32 next_count后比较、COMPARE0回绕、one-shot保留命中COUNT/只清enable、periodic归0、compare越过不立即match|
|写优先|只用成功有效commit；同值/0值也抑制；WSTRB0与仅保留字节非零WSTRB不抑制；EVENT写/读/错误不抑制|
|事件与时序|raw match≠sticky EVENT；reset门控、每匹配沿事件、W1C set胜clear、提交前快照和消费沿无refill|
|地址错误|本地00/04/08/0c≠系统0200窗口；高位不截断、不承诺leaf DECERR；独立黄金+C头同源|
|验证边界|实际direct/适用Chisel/FIRRTL；native/generated层级unsupported；不为测试扩产品路径；formal/cover/原始综合分列|
|工具身份|强制sby标签对象/剥离commit及当前安装绑定；版本字符串/安装skip/hygiene不足；无未跑证明PASS|
|范围与兼容|逐符号FR142/minor、旧API保持、不交付IRQ/GPIO/UART/M3、FR189 deferred保留|
|可读性与不确定项|精确公开签名/证明深度/测试命名由build定；不是产品合同缺项，无需用户重批|

实际`python3 scripts/check_phase24_gate.py`退出0：Phase24 6 epics/22 stories、M0/NFR14门禁及FR189 deferred通过。该脚本不覆盖全部跨故事边；128.1/127.2前置已另行核验。`git diff --check`退出0。sprint差异仅128.2 backlog→ready-for-dev与last_updated，其余不变。

本步骤未执行ATDD、产品实现、Timer RTL、formal或综合；没有提交、推送或发布。只新增故事与本验证文档，并改sprint指定两行。实际功能风险仍由ATDD/build和后续七步证明，创建上下文完整不等于功能已验证。
