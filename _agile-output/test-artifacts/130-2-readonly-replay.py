#!/usr/bin/env python3
"""Execute FR199 replay with the original checkout and fixture mounted read-only."""

import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import time

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--root", type=Path, required=True)
parser.add_argument("--fixture", type=Path, required=True)
parser.add_argument("--evidence", type=Path, required=True)
args = parser.parse_args()
repo = args.root.resolve()
fixture = args.fixture.resolve()
scratch = Path(tempfile.mkdtemp(prefix="bitloom-fr199-readonly-proof-"))
command = ["bwrap", "--die-with-parent", "--ro-bind", "/usr", "/usr"]
for name in ("/bin", "/lib", "/lib64", "/sbin"):
    runtime = Path(name)
    if runtime.is_symlink():
        command += ["--symlink", os.readlink(runtime), name]
    elif runtime.exists():
        command += ["--ro-bind", name, name]
command += [
    "--proc", "/proc", "--dev", "/dev", "--tmpfs", "/tmp",
    "--ro-bind", str(repo), str(repo),
    "--ro-bind", str(fixture), str(fixture),
    "--bind", str(scratch), "/scratch", "--chdir", "/scratch",
    "--clearenv", "--setenv", "PATH", "/usr/bin:/bin",
    "--setenv", "TMPDIR", "/scratch",
    "--setenv", "BITLOOM_BIN", str(repo / "target/debug/cargo-bitloom"),
    "/usr/bin/python3", str(repo / "scripts/phase24-external-ip-replay.py"),
    "replay", "--compile", "--manifest", str(fixture / "source.json"),
    "--lock", str(fixture / "source.lock.json"),
    "--cache", str(fixture / "cache"), "--evidence", "/scratch/read-only.json",
]
started = time.time()
try:
    result = subprocess.run(command, text=True, capture_output=True, timeout=180)
except subprocess.TimeoutExpired as error:
    result = subprocess.CompletedProcess(
        command, 124, str(error.stdout or ""),
        "timeout after 180 seconds: " + str(error.stderr or ""),
    )
record = {
    "command": command,
    "startedUtc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(started)),
    "exitCode": result.returncode,
    "stdout": result.stdout,
    "stderr": result.stderr,
    "scratch": str(scratch),
    "durationSeconds": time.time() - started,
}
args.evidence.parent.mkdir(parents=True, exist_ok=True)
args.evidence.write_text(json.dumps(record, indent=2) + "\n")
for path in scratch.glob("*.json"):
    shutil.copy2(path, args.evidence.parent / ("readonly-inner-" + path.name))
print(json.dumps({"exitCode": result.returncode, "stderr": result.stderr, "scratch": str(scratch)}))
raise SystemExit(result.returncode)
