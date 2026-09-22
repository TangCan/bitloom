# Story129.1 [P0] 真实RTL复位负控制

Worker B仅新增风险探针自动化，没有产品修改。复用code-review归档的direct RTL、端口shim、边界和TB；脚本内固定原归档SHA，按四个精确成员解包，每次输出必须是新目录。生成RTL/TB保持字节内容，变异只替换边界`core_reset`表达式。

| 场景 | 实际编译 | 实际vvp | 必须出现的独立TB判断 |
|---|---|---|---|
| 正常 `~aresetn` | 0 | 0 | RESET PROBE PASS，双子状态、同步、优先级、恢复 |
| 断线 `1'b0` | 0 | 1 | aresetn failed common synchronous clear / write priority |
| 错极性 `aresetn` | 0 | 1 | nonzero setup failed（正常工作沿仍被清零，不能建立非零初态） |

normal与`python3 -O`各3例实际通过；每模式8条子命令（2工具版本、3编译、3仿真），无编译错误充当负控制成功，无assert被优化消除。首轮默认PATH缺iverilog而退出1，失败results也归档；提供本故事已有工具目录后实际完成。该结果验证两子状态TB检错能力，不是FR198完整系统、其他后端变异测试、形式证明或板级复位同步器验收。

从仓库可重放（PATH先提供Icarus/vvp，输出目录应不存在）：

```sh
python3 _agile-output/test-artifacts/129-1-automate-reset-controls.py --output-dir /tmp/bitloom-reset-controls-new
python3 -O _agile-output/test-artifacts/129-1-automate-reset-controls.py --output-dir /tmp/bitloom-reset-controls-opt-new
```

本机实际执行另将`/tmp/bitloom-maintenance-tools/bin`置于PATH开头；这只是本机工具位置，不是移植要求。最终独立输出目录为`/tmp/bitloom 1291 automate reset normal 20260922d`与`/tmp/bitloom 1291 automate reset optimized 20260922d`。`129-1-automate-reset-evidence.tar.gz`保留原始命令、时间、工具路径/二进制SHA、源SHA、日志、变异源码和VCD；`129-1-automate-reset-manifest.json`固定归档及每成员SHA。原始输入为`129-1-code-review-reset-evidence.tar.gz`，无需旧/tmp探针目录。确定性seed=N/A。
