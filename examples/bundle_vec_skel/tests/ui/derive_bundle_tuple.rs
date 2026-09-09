//! Unsupported: tuple struct — stable rhdl::E0180.
use bitloom_prelude::{Bool, Bundle};

#[derive(Bundle)]
struct TupleBundle(Bool);

fn main() {}
