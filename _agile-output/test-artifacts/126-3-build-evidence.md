# Story126.3 实施验证证据

2026-09-20。未提交、未改story/sprint状态；独立code-review、automate及clean全工作区回归由主代理接续。FR195仅注册切片子集，M1仍待126.4。

环境：Rust1.97.1、Icarus12.0、Yosys0.33、SBY yosys-0.47、Z3 4.8.12。工具PATH为 `/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin:$PATH`，设置 `CARGO_PROFILE_TEST_OPT_LEVEL=1`（保留断言与溢出检查）。版本日志在126-3-evidence内。

## 实际结果

- `cargo test -p bitloom --test fr195_rv_reg_slice -- --nocapture`：8项通过。实际执行合并命令还请求formal，见`126-3-build-green.log`前半；其后首次formal UNKNOWN单独如实保留。四宽1/8/32/64×三种子×Interpreter/Compiled，12次单模块Icarus以及双实例Icarus通过。
- 最终 `cargo test -p bitloom --test fr195_rv_reg_slice_formal -- --ignored --nocapture`：2项通过，见`126-3-formal-final.log`；各宽度1/8的prove basecase与temporal induction PASS，cover PASS且真实命中两个cover。实际原始产物`target/fr195-formal/width{1,8}-2897658`，精选源码/配置/结构/日志/见证归档`126-3-evidence/formal-width*`，clean之后仍能复现。
- `cargo test -p bitloom --test fr119_symbiyosys_smt_path --test fr161_formal_sby_image_hygiene --test nfr14_risk_epic93_formal_sby_image_hygiene_fr161`：8+6+1=15项通过，见`126-3-formal-regression.log`。FR119负例强制非零、实际status=FAIL、真实trace.vcd；语法ERROR不再算负例通过。
- `bash scripts/formal-sby-check.sh`：PASS；`BITLOOM_SBY_MODE=fail bash scripts/formal-sby-check.sh`：真实FAIL，wrapper_exit=1，保留expect fail，见`126-3-fr119-{pass,fail}.log`及`126-3-evidence/fr119-fail`。
- `bash scripts/ci-sby-hygiene-check.sh`通过。真实fresh clone验证tag对象bfc1c47eb786496fe794481ff88e75728f0529a6严格解引用commit daed0e1544fd96ee7dab843e5a891d92784c6230，fresh前缀上游make安装、`env -u PYTHONPATH .../bin/sby --version`成功，见`126-3-fresh-install.log`。未在宿主执行sudo apt安装；验证的核心命令与修复后的CI脚本一致。
- `cargo fmt --all`执行。实施阶段未cargo clean，全部workspace回归留到用户指定第六步。

固定随机种子：0x12631950a551、0xdeadbeef8012、0x73592401ffff。每流420拍，最后20拍排空；accepted=245–249、delivered=238–242、cancelled=7，各epoch与最终守恒，最长背压38拍、连续吞吐44–45拍。具体每宽每种子统计和真实vvp输出在build日志；RTL精选日志及生成设计SHA256在`126-3-evidence/rtl`。重复testbench/二进制不入归档，测试可重生成。

## 有证据的脚手架修正

1. 初始八项中六项通过；双实例ATDD定义两个顶层Reset，现有E0121拒绝（`126-3-build-initial.log`）。修为共享唯一rst，独立lane错位取消用flush，保留不同payload/ready及逐拍独立队列。未改HIR验证或放宽行为。
2. SBY status实际包含`PASS 0 0`，严格读取第一状态字段；FAIL/UNKNOWN/缺文件/超时仍失败。
3. 纯端口队列的head性质不能归纳约束隐蔽back：初始24步basecase全过但induction UNKNOWN，见build-green日志。加强observer的纯assert：count==pending、满态back==q1；不增加内部假设，初始和转移同样由solver证明。实际生成RTL原样保存design.v，并只在其endmodule前追加observer形成design_formal.v；无跨层隐式nets，生成Yosys日志无implicit/undriven告警。独立端口事务队列、所有原合同断言保留。此处显式修订原ATDD“只读端口”实现说明以获得非空无限安全归纳证明。
4. Z3默认nounroll展开较慢；正式引擎参数`smtbmc --unroll z3`，prove depth4、cover depth12，最终每宽证明与见证均PASS；归纳成功是无限安全证明，不把有限深度BMC冒充证明。seen_full在reset/flush时同步清零，故满后空cover不能靠取消伪造。证明不声称无限活性。

最终格式化后联合复跑：`cargo test -p bitloom --test fr195_rv_reg_slice --test fr195_rv_reg_slice_formal --test fr119_symbiyosys_smt_path --test fr161_formal_sby_image_hygiene --test nfr14_risk_epic93_formal_sby_image_hygiene_fr161 -- --include-ignored --nocapture`，25项全通过，日志`126-3-build-final.log`。两个早期nounroll诊断试跑在最终unroll证明通过后主动终止，未作为通过证据；UNKNOWN历史保留。
