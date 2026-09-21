# Story127.3 create-story 验证记录

日期2026-09-21；基线9393c261cdfb2ca3699edb959efd1986bbe5275e。只完成七步中的第1步；没有执行产品测试或提交。

## 激活与发现

完整读取 `.agents/skills/bmad-create-story/SKILL.md`、discover-inputs.md、template.md、checklist.md。按明确指定使用deprecated create-story，未擅自替换用户流程；SKILL包含工作流，目录没有独立workflow.md，不声称读取不存在文件。`uv run _bmad/scripts/resolve_customization.py --skill .agents/skills/bmad-create-story --project-root . --key workflow` exit0：prepend/append/persistent_facts均空，on_complete空。读取_bmad/bmm/config.yaml，Richard、rhdl、Chinese、intermediate及规划/实施路径已解析；向主代理传达中文激活说明。

目标明确127.3，不自动选择deferred122。完整加载sprint文本171349 bytes，development_status块709条逐键解析；127.1/127.2/126.3 done、Epic127 in-progress、127.3 backlog、127.4 backlog。只改127.3为ready-for-dev及last_updated，保留其余状态、注释、结构。最终真实 `python3 scripts/check_phase24_gate.py` exit0：`PASS: Phase 24 6 epics / 22 stories; M0 and NFR14 gates; FR189/Epic122 deferred`。脚本不能代替跨故事依赖人工核验。

SELECTIVE_LOAD发现：epics.md Phase24共同条款及Epic127全部4故事；PRD addendum Phase24；architecture spine基础规则与AD30/31；完整正式接口合同、127上下文/NFR14、127.1关闭、127.2完整故事/CSR文档/独立review；实际CSR RTL、rv slice、旧AXI（主代理完整核验）。UPDATE面mod.rs、IP索引、FR142表面、CI、示例脚本已读。无适用UX输入，未强加UI。最近5个完整提交号和主题实际读取，基线已解析。

## 独立分析和checklist

研究子代理 `contract_audit` 独立只读分析合同与真实CSR，随后在新任务中完整读取checklist并复核新story。主代理并行只读检查旧bank、结构检查复用、CI/API追加面。未委派产品实现，无其它写者。

已采纳全部实质改进：CSR响应消费与AXI响应消费分开；offer在ready低时锁定；AXI五个ready/valid无任何输入组合路径（含rst）；reset计数优先和内部CSR reset屏蔽分开；保守气泡；零WSTRB不丢事务；DECERR合规peer与127.4真实译码分开；旧bank顺序不移植；首次使用reset沿和formal初始reset假设。最后一轮独立checklist无协议/范围阻断，提出首次reset前提与sprint同步两项，均实际修正。

8项AC逐项覆盖：入口/组合；独立捕获；仲裁/offer/提交；预留/响应；reset/结构；真实leaf/地址；独立验证/形式/综合；兼容/七步。任务均未勾选，story为ready-for-dev，未写done。禁止第二IR、capturing closure、native层级补实现、工具升钉与提前M2关闭；记录5–7有效人日，未转成日期/墙钟承诺。模板Story/AC/Tasks/Dev Notes/References/Dev Agent Record/File List完整。

## 官方上游核验

2026-09-21通过网页工具读取官方cocotbext-axi仓库：高层master会拆分/对齐访问，因此raw channel必须独立覆盖未对齐地址。Arm latest入口重定向后无正文，不借其背书项目规则。cocotb stable release-notes显示开发版标题，不据此断言稳定版本。

实际HTTPS读取PyPI维护者包索引：cocotb2.1.0、cocotbext-axi0.1.28；只提取info.version，exit0。故事仍使用固定cocotb2.0.1/axi0.1.28（已有风险门探针），不升级或修改requirements。权威链接在story中。未新增依赖，不以版本发现冒充硬件验证。

## 完成动作

最终 `uv run _bmad/scripts/resolve_customization.py --skill .agents/skills/bmad-create-story --project-root . --key workflow.on_complete` exit0，值为空，无终端附加动作。用户七步连续授权优先于checklist常规选择菜单，必要改进直接应用，无再次审批。下一步ATDD由主代理启动。没有commit/push/publish。
