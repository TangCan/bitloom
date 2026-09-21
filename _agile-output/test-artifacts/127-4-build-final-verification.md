# Story127.4 build阶段主代理验收

本文件记录七步中的第三步，独立code-review、automate、最终clean/fmt/workspace和commit另行验收。基线91828619f66f787c63483dda4b0ee558bb7501e8；不关闭M2。

主代理完整回读产品/测试/文档差异并核对固定I/O矩阵。build三层审阅共10项，8项测试补强完成、2项有具体反证驳回，无defer；逐项判定在spec的Review Triage Log。第三审阅层受容量限制，在首层完成后以新无上下文代理执行；全部结果返回后才处理。

## 内审补丁后实际重验

| 命令范围 | 实际结果 | 主代理日志 |
|---|---|---|
| decoder三个普通target | 14 passed，4 ignored；44.761秒 | 127-4-root-postreview-ordinary.log |
| decoder专用ignored | 4 passed；11.885秒 | 127-4-root-postreview-formal.log |
| FR193/194/195/CSR/桥普通兼容 | 91 passed；593.916秒 | 127-4-root-postreview-compat.log |
| 旧CSR/桥专用ignored | 7 passed；37.182秒 | 127-4-root-postreview-compat-formal.log |
| decoder、FR194、CSR、桥原文例 | 四命令均exit0 | 127-4-root-postreview-*example.log |

所有命令有同名前缀JSON记录命令、环境、时间、退出码和日志SHA。普通与专用分列，重复运行不叠加测试数。

主代理独立解析16个seed：16000随机、4096并发完成；每seed 16种WSTRB计数总和等于随机写数，随机读写和1000；12348条AW/W接受沿记录的实际0/1/7/31间距、先后和覆盖一致。见127-4-root-postreview-seed-verification.json。proof深度8基例+归纳PASS，7个cover分别到达；port-only BMC control PASS，串owner/双选/高alias/ready错误门控4个mutant均明确assert FAIL exit2并保存VCD，见127-4-root-postreview-formal-verification.json。cover不是活性保证。

新归档127-4-root-postreview-artifacts.tar.gz包含2530成员，逐成员byte/SHA回读通过，整包SHA `d491ccc3bb1ecf83a8bb4d305e396b6041f8f96a2c754baf6188249f31f33d88`。另一次独立检查确认20个源码/文档/脚本/CI成员与当时工作树相同。归档spec是当时元数据快照，后续步骤状态/勾选会变化；不能据此宣称后续补丁与旧归档相同。首次build归档及initial failure证据原样保留。

Step05完成：spec done、story/sprint review；用户七步次序覆盖技能的提前commit建议，尚未提交。四叶是实际CsrBlock及外部测试peer，不是Epic128外设；native/generated层级unsupported、FR189 deferred/NFR91和未交付阶段均保持。
