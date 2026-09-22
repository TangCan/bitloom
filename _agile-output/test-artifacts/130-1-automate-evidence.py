#!/usr/bin/env python3
"""Validate Story130.1 probe evidence and prove the validator can fail."""

from __future__ import annotations

import argparse
import copy
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import re
import shutil
import sys
import tempfile
from typing import Callable


ROOT = Path(__file__).resolve().parents[2]
ARTIFACTS = ROOT / "_agile-output/test-artifacts"
PROBE = ARTIFACTS / "130-1-build-probe.py"
BLACKBOX = ROOT / "crates/bitloom-prelude/src/ip/blackbox.rs"
NORMAL = ARTIFACTS / "130-1-build-probe-normal/results.json"
OPTIMIZED = ARTIFACTS / "130-1-build-probe-opt/results.json"
EXPECTED_LABELS = [
    "version-git",
    "version-sha256sum",
    "version-bwrap",
    "version-unshare",
    "version-iverilog",
    "version-vvp",
    "version-yosys",
    "sha256-blackbox",
    "network-denial-unshare",
    "git-head",
    "git-recursive-closure",
    "git-archive",
    "network-denial-bwrap",
    "rust-blackbox-contract",
    "rtl-compile",
    "rtl-simulate",
    "rtl-synthesis",
]
REQUIRED_TOOLS = {"git", "sha256sum", "bwrap", "unshare", "iverilog", "vvp", "yosys"}
FORBIDDEN_CLAIM_KEYS = {"fr199_delivered", "fr200_delivered", "support_level", "external_support_promoted"}
VENDOR_LITERAL_SHA256 = "eb2d36f53786cfa3c7d184a5370766a6aaedad358564ed4ff4f3223a8b48e29b"


class EvidenceError(RuntimeError):
    """The archived evidence does not satisfy the Story130.1 contract."""


def require(condition: bool, message: str) -> None:
    if not condition:
        raise EvidenceError(message)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def command(data: dict[str, object], label: str) -> dict[str, object]:
    commands = data["commands"]
    require(isinstance(commands, list), "commands must be a list")
    matches = [entry for entry in commands if isinstance(entry, dict) and entry.get("label") == label]
    require(len(matches) == 1, f"expected exactly one command label: {label}")
    return matches[0]


