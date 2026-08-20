# Story 13.3: Host shim 使用 registry 后端并解析用户包

Status: done

## Story

As a 本机用户,
I want `cargo bitloom build` 对**我的** Cargo 工作区生效且不 path 依赖工具链仓库,
So that 安装的 CLI 能真正驱动已发布的 emit 栈。

## Acceptance Criteria

见 `epics.md` Story 13.3（FR50, FR51）。

## Tasks / Subtasks

- [x] `cargo metadata` 解析 `--package` → manifest 路径
- [x] host shim 依赖 `bitloom-vlog`/`bitloom-hir` 钉死为 CLI 同版本（crates.io），无 monorepo path（standalone）
- [x] monorepo 自动 path 后端（避免 FrozenHir 类型双份）；`BITLOOM_FORCE_REGISTRY` / `BITLOOM_DEV_PATH`
- [x] ATDD：`host_registry_shim` + unit `host_cargo_uses_registry_backends_outside_monorepo`
- [x] regression `just test`

## Dev Notes

- AD-14 保留进程内 elaborate+emit
- 版本：`env!("CARGO_PKG_VERSION")`
