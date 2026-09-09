//! Unsupported: enum — stable rhdl::E0180.
use bitloom_prelude::Bundle;

#[derive(Bundle)]
enum NotABundle {
    A,
    B,
}

fn main() {}
