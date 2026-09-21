# 127.2 独立 code-review

2026-09-21；full模式，spec-127-2-csr-product.md与其5项context，baseline77ea01392e5be74b9f2622009ad146edb2468ea9。四层全部完成；前三层同启，第四层因并发上限在第一个返回后启动，全部返回才裁决。无失败层；edge返回[]，acceptance无可操作违反项并保留AC8未完成，不视为空失败。

| ID | source | title / detail | verdict与证据 | route |
|---|---|---|---|---|
|1|blind|同大写include guard的不同头跳过碰撞诊断|low：Probe/PROBE确共用guard；docs已要求不同内容不得复用同一include guard，属于明确不支持的组合。加入完整配置身份机制超出直接修正，固定guard格式不能随意改变。补明大小写折叠限制，不声称guard能检测复用。|rejected|
|2|blind|read_reject只在RO行为覆盖|medium：leaf RW/external RW/W1C合法开关缺独立行为断言。|patch|
|3|blind|formal未开leaf write_reject|medium：虽然Probe范围已说明，打开这些端口可直接覆盖拒绝与事件交互，保留独立参考与所有旧性质。|patch|
|4|blind|新directed刺激没有独立producer monitor|medium：上一轮确曾撤回未接收请求而scoreboard未发现，增加实际native/RTL输入稳定检查。|patch|
|5|blind|generated Rust standalone未验|low：当前文档只承诺两native和RTL，层级拒绝也不是standalone验收；明确其未验证边界即可，不新增backend实现。|patch documentation|
|6|blind|singleton可选端口/无leaf存储未实际综合|low：配置测试已有五种组合elaborate，但原始综合仅Probe；追加五个singleton原始综合，不声称所有配置形式证明。|patch|
|7|blind|codec隐式discriminant与硬编码decode易漂移|low：现版本映射正确，未来增加/调换variant时维护点可能分离；改显式match固定编码是直接修正，无public API变化。|patch|
|8|blind|story review但task全未勾，ATDD红阶段不够醒目|low：build已有附记但头部checkbox陈旧；只勾已完成T1–T5，标明ATDD历史，T6保持未完成。|patch root|
|9|blind|历史源码以普通.rs/.py副本重复干扰搜索|low：本轮diff12685行大量来自快照，rg易把旧实现视为活代码；将immutable副本归档并保留逐member摘要、历史原manifest，不丢字节。|patch root|
|10|blind|缺base+byte-offset C使用例|low：虽有常量编译，仍可误用u32指针缩放；补uintptr_t/byte base示例并严格编译，不运行MMIO。|patch|
|11|verification|多field硬件mask漏字段回归缺口|medium，预核验：全部执行fixture单field，多field仅软件/重排；r.fields[0].mask突变不会被现有硬件tests检测。|patch|

10 patch，0 decision-needed，0 defer，1 rejected；按既有完整授权全部修补，不再请求常规确认。每项保留独立裁决，未因同在一个文件而合并。仍需automate及最终clean/fmt/just test和commit，review不能提前关闭story。

## Rejected

1 low：同include guard不同定义已被文档明确排除；保留标准重复include机制与固定guard格式，不加入复杂配置身份协议。文档额外明确大小写归一化后同名也属此限制。

## 修补完成

10个patch全部处理，root复核新增源码与独立黄金、producer monitor、formal参考方程及证据摘要11项全部OK。19功能/config、3codec、3真实formal/综合和文档例通过；singleton新增结构断言后单项重跑通过。见[修补证据](127-2-independent-fix-evidence.md)。R8/R9分别为story历史任务与快照归档，非产品测试项。R1边界说明已补。无未解决high/medium，无延期；on_complete解析为空。用户七步优先：不按skill提前将story done，保持review，下一步automate。
