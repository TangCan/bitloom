# Story126.3 最终验证索引

2026-09-20，用户七步顺序完成，一故事一提交。范围为RvRegSlice注册切片及实际形式验证所必需的工具修复，不关闭参数FIFO、FR195整体或M1。

| 步骤 | 证据与结果 |
|---|---|
| create-story | 126-3-两槽-ready-valid-注册切片.md，正式AC与前故事合同 |
| ATDD | atdd-checklist-126-3-两槽-ready-valid-注册切片.md；API/formal首红均E0432缺API |
| build | 126-3-build-evidence.md；产品、真实RTL、形式工具前置修复 |
| review | spec的内审逐项裁定与story的独立四层审阅；27项定向通过，后续工具修补/并发隔离/文档编译通过（126-3-review2-tools.md） |
| automate | automation-126-3.md；20项通过，最终WIDTH1/8各四条实际cover及归纳证明PASS |
| regression | 126-3-clean-workspace-regression.log；clean→fmt→just test整条命令退出0，458个结果块合计1732通过、0失败、8忽略 |
| commit | 本次Story126.3单独提交；无发布或push |

环境：Rust1.97.1；Icarus12.0；Yosys0.33；SBY yosys-0.47；Z3 4.8.12。PATH包含实际Icarus和SBY安装位置，CARGO_PROFILE_TEST_OPT_LEVEL=1，断言与溢出检查保持启用。

```sh
PATH=/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin:$PATH CARGO_PROFILE_TEST_OPT_LEVEL=1 bash -c 'cargo clean && cargo fmt --all && just test'
```

8个ignored包含两个新组件专用formal测试与6个既有doc-test。新组件证明已由automation阶段`--include-ignored`真实执行；普通workspace结果不替代证明。最终原始设计/observer/配置/工具版本/归纳与cover日志/真实见证归档于126-3-automation-evidence，clean不影响归档。四cover每宽分别命中同时出入、满后排空、满槽reset恢复、满槽flush恢复。

全部本地结果不冒充远端CI执行。fresh pinned clone、tag peel与上游安装已实测；未在宿主执行CI sudo/apt配置。当前无未解决产品项或review延期项。

提交检查：产品与文档git diff --cached --check通过。原始工具日志/生成设计保留真实EOF空行以不改变证据hash；全差异以core.whitespace=-blank-at-eof检查通过，未放宽产品代码的空白检查。Phase24门禁通过，最终统计437 done、16 backlog、2 deferred。
