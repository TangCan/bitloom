# Story128.5 最终验收

create-story、ATDD、build、独立code-review、automate及实际clean/fmt/just test依序完成，随本故事单独提交。构建三路内审2修补/9驳回，独立四层审查10驳回；自动化双路AC覆盖审计无新增P0/P1缺口，未新增重复测试。没有未决审查项。

完整回归：482个结果块、1870 passed / 0 failed / 47 ignored，三命令均exit0，just test耗时1009.308秒。570相关源码起止一致；命令/UTC/耗时/日志见`128-5-final-regression.json`和`128-5-final-{clean,fmt,test}.log`。新UART的7项ignored专用入口另在完整source-bound运行实际执行（4后端+3formal），不把workspace ignored计PASS。清理输出：Removed 40850 files, 29.4GiB total。

交付独立UartCsr：固定15端口/六寄存器，32位DIV、双ParamSyncFifo、RX同步中心采样、四原始事件和W1C；旧UART8/VIP/手写FL保持。direct/FIRRTL/Chisel实际执行相同三seed共72413拍独立绝对deadline/VecDeque黄金，12类3785拍边界另跑；共享/改名双实例及实际IRQ组合、16帧真实wire loopback通过。全宽80000000/fffffffe/ffffffff为direct近终点注入和算术/形式补充，不声称极值三后端长周期仿真。

有限安全prove及三短bus cover实际通过，原始无observer综合1431cells；串行倒计时/FIFO/raw事件三DUT故障控制及ready驱动formal故障指定断言检出并有VCD。独立审查再用未改三seed TB核RX capture前后一级错误，两者均在cycle21804 rdata断言失败，正常control通过；见`128-5-code-review-rx-capture-audit.json`，不是编译错误或超时冒充检出。

最终完整定向run `20260921T135428.448283Z-1415681`：39命令全部PASS、570源起止/当前一致。当前631成员原始归档`128-5-build-review-raw.tar.gz` SHA256 `394efe3aa88688493b26f0581a59d326e5aa66211701e2e3cb56b25d4428813a`，主代理逐成员与原文件独立比对；历史3196成员归档三分片另保留，不能用历史PASS并集认证当前代码。clean后6归档/分片字节及SHA均不变，见`128-5-postclean-archive-hashes.json`。证据harness normal/-O各10项实际通过，包含必需gate日志缺失拒绝。

旧UART/VIP/FL24项、FIRRTL21项、numeric、SemVer、prelude-only原文例编译展开、软件产物及C11消费者实际通过。CI接普通/后端/formal/失败工件，本地对应验证通过不冒称远端CI已跑。native/generated层级入口仍unsupported；有限proof不代表完整协议证明、物理MTBF或波特率失配签核。

四API符号显式登记FR142/minor，未改工具钉或包版本。随本故事提交关闭Epic128/FR197/M3（128.1–5全完成），映射见`../implementation-artifacts/epic-128-closeout.md`。Epic129/130、FR198–201及整个Phase24仍待交付；FR189/Epic122 deferred与NFR91保留，不push/publish。

提交前whitespace检查单列原始日志及已编译原文例的末尾空行，不改实证字节以掩盖提示；最终具体输出保存于`128-5-final-diff-check.log`。
