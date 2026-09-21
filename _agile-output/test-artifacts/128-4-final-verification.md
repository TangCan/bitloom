# Story128.4 最终验收

按顺序完成create-story、ATDD、build、独立code-review、automate、实际clean/fmt/just test，随后单故事提交。构建三路内审7项修补、独立四层审查6项修补/5项驳回，无未决项；自动化双路覆盖审计未发现非重复P0/P1缺口，未新增重复测试。

实际cargo clean删除37978文件/17.8GiB；cargo fmt --all通过；just test耗时955.120秒。479组，1855 passed、0 failed、40 ignored；三命令均exit0，566相关源码起止不变。原始命令/时间/日志见128-4-final-regression.json及final-{clean,fmt,test}.log。新GPIO的5项专用ignored已在独立门禁实际执行：3项formal/综合/变异、两个exact后端入口；未把ignored计PASS。

交付GpioCsr固定16端口与六32位CSR，唯一OUT，共享CSR叶，sync1/sync2/history，沿前DIR、新沿raw与粘滞分离；旧GPIO8/FL和基座不变。direct/FIRRTL/Chisel各55151帧独立黄金通过；共享/重命名双实例及实际GPIO→IRQ三后端通过，包括mask保pending、双W1C/rise同沿set优先及live raw期间reset取消。

独立formal prove basecase+induction通过，10个cover目标关联9份VCD均到达，原始无observer综合990cells无latch/check通过。三个形式DUT变异在指定断言产生反例；另direct IN=OUT变异复用未改黄金，原始control通过、变异指定rdata错误exit1且有波形。

证据harness普通/-O各17通过；ATDD具名11目标/两个专用ignored、形式3目标、两个后端exact、防缺seed/计数/反例/日志、源码与SBY身份、软件字节比较均强校验。最后完整run为20260921T121311.335406Z-902189，全gate通过，678成员归档SHA256 95f6b137604dd5064e4536c9519a532f355871a2dce07e9dcb2df8a56df852d9。主代理清理前逐成员与原文件比较、566源起止/当前一致；clean后3份归档SHA不变。详细manifest/audit与历史失败均保留。

旧GPIO/VIP/FL/SocPad/ChipPadRing/拆分六目标32通过；FIRRTL库21项、numeric13项加18套实际Chisel RTL、SemVer三包、prelude-only原文编译展开/软件金样/C11消费者均通过。例子本身只声明编译展开，不冒称原图跑三后端。native/generated含实例入口明确unsupported并核验拒绝。

CI接线完成且本地对应门禁已跑；未声称远端CI运行。四API符号FR142/minor登记，工具钉与版本未变，不push/publish。仅关闭128.4 GPIO；128.5/M3/FR197整体、Epic129–130仍开放。FR189最近官方检查2026-09-21T08:45:25Z无严格大于1.159.0，122.2/122.3仍deferred，最终处理到延期项时再核验。

提交检查：除原始日志外，git diff --check仅提示已实际编译的docs/ip/gpio-csr-example.rs末尾额外空行；保留已测试原文字节，此为格式提示，非编译/回归失败。其余非日志文件检查通过。