def validate(result_path: Path, expected_optimize: int) -> None:
    data = json.loads(result_path.read_text(encoding="utf-8"))
    require(data.get("success") is True, "report success must be true")
    require(data.get("optimize") == expected_optimize, "python optimization mode mismatch")
    require(data.get("script_sha256") == sha256(PROBE), "probe script digest drift")

    source = data.get("source")
    require(isinstance(source, dict), "source identity missing")
    require(source.get("path") == "crates/bitloom-prelude/src/ip/blackbox.rs", "blackbox source path drift")
    require(source.get("sha256") == sha256(BLACKBOX), "blackbox source digest drift")
    require(source.get("vendor_literal_sha256") == VENDOR_LITERAL_SHA256, "vendor literal digest drift")

    commands = data.get("commands")
    require(isinstance(commands, list), "commands must be a list")
    labels = [entry.get("label") if isinstance(entry, dict) else None for entry in commands]
    require(labels == EXPECTED_LABELS, "command ledger is incomplete, duplicated, or reordered")
    for entry in commands:
        require(isinstance(entry, dict), "command entry must be an object")
        label = entry.get("label")
        argv = entry.get("argv")
        require(isinstance(argv, list) and argv and all(isinstance(arg, str) for arg in argv), f"{label}: argv missing")
        require(isinstance(entry.get("cwd"), str) and entry.get("cwd"), f"{label}: cwd missing")
        require(isinstance(entry.get("started_utc"), str) and entry.get("started_utc"), f"{label}: UTC start missing")
        require(isinstance(entry.get("duration_seconds"), (int, float)), f"{label}: duration missing")
        returncode = entry.get("returncode")
        require(isinstance(returncode, int), f"{label}: return code missing")
        accepted = {0, 1} if label == "network-denial-unshare" else {0}
        require(returncode in accepted, f"{label}: unaccepted return code {returncode}")
        log_name = entry.get("log")
        require(isinstance(log_name, str) and Path(log_name).name == log_name, f"{label}: unsafe log path")
        log_path = result_path.parent / log_name
        require(log_path.is_file(), f"{label}: log missing")
        require(entry.get("log_sha256") == sha256(log_path), f"{label}: log digest drift")

    tools = data.get("tools")
    require(isinstance(tools, dict) and set(tools) == REQUIRED_TOOLS, "tool identity set drift")
    for name, identity in tools.items():
        require(isinstance(identity, dict), f"{name}: tool identity missing")
        require(isinstance(identity.get("path"), str) and identity.get("path"), f"{name}: tool path missing")
        require(re.fullmatch(r"[0-9a-f]{64}", str(identity.get("sha256"))) is not None, f"{name}: tool digest invalid")

    unshare = command(data, "network-denial-unshare")
    unshare_log = (result_path.parent / str(unshare["log"])).read_text(encoding="utf-8")
    if unshare["returncode"] == 0:
        require(data.get("unshare_net_supported") is True, "successful unshare was not recorded")
    else:
        require(data.get("unshare_net_supported") is False, "failed unshare was recorded as supported")
        require("Operation not permitted" in unshare_log, "unshare failure reason was not preserved")

    bwrap = command(data, "network-denial-bwrap")
    bwrap_log = (result_path.parent / str(bwrap["log"])).read_text(encoding="utf-8").strip()
    require(bwrap_log == "[(1, 'lo')]", "bwrap namespace was not loopback-only")

    rust = command(data, "rust-blackbox-contract")
    rust_argv = rust["argv"]
    require(isinstance(rust_argv, list), "Rust contract argv missing")
    require("--locked" in rust_argv and "--offline" in rust_argv, "Rust contract was not locked and offline")

    simulation = command(data, "rtl-simulate")
    simulation_log = (result_path.parent / str(simulation["log"])).read_text(encoding="utf-8")
    require("BLACKBOX BINDING MECHANISM PASS behavior=absent" in simulation_log, "empty blackbox behavior marker drift")
    require("behavior=tested" not in simulation_log, "empty blackbox was promoted to behavior evidence")

    limitations = data.get("limitations")
    require(isinstance(limitations, list), "limitations missing")
    joined = "\n".join(str(item) for item in limitations)
    require("no behavioral assignment" in joined, "missing no-behavior limitation")
    require("not promote" in joined and "support-matrix" in joined, "missing support-promotion prohibition")
    require("No upstream source" in joined and "offline replay" in joined, "missing FR199 limitation")
    require(FORBIDDEN_CLAIM_KEYS.isdisjoint(data), "forbidden FR199/FR200/support claim present")

    repository = data.get("repository_identity")
    require(isinstance(repository, dict), "repository identity missing")
    require(re.fullmatch(r"[0-9a-f]{40}", str(repository.get("commit"))) is not None, "repository commit invalid")
    require(re.fullmatch(r"[0-9a-f]{64}", str(repository.get("archive_sha256"))) is not None, "archive digest invalid")

    rtl_sources = data.get("rtl_sources")
    require(isinstance(rtl_sources, dict), "RTL source identities missing")
    for name in ("vendor_ext_ip.v", "tb.sv"):
        rtl_path = result_path.parent / name
        require(rtl_path.is_file(), f"{name}: archived RTL source missing")
        require(rtl_sources.get(name) == sha256(rtl_path), f"{name}: archived RTL digest drift")


Mutation = Callable[[Path, dict[str, object]], None]


def mutate_source_digest(_: Path, data: dict[str, object]) -> None:
    source = data["source"]
    require(isinstance(source, dict), "test fixture source missing")
    source["sha256"] = "0" * 64


def mutate_script_digest(_: Path, data: dict[str, object]) -> None:
    data["script_sha256"] = "0" * 64


