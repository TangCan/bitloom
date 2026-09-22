#!/usr/bin/env python3
"""Optimization-safe static acceptance gate for Story 129.2."""
from pathlib import Path
import hashlib

root = Path(__file__).parents[2]
story = root / "_agile-output/implementation-artifacts/129-2-外设子系统与使用配方.md"
checklist = root / "_agile-output/test-artifacts/atdd-checklist-129-2-外设子系统与使用配方.md"
rust = root / "crates/bitloom/tests/fr198_peripheral_system.rs"
benches = [
    root / "crates/bitloom/tests/fr198_peripheral_system/axi_tb.sv",
    root / "crates/bitloom/tests/fr198_peripheral_system/direct_tb.sv",
]

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

for path in [story, checklist, rust, *benches]:
    require(path.is_file() and path.stat().st_size > 0, f"missing {path}")

combined = story.read_text() + checklist.read_text()
for number in range(1, 9):
    require(f"{number}." in combined, f"missing AC {number}")
for token in ["AXI", "CSR", "UART", "GPIO", "Timer", "IRQ", "0x0400", "SLVERR", "DECERR", "aresetn", "checkout"]:
    require(token in combined, f"missing contract {token}")

source = rust.read_text()
for token in [
    'any(|p| p.name == decoder_port)',
    '"uart_error_event"',
    'expect("required iverilog executable is missing")',
    'tool_identity("vvp", "-V")',
    'check -assert',
    'assertions >= 250',
    'transactions >= 60',
    'AXI_RESET_BOUNDARY',
    'DIRECT_RESET_BOUNDARY',
]:
    require(token in source, f"missing implementation guard: {token}")
require('s.assign_or("u_err_event", "u_err_event"' not in source, "UART error self-feedback returned")

for topology, path in zip(["axi", "direct"], benches):
    tb = path.read_text()
    require(f"topology={topology}" in tb, f"{topology}: missing counted PASS marker")
    require(tb.count("`CHECK(") >= 20, f"{topology}: too few external assertions")
    for token in ["$fatal", "$dumpfile", "WSTRB", "IRQ", "UART", "reset", "DECERR"]:
        require(token in tb, f"{topology}: missing {token} coverage")
    require('initial begin #20 rst=0; #100; $display' not in tb, f"{topology}: unconditional smoke PASS returned")

print("ATDD GREEN: 8 ACs, 2 behavioral benches, hard tool/structure/count/VCD guards")
print("rust_sha256", hashlib.sha256(rust.read_bytes()).hexdigest())
