# Story127.3 build修补主代理验收

2026-09-21。B1–B10已逐项检查源码与修补证据，产品RTL未因修补改变。主代理独立复核修补归档全部505成员SHA256；当前6个源码文件与该快照一致。

| 检查 | 实际结果 |
|---|---|
| 两桥目标普通入口 | exit0，11 passed / 0 failed / 3 ignored，291.834s；16seed×1000全部完成，新增并发与真实CSR场景通过 |
| 专用formal/综合/变异 | exit0，3 passed / 0 failed / 0 ignored，27.907s；不以ignored代替证明 |
| 6个既有兼容目标 | exit0，78 passed / 0 failed / 0 ignored，202.409s |
| 文档原文prelude-only示例 | exit0，0.417s，ExampleAxiCsr 3 modules |

命令、环境、实际退出码、测量耗时及日志SHA256分别在`127-3-review-root-{ordinary,formal,compatibility,example}.json`，stdout/stderr原字节在同名log。修补细节与失败记录见[修补证据](127-3-review-fix-evidence.md)，归档独立校验见`127-3-review-root-archive-verification.json`。

三路build审查10项均修补，无本轮defer；本次验收没有取代独立code-review、automate或用户要求的cargo clean + cargo fmt --all + just test。故事进入review，提交保留到第七步。FR196/M2及127.4不关闭。

主代理复跑产生的额外工具产物已保存到`127-3-review-root-tool-artifacts.tar.gz`：377成员，压缩21,315,655字节，包与全部成员SHA256回读通过；对应PID1504733行为及1525214形式/综合，避免最终cargo clean删除唯一原始工具证据。源码对应本报告时的review-fix快照，后续独立review修补另有新归档。
