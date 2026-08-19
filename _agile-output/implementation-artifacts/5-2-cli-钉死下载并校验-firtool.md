# Story 5.2: CLI 钉死下载并校验 firtool

Status: done

## Completion

- `rhdl-rs` module `firtool`: ensure/download/sha256/`RHDL_FIRTOOL_PATH`/`RHDL_FIRTOOL_CACHE`
- CLI: `cargo rhdl firtool info|ensure`
- Unit tests: parse digest, sha256 empty, missing override
- Full network ensure may be slow; logic verified locally
