#!/usr/bin/env python3
"""Independent FR200 evidence consumer and falsifiability tests (no network).

This validates recorded evidence consistency, not cryptographic authenticity.
Actual RTL and namespace execution remain obligations of the pilot producer.
"""
from __future__ import annotations

import argparse
import copy
import hashlib
import json
from pathlib import Path
import re
import sys

SHA256 = re.compile(r"[0-9a-f]{64}")
RECORDS = ("offline.json", "offline-binding.json", "offline-behavior.json", "offline-isolation.json")
IDENTITY_FIELDS = ("name", "url", "ref", "tagType", "tagObject", "commit", "closureDigest", "treeDigest", "cachePath")


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def load(work: Path, mode: str) -> dict:
    evidence = work / ("evidence-optimized" if mode == "optimized" else "evidence-normal")
    manifest_bytes = (work / "source.json").read_bytes()
    lock_bytes = (work / "source.lock.json").read_bytes()
    return {"manifest": json.loads(manifest_bytes), "lock": json.loads(lock_bytes),
            "manifestSha256": digest(manifest_bytes), "lockSha256": digest(lock_bytes),
            "binding": json.loads((evidence / "offline-binding-output.json").read_text()),
            "records": {name: json.loads((evidence / name).read_text()) for name in RECORDS}}


def validate_mounts(command: list[str]) -> None:
    require(command and Path(command[0]).name == "bwrap", "isolation command missing")
    require(all(flag in command for flag in ("--unshare-net", "--clearenv")), "network/environment isolation missing")
    mounts = {}
    for index, token in enumerate(command):
        if token in ("--ro-bind", "--bind"):
            require(index + 2 < len(command), "incomplete isolation mount")
            source, destination = command[index + 1:index + 3]
            require(source != "/" and destination in {"/usr", "/bin", "/lib", "/lib64", "/sbin", "/input", "/cache", "/scratch"}, "host root exposed")
            require(destination not in mounts, "duplicate isolation mount")
            mounts[destination] = (token, source)
    require(all(mounts.get(p, (None,))[0] == "--ro-bind" for p in ("/input", "/cache")), "copied inputs/cache must be read-only")
    require(mounts.get("/scratch", (None,))[0] == "--bind", "writable scratch missing")
    require(mounts["/input"][1] != mounts["/cache"][1], "input and cache mounts overlap")


