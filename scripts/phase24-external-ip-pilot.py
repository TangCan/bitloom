#!/usr/bin/env python3
"""Provision and exercise the FR200 pilot. Every missing tool/failure is fatal."""
from __future__ import annotations

import argparse
import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("replay_runner", ROOT / "scripts/phase24-external-ip-replay.py")
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)


def require(condition: bool, message: str) -> None:
    if not condition:
        raise RuntimeError(message)


def execute(command: list[str], evidence: Path, expected: str | None = None) -> dict:
    started = time.time()
    try:
        p = subprocess.run(command, capture_output=True, text=True, timeout=660)
        record = {"command": command, "exitCode": p.returncode, "stdout": p.stdout, "stderr": p.stderr}
    except subprocess.TimeoutExpired as error:
        record = {"command": command, "exitCode": 124, "stdout": str(error.stdout or ""), "stderr": "timeout: " + str(error.stderr or "")}
    record["startedUtc"] = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(started))
    record["durationSeconds"] = time.time() - started
    evidence.write_text(json.dumps(record, indent=2) + "\n")
    if expected is None:
        require(record["exitCode"] == 0, f"command failed; see {evidence}")
    else:
        require(record["exitCode"] not in (0, 124) and expected in record["stderr"], f"mutation missed {expected}; see {evidence}")
    return record


