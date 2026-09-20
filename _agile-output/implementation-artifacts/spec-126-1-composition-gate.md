---
title: 'Story 126.1 组合基础风险门禁'
type: chore
created: 2026-09-20
status: done
route: dispatch
baseline_commit: f00a6ca2074848e0cd8890f69712dd5ec1e46154
review_loop_iteration: 0
context:
  - AGENTS.md
  - docs/ip/phase24-contract.md
  - _agile-output/implementation-artifacts/epic-126-context.md
  - _agile-output/implementation-artifacts/epic-125-closeout.md
---
<frozen-after-approval>
## Intent
M0 已关闭，用户再次要求继续；落实 Epic126 风险门禁，随后执行126.2模块组合基础。当前不执行126.3/126.4，不宣称M1完成。
## Boundaries & Constraints
一故事一提交，主代理负责提交。保持工具钉、历史关闭、FR189 deferred/NFR91。不得改产品代码或重跑M0长矩阵。
</frozen-after-approval>
## Code Map
- sprint-status.yaml 同目录：Epic126和126.1改in-progress，其余故事backlog；更新授权字段为M0-complete-and-126.1-126.2，保留M0批准历史。
- AGENTS.md Phase24行：反映新增继续指令与当前范围，不能仍称126–130全部未授权。
- docs/ip/phase24-contract.md、架构AD30/31是接口依据。
- 主代理补充允许当前状态同步：docs/ip/phase24-contract.md、ARCHITECTURE-SPINE.md Phase24段、epics.md Phase24末尾及授权字段；保留M0原批准历史。
## Tasks & Acceptance
- [x] 新建同目录epic-126-nfr14.md中文风险记录：执行者Codex/owner Richard，126.1估0.5–1日、126.2估3–4日、126.3/4引用原计划估算；区分当前授权与后续规划。
- [x] 记录单session模块身份与复用、活动模块不可覆盖、明确top可先定义、全图环/重复/缺模块/端口方向宽度、多驱动、公共API清单与SemVer风险。相同参数可复用，不同参数elaborate专门化，Instance.params不会被后端实现，不得作为参数化承诺。
- [x] 记录native层级unsupported、实际RTL验证、无新IR、设计仅依赖prelude、同步复位；将126.3/4的流/FIFO/formal工具风险登记但不实施。
- [x] 工具探针记录实际rustc/iverilog/vvp版本与路径；Icarus路径/tmp/bitloom-maintenance-tools/bin。缺工具严格失败，停止条件：不能通过真实RTL/旧API兼容/无法解决身份冲突；不得以emit成功代替执行。126.1只要求工具可用，行为证据待126.2。
- [x] 更新上述当前授权/状态；门禁通过并以临时文件状态变异证明：126.1未done时126.2不能in-progress，126.1done后126.2允许。
- [x] 同步合同、架构和epics当前授权注记，保留M0历史，不扩大实现范围。
## Implementation Notes
修改本spec、epic-126-nfr14.md、sprint-status.yaml、AGENTS.md；主代理追加允许上述三个规划/合同文件的当前状态同步。不修改epic-126-context.md（另一个代理正在翻译）。不要git commit。把验证命令及结果写本spec Verification；任务完成打勾，状态保持in-progress交主代理review。具体表述可随发现调整，但不得扩大功能范围。
## Spec Change Log
## Review Triage Log
- blind-1 / low：复现片段依赖活动状态，结项后重放会失效；仅spec示例问题，已将两个状态独立构造并检查键数。
- blind-2 / low：提交后done的叙述顺序不当；本故事实际在提交前统一完成状态，已更正记录措辞。
- blind-3 / low / patch：临时工具路径不能跨机器复用；补充现有CI安装入口及重新核验版本要求。
- blind-4 / medium / patch：风险登记的首批迁移范围不够具体；补充Gpio兼容及8/16位参数夹具，不扩产品范围。
- blind-5 / medium / patch：身份描述不够精确；补充名字、规范参数及定义内容核验的已选策略。
- blind-6 / low / patch：后续formal风险未列工具与属性类别；引用现有sby/yosys/z3安装链并区分安全性/公平性活性，仅记录未来验证。
- edge-case：无发现。
- verification-gap：无验证缺口。
## Verification
python3 scripts/check_phase24_gate.py；门禁负向检查使用临时复制文件，先读取脚本CLI参数；git diff --check。不跑workspace测试。


2026-09-20 实测：

- `command -v rustc`、`rustc --version`、`rustup which rustc`：Rust 1.97.1，入口和实际工具链路径已记入 NFR14。
- `/tmp/bitloom-maintenance-tools/bin/iverilog -V` 与 `/tmp/bitloom-maintenance-tools/bin/vvp -V`：均退出0，Icarus 12.0 stable；只证明可调用，不是126.2行为证据。
- `python3 scripts/check_phase24_gate.py`：退出0，6 epics / 22 stories、M0/NFR14 和 FR189 deferred 检查通过。
- 读取脚本CLI后，用 Python `tempfile.TemporaryDirectory` 复制 sprint-status；先仅把126.2改为in-progress：退出1，准确报告 `126.2 requires 126.1 NFR14 done`；再把临时126.1改done：退出0。原状态文件仍126.1 in-progress、126.2 backlog。
- `git diff --check`：退出0。
- 未运行 workspace 或 M0 长矩阵；未修改产品代码、未提交。主代理评审通过后，在本故事提交内将风险故事标done。

复现状态变异：

```python
from pathlib import Path
import re, subprocess, tempfile
source = Path('_agile-output/implementation-artifacts/sprint-status.yaml').read_text()
with tempfile.TemporaryDirectory(prefix='bitloom-126-gate-') as tmp:
    path = Path(tmp) / 'sprint-status.yaml'
    pending = source
    for key, value in [('126-1-组合基础-nfr14', 'in-progress'),
                       ('126-2-共享模块定义与实例校验', 'in-progress')]:
        pending, count = re.subn(r'^  ' + re.escape(key) + r': [\w-]+$',
                                 '  ' + key + ': ' + value, pending, flags=re.M)
        assert count == 1, key
    for content, expected in [
        (pending, 1),
        (pending.replace('  126-1-组合基础-nfr14: in-progress',
                         '  126-1-组合基础-nfr14: done'), 0),
    ]:
        path.write_text(content)
        result = subprocess.run(['python3', 'scripts/check_phase24_gate.py', str(path)],
                                capture_output=True, text=True)
        assert result.returncode == expected, result.stdout + result.stderr
        if expected:
            assert '126.2 requires 126.1 NFR14 done' in result.stderr
```

## 完成记录
三层审阅完成，风险说明补充已核验。门禁正反检查与空白检查通过；126.1完成，产品行为待126.2。
