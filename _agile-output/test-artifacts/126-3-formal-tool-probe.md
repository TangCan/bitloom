# Story126.3 真实 formal 工具与 FR119 前置探针

日期：2026-09-20。范围仅为独立 `/tmp` 工具安装、真实 prove/cover 探针、既有 FR119 fixture 兼容性调查；本探针未修改仓库文件、工具 pin 或全局环境，未提交，未使用假工具。

已读 `scripts/ci-sby-pins.env`、`scripts/ci-install-sby.sh`、`scripts/formal-sby-check.sh` 与 FR119 四个 fixture 文件。上游参考：[SBY 配置文档](https://yosyshq.readthedocs.io/projects/sby/en/latest/reference.html)，涵盖多任务、prove、cover、smtbmc 和 expect 的退出码语义。

## 已验证可复用的工具环境

```sh
export PATH=/tmp/bitloom-1263-sby-installed/bin:$PATH
```

| 项目 | 实测 |
|---|---|
| SBY | `SBY yosys-0.47`，真实上游 `make install` |
| SBY 源码 | `/tmp/bitloom-1263-sby-src` |
| 安装前缀 | `/tmp/bitloom-1263-sby-installed` |
| Yosys | `/usr/bin/yosys`；`Yosys 0.33 (git sha1 2584903a060)` |
| Z3 | `/usr/bin/z3`；`Z3 version 4.8.12 - 64 bit` |
| Python 模块 | 宿主现有 `click` 可用；无需额外 PYTHONPATH |

原有 `/tmp/plugctx-action-sby` 位于无关提交 `85dd50b92a3acf170954a06f20f49e2162ce5273`，未使用。

## 仓库 pin 的对象类型与安装器问题

仓库配置的 URL、tag 和 SHA 相互对应，但 SHA 是 annotated tag 对象，注释称其为 commit 不准确：

- URL：`https://github.com/YosysHQ/sby.git`
- tag：`yosys-0.47`
- tag 对象：`bfc1c47eb786496fe794481ff88e75728f0529a6`
- 解引用 commit：`daed0e1544fd96ee7dab843e5a891d92784c6230`

执行命令：

```sh
git clone --depth 1 --branch yosys-0.47 https://github.com/YosysHQ/sby.git /tmp/bitloom-1263-sby-src
git -C /tmp/bitloom-1263-sby-src fetch --depth 1 origin bfc1c47eb786496fe794481ff88e75728f0529a6
git -C /tmp/bitloom-1263-sby-src checkout --detach bfc1c47eb786496fe794481ff88e75728f0529a6
git -C /tmp/bitloom-1263-sby-src cat-file -t bfc1c47eb786496fe794481ff88e75728f0529a6
git -C /tmp/bitloom-1263-sby-src cat-file -p bfc1c47eb786496fe794481ff88e75728f0529a6
```

最后两条分别输出 `tag` 与指向上述 commit 的 tag 内容。源码 checkout 是仓库固定对象所对应的源码，未追踪浮动 HEAD。

问题一：既有安装脚本将 `git rev-parse HEAD` 的 commit SHA 与 tag 对象 SHA 比较，正确 checkout 仍会被拒绝。最小修复应保留现有 SHA 值，分别验证 tag 对象与其解引用 commit，避免误将不同对象类型比较。

问题二：脚本直接复制 `sby.py` 到 bin，把模块放到 share/sby，但没有将模块目录接入 Python 搜索路径。独立复现命令：

```sh
mkdir -p /tmp/bitloom-1263-copy-install/bin /tmp/bitloom-1263-copy-install/share/sby
install -m 0755 /tmp/bitloom-1263-sby-src/sbysrc/sby.py /tmp/bitloom-1263-copy-install/bin/sby
cp -a /tmp/bitloom-1263-sby-src/sbysrc/. /tmp/bitloom-1263-copy-install/share/sby/
env -u PYTHONPATH /tmp/bitloom-1263-copy-install/bin/sby --version
```

实测退出码 1：

```text
ModuleNotFoundError: No module named 'sby_cmdline'
```

日志：`/tmp/bitloom-1263-copy-install/result.log`。

上游 Makefile 正式安装将模块放在 `share/yosys/python3`，替换 launcher 的 `##yosys-sys-path##` 与版本占位符。实测正式安装成功：

```sh
make -C /tmp/bitloom-1263-sby-src install PREFIX=/tmp/bitloom-1263-sby-installed
/tmp/bitloom-1263-sby-installed/bin/sby --version
```

输出 `SBY yosys-0.47`。最小修复建议为调用上游安装规则，并显式检查版本命令成功。

早期另有源码 wrapper `/tmp/bitloom-1263-formal/bin/sby` 直接调用 Python 源码，其 prove/cover 也成功；现在应使用上述正式安装前缀。源码 wrapper 的 `--version` 为 `unknown SBY version`，正式安装已解决版本标识。

## 真实 prove 与 cover 探针

文件：`/tmp/bitloom-1263-formal/smoke.sv`、`smoke.sby`。2-bit 计数器初始为零，递增至二后归零；断言 `count <= 2`，覆盖 `count == 2`。没有限制可达行为的环境假设。

配置有 prove 与 cover 两个任务，深度 6、引擎 `smtbmc z3`、脚本 `read -formal smoke.sv` 与 `prep -top smoke`。

正式安装版本复现命令：

```sh
cd /tmp/bitloom-1263-formal
env -u PYTHONPATH PATH=/tmp/bitloom-1263-sby-installed/bin:$PATH sby --prefix installed_smoke -f smoke.sby
```

实测退出 0：prove 的 basecase 与 induction 均 PASS，完成 k-induction 证明；cover 在 step3 命中并产生真实见证，两任务均 `DONE (PASS, rc=0)`。

证据：

- 综合日志：`/tmp/bitloom-1263-formal/installed-smoke.log`
- prove 日志：`/tmp/bitloom-1263-formal/installed_smoke_prove/logfile.txt`
- cover 日志：`/tmp/bitloom-1263-formal/installed_smoke_cover/logfile.txt`
- cover 见证：`/tmp/bitloom-1263-formal/installed_smoke_cover/engine_0/trace0.vcd`、`trace0.yw`
- 两个 status 文件均为 `PASS 0 0`

早期 wrapper 探针证据亦保存在相同目录的 `smoke_prove` 和 `smoke_cover` 子目录。

这些结果证明真实工具可执行，不等同于 Story126.3 产品验收，也不承诺所有未来 fixture 均兼容。

## FR119 语法、reset 与失败退出码调查

原始运行日志 `/tmp/bitloom-1263-existing-formal.log` 显示，Yosys0.33 无法解析原 fixture 的并发 SVA 时钟事件：

```text
fr119_pass.sv:18: ERROR: syntax error, unexpected '@'
DONE (ERROR, rc=16)
```

仅在 `/tmp/bitloom-1263-fr119` 复制并修改 fixture，验证以下同步过程写法兼容，并保留旧源码检查所要求的 `assume property` 与 `assert property`：

```verilog
reg [3:0] q;
always @(posedge clk) begin
  if ($initstate) assume property (rst);
  if (rst) q <= 4'd0;
  else q <= din;
  if (!rst) begin
    assume property (din < 4'd10);
    assert property (q < 4'd10); // FAIL fixture uses q == 4'd0
  end
end
```

第一拍假设 reset 有效，避免任意初态 q 在 reset 更新前被错误检查。之后断言检查时钟边沿采样到的更新前状态，数据更新仍采用非阻塞赋值；reset 在采样边沿抑制检查。

真实运行命令：

```sh
PATH=/tmp/bitloom-1263-sby-installed/bin:$PATH BITLOOM_SBY_FIXTURE_DIR=/tmp/bitloom-1263-fr119 bash scripts/formal-sby-check.sh
PATH=/tmp/bitloom-1263-sby-installed/bin:$PATH BITLOOM_SBY_FIXTURE_DIR=/tmp/bitloom-1263-fr119 BITLOOM_SBY_MODE=fail bash scripts/formal-sby-check.sh
```

| 配置 | 实际 solver 结果 | SBY/wrapper 退出码 | 日志 |
|---|---|---|---|
| PASS，`expect pass` | 深度8全部通过 | 0/0 | `pass-wrapper.log` |
| FAIL，原始 `expect fail` | step3真实断言反例 | 0/0 | `fail-expect-fail-wrapper.log` |
| FAIL，仅将 expect 改成 pass 的对照实验 | 同样的真实反例 | 2/2 | `fail-expect-pass-wrapper.log` |

上述日志均在 `/tmp/bitloom-1263-fr119/`。第二项实测原 wrapper 错误打印 `OK mode=fail`，所以确有合同冲突：SBY 的“预期失败任务成功”与 wrapper 所需“模型失败返回非零”不是同一层语义。

反例文件：`/tmp/bitloom-1263-fr119/fr119_fail/engine_0/trace.vcd`、`trace_tb.v`。反例先 reset=1，随后 reset=0 且 din=2，之后 q=2 违反“始终为零”；因此是有效设计行为上的故意失败，并非语法错误或未初始化误报。

当前临时 FAIL `.sby` 保留最后的对照实验 `expect pass`；原始 `expect fail` 配置复制保存在 `/tmp/bitloom-1263-fr119/original-expect-fail.sby`。这只是诊断对照，不建议据此改变仓库已有 `expect fail` 合同。

## 结合既有测试合同的最小修复建议

保留 FAIL fixture 的 `expect fail`，让 SBY 继续表达“这是预期产生反例的故障模型”。修复 wrapper，使其在 SBY 完成后读取实际 task status，而不是只看进程退出码：

1. SBY 自身非零时，保留错误并返回非零，不能将语法 ERROR 当作有效反例。
2. PASS 模式必须核验 status 首字段为 PASS，其他结果全部失败。
3. FAIL 模式核验 status 首字段确为 FAIL，输出清晰的真实断言失败说明并返回非零；若实际为 PASS、缺文件或未知状态，则输出对应错误并返回非零。
4. 负向回归同时核验“非零退出”与实际 FAIL/status/反例证据，防止解析错误误过门禁。
5. 两个 SV fixture 改为上文真实验证过的过程断言与显式初始 reset 假设。

此建议不改变故障模型含义，也保留已有测试对 `expect fail` 的要求。wrapper status 修复尚未在此探针实施，需主任务改动后执行真实 PASS/FAIL 验证。工具 pin 和安装器修复同样仅给出证据与建议，由主任务实施。
