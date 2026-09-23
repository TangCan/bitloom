# 三层审查范围

内容：`0e2a873^..4d1b19f`统一diff，临时文件80366806字节，以归档证据为主。三代理并行独立读取，未假称逐行阅读全部日志。

- adversarial：当前组合校验、CSR/桥/译码、Timer/IRQ/GPIO/UART、系统夹具、合同与旧补审；10候选含维护机会，不代表10个缺陷。
- edge-case-hunter：external_ip.rs的source/lock/fetch/verify/replay/binding/behavior、隔离runner与HIR/builder/Chisel边界；之后读取commit subjects。
- verification-gap：CI/Justfile、FR201核心runner/隔离重放/六路线执行与证据消费、FR193测试、FR194–196注册、两个系统独立SV oracle、normal/-O负测；返回[]。只表示该切片无新确证发现，不是全仓证明。

派生聚合视图：产品crate依赖及组合/HIR边界保持；生产CSR端口枚举存在重复，未见当前行为分歧，独立oracle中的地址重复属有意；external_ip.rs增长2261行涵盖多项职责，行数本身不证明缺陷。父代理已再次读取有关源文件裁定，详见review-findings.json。git-evidence.json提供23提交（22故事与1维护）、非合并churn；product-growth.json提供当前物理行数，无merge或未测二进制的产品源码增长项。
