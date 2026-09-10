//! FR118 positive fixture — prelude dep, **no** metadata design_roots;
//! discovery reads `#[bitloom::top]` (syn-scan). Elaborate uses LSP registry.
#![allow(dead_code)]

#[bitloom::top]
struct Fr118OkCounter;