def validate(bundle: dict) -> int:
    intent, lock, binding = bundle["manifest"], bundle["lock"], bundle["binding"]
    require(lock["manifestSha256"] == bundle["manifestSha256"], "manifest identity mismatch")
    source = next(s for s in lock["sources"] if s["name"] == "common_cells")
    rtl = next(f for f in source["files"] if f["path"] == "src/fifo_v3.sv")
    require(binding["upstream"] == {"name": intent["name"], "url": source["url"], "ref": source["ref"],
            "commit": source["commit"], "sourcePath": "common_cells:src/fifo_v3.sv", "sourceSha256": rtl["sha256"]}, "bound source identity differs")
    require(binding["module"] == lock["compile"]["top"] == "fifo_v3", "bound module differs")
    for key in ("parameters", "ports", "clockReset"):
        require(binding[key] == lock["compile"][key], f"bound {key} differs")
    wrapper = binding["wrapper"]
    require(wrapper.get("name") == "BitloomExternalFifo" and wrapper.get("version") == 1, "wrapper identity missing")
    verilog = wrapper.get("verilog", "")
    require(bool(verilog.strip()) and wrapper.get("sha256") == digest(verilog.encode()), "wrapper content/hash missing or changed")
    require(verilog.count("module BitloomExternalFifo (") == 1 and verilog.count("endmodule") == 1
            and verilog.count("  fifo_v3 fifo (") == 1 and "assign " not in verilog, "wrapper is not the composed parent")
    for port in lock["compile"]["ports"]:
        require(verilog.count(f'.{port["name"]}({port["name"]})') == 1, "wrapper port connection missing")
    require(wrapper.get("parameterBinding") == "fixed upstream defaults; no backend parameter overrides", "fixed parameter boundary missing")
    require(binding.get("supportLevel") == "locked", "binding support claim drift")
    tools = lock["tools"]
    for tool in ("bitloom", "git", "yosys", "timeout", "iverilog", "vvp"):
        require(bool(tools.get(tool)) and bool(tools.get(tool + "Path"))
                and SHA256.fullmatch(tools.get(tool + "Sha256", "")) is not None, f"{tool} executable identity missing")
    runtime = tools.get("iverilogRuntimeSha256", {})
    base = tools.get("iverilogBase", "")
    require(bool(base) and bool(runtime), "compiler helper identity missing")
    for name in ("ivl", "ivlpp", "vvp.tgt", "vvp.conf"):
        require(SHA256.fullmatch(runtime.get(str(Path(base) / name), "")) is not None, "compiler helper identity missing")
    require(any(name.endswith("system.vpi") for name in runtime), "simulator runtime identity missing")
    expected_sources = [{key: s[key] for key in IDENTITY_FIELDS} for s in lock["sources"]]
    for name in RECORDS:
        record = bundle["records"][name]
        require(record.get("exitCode") == 0, f"{name}: unsuccessful execution")
        require(isinstance(record.get("durationSeconds"), (float, int)) and record["durationSeconds"] > 0, f"{name}: execution duration missing")
        require(bool(record.get("startedUtc")), f"{name}: execution timestamp missing")
        require(not re.search(r"\b(?:FATAL|ERROR|UNKNOWN|TIMEOUT)\b", record.get("stdout", "") + record.get("stderr", "")), f"{name}: failure hidden by success marker")
        validate_mounts(record.get("command", []))
        if name != "offline-isolation.json":
            action = {"offline.json": "replay", "offline-binding.json": "binding", "offline-behavior.json": "behavior"}[name]
            require(record.get("stage") == "network-isolated-" + action, "wrong evidence stage")
            require(record.get("manifestSha256") == bundle["manifestSha256"] and record.get("lockSha256") == bundle["lockSha256"], "record input hashes differ")
            require(record.get("sources") == expected_sources, "record source closure differs")
            require(record.get("tools") == tools, "record tool identity differs")
            command = record["command"]
            for flag, value in (("--manifest", "/input/source.json"), ("--lock", "/input/source.lock.json"), ("--cache", "/cache")):
                require(flag in command and command[command.index(flag) + 1] == value, "isolated input argument missing")
            require(["/input/cargo-bitloom", "external-ip", action] == command[command.index("/input/cargo-bitloom"):command.index("/input/cargo-bitloom") + 3], "wrong isolated executable/action")
            environment = record.get("isolatedEnvironment", {})
            require(environment.get("TMPDIR") == "/scratch" and environment.get("BITLOOM_NETWORK_ISOLATED") == "1", "isolated environment missing")
    replay = bundle["records"]["offline.json"]
    require("hdl_compile=passed" in replay["stdout"].splitlines() and "network=denied" in replay["stdout"].splitlines(), "real compile/network result missing")
    namespaces = re.findall(r"^network_namespace=(net:\[\d+\])->(net:\[\d+\])$", replay["stdout"], re.M)
    require(len(namespaces) == 1 and namespaces[0][0] != namespaces[0][1], "distinct network namespace result missing")
    require(f'adapter_input_sha256={rtl["sha256"]}' in replay["stdout"].splitlines(), "compiled source hash differs")
    behavior = bundle["records"]["offline-behavior.json"]["stdout"]
    require(f'behavior=passed rtl_sha256={rtl["sha256"]} wrapper_sha256={wrapper["sha256"]} simulator=iverilog' in behavior.splitlines(), "behavior source/wrapper identity missing")
    results = re.findall(r"^FIFO_MODEL_PASS cycles=(\d+) depth=8 width=32 populated_reset=1 flush=1 wrapper=BitloomExternalFifo$", behavior, re.M)
    require(len(results) == 1 and int(results[0]) >= 86, "nonzero complete model checks missing")
    isolation = bundle["records"]["offline-isolation.json"]
    require(isolation["stdout"].strip() == "ISOLATION_PASS filesystem=denied network=denied", "isolation probe result missing")
    probe_command = isolation["command"]
    require("-c" in probe_command, "host path probe command missing")
    targets = probe_command[probe_command.index("-c") + 2:]
    require(len(targets) == 5 and all(Path(p).is_absolute() for p in targets), "host path probe targets missing")
    require(targets[1] == replay["cacheRoot"] and targets[2].endswith(".json"), "host path probe cache/manifest differs")
    require(Path(targets[4]).parent == Path(targets[3]) and Path(targets[3]).name.startswith("bitloom-host-only-"), "host sentinel must be separate directory/file")
    require(all(not Path(targets[3]).is_relative_to(Path(p)) for p in targets[:3]), "host sentinel must not write protected inputs")
    integrity = isolation.get("cacheIntegrity", {})
    before = integrity.get("originalBefore", "")
    require(SHA256.fullmatch(before) is not None and integrity.get("originalAfter") == before
            and integrity.get("copyAfter") == before, "read-only cache integrity missing")
    return int(results[0])