def mutate_empty_commands(_: Path, data: dict[str, object]) -> None:
    data["commands"] = []


def mutate_nonzero_exit(_: Path, data: dict[str, object]) -> None:
    command(data, "rtl-compile")["returncode"] = 9


def mutate_missing_log(directory: Path, data: dict[str, object]) -> None:
    entry = command(data, "rtl-synthesis")
    (directory / str(entry["log"])).unlink()


def mutate_log_digest(_: Path, data: dict[str, object]) -> None:
    command(data, "git-head")["log_sha256"] = "0" * 64


def mutate_network_claim(directory: Path, data: dict[str, object]) -> None:
    entry = command(data, "network-denial-bwrap")
    log = directory / str(entry["log"])
    log.write_text("[(1, 'lo'), (2, 'eth0')]\n", encoding="utf-8")
    entry["log_sha256"] = sha256(log)


def mutate_behavior_claim(directory: Path, data: dict[str, object]) -> None:
    entry = command(data, "rtl-simulate")
    log = directory / str(entry["log"])
    log.write_text("BLACKBOX BINDING MECHANISM PASS behavior=tested\n", encoding="utf-8")
    entry["log_sha256"] = sha256(log)


def mutate_support_claim(_: Path, data: dict[str, object]) -> None:
    data["fr200_delivered"] = True
    data["support_level"] = "behavior-tested"


MUTATIONS: list[tuple[str, Mutation, str]] = [
    ("source-digest-drift", mutate_source_digest, "blackbox source digest drift"),
    ("script-digest-drift", mutate_script_digest, "probe script digest drift"),
    ("empty-command-ledger", mutate_empty_commands, "command ledger"),
    ("hidden-command-failure", mutate_nonzero_exit, "unaccepted return code"),
    ("missing-command-log", mutate_missing_log, "log missing"),
    ("command-log-digest-drift", mutate_log_digest, "log digest drift"),
    ("forged-network-isolation", mutate_network_claim, "loopback-only"),
    ("empty-blackbox-promotion", mutate_behavior_claim, "behavior marker drift"),
    ("support-level-promotion", mutate_support_claim, "forbidden FR199/FR200/support claim"),
]


def exercise_mutation(source_result: Path, expected_optimize: int, name: str, mutation: Mutation, expected: str) -> None:
    with tempfile.TemporaryDirectory(prefix=f"bitloom-130-1-{name}-") as raw:
        directory = Path(raw) / "evidence"
        shutil.copytree(source_result.parent, directory)
        result_path = directory / "results.json"
        data = json.loads(result_path.read_text(encoding="utf-8"))
        mutation(directory, data)
        result_path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")
        try:
            validate(result_path, expected_optimize)
        except EvidenceError as exc:
            require(expected in str(exc), f"{name}: wrong rejection: {exc}")
            return
        raise EvidenceError(f"{name}: mutation was accepted")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    report: dict[str, object] = {
        "story": "130.1",
        "started_utc": datetime.now(timezone.utc).isoformat(),
        "python_optimize": sys.flags.optimize,
        "scenarios": [],
        "success": False,
    }
    scenarios = report["scenarios"]
    require(isinstance(scenarios, list), "internal scenario list error")
    try:
        for label, path, optimize in (("normal-baseline", NORMAL, 0), ("optimized-baseline", OPTIMIZED, 1)):
            validate(path, optimize)
            scenarios.append({"name": label, "expected": "PASS", "observed": "PASS"})
        for name, mutation, expected in MUTATIONS:
            exercise_mutation(NORMAL, 0, name, mutation, expected)
            scenarios.append({"name": name, "expected": "REJECT", "observed": "REJECT"})
        report["success"] = True
        print(f"PASS: Story130.1 evidence automation {len(scenarios)}/{len(scenarios)} scenarios")
    except Exception as exc:
        report["error"] = f"{type(exc).__name__}: {exc}"
        print(report["error"], file=sys.stderr)
    finally:
        report["finished_utc"] = datetime.now(timezone.utc).isoformat()
        args.output.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return 0 if report["success"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
