#!/usr/bin/env python3
"""Story 130.1 RED check: the external-IP NFR14 record is not built yet."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
TARGET = ROOT / "_agile-output/implementation-artifacts/epic-130-nfr14.md"


if not TARGET.is_file():
    raise SystemExit(f"ATDD RED: missing NFR14 document: {TARGET.relative_to(ROOT)}")

print(f"PASS: NFR14 document exists: {TARGET.relative_to(ROOT)}")
