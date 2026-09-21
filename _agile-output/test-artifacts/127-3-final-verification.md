# Story127.3 最终验收

2026-09-21。固定AXI4-Lite→CSR桥子集完成；只关闭127.3，Epic127仍in-progress，127.4/四窗译码/M2/FR196整体未关闭。产品公开名Bitloom，与samitbasu/rhdl无关；无版本升级、发布或推送。

## 七步与实际结果

| 步骤 | 证据 |
|---|---|
| create-story | 8项AC、依赖/合同/context/checklist；127-3-create-story-validation.md |
| ATDD | 实际缺API E0432首红exit101，固定31端口及API；127-3-atdd-red.*及原始源码快照 |
| build | 单session共享无捕获body、独立AW/W/AR、锁定offer、CSR执行owner、B/R保持；3层内审10项修补；根验收11普通+3专用+78兼容与prelude-only例通过 |
| code-review | 4层全部完成，10项验证补强全部修补；13普通+3专用真实通过，0待决/0defer；127-3-independent-review.md |
| automate | 同源SV monitor 24负例+8正例、真实组合重跑；新增错误写非零无关数据2条cover，prove8与全部7条cover64通过；127-3-automation-summary.md |
| regression | 实际cargo clean、cargo fmt --all、just test依次exit0；465个结果块，1801 passed / 0 failed / 21 ignored；源码格式化无变化 |
| commit | 本验收及全部实现/测试/证据归于一个包含Story 127.3的本地提交，完整SHA由git记录核验；不推送 |

真实完整回归开始UTC `2026-09-21T04:20:03.897078+00:00`，结束UTC `2026-09-21T04:33:50.114197+00:00`，总耗时826.217s。clean 7.810s，删除31,143文件/19.6GiB；fmt 1.069s；just test 817.267s。精确环境、各命令exit、耗时及日志SHA见`127-3-final-regression.json`及`127-3-final-{clean,fmt,test}.json/log`，汇总见`127-3-final-test-summary.json`。不是用定向测试代替完整workspace。

21 ignored由新增桥3专用、既有M1 8专用、CSR 4专用、6个既有doc-test组成。新增桥专用已在本故事显式运行通过，见独立review修补与automate证据；既有专用的历史验收不宣称本次workspace自动重跑。最终桥17入口=14普通+3专用。没有把ignored算作证明成功。

## AC覆盖核对

| AC | 已检查的实现/实际验证 |
|---|---|
|1|固定31端口、独立/共享定义同body、复用及双实例RTL；非法名/冲突poison与宽度/方向诊断；可编译prelude-only三模块例；FR142逐符号/minor登记 |
|2|两native引擎及原始RTL矩阵，AW/W偏斜0/1/7/31、16WSTRB/8PROT/全地址；独立并发生产者与partial-write读进展 |
|3|锁定offer不被后来AR替换、提交轮转、唯一owner、空闲合格请求下一沿offer义务；延迟31拍CSR响应期间捕获下一AW/W/AR并正确完成 |
|4|独立B/R背压/容量、错误00/10/11、零WSTRB、锁定读快照；实际7cover含错误写非零rdata到正确B消费；响应/提交同沿被独立oracle拒绝 |
|5|8个不同取消状态及reset/握手竞争计数，真实CSR共同reset恢复；原始综合150FF、5输出无输入组合路径，专用clk核对及负例 |
|6|真实CsrBlock组合RW/RO/WO/W1C、反值字节mask、提交时快照、同沿set胜clear、动态拒绝和原始非法/高地址无alias；native/generated层级明确拒绝后实际RTL验收 |
|7|16seed×1000真实完成及双引擎并发512事务各，独立事件/取消账本、尾部观察；producer监测器Rust及同源SV负例；prove/cover、原始综合、三RTL mutant实际断言FAIL/2且原版PASS |
|8|中文文档/API/CI、78旧IP定向兼容、完整clean/fmt/justtest及单故事提交；旧bank源码未改，M2保持开放 |

安全没有CSR响应完成上界或B/R公平性假设；失败读需rdata0，写响应无关rdata不限。cover只证明可达；BMC10只作mutant敏感性，安全结论来自基例+归纳prove8。保守气泡/共同同步reset、native/generated层级限制、无PPA/VIP/板级签核宣称保持。

## 证据完整性与失败保留

ATDD、初始build、build修补、根复跑、独立review修补与automate分别保存不可覆盖的源码/工具快照和逐成员SHA。主代理逐包回读验证；最终clean后的workspace桥工具产物另存`127-3-final-tool-artifacts.tar.gz`（448成员，27,193,007字节）及manifest，已逐成员回读，省略可重建simulation可执行文件。

初始形式depth32不可归纳/超时、最初缺API、一次并发刺激u64超32位等历史失败均保留；修复刺激位宽未改oracle/产品来迎合。根归档校验器首次误用bytes而该manifest字段为size_bytes，已记录并按正确schema重验550成员；不是产物错误。本次独立review修补和最终完整回归无非预期失败。

FR189/122.2/122.3仍deferred，NFR91未清空。整体全部Story目标仍active；下一故事127.4按同样七步执行。
