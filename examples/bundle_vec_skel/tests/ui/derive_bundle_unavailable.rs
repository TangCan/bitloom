//! `#[derive(Bundle)]` is not available (documented defer) — hand-write `Bundle::leaves`.
use bitloom_prelude::Bundle;

#[derive(Bundle)]
struct WantDerive {
    data: u8,
}

fn main() {}