def self_test(bundle: dict) -> list[dict]:
    """[P0] Given real evidence, when one claim is corrupted, reject it."""
    def remove_input_mount(changed: dict) -> None:
        command = changed["records"]["offline-behavior.json"]["command"]
        index = command.index("/input")
        del command[index - 2:index + 1]

    mutations = [
        ("zero-checks", lambda b: b["records"]["offline-behavior.json"].update(stdout=re.sub(r"cycles=\d+", "cycles=0", b["records"]["offline-behavior.json"]["stdout"])), "model checks"),
        ("marker-only", lambda b: b["records"]["offline-behavior.json"].update(stdout="FIFO_MODEL_PASS\n"), "behavior source/wrapper"),
        ("forged-success", lambda b: b["records"]["offline-behavior.json"].update(exitCode=1), "unsuccessful execution"),
        ("fatal-with-pass", lambda b: b["records"]["offline-behavior.json"].update(stderr="FATAL mismatch"), "failure hidden"),
        ("missing-simulator", lambda b: b["lock"]["tools"].pop("vvpSha256"), "executable identity"),
        ("missing-helper", lambda b: b["lock"]["tools"].update(iverilogRuntimeSha256={}), "compiler helper"),
        ("missing-wrapper", lambda b: b["binding"]["wrapper"].update(verilog="", sha256=digest(b"")), "wrapper content"),
        ("wrapper-hash", lambda b: b["binding"]["wrapper"].update(sha256="0" * 64), "wrapper content"),
        ("source-hash", lambda b: b["binding"]["upstream"].update(sourceSha256="0" * 64), "source identity"),
        ("missing-source", lambda b: b["records"]["offline-behavior.json"].update(sources=[]), "source closure"),
        ("missing-identity", lambda b: b["records"]["offline-behavior.json"].update(tools={}), "tool identity"),
        ("missing-isolated-input", remove_input_mount, "copied inputs"),
        ("host-root-visible", lambda b: b["records"]["offline-behavior.json"]["command"].extend(["--ro-bind", "/", "/"]), "host root"),
        ("missing-network-isolation", lambda b: b["records"]["offline-behavior.json"]["command"].remove("--unshare-net"), "network/environment"),
        ("cache-modified", lambda b: b["records"]["offline-isolation.json"]["cacheIntegrity"].update(copyAfter="0" * 64), "cache integrity"),
        ("no-probe", lambda b: b["records"]["offline-isolation.json"].update(stdout=""), "isolation probe"),
        ("missing-host-targets", lambda b: b["records"]["offline-isolation.json"]["command"].pop(), "host path probe targets"),
        ("zero-duration", lambda b: b["records"]["offline-behavior.json"].update(durationSeconds=0), "duration missing"),
    ]
    validate(bundle)  # A missing prerequisite must never masquerade as a killed mutant.
    results = []
    for name, mutate, diagnostic in mutations:
        changed = copy.deepcopy(bundle)
        mutate(changed)
        try:
            validate(changed)
        except ValueError as error:
            require(diagnostic in str(error), f"{name}: rejected for wrong reason: {error}")
            results.append({"name": name, "status": "rejected", "diagnostic": str(error)})
        else:
            raise ValueError(f"{name}: invalid evidence accepted")
    return results


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--mode", choices=("normal", "optimized"), default="normal")
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()
    bundle = load(args.work, args.mode)
    cycles = validate(bundle)
    tests = self_test(bundle) if args.self_test else []
    report = {"status": "passed", "pythonOptimize": sys.flags.optimize, "evidenceMode": args.mode,
              "work": str(args.work.resolve()), "modelCycles": cycles,
              "positiveBaselines": 1, "mutationsRejected": len(tests), "tests": tests,
              "manifestSha256": bundle["manifestSha256"], "lockSha256": bundle["lockSha256"]}
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
