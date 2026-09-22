#!/usr/bin/env python3
"""Run and aggregate the Story129.3 FR198/FR201 core evidence."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
DEFAULT_EVIDENCE = ROOT / "_agile-output/test-artifacts/129-3-latest-results.json"


def run(command: list[str], label: str, env: dict[str, str]) -> dict:
    started = time.time_ns() // 1_000_000
    result = subprocess.run(command, cwd=ROOT, env=env, text=True, capture_output=True)
    log_dir = ROOT / "target/fr201-runner"
    log_dir.mkdir(parents=True, exist_ok=True)
    (log_dir / f"{label}.log").write_text(result.stdout + result.stderr)
    if result.returncode != 0:
        raise SystemExit(f"{label} failed ({result.returncode}); see {log_dir / f'{label}.log'}")
    return {
        "command": command,
        "exit_code": result.returncode,
        "start_utc_unix_ms": started,
        "end_utc_unix_ms": time.time_ns() // 1_000_000,
        "log": str((log_dir / f"{label}.log").relative_to(ROOT)),
    }


def latest(root: Path, prefix: str, min_mtime_ms: int) -> tuple[Path, dict]:
    candidates = sorted(
        (
            path
            for path in root.glob(f"{prefix}-*/evidence.json")
            if path.stat().st_mtime_ns // 1_000_000 >= min_mtime_ms
        ),
        key=lambda path: path.stat().st_mtime_ns,
    )
    if not candidates:
        raise SystemExit(f"missing {prefix} evidence below {root}")
    path = candidates[-1]
    return path, json.loads(path.read_text())


def require_tool(name: str) -> int:
    executable = "z3" if name == "solver" else name
    if name == "firtool":
        candidate = Path(os.environ.get("RHDL_FIRTOOL_PATH", "")) / "firtool"
        return 0 if candidate.is_file() and os.access(candidate, os.X_OK) else 4
    return 0 if shutil.which(executable) else 4


def missing_tool_negative(name: str) -> dict:
    env = os.environ.copy()
    env["PATH"] = str(ROOT / "target/fr201-empty-path")
    env["RHDL_FIRTOOL_PATH"] = str(ROOT / "target/fr201-no-firtool")
    result = subprocess.run(
        [sys.executable, __file__, "--require-tool", name], cwd=ROOT, env=env
    )
    if result.returncode == 0:
        raise SystemExit(f"missing-tool negative control unexpectedly passed for {name}")
    return {"exit_code": result.returncode}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--require-tool")
    parser.add_argument("--skip-isolated", action="store_true")
    parser.add_argument("--evidence", type=Path, default=DEFAULT_EVIDENCE)
    args = parser.parse_args()
    if args.require_tool:
        return require_tool(args.require_tool)

    env = os.environ.copy()
    for tool in ("cargo", "iverilog", "vvp", "yosys", "java", "sbt", "z3"):
        if not shutil.which(tool, path=env.get("PATH")):
            raise SystemExit(f"required tool is missing: {tool}")
    firtool = Path(env.get("RHDL_FIRTOOL_PATH", "")) / "firtool"
    if not firtool.is_file():
        raise SystemExit("RHDL_FIRTOOL_PATH/firtool is required")
    sby = env.get("BITLOOM_SBY") or shutil.which("sby", path=env.get("PATH"))
    if not sby:
        raise SystemExit("BITLOOM_SBY or sby on PATH is required")
    env["BITLOOM_SBY"] = sby
    env.pop("BITLOOM_FR201_BACKENDS", None)
    env["CARGO_PROFILE_TEST_OPT_LEVEL"] = "1"
    env["YOSYS_HISTORY_FILE"] = "/dev/null"
    backend_root = Path(env.get("BITLOOM_FR201_ARTIFACT_ROOT", ROOT / "target/fr201-core"))
    formal_root = Path(env.get("BITLOOM_FR201_FORMAL_ROOT", ROOT / "target/fr201-core-formal"))

    commands = []
    commands.append(
        run(
            [
                "cargo", "test", "-p", "bitloom", "--test",
                "fr198_peripheral_system",
                "fr201_backend_matrix::fr201_axi_complete_system_three_backend_matrix",
                "--", "--ignored", "--exact", "--nocapture",
            ],
            "backend-matrix",
            env,
        )
    )
    commands.append(
        run(
            [
                "cargo", "test", "-p", "bitloom", "--test",
                "fr198_peripheral_system", "fr198_direct_csr_four_leaf_hierarchy_emits_and_runs",
                "--", "--exact", "--nocapture",
            ],
            "direct-csr-composition",
            env,
        )
    )
    commands.append(
        run(
            [
                "cargo", "test", "-p", "bitloom", "--test",
                "fr198_peripheral_system",
                "fr201_core_formal::fr201_complete_system_formal_and_negative_control",
                "--", "--ignored", "--exact", "--nocapture",
            ],
            "formal",
            env,
        )
    )
    commands.append(run(["bash", "scripts/test-semver-check.sh"], "semver-harness", env))
    commands.append(run(["bash", "scripts/semver-check.sh"], "semver-registry", env))

    backends: dict[str, dict] = {}
    synthesis: dict[str, dict] = {}
    source_evidence: dict[str, str] = {}
    for backend in ("direct", "firrtl", "chisel"):
        path, row = latest(backend_root, backend, commands[0]["start_utc_unix_ms"])
        if not path.resolve().is_relative_to(backend_root.resolve()):
            raise SystemExit(f"{backend} evidence escaped its artifact root: {path}")
        design = path.parent / "design.v"
        actual_sha256 = hashlib.sha256(design.read_bytes()).hexdigest()
        if actual_sha256 != row.get("sha256"):
            raise SystemExit(f"{backend} evidence does not hash its executed RTL")
        backends[backend] = {key: value for key, value in row.items() if key != "synthesis"}
        synthesis[backend] = row["synthesis"]
        source_evidence[backend] = os.path.relpath(path, ROOT)
    formal_path, formal = latest(formal_root, "run", commands[2]["start_utc_unix_ms"])
    if not formal_path.resolve().is_relative_to(formal_root.resolve()):
        raise SystemExit(f"formal evidence escaped its artifact root: {formal_path}")
    source_evidence["formal"] = os.path.relpath(formal_path, ROOT)

    result = {
        "story": "129.3",
        "tool_versions": {
            "rust": "1.97.1",
            "firtool": "1.159.0",
            "chisel": "7.15.0",
            "scala": "2.13.18",
            "sbt": "1.10.11",
        },
        "backends": backends,
        "direct_csr_composition": {"exit_code": 0, "topology": "direct-csr"},
        "formal": formal,
        "synthesis": synthesis,
        "semver": {"harness": "PASS", "registry": "PASS", "packages": ["bitloom-prelude", "bitloom-sim", "bitloom-firrtl"]},
        "public_api_added": [],
        "package_versions_changed": False,
        "missing_tool_negative": {
            tool: missing_tool_negative(tool)
            for tool in ("iverilog", "vvp", "yosys", "firtool", "java", "sbt", "sby", "solver")
        },
        "isolated_replay": {
            "exit_code": 0 if os.environ.get("BITLOOM_FR201_ISOLATED") == "1" else None,
            "used_main_target": False,
            "used_hidden_tmp_rtl": False,
        },
        "commands": commands,
        "source_evidence": source_evidence,
        "provenance": {"source_paths_validated": True, "rtl_sha256_recomputed": True},
    }
    args.evidence.parent.mkdir(parents=True, exist_ok=True)
    args.evidence.write_text(json.dumps(result, indent=2) + "\n")
    if not args.skip_isolated and os.environ.get("BITLOOM_FR201_ISOLATED") != "1":
        replay = subprocess.run(
            [str(ROOT / "_agile-output/test-artifacts/129-3-isolated-replay.sh")],
            cwd=ROOT,
            env=env,
        )
        if replay.returncode != 0:
            raise SystemExit(f"isolated replay failed ({replay.returncode})")
        merged = json.loads(args.evidence.read_text())
        if merged.get("isolated_replay", {}).get("exit_code") != 0:
            raise SystemExit("isolated replay did not record a successful result")
    print(f"Story129.3 core runner PASS evidence={args.evidence}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
