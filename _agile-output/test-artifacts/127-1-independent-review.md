# Story127.1 独立 code-review

2026-09-21，完整未提交diff187960bytes/2160行，baseline e88a22bef266b52174be51f831d7ef510b65f108。full模式；blind、edge、verification、acceptance四层全部成功；受线程容量限制分批启动，收齐后裁定。edge为空、verification无缺口、acceptance无违反。

| ID / 来源 | 裁定 | 证据与处理 |
|---|---|---|
|1 blind|low / patch|review重跑再次生成pyc，已删除本次缓存；后续运行设置PYTHONDONTWRITEBYTECODE。|
|2 blind|low / patch|GPI记录3.12.4与launcher3.12.3不同；官方源码明确打印编译PY_VERSION。已补runtime查询日志及未在嵌入进程直接查询的边界。|
|3 blind|medium / patch|review日志与初次JUnit属不同seed；归档review对应JUnit/stub，分开两次记录。|
|4 blind|low / patch|历史manifest命名当前源码却放旧hash；改为直接命名已归档旧源码，避免机器复核误报。|
|5 blind|false / rejected|AC5明定后续ready-for-dev场景；27场景完整满足本故事，并未声称覆盖门禁全部状态转移。其它advance状态可在automate评估，非当前验收缺陷。|
|6 blind|false / rejected|AC5要求M0未关闭，本例真实失败；未承诺测试gate所有库存规则。M0假done路径是已有逻辑，非本故事改动。|
|7 blind|false / rejected|本故事AC1要求保留实际deferred，diff证明确实保留；未声称已穷举FR189门禁负例，无该路径变更。|
|8 blind|medium / patch|仅RO/event/拒绝端口不足以交代Timer自主COUNT和GPIO OUT别名写，补单owner、外部状态读回/合并写提交及合同优先级测试peer。|
|9 blind|low / patch|原矩阵未列无组合路径的验证方法，补127.3 HIR/网表依赖锥检查，不能用稳定波形代替。|
|10 blind|low / patch|多个field容易误读混合访问支持，明确首版单寄存器单访问类型，混合字段诊断拒绝且有负例；不减现有正式合同。|

结果：0决策问题、7项修补、0延期、3项拒绝。所有修补属风险合同精确性和证据归档，无产品/API改变。M13等待automate、完整回归与提交；故事保持review，不能按技能默认提前done。
