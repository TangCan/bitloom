#!/usr/bin/env python3
"""Independent Story130.2 manifest/lock/evidence contract consumer."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "ip/external/pulp-common-cells-fifo-v3.source.json"
LOCK = ROOT / "ip/external/pulp-common-cells-fifo-v3.source.lock.json"
ONLINE = ROOT / "_agile-output/test-artifacts/130-2-online-fetch.json"
OFFLINE = ROOT / "_agile-output/test-artifacts/130-2-offline-replay.json"
LICENSES = ROOT / "ip/external/licenses"
HEX = set("0123456789abcdef")
SOURCE_KEYS = (
    "name", "url", "ref", "tagType", "tagObject", "commit",
    "closureDigest", "treeDigest", "cachePath",
)
TOOL_KEYS = (
    "adapter", "adapterVersion", "bitloom", "bitloomPath", "bitloomSha256",
    "git", "gitPath", "gitSha256", "timeout", "timeoutPath", "timeoutSha256",
    "yosys", "yosysPath", "yosysSha256",
)


class ContractError(ValueError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ContractError(message)


def load(path: Path) -> dict:
    value = json.loads(path.read_text(encoding="utf-8"))
    require(isinstance(value, dict), f"{path}: root must be an object")
    return value


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def is_sha(value: object) -> bool:
    return isinstance(value, str) and len(value) == 64 and set(value) <= HEX


def source_projection(source: dict) -> dict:
    return {key: source[key] for key in SOURCE_KEYS}


def validate_isolation_command(command: list[str]) -> None:
    require(bool(command) and Path(command[0]).name == "bwrap", "bubblewrap executable")
    require(all(flag in command for flag in ("--unshare-net", "--unshare-pid", "--clearenv")), "isolated namespaces/environment")
    mounts = {}
    allowed = {"/usr", "/bin", "/lib", "/lib64", "/sbin", "/input", "/cache", "/scratch"}
    for index, item in enumerate(command):
        if item in {"--bind", "--ro-bind"}:
            require(index + 2 < len(command), "complete mount arguments")
            source, destination = command[index + 1:index + 3]
            require(source != "/" and destination in allowed, "no host root/extra mount")
            require(destination not in mounts, "unique mount destination")
            mounts[destination] = (item, source)
    for target in ("/input", "/cache"):
        require(mounts.get(target, (None,))[0] == "--ro-bind", "read-only copied input/cache")
    require(mounts.get("/scratch", (None,))[0] == "--bind", "writable scratch")
    require(mounts["/input"][1] != mounts["/cache"][1], "distinct copied mounts")


def validate(manifest_path: Path, lock_path: Path, online_path: Path, offline_path: Path) -> None:
    manifest = load(manifest_path)
    lock = load(lock_path)
    online = load(online_path)
    offline = load(offline_path)
    isolation = load(offline_path.with_name(offline_path.stem + "-isolation.json"))

    require(manifest.get("schemaVersion") == 1, "manifest schemaVersion")
    require(lock.get("schemaVersion") == 1, "lock schemaVersion")
    require(lock.get("manifestSha256") == sha256(manifest_path), "manifest digest binding")
    for key in ("name", "owner"):
        require(lock.get(key) == manifest.get(key), f"manifest/lock {key} binding")
    require(manifest.get("supportLevel") == "catalogued", "manifest support level")
    require(lock.get("supportLevel") == "locked", "lock support level")
    require(lock.get("dependencies") == manifest.get("dependencies"), "dependency binding")
    require(lock.get("compile") == manifest.get("compile"), "compile binding")
    require(lock.get("maintenance") == manifest.get("maintenance"), "maintenance binding")
    require(lock.get("generator") == {
        "kind": "none; upstream tracked SystemVerilog is consumed directly",
        "version": "none",
        "inputs": [],
    }, "generator contract")
    require(lock.get("binding") == {
        "kind": "source-only; no Bitloom wrapper in Story 130.2",
        "version": "none",
        "owner": manifest.get("owner"),
    }, "source-only binding contract")

    intents = manifest.get("sources")
    sources = lock.get("sources")
    require(isinstance(intents, list) and len(intents) == 3, "three source intents")
    require(isinstance(sources, list) and len(sources) == len(intents), "three source locks")
    closures = {source.get("closureDigest") for source in sources}
    require(len(closures) == 1 and all(is_sha(value) for value in closures), "shared closure digest")
    for intent, source in zip(intents, sources):
        for key in ("name", "sourceType", "url", "ref"):
            require(source.get(key) == intent.get(key), f"source intent {key} binding")
        for key in ("tagObject", "commit"):
            value = source.get(key)
            require(isinstance(value, str) and len(value) == 40 and set(value) <= HEX, f"source {key}")
        require(source.get("tagType") in {"lightweight", "annotated"}, "source tag type")
        require(is_sha(source.get("treeDigest")), "source tree digest")
        require(str(source.get("cachePath", "")).startswith("closures/v1-"), "source cache path")
        files = source.get("files")
        require(isinstance(files, list) and files, "source files")
        paths = [item.get("path") for item in files]
        require(paths == sorted(paths) and len(paths) == len(set(paths)), "ordered unique source files")
        require(all(is_sha(item.get("sha256")) for item in files), "source file digests")
        license_info = source.get("license", {})
        require(is_sha(license_info.get("sha256")), "license digest")
        archive = lock_path.parent / str(license_info.get("archivePath", ""))
        require(archive.is_file(), f"license archive exists: {archive}")
        require(sha256(archive) == license_info.get("sha256"), "license archive digest")
        require(isinstance(license_info.get("noticePaths"), list), "NOTICE facts")
        require(bool(license_info.get("copyrightAttributionFacts")), "attribution facts")
        require(bool(license_info.get("redistribution")), "redistribution facts")

    tools = lock.get("tools")
    require(isinstance(tools, dict), "locked tools")
    require(set(TOOL_KEYS) <= set(tools), "complete locked tool fields")
    require(tools.get("adapter") == "bitloom-yosys-sv-compat", "adapter name")
    require(tools.get("adapterVersion") == 1, "adapter version")
    for key in ("bitloomSha256", "gitSha256", "timeoutSha256", "yosysSha256"):
        require(is_sha(tools.get(key)), f"tool digest {key}")

    # FR199's source-only contract does not require simulation. If a later
    # lock includes simulator identities, partial identity sets still fail.
    if any(key.startswith(("iverilog", "vvp")) for key in tools):
        for key in ("iverilogSha256", "vvpSha256"):
            require(is_sha(tools.get(key)), f"tool digest {key}")
        runtime = tools.get("iverilogRuntimeSha256", {})
        require(bool(runtime) and bool(tools.get("iverilogBase")), "compiler helpers locked")
        for name in ("ivl", "ivlpp", "vvp.tgt", "vvp.conf"):
            require(is_sha(runtime.get(str(Path(tools["iverilogBase"]) / name))), "compiler helper digest")
    expected_sources = [source_projection(source) for source in sources]
    expected_manifest_sha = sha256(manifest_path)
    expected_lock_sha = sha256(lock_path)
    for evidence, stage in ((online, "lock"), (offline, "network-isolated-replay")):
        require(evidence.get("stage") == stage, f"evidence stage {stage}")
        require(evidence.get("exitCode") == 0, f"evidence exit {stage}")
        require(isinstance(evidence.get("command"), list) and evidence["command"], f"command {stage}")
        require(isinstance(evidence.get("startedUtc"), str) and evidence["startedUtc"].endswith("Z"), f"UTC {stage}")
        require(isinstance(evidence.get("durationSeconds"), (int, float)) and evidence["durationSeconds"] >= 0, f"duration {stage}")
        require(evidence.get("manifestSha256") == expected_manifest_sha, f"manifest evidence digest {stage}")
        require(evidence.get("lockSha256") == expected_lock_sha, f"lock evidence digest {stage}")
        require(evidence.get("sources") == expected_sources, f"source evidence {stage}")
        require(evidence.get("tools") == tools, f"tool evidence {stage}")

    isolated = offline.get("isolatedEnvironment")
    require(isinstance(isolated, dict), "isolated environment")
    require(isolated.get("GIT_CONFIG_NOSYSTEM") == "1", "Git system config isolation")
    require(isolated.get("GIT_CONFIG_GLOBAL") == "/dev/null", "Git global config isolation")
    require(isolated.get("BITLOOM_NETWORK_ISOLATED") == "1", "network isolation marker")
    require(isolated.get("HOME") == "/tmp/home", "isolated HOME")
    require(isolated.get("XDG_CACHE_HOME") == "/tmp/xdg-cache", "isolated XDG")
    require(isolated.get("TMPDIR") == "/scratch", "isolated scratch")
    validate_isolation_command(offline["command"])
    for flag, value in (("--manifest", "/input/source.json"), ("--lock", "/input/source.lock.json"), ("--cache", "/cache")):
        require(flag in offline["command"] and offline["command"][offline["command"].index(flag)+1] == value, "copied CLI input arguments")
    require(isolation.get("exitCode") == 0, "host negative probe exit")
    require(isolation.get("stdout", "").strip() == "ISOLATION_PASS filesystem=denied network=denied", "host negative probe outcome")
    validate_isolation_command(isolation.get("command", []))
    probe_command = isolation["command"]
    require("-c" in probe_command, "host probe command")
    targets = probe_command[probe_command.index("-c") + 2:]
    require(len(targets) == 5 and all(Path(p).is_absolute() for p in targets), "host probe exact targets")
    require(targets[1] == offline.get("cacheRoot"), "host probe original cache")
    online_command = online["command"]
    require("--manifest" in online_command and targets[2] == online_command[online_command.index("--manifest") + 1], "host probe original manifest")
    require(Path(targets[4]).parent == Path(targets[3]) and Path(targets[3]).name.startswith("bitloom-host-only-"), "separate host sentinel")
    require(all(not Path(targets[3]).is_relative_to(Path(p)) for p in targets[:3]), "no sentinel writes to original inputs")
    integrity = isolation.get("cacheIntegrity", {})
    require(is_sha(integrity.get("originalBefore")) and integrity.get("originalBefore") == integrity.get("originalAfter") == integrity.get("copyAfter"), "copied cache integrity")

    output = {}
    for line in str(offline.get("stdout", "")).splitlines():
        if "=" in line:
            key, value = line.split("=", 1)
            output[key] = value
    require(output.get("network") == "denied", "network denied marker")
    namespaces = output.get("network_namespace", "").split("->")
    require(len(namespaces) == 2 and namespaces[0] != namespaces[1], "distinct network namespaces")
    require(output.get("hdl_compile") == "passed", "HDL compile marker")
    require(output.get("adapter") == "bitloom-yosys-sv-compat@1", "adapter marker")
    require(is_sha(output.get("adapter_input_sha256")), "adapter input digest")
    require(is_sha(output.get("adapter_output_sha256")), "adapter output digest")
    common = next(source for source in sources if source["name"] == "common_cells")
    fifo = next(item for item in common["files"] if item["path"] == "src/fifo_v3.sv")
    require(output["adapter_input_sha256"] == fifo["sha256"], "adapter input/source binding")


def cli_validate(paths: list[str]) -> int:
    try:
        validate(*(Path(value) for value in paths))
    except (ContractError, KeyError, StopIteration, OSError, json.JSONDecodeError) as error:
        print(f"evidence-contract: {error}", file=sys.stderr)
        return 1
    print("evidence-contract: valid")
    return 0


class EvidenceConsumerTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tmp = Path(tempfile.mkdtemp(prefix="bitloom-fr199-evidence-"))
        self.addCleanup(shutil.rmtree, self.tmp, True)
        (self.tmp / "licenses").mkdir()
        self.paths = [self.tmp / name for name in ("manifest.json", "lock.json", "online.json", "offline.json")]
        for source, target in zip((MANIFEST, LOCK, ONLINE, OFFLINE), self.paths):
            shutil.copy2(source, target)
        shutil.copy2(OFFLINE.with_name(OFFLINE.stem + "-isolation.json"), self.tmp / "offline-isolation.json")
        for license_path in LICENSES.iterdir():
            shutil.copy2(license_path, self.tmp / "licenses" / license_path.name)

    def invoke(self, optimized: bool) -> subprocess.CompletedProcess[str]:
        command = [sys.executable]
        if optimized:
            command.append("-O")
        command.extend([str(Path(__file__).resolve()), "--validate", *(str(path) for path in self.paths)])
        return subprocess.run(command, text=True, capture_output=True, check=False, timeout=30)

    def assert_modes(self, expected: int) -> None:
        for optimized in (False, True):
            result = self.invoke(optimized)
            self.assertEqual(expected, result.returncode, result.stdout + result.stderr)

    def mutate(self, index: int, pointer: tuple[str, ...], value: object) -> None:
        document = load(self.paths[index])
        target = document
        for key in pointer[:-1]:
            target = target[int(key)] if isinstance(target, list) else target[key]
        key = pointer[-1]
        if isinstance(target, list):
            target[int(key)] = value
        else:
            target[key] = value
        self.paths[index].write_text(json.dumps(document, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    def test_p0_canonical_evidence_validates_in_normal_and_optimized_modes(self) -> None:
        self.assert_modes(0)

    def test_p0_manifest_mutation_fails_in_both_modes(self) -> None:
        self.mutate(0, ("owner",), "mutated-owner")
        self.assert_modes(1)

    def test_p0_source_lock_mutation_fails_in_both_modes(self) -> None:
        self.mutate(1, ("sources", "0", "closureDigest"), "0" * 64)
        self.assert_modes(1)

    def test_p0_tool_mutation_fails_in_both_modes(self) -> None:
        self.mutate(1, ("tools", "bitloomSha256"), "0" * 64)
        self.assert_modes(1)

    def test_p0_generator_mutation_fails_in_both_modes(self) -> None:
        self.mutate(1, ("generator", "inputs"), ["generated/input.json"])
        self.assert_modes(1)

    def test_p0_network_namespace_mutation_fails_in_both_modes(self) -> None:
        self.mutate(3, ("isolatedEnvironment", "BITLOOM_NETWORK_ISOLATED"), "0")
        self.assert_modes(1)

    def test_p0_adapter_marker_mutation_fails_in_both_modes(self) -> None:
        offline = load(self.paths[3])
        offline["stdout"] = offline["stdout"].replace(
            "adapter=bitloom-yosys-sv-compat@1", "adapter=unknown@9"
        )
        self.paths[3].write_text(json.dumps(offline, indent=2, sort_keys=True) + "\n", encoding="utf-8")
        self.assert_modes(1)


    def test_p0_host_root_mount_fails_in_both_modes(self) -> None:
        offline = load(self.paths[3])
        offline["command"].extend(["--ro-bind", "/", "/"])
        self.mutate(3, ("command",), offline["command"])
        self.assert_modes(1)

    def test_p0_writable_cache_mount_fails_in_both_modes(self) -> None:
        offline = load(self.paths[3])
        index = offline["command"].index("/cache")
        offline["command"][index-2] = "--bind"
        self.mutate(3, ("command",), offline["command"])
        self.assert_modes(1)

    def test_p0_missing_host_probe_fails_in_both_modes(self) -> None:
        path = self.tmp / "offline-isolation.json"
        probe = load(path)
        probe["stdout"] = ""
        path.write_text(json.dumps(probe))
        self.assert_modes(1)

    def test_p0_cache_integrity_drift_fails_in_both_modes(self) -> None:
        path = self.tmp / "offline-isolation.json"
        probe = load(path)
        probe["cacheIntegrity"]["copyAfter"] = "0" * 64
        path.write_text(json.dumps(probe))
        self.assert_modes(1)


def main() -> int:
    global MANIFEST, LOCK, ONLINE, OFFLINE, LICENSES
    parser = argparse.ArgumentParser()
    parser.add_argument("--validate", nargs=4, metavar=("MANIFEST", "LOCK", "ONLINE", "OFFLINE"))
    parser.add_argument("--fixtures", nargs=4, metavar=("MANIFEST", "LOCK", "ONLINE", "OFFLINE"), help="explicit real archived fixture inputs for self-tests")
    args = parser.parse_args()
    if args.fixtures:
        MANIFEST, LOCK, ONLINE, OFFLINE = (Path(value) for value in args.fixtures)
        LICENSES = LOCK.parent / "licenses"
    if args.validate:
        return cli_validate(args.validate)
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(EvidenceConsumerTests)
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    return 0 if result.wasSuccessful() else 1


if __name__ == "__main__":
    raise SystemExit(main())
