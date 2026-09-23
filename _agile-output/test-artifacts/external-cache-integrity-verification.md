# 外部IP缓存完整性修复验证（2026-09-23）

范围：复盘M1（失败回滚删除共享缓存）与M2（缓存祖先符号链接越界）。基线为`063991d`，完整基线标识见本轮spec。未改变锁schema、来源身份、公开API、工具钉或包版本；未重新获取上游源码、下载厂商工具或推送发布。

## 修复与验收

- `publish_closure_cache`以实际创建/复用结果返回所有权；`finalize_lock`仅在本次创建闭包时回滚删除。锁文件是否已存在不再替代缓存所有权。许可证发布、最终验证、锁文件写入失败共用该处理。
- `cache_directory`验证cache锚点和相对路径各级为真实目录。`lock`在fetch之前检查锚点/closures；发布闭包和验证source再次检查对应路径。普通链接、悬空链接、非目录均失败。它不声称抵御恶意并发替换目录的TOCTOU，不提供新的并发事务协议。
- 发布拒绝时清理本次获取的暂存目录和暂存许可证，保留已有闭包/链接目标。实际获取入口传入的是该调用产生的暂存路径，不把共享闭包当暂存清理。

## 确定性回归

先只提取原发布收尾代码为可测私有函数，不改变其逻辑，然后执行5项测试：2通过/3失败（共享缓存许可证失败、共享缓存验证失败、祖先链接），见[红灯](external-cache-integrity/red.log)。修复后5项通过；独立审查补齐边界后9项全部通过，见[最终定向结果](external-cache-integrity/review-green.log)。

测试使用临时文件系统和真实生产收尾函数，无网络、无mock清理副本。覆盖既有锁/新锁共享缓存、仅新建闭包回滚、恢复许可证后仍可验证旧缓存、最终锁目标写入失败、正常新建/复用、4级verify链接、3级publish链接及悬空目标、发布失败暂存清理，以及实际lock入口在fetch前拒绝链接且目标仍为空。

## 真实CLI与隔离重放

[命令结果](external-cache-integrity/cli-results.json)：正常verify成功；`closures`与digest两级祖先链接均报`bitloom.external-ip.path`且非零退出；恢复原目录后verify成功。

使用原真实三仓库闭包副本，保留完整源/许可证身份，仅将本轮测试锁的`tools.bitloomSha256`绑定到最终本地CLI。原锁与原缓存不改；这是基于已有闭包的本地工具重绑定，不是新一轮在线来源解析。见[来源说明](external-cache-integrity/test-lock-provenance.json)与[test锁](external-cache-integrity/tested-source.lock.json)。

随后实际执行bubblewrap禁网、独立复制、只读输入的replay/pilot：

- [HDL编译](external-cache-integrity/offline-replay.json)：network=denied、hdl_compile=passed。
- [绑定](external-cache-integrity/offline-replay-binding.json)：真实HIR父模块与锁定fifo_v3绑定成功。
- [行为](external-cache-integrity/offline-replay-behavior.json)：独立FIFO oracle 104周期通过，含populated_reset及flush。
- [隔离与输入保持](external-cache-integrity/offline-replay-isolation.json)：filesystem/network denied，原缓存前后与副本后哈希均相同。

审查前的一轮CLI结果保留在`pre-review/`，最终验收只引用顶层文件。执行脚本快照保留绝对路径，属于证据而非用户通用入口。

## 审查与边界

oneshot流程的独立Blind Hunter提出4项，逐项核查并修复，无延期项；详见`spec-external-cache-integrity.md` Review Triage Log。本轮没有改变上游RTL/外设逻辑，不重跑远端CI、上游大随机套件、全部专用外部后端/formal或物理流程；既有对应源绑定证据保持历史意义，不把ignored计PASS。

最终格式与工作区回归结果见[命令元数据](external-cache-integrity/regression-commands.json)与workspace-summary.json；源身份见[SHA清单](external-cache-integrity/final-source-sha256.json)。M1/M2需以本轮验证完成结果关闭，M3/M4继续open。

最终工作区结果：**487个结果块，1903 passed / 0 failed / 56 ignored**，命令退出0。使用与历史成功验收一致的test opt-level=1；初次默认配置运行在旧FR193长随机测试中主动SIGTERM，退出101，保留在workspace-unoptimized-interrupted.log及对应命令记录，不计PASS。格式检查退出0。

M1/M2状态已由追踪脚本更新为done，M3/M4仍open；本轮闭合后Epic130复评accepted，Phase24仍accepted-with-open-items。