def validate_binding(binding: dict, intent: dict, lock: dict) -> None:
    source = next(s for s in lock["sources"] if s["name"] == "common_cells")
    rtl = next(f for f in source["files"] if f["path"] == "src/fifo_v3.sv")
    expected = {
        "schemaVersion": 1, "module": lock["compile"]["top"],
        "parameters": lock["compile"]["parameters"], "ports": lock["compile"]["ports"],
        "clockReset": lock["compile"]["clockReset"], "supportLevel": "locked",
        "wrapper": binding.get("wrapper"),
        "adapter": {"name": "bitloom-yosys-sv-compat", "version": 1},
        "upstream": {"name": intent["name"], "url": source["url"], "ref": source["ref"],
                     "commit": source["commit"], "sourcePath": lock["compile"]["files"][0], "sourceSha256": rtl["sha256"]},
    }
    wrapper = binding.get("wrapper", {})
    require(wrapper.get("name") == "BitloomExternalFifo" and wrapper.get("version") == 1, "wrapper identity missing")
    verilog = wrapper.get("verilog", "")
    require(bool(verilog) and wrapper.get("sha256") == hashlib.sha256(verilog.encode()).hexdigest(), "wrapper empty or hash mismatch")
    require(wrapper.get("parameterBinding") == "fixed upstream defaults; no backend parameter overrides", "wrapper parameter contract missing")
    require(binding == expected, "binding differs from complete independently constructed locked descriptor")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", required=True, type=Path)
    parser.add_argument("--reuse", action="store_true", help="rerun all offline and mutation gates against an explicitly provisioned work directory")
    args = parser.parse_args()
    work = args.work.resolve()
    if not args.reuse:
        require(not work.exists(), f"fresh gate requires absent work directory: {work}")
        work.mkdir(parents=True)
    require(work.is_dir(), "missing explicitly provisioned work directory")
    evidence = work / ("evidence-optimized" if sys.flags.optimize else "evidence-normal")
    evidence.mkdir(exist_ok=True)
    for marker in ("BITLOOM_NETWORK_ISOLATED", "BITLOOM_HOST_NETNS"):
        os.environ.pop(marker, None)
    os.environ["BITLOOM_LOCKED_BINARY_PATH"] = "bitloom"
    manifest, lock, cache = work / "source.json", work / "source.lock.json", work / "cache"
    # Bind the entire run to one executable even if another workspace build
    # completes concurrently. Reuse intentionally keeps that recorded binary.
    frozen_binary = work / "cargo-bitloom"
    if not args.reuse:
        shutil.copy2(runner.binary(), frozen_binary)
    require(frozen_binary.is_file(), "missing provisioned CLI executable")
    os.environ["BITLOOM_BIN"] = str(frozen_binary)
    binary = str(frozen_binary)
    if not args.reuse:
        shutil.copy2(ROOT / "ip/external/pulp-common-cells-fifo-v3.source.json", manifest)
        execute([binary, "external-ip", "lock", "--manifest", str(manifest), "--lock", str(lock), "--cache", str(cache)], evidence / "fetch.json")
    intent, locked = json.loads(manifest.read_text()), json.loads(lock.read_text())
    base = [binary, "external-ip"]
    inputs = ["--manifest", str(manifest), "--lock", str(lock), "--cache", str(cache)]
    # Positive baseline before every negative gate prevents missing-cache/tool false positives.
    execute(base + ["binding"] + inputs + ["--out", str(work / "binding.json")], evidence / "baseline-binding.json")
    validate_binding(json.loads((work / "binding.json").read_text()), intent, locked)
    behavior = execute(base + ["behavior"] + inputs, evidence / "baseline-behavior.json")
    binding = json.loads((work / "binding.json").read_text())
    for marker in ("wrapper_sha256=" + binding["wrapper"]["sha256"], "populated_reset=1", "flush=1", "wrapper=BitloomExternalFifo"):
        require(marker in behavior["stdout"], f"behavior did not execute the bound wrapper/reset/flush: {marker}")
    replay_command = [sys.executable, str(ROOT / "scripts/phase24-external-ip-replay.py"), "replay", "--compile", "--pilot"] + inputs + ["--evidence", str(evidence / "offline.json")]
    execute(replay_command, evidence / "offline-command.json")
    for suffix, marker in [("", "hdl_compile=passed"), ("-binding", "binding="), ("-behavior", "FIFO_MODEL_PASS"), ("-isolation", "ISOLATION_PASS")]:
        record = json.loads((evidence / f"offline{suffix}.json").read_text())
        require(record["exitCode"] == 0 and marker in record["stdout"], f"offline {suffix} evidence missing")
    validate_binding(json.loads((evidence / "offline-binding-output.json").read_text()), intent, locked)

    manifest_bytes, lock_bytes = manifest.read_bytes(), lock.read_bytes()
    mutations = [
        ("depth", lambda c: c["parameters"].update(DEPTH="9")),
        ("width", lambda c: c["parameters"].update(DATA_WIDTH="16")),
        ("port-width", lambda c: c["ports"][7].update(width="1")),
        ("port-direction", lambda c: c["ports"][7].update(direction="output")),
        ("reset", lambda c: c["clockReset"].update(resetPolarity="active-high")),
        ("reset-kind", lambda c: c["clockReset"].update(resetKind="synchronous")),
        ("module", lambda c: c.update(top="empty_wrapper")),
    ]
    for name, mutate in mutations:
        changed, changed_lock = copy.deepcopy(intent), copy.deepcopy(locked)
        mutate(changed["compile"])
        changed_lock["compile"] = changed["compile"]
        data = (json.dumps(changed, indent=2) + "\n").encode()
        changed_lock["manifestSha256"] = hashlib.sha256(data).hexdigest()
        try:
            manifest.write_bytes(data)
            lock.write_text(json.dumps(changed_lock))
            execute(base + ["binding"] + inputs + ["--out", str(work / "rejected.json")], evidence / f"mutation-{name}.json", "bitloom.external-ip.compile-contract")
            require(not (work / "rejected.json").exists(), "invalid shape emitted binding")
        finally:
            manifest.write_bytes(manifest_bytes)
            lock.write_bytes(lock_bytes)
    for field in ("bitloomSha256", "yosysSha256", "iverilogSha256", "vvpSha256"):
        changed = copy.deepcopy(locked)
        require(field in changed["tools"], f"tool identity missing: {field}")
        changed["tools"][field] = "00" * 32
        try:
            lock.write_text(json.dumps(changed))
            execute(base + ["binding"] + inputs + ["--out", str(work / "rejected.json")], evidence / f"mutation-{field}.json", "bitloom.external-ip.tool-drift")
        finally:
            lock.write_bytes(lock_bytes)
    changed = copy.deepcopy(locked)
    runtime = changed["tools"].get("iverilogRuntimeSha256", {})
    require(bool(runtime), "real compiler helper identities missing")
    runtime[next(iter(runtime))] = "00" * 32
    try:
        lock.write_text(json.dumps(changed))
        execute(base + ["binding"] + inputs + ["--out", str(work / "rejected.json")], evidence / "mutation-iverilog-runtime.json", "bitloom.external-ip.tool-drift")
    finally:
        lock.write_bytes(lock_bytes)
    source = next(s for s in locked["sources"] if s["name"] == "common_cells")
    rtl = cache / source["cachePath"] / "src/fifo_v3.sv"
    original = rtl.read_bytes()
    try:
        rtl.write_text("module fifo_v3; endmodule\n")
        execute(base + ["behavior"] + inputs, evidence / "mutation-empty-rtl.json", "bitloom.external-ip.content-drift")
        rtl.unlink()
        execute(base + ["behavior"] + inputs, evidence / "mutation-missing-rtl.json", "bitloom.external-ip.undeclared-file")
    finally:
        rtl.write_bytes(original)
    extra = rtl.parent / "undeclared.sv"
    try:
        extra.write_text("module undeclared; endmodule\n")
        execute(base + ["binding"] + inputs + ["--out", str(work / "rejected.json")], evidence / "mutation-undeclared.json", "bitloom.external-ip.undeclared-file")
    finally:
        extra.unlink(missing_ok=True)
    license_path = lock.parent / "licenses/common_cells-LICENSE"
    license_bytes = license_path.read_bytes()
    try:
        license_path.write_text("license drift")
        execute(base + ["binding"] + inputs + ["--out", str(work / "rejected.json")], evidence / "mutation-license.json", "bitloom.external-ip.license-drift")
        license_path.unlink()
        license_path.symlink_to(rtl)
        execute(base + ["binding"] + inputs + ["--out", str(work / "rejected.json")], evidence / "mutation-license-link.json", "bitloom.external-ip.license-drift")
    finally:
        license_path.unlink(missing_ok=True)
        license_path.write_bytes(license_bytes)
    escape = rtl.parent / "escape"
    try:
        escape.symlink_to(manifest)
        execute(base + ["binding"] + inputs + ["--out", str(work / "rejected.json")], evidence / "mutation-cache-link.json", "bitloom.external-ip.path: cache symlink is forbidden")
    finally:
        escape.unlink(missing_ok=True)
    changed = copy.deepcopy(locked)
    changed["sources"][0]["cachePath"] = "../outside"
    try:
        lock.write_text(json.dumps(changed))
        execute(base + ["binding"] + inputs + ["--out", str(work / "rejected.json")], evidence / "mutation-cache-escape.json", "bitloom.external-ip.path")
    finally:
        lock.write_bytes(lock_bytes)
    for name, mutate, diagnostic in [
        ("source-hash", lambda x: x["sources"][0]["files"][0].update(sha256="00" * 32), "content-drift"),
        ("license-path", lambda x: x["sources"][0]["license"].update(archivePath="../outside-LICENSE"), "license-drift"),
    ]:
        changed = copy.deepcopy(locked)
        mutate(changed)
        try:
            lock.write_text(json.dumps(changed))
            execute(base + ["binding"] + inputs + ["--out", str(work / "rejected.json")], evidence / f"mutation-{name}.json", "bitloom.external-ip." + diagnostic)
        finally:
            lock.write_bytes(lock_bytes)
    execute(base + ["replay", "--compile"] + inputs, evidence / "mutation-host-network.json", "bitloom.external-ip.network")
    execute(base + ["binding"] + inputs + ["--out", str(work / "final-binding.json")], evidence / "final-baseline.json")
    require(manifest.read_bytes() == manifest_bytes and lock.read_bytes() == lock_bytes, "mutation modified baseline inputs")
    (evidence / "summary.json").write_text(json.dumps({"status": "passed", "pythonOptimize": sys.flags.optimize, "manifestSha256": runner.sha256(manifest), "lockSha256": runner.sha256(lock), "records": sorted(p.name for p in evidence.glob("*.json") if p.name != "summary.json")}, indent=2) + "\n")
    print(f"FR200_PILOT_PASS evidence={work}")


if __name__ == "__main__":
    main()
