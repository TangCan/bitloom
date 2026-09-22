#!/usr/bin/env python3
"""[P0] Replay real RTL baseline and reset-wiring negative controls.

Tests the archived two-child probe's detection ability, not FR198 system delivery.
Only boundary wiring is mutated; the generated RTL and testbench are unchanged.
No asserts: checks remain effective under python -O.
"""
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import time

ARCHIVE_SHA = "671aabb8561062f0876ab60ed8c1754a2512dbb8e8152ee1e8a860906bc858e9"
MEMBERS = {
    "direct.v": "run/adapted/direct.v",
    "boundary.v": "run/adapted/direct-run/boundary.v",
    "ports.v": "run/adapted/direct-run/ports.v",
    "tb.sv": "run/adapted/direct-run/tb.sv",
}
PASS = "RESET PROBE PASS children=2 synchronous=1 priority=1 recovery=1 seed=N/A"
CASES = (
    ("baseline", "~aresetn", None),
    ("reset_disconnected", "1'b0", "aresetn failed common synchronous clear / write priority"),
    ("reset_wrong_polarity", "aresetn", "nonzero setup failed"),
)


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(condition: bool, message: str) -> None:
    if not condition:
        raise RuntimeError(message)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--archive", type=Path, default=Path(__file__).with_name("129-1-code-review-reset-evidence.tar.gz"))
    parser.add_argument("--output-dir", type=Path, required=True)
    args = parser.parse_args()
    output = args.output_dir.resolve()
    output.mkdir(parents=True, exist_ok=False)
    report = {"scope": "two-child reset probe TB sensitivity; not FR198", "priority": "P0", "python": sys.version, "optimize": sys.flags.optimize, "script_sha256": sha(Path(__file__)), "output_directory": str(output), "archive": str(args.archive.resolve()), "commands": [], "cases": [], "success": False}

    def run(argv: list[str], cwd: Path, label: str) -> subprocess.CompletedProcess[str]:
        started = time.monotonic()
        entry = {"argv": argv, "cwd": str(cwd), "started_utc": datetime.now(timezone.utc).isoformat()}
        report["commands"].append(entry)
        log = ""
        try:
            result = subprocess.run(argv, cwd=cwd, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=30, check=False)
            log = result.stdout
            entry["returncode"] = result.returncode
        except subprocess.TimeoutExpired as exc:
            raw = exc.stdout or b""
            log = raw.decode(errors="replace") if isinstance(raw, bytes) else raw
            entry["timeout"] = True
            raise RuntimeError(f"{label}: timeout") from exc
        finally:
            entry["duration_seconds"] = time.monotonic() - started
            logfile = cwd / f"{label}.log"
            logfile.write_text(log)
            entry["log"] = str(logfile)
            entry["log_sha256"] = sha(logfile)
        return result

    try:
        require(args.archive.is_file(), "missing source archive")
        report["archive_sha256"] = sha(args.archive)
        require(report["archive_sha256"] == ARCHIVE_SHA, "source archive SHA mismatch")
        executables = {}
        for name in ("iverilog", "vvp"):
            path = shutil.which(name)
            require(path is not None, f"required tool missing: {name}")
            executables[name] = str(Path(path).resolve())
            version = run([executables[name], "-V"], output, f"{name}-version")
            require(version.returncode == 0, f"{name} version failed")
        report["tools"] = {name: {"path": path, "sha256": sha(Path(path))} for name, path in executables.items()}
        sources = {}
        with tarfile.open(args.archive, "r:gz") as archive:
            for filename, member in MEMBERS.items():
                info = archive.getmember(member)
                require(info.isfile(), f"not a regular file: {member}")
                stream = archive.extractfile(info)
                require(stream is not None, f"missing archive source: {member}")
                sources[filename] = stream.read().decode()
        report["source_sha256"] = {name: hashlib.sha256(value.encode()).hexdigest() for name, value in sources.items()}
        needle = "wire core_reset = ~aresetn;"
        require(sources["boundary.v"].count(needle) == 1, "mutation anchor must occur once")
        for name, expression, diagnostic in CASES:
            case_dir = output / name
            case_dir.mkdir()
            for filename, source in sources.items():
                if filename == "boundary.v":
                    source = source.replace(needle, f"wire core_reset = {expression};")
                (case_dir / filename).write_text(source)
            case = {"name": name, "priority": "P0", "reset_expression": expression, "expected_diagnostic": diagnostic, "files_sha256": {filename: sha(case_dir / filename) for filename in sources}}
            report["cases"].append(case)
            compiled = run([executables["iverilog"], "-g2012", "-s", "tb", "-o", "simulation", "direct.v", "ports.v", "boundary.v", "tb.sv"], case_dir, "compile")
            require(compiled.returncode == 0, f"{name}: compilation failed, not a valid behavioral control")
            simulation = run([executables["vvp"], "simulation"], case_dir, "simulation")
            case["simulation_returncode"] = simulation.returncode
            if diagnostic is None:
                require(simulation.returncode == 0 and PASS in simulation.stdout, "baseline did not pass")
            else:
                require(simulation.returncode > 0, f"{name}: mutant did not fail behaviorally")
                require(diagnostic in simulation.stdout and "FATAL:" in simulation.stdout, f"{name}: missing expected fatal diagnostic")
                require(PASS not in simulation.stdout, f"{name}: contradictory PASS")
            case["success"] = True
        report["success"] = True
        print("PASS: real RTL baseline and 2 behavioral reset negative controls")
    except Exception as exc:
        report["error"] = f"{type(exc).__name__}: {exc}"
        print(report["error"], file=sys.stderr)
    finally:
        (output / "results.json").write_text(json.dumps(report, indent=2) + "\n")
    return 0 if report["success"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
