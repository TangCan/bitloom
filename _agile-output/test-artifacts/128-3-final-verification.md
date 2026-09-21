# Story 128.3 最终验收

七步已顺序执行：create-story、ATDD、build（并行内审）、独立四层code-review、automate、实际clean/fmt/workspace回归，随后单故事提交。审查9组修补完成，1项驳回，无待决定事项。

实际执行cargo clean、cargo fmt --all、just test全部退出0；clean删除38,082文件/19.7 GiB，fmt未改变产品源码。476测试组，1844通过、0失败、35忽略；just test耗时937.108秒。原始命令与日志见128-3-final-regression.json及final-{clean,fmt,test}.log。5项IRQ专用ignored测试已由真实独立门禁执行通过，忽略不当作PASS。

回归期间唯一指纹差异是被.gitignore排除的SBY运行日志fr119_pass/logfile.txt，产品/Rust测试/CI源码均未改变。随后仅修正证据runner对该生成日志的分类并增加harness回归；普通及python -O各12项通过。修正后完整runner再次执行全部门禁通过，起止及当前源码指纹一致；详见128-3-final-source-audit.json、128-3-final-fingerprint-validation.json及128-3-final-fingerprint-gates-audit.json。未将修正前的workspace执行冒称为修正后的Python脚本测试。

固定五路IRQ、PENDING/ENABLE/TEST/RAW及Timer原始事件组合交付。三个实际RTL后端各97,738帧独立参考通过；双实例/重命名实例/Timer组合覆盖三个后端。prove深度16及归纳通过，cover深度24命中23目标，原始RTL综合138cells、无latch。三个formal DUT变异各产生指定反例；另170帧direct负控制原始PASS，两种真实变异均编译成功、在预期断言失败并产生VCD。

含Instance的native/generated入口明确不支持并验证拒绝；不计行为PASS。prelude-only文档示例实际编译和elaborate，等价Timer组合夹具执行三个后端，不声称示例原图跑过三个后端。四个公开符号登记FR142/minor。Concat限定为chisel3.util.Cat，真实JVM后端验证通过。既有FIRRTL、numeric、SemVer、软件产物比较及C头门禁通过。

CI入口和失败工件已接线，本地执行相应检查；未声称远端CI执行。仅关闭128.3，Epic128/FR197/M3及GPIO/UART仍开放。工具钉、包版本未改变，无push/publish。FR189最近官方检查2026-09-21T08:45:25Z仍无严格大于1.159.0，122.2/122.3保持deferred。

六份原始归档的最终SHA见128-3-final-archive-integrity.json；clean前五份归档在clean后SHA不变，见128-3-post-clean-archive-audit.json。最新归档独立核对737个成员与原文件字节，SHA256 65421eea24ce5b30ceb8ea9e0f3748f25a480be353ca95af7b919694125b63b9。历史失败与旧矩阵计数保留，最终验收采用最新选定run。
