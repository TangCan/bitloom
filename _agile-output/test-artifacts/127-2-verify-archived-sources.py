#!/usr/bin/env python3
"""Verify immutable review evidence, resolving archived source paths by member."""
from pathlib import Path
import hashlib
import json
import tarfile

ROOT = Path(__file__).resolve().parents[2]
ARTIFACTS = ROOT / "_agile-output/test-artifacts"


def check(data, expected, label):
    if hashlib.sha256(data).hexdigest() != expected:
        raise RuntimeError(f"SHA256 mismatch: {label}")


def main():
    index = json.loads((ARTIFACTS / "127-2-review-source-snapshots-members.json").read_text())
    archive = ROOT / index["archive"]
    check(archive.read_bytes(), index["sha256"], archive)
    with tarfile.open(archive) as source:
        members = {}
        for name, digest in index["members"].items():
            data = source.extractfile(name).read()
            check(data, digest, name)
            members[name] = data
        count = 0
        for line in (ARTIFACTS / "127-2-review-fix-sha256.txt").read_text().splitlines():
            digest, name = line.split("  ", 1)
            data = members[name] if name in members else (ROOT / name).read_bytes()
            check(data, digest, name)
            count += 1
    print(f"PASS: {len(members)} archived source members; {count} original review evidence hashes")


if __name__ == "__main__":
    main()
