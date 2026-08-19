# FR35 — optional HLS (Bambu)

Pinned backend: **bambu** (AD-25). RHDL never schedules.

```bash
# default
cargo run -p bitloom -- hls   # → unsupported unless enabled

RHDL_HLS_ENABLE=1 cargo run -p bitloom -- hls --function add --out-dir /tmp/rhdl-hls
# requires `bambu` or RHDL_BAMBU_PATH
```

`#[rhdl::hls]` marks host functions; `rhdl_hls::emit_c_stub` writes C for the external tool.
