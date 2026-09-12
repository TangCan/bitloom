# Automation Summary — Story 92.2

## ATDD

- `cargo test -p bitloom-lsp --test fr160_non_cargo_path_scan`
- `cargo test -p bitloom --test fr160_non_cargo_path_scan`

Locks: docs / FR118 alone ban; no-Cargo.toml fixture finds `Fr160BareTop`; missing path NotFound; unix PermissionDenied; FR118 members still green.
