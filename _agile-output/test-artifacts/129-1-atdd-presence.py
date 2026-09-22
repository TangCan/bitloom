#!/usr/bin/env python3
"""Only check Story129.1 NFR14 document presence; not content or RTL behavior."""
from pathlib import Path
import sys

root = Path(__file__).resolve().parents[2]
target = root / "_agile-output/implementation-artifacts/epic-129-nfr14.md"
if not target.is_file():
    print(f"RED: missing NFR14 document: {target}")
    print("Presence only: no content-quality, FR198 behavior or FR201 conclusion.")
    sys.exit(1)
print(f"PRESENT: {target}")
print("Presence only: manual content review and actual probe evidence remain required.")
sys.exit(0)
