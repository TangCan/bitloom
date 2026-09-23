# 发布候选独立审查与裁定

审查层：按bmad-build oneshot执行Blind Hunter；该路线没有要求其它层。release_investigate另做只读发布路径调查。

Blind Hunter检查当时约10.19kB实质变更，N=min(floor(sqrt(10.19)+1),10)=4，返回四项：

1. **medium / patch**：hir/builder依赖直接取sim包版本会让未来独立sim补丁要求不存在的家族版本。修正为与本轮实际依赖对应的最低版本1.2.0；sim自身仍随CARGO_PKG_VERSION。功能生成器也使用hir最低版本。没有扩大API或修改仿真语义。
2. **medium / patch**：只比较版本helper的单元断言不能证明实际生成清单。仓库外安装解包候选后实际运行gen-func/gen-cycle，归档完整Cargo.toml，断言依赖全部为registry 1.2.0而非path，再编译运行两种生成物。
3. **medium / patch**：根Cargo.lock被忽略，--locked操作不能仅凭源码提交重现。归档workspace-Cargo.lock.txt与SHA256；文档要求正式上传前恢复并核验锁文件，重解析必须重新验证。
4. **medium / patch**：正式上传后的验收只覆盖new/build，遗漏本轮修改的生成器。发布清单已加入不带patch环境的gen-func/gen-cycle生成、依赖检查及运行。

未新增延期事项；这些修正不改变FR189/Epic122 deferred或NFR91。十包联合publish dry-run随后实际通过，未上传；五个旧单包联网测试的离线失败/过滤与联合dry-run结果分开记录，正式逐包上传仍未执行。

修正后Blind Hunter复核：四项已落实，未发现仍未解决的实质问题；复核时长回归尚在执行，最终结果以后续原始日志为准。
