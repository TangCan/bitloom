# Story130.2 关闭复核（2026-09-23）

状态：独立FR199快照最终门禁全部通过，等待主代理最终复核与单Story提交处理；Story/sprint仍review，不提前宣称done。

历史提交 `abc61b4` 的标题不能替代验收。该提交之后发现的 FR199 缺陷按原合同修复；FR200 实现单独保留，不并入本次来源修复提交。

| 问题 | 当前修复与审查依据 |
|---|---|
| Bender dependency 匹配不精确 | 只读取根 dependencies 段，完整比较 git/version、依赖集合；拒绝重复/额外字段，版本前缀不能代替精确版本。支持锁定源码实际使用的行内/块映射。 |
| 宿主根目录可读 | bubblewrap 仅挂载显式 runtime、复制 input/cache 与 scratch；清空环境，实际探测原 checkout/cache/manifest、host sentinel 不可见及网络不可达。 |
| 只读缓存被编译临时目录污染 | 编译写入 TMPDIR scratch；原缓存与复制缓存前后摘要必须相等。 |
| 真实源码默认参数核验不足 | 对冻结 FIFO 的 DATA_WIDTH、DEPTH、FALL_THROUGH、dtype、ADDR_DEPTH 默认值以及已有端口/include 合同精确核验。 |
| 工具子进程无界等待 | 由 GNU timeout 限时并设置 kill-after，非零结果有可定位诊断。 |
| 验收修改 canonical lock | ATDD 在独立 fixture 创建 lock、冻结 CLI；退出核验真实 manifest/lock 未变。 |
| 旧 evidence consumer 认可不足的隔离 | 独立消费只读挂载、clearenv、host probe、cache integrity；普通与优化模式都须拒绝变异证据。 |
| CLI 新入口漏显式登记 | lock/verify/replay 逐项登记 FR142/SemVer minor；未改版本、未发布。 |

主代理已读取 FR199 独立差异及原 Story/NFR14/AD-31，并复核依赖解析、冻结参数、只读重放和新增 CLI 登记。四层先前审查中的来源问题不再按旧 false/defer 跳过；独立复核代理已读取 FR199 专属最终实现差异，未发现新增阻断问题。随后发现的探针向原缓存写临时文件问题已修复：探针位于独立临时目录；原 checkout/cache/manifest 只作可见性检查。原输入实际只读挂载下的重放已经通过，完整最终证据另列。真实网络/HDL、consumer 与 clean/fmt/workspace 退出结果必须在最终记录中另列；ignored 不计 PASS。

## 独立复核补充（r1303_blind，2026-09-23）

已独立复核 `/tmp/bitloom-130-2-closeout` 相对HEAD的FR199范围差异：来源闭包/精确Bender依赖、固定源码默认值、bounded subprocess、scratch临时编译、复制cache与allowlist禁网重放、ATDD自有lock、证据consumer与CLI显式登记。未发现新的范围内阻断问题；FR200 binding/behavior/simulator身份未混入该source-only实现。

审查发现原host sentinel写入原输入违反只读合同；修复后两份runner均使用独立 `tempfile.mkdtemp(prefix="bitloom-host-only-")`，原checkout/cache/manifest只传入存在性负探针。consumer核验恰好5目标、原cache/manifest身份、独立sentinel目录/文件关系，以及无原输入下sentinel写入。读到的真实外层只读验证记录是 `130-3-review-verification/readonly-input-replay.json`：exit0，实际隔离probe与Yosys编译通过。它证明该runner修复可行，不代替FR199独立快照的最终门禁。

Legacy consumer保持FR199工具字段；仅当lock出现可选模拟器身份时要求其完整，不要求binding/behavior。修复后的consumer已在真实FR200来源夹具运行11cases/22 normal/-O子进程并通过，原始日志 `130-3-automation-legacy-evidence.log`。独立FR199快照clean/fmt/workspace、自己的fresh lock/replay及其consumer门禁仍由verification worker执行，本补充不宣称其canonical或最终回归已通过。

## 独立快照最终实跑结果

详见 `130-2-final-verification.md` 与 `130-2-final-regression/commands.json`。实际clean/fmt/workspace全通过：485结果块、1889 passed / 0 failed / 50 ignored。随后本快照独立fresh fetch、210文件完整身份比较、source-only禁网重放、原输入真实只读挂载重放均exit0；专用联网变异1/1、ATDD普通/优化各3/3、consumer普通/优化各11/11通过。此前引用根130.3夹具的记录仅为修复过程历史，当前关闭依据是本独立FR199快照自己的门禁与raw evidence。主代理最终复核及提交动作尚待执行。

## 主代理最终结论

独立来源快照已完成全部 12 条实际命令，零失败；前文 pending 是历史阶段状态。来源、隔离、负测、独立 consumer 与 1889 passed 的 workspace 结果已由主代理逐项复核，无未解决阻断项。只关闭 Story130.2，FR200 分责保持。
