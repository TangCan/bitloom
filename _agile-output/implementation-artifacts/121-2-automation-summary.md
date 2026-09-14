# Automation summary — Story 121.2 / FR188

- ATDD: `cargo test -p bitloom --test fr188_community_style_guide_pack`
- Live gate: `just chisel-style-guide-pack-check`
- Full gate: `cargo clean && cargo fmt --all && just test`
- CI: required `chisel-style-guide-pack` job
