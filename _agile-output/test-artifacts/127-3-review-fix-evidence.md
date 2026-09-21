# Story127.3 build-review 修补 B1–B10

2026-09-21。仅修改三个桥测试文件，产品RTL/API未改；原始`127-3-build-*`归档保持不动。本记录不是完整workspace回归、故事done或提交宣称。

| 问题 | 最小修补及实际断言 |
|---|---|
| B1 | 真实leaf STATUS先为0xa5；AR接受后、CSR提交前断言提交数未变，再改为0x5a；RDATA必须0x5a，并继续验证响应受阻快照 |
| B2 | 每种WSTRB分别先全有效位1再写0，以及先0再写1；独立黄金mask同时检查选中位清/置、未选中位保留 |
| B3 | 仅在真实CSR W1C clear握手沿注入相同bit事件，断言恰好碰撞一次，结果保持置位，验证set-wins |
| B4 | 真实bridge+CsrBlock共同reset分别命中offer未提交、提交副作用后未CSR消费、B保持、R保持；检查leaf清零、各阶段提交/副作用/CSR消费/AXI消费取消差额、12拍无重放及每次新写/读恢复恰好2提交/2响应 |
| B5 | 保留16×1000串行随机预算，新增三个独立持有producer的并发压力；每个native engine各256写+256读，复用独立Score并真实RTL回放；有20,000拍超时和有限请求/响应ready窗口 |
| B6 | gap/WSTRB只统计实际写，并断言各自总数等于完成写数；从真实握手frames统计AW-first/W-first以及B/R受阻拍，每seed非零 |
| B7 | 删除peer age及≤7拍响应assume；安全允许任意延迟甚至永不响应，仅保留因果、合法错误及受阻保持；无B/R公平性。cover也没有额外响应界限assume |
| B8 | 根据公开端口及独立ghost，前沿非reset、无CSRoffer/执行、捕获完整且响应有空间，下一非reset沿必须出现offer；是assert，不是假设 |
| B9 | 无实现引理的port-only原始control BMC10 PASS；配对/路由/offer保持三RTL变异均必须真实assert counterexample、SBY FAIL/exit2/VCD，ERROR/UNKNOWN/timeout均拒绝 |
| B10 | 桥非法模块名E0241、与已有模块冲突E0244，均检查helper错误及finish poisoning；桥连接位宽E0203和child输出驱动parent输入E0257负例 |

## 真实结果

- 真实CSR集成最终定向重跑exit0，0.13s：177提交、176CSR消费、B110/R64。差额是有意取消：1个已提交执行与2个已存AXI响应；四个reset阶段逐项日志存在，不能误要求跨epoch总数全相等。最新`127-3-review-fix-integration.log`。
- 并发最终重跑exit0，6.18s，Interpreter/Compiled及两个实际RTL均通过。每engine 512提交/消费，B256/R256；重叠捕获3019拍；AW/W/AR/B/R/CSR请求stall分别3133/3141/3534/1879/2254/678拍。`127-3-review-fix-concurrent-final.log`。
- 专用形式/综合目标3 passed、0 failed，28.00s：无响应延迟界限的prove8完整basecase+induction PASS、cover64五项PASS、原始综合150FF与5个边界PASS；`127-3-review-fix-formal.log`，实际目录`proof-1482176`与`synthesis-1482176`。
- 最终offer保持变异特意改为offer只维持一拍、无视req_ready（先前ready-gating变异证据也保留）。最终控制+三变异定向exit0，15.79s，`127-3-review-fix-mutations.log`及`mutations-1484210`：control PASS/0；三个mutant FAIL/2，均有counterexample VCD。offer-hold精确击中原始port oracle的stalled-valid保持assert（生成文件296–297行），不是实现引理。mutation BMC只是敏感性检查，无界安全结论来自单独prove。
- 16seed重跑结果与首个并发刺激失败见末尾补记；`127-3-review-fix-behavior.log`必须原样保留，不能用定向重跑覆盖它。

14测试入口：10行为、4形式/结构（其中3显式ignored工具入口）；增加并发、桥诊断和形式变异三个入口。此轮只运行两个编辑文件所覆盖的桥test targets及其定向过滤，没有运行无关兼容或workspace。root后续执行完整验收。

## 失败保留与精确假设

第一次并发生成器用u64乘法产生超过32位的W数据：native端口正确截断，独立Score保存完整u64，出现2759415514与7054382810（差2^32）的预期值不一致。固定种子`0x1273_c0ffee`，原式`(w * 0x1234567) ^ 0xa55a1234`；修复为在**刺激**处`& 0xffff_ffff`。未修改产品或Score来迎合错误。原失败stdout保留，针对并发重跑通过；随后只增加实际CSR stall计数又定向重跑通过。

安全证明无CSR完成上界、无B/R最终ready条件，也无整体活性宣称；响应仍须因果、单个、合法error00/10/11、错误数据0且valid受阻保持。新增下一拍offer义务只约束已捕获且有容量的空闲状态，不要求阻塞中的CSR请求得到accept。所有原有assert与内部对应assert仍在，未新增DUT行为假设。

命令统一环境前缀`PATH=/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin:$PATH CARGO_PROFILE_TEST_OPT_LEVEL=1 BITLOOM_REQUIRE_RTL=1`：

```text
cargo test -p bitloom --test fr196_axi_lite_csr -- --nocapture
cargo test -p bitloom --test fr196_axi_lite_csr p0_concurrent -- --nocapture
cargo test -p bitloom --test fr196_axi_lite_csr p0_same_session -- --nocapture
cargo test -p bitloom --test fr196_axi_lite_csr_formal -- --ignored --nocapture
cargo test -p bitloom --test fr196_axi_lite_csr_formal p0_port_oracle_kills -- --ignored --nocapture
```

上文耗时为Rust runner报告值；底层SBY/综合/变异另有真实命令、退出码和测量elapsed JSON。每个seed打印累计elapsed及实际握手覆盖，未把串行随机称作并发，新增并发单独记账。rustfmt --edition2024 --check及git diff --check均exit0。

## 持久归档

新`127-3-review-fix-source-snapshot.tar.gz`及`127-3-review-fix-tool-artifacts.tar.gz`配套逐成员SHA256 manifest；生成后回读核验所有成员。工具包保存本轮各PID生成RTL、testbench、证明与变异配置/日志/轨迹/版本/JSON，省略可重建的Icarus simulation可执行文件。原始build归档没有被重写。源hash仅指本轮明确snapshot；root后续改文档或测试时不能误称live内容仍等于此快照。

## 16seed与最终修补结果补记

本轮16个seed各1000事务全部PASS，实际仍为7955写、8045读、16000提交/消费；每seed的write-only gap/WSTRB总数均等于该seed完成写数，AW-first/W-first/B-stall/R-stall全部非零。独立提取后保存`127-3-review-fix-results.json`。初始整行为目标最终是**9 passed、1 failed、exit101、293.91s**（唯一失败是已说明的并发刺激位宽bug），没有将此命令伪称exit0。修复后仅定向重跑并发，最终PASS；其他新增检查与16seed已通过，因此不重复无修改的5分钟矩阵。root后续统一完整验收。专用formal与最终变异测试均真实exit0；所有B1–B10修补均已有各自通过证据。

归档完成：source6成员/27,105字节，tool499成员/22,194,126字节；505个成员全部从tar.gz回读SHA256校验通过。没有待结束的本轮测试或归档进程，root可继续独立验收并按既定流程clean。
