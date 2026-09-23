#!/usr/bin/env python3
"""FR199 two-stage fetch and network-isolated offline replay orchestrator."""

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
import tempfile


ROOT = Path(__file__).resolve().parents[1]


def binary() -> Path:
    override = os.environ.get("BITLOOM_BIN")
    candidate = Path(override) if override else ROOT / "target/debug/cargo-bitloom"
    if not candidate.is_file():
        raise SystemExit(f"missing cargo-bitloom binary: {candidate}; run cargo build -p bitloom")
    return candidate.resolve()


def run(command: list[str], evidence: Path | None = None) -> None:
    started = time.time()
    try:
        result = subprocess.run(command, text=True, capture_output=True, check=False, timeout=600)
    except subprocess.TimeoutExpired as error:
        result = subprocess.CompletedProcess(command, 124,
            error.stdout.decode() if isinstance(error.stdout, bytes) else error.stdout or "",
            (error.stderr.decode() if isinstance(error.stderr, bytes) else error.stderr or "") + "\nsubprocess timeout after 600 seconds")
    record = {
        "command": command,
        "startedUtc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(started)),
        "durationSeconds": time.time() - started,
        "exitCode": result.returncode,
        "stdout": result.stdout,
        "stderr": result.stderr,
    }
    if evidence:
        evidence.parent.mkdir(parents=True, exist_ok=True)
        evidence.write_text(json.dumps(record, indent=2, sort_keys=True) + "\n")
    sys.stdout.write(result.stdout)
    sys.stderr.write(result.stderr)
    if result.returncode:
        raise SystemExit(result.returncode)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def tree_digest(root: Path) -> str:
    digest = hashlib.sha256()
    for path in sorted(root.rglob("*")):
        digest.update(str(path.relative_to(root)).encode() + b"\0")
        if path.is_symlink():
            digest.update(b"link:" + os.readlink(path).encode())
        elif path.is_file():
            digest.update(b"file:" + sha256(path).encode())
        else:
            digest.update(b"directory")
    return digest.hexdigest()


def enrich_evidence(evidence: Path, manifest: Path, lock: Path, cache: Path, stage: str) -> None:
    record = json.loads(evidence.read_text())
    resolved = json.loads(lock.read_text())
    record.update(
        {
            "stage": stage,
            "manifestSha256": sha256(manifest),
            "lockSha256": sha256(lock),
            "cacheRoot": str(cache),
            "sources": [
                {
                    key: source[key]
                    for key in (
                        "name", "url", "ref", "tagType", "tagObject", "commit",
                        "closureDigest", "treeDigest", "cachePath",
                    )
                }
                for source in resolved["sources"]
            ],
            "tools": resolved["tools"],
        }
    )
    evidence.write_text(json.dumps(record, indent=2, sort_keys=True) + "\n")


def common(args: argparse.Namespace) -> list[str]:
    command = [
        str(binary()),
        "external-ip",
        args.action,
        "--manifest",
        str(Path(args.manifest).resolve()),
        "--lock",
        str(Path(args.lock).resolve()),
        "--cache",
        str(Path(args.cache).resolve()),
    ]
    if args.action == "verify":
        command.append("--offline")
    return command


def fetch(args: argparse.Namespace) -> None:
    evidence = Path(args.evidence).resolve() if args.evidence else None
    run(common(args), evidence)
    if evidence:
        enrich_evidence(
            evidence,
            Path(args.manifest).resolve(),
            Path(args.lock).resolve(),
            Path(args.cache).resolve(),
            args.action,
        )


def replay(args: argparse.Namespace) -> None:
    if not shutil.which("bwrap"):
        raise SystemExit("bubblewrap is required for network-isolated replay")
    manifest = Path(args.manifest).resolve()
    lock = Path(args.lock).resolve()
    cache = Path(args.cache).resolve()
    if not args.compile:
        raise SystemExit("replay requires --compile")
    if args.require_commit:
        resolved = json.loads(lock.read_text())
        commits = {source["commit"] for source in resolved.get("sources", [])}
        if args.require_commit not in commits:
            raise SystemExit(f"required commit is not locked: {args.require_commit}")
    resolved = json.loads(lock.read_text())
    # Preserve the transport and scratch on failure for diagnosis. Nothing from
    # the original checkout or cache is mounted into the child namespace.
    transport = Path(tempfile.mkdtemp(prefix="bitloom-offline-"))
    bundle = transport / "input"
    bundle.mkdir()
    original_cache_digest = tree_digest(cache)
    copied_cache = transport / "cache"
    shutil.copytree(cache, copied_cache, symlinks=True)
    scratch = transport / "scratch"
    scratch.mkdir()
    shutil.copy2(manifest, bundle / "source.json")
    shutil.copy2(lock, bundle / "source.lock.json")
    shutil.copy2(binary(), bundle / "cargo-bitloom")
    licenses = lock.parent / "licenses"
    if not licenses.is_dir():
        raise SystemExit(f"missing license archive: {licenses}")
    shutil.copytree(licenses, bundle / "licenses")
    host_netns = os.readlink("/proc/self/ns/net")
    prefix = ["bwrap", "--unshare-net", "--unshare-pid", "--die-with-parent", "--clearenv",
              "--ro-bind", "/usr", "/usr"]
    for runtime in ("/bin", "/lib", "/lib64", "/sbin"):
        path = Path(runtime)
        if path.is_symlink():
            prefix += ["--symlink", os.readlink(path), runtime]
        elif path.exists():
            prefix += ["--ro-bind", runtime, runtime]
    prefix += ["--proc", "/proc", "--dev", "/dev", "--tmpfs", "/tmp",
               "--ro-bind", str(bundle), "/input",
               "--ro-bind", str(copied_cache), "/cache",
               "--bind", str(scratch), "/scratch", "--chdir", "/scratch"]
    environment = {
        "PATH": "/usr/bin:/bin", "HOME": "/tmp/home", "TMPDIR": "/scratch",
        "XDG_CACHE_HOME": "/tmp/xdg-cache", "GIT_CONFIG_NOSYSTEM": "1",
        "GIT_CONFIG_GLOBAL": "/dev/null", "BITLOOM_NETWORK_ISOLATED": "1",
        "BITLOOM_HOST_NETNS": host_netns,
        "BITLOOM_LOCKED_BINARY_PATH": resolved["tools"]["bitloomPath"],
    }
    for key, value in environment.items():
        prefix += ["--setenv", key, value]
    evidence = Path(args.evidence).resolve() if args.evidence else transport / "replay.json"
    # Test filesystem and network denial in the same allowlisted namespace.
    probe = """import os, pathlib, socket, sys
for p in sys.argv[1:]:
    if pathlib.Path(p).exists():
        raise SystemExit('host path visible: ' + p)
if os.readlink('/proc/self/ns/net') == os.environ['BITLOOM_HOST_NETNS']:
    raise SystemExit('host network namespace visible')
s = socket.socket(); s.settimeout(1)
try:
    s.connect(('1.1.1.1', 443))
except OSError:
    print('ISOLATION_PASS filesystem=denied network=denied')
else:
    raise SystemExit('network unexpectedly reachable')
"""
    isolation_evidence = evidence.with_name(evidence.stem + "-isolation.json")
    # Host probes must not modify the checkout, manifest or original cache.
    sentinel_directory = Path(tempfile.mkdtemp(prefix="bitloom-host-only-"))
    sentinel = sentinel_directory / "sentinel"
    sentinel.write_text("host-only sentinel\n")
    try:
        run(prefix + ["/usr/bin/python3", "-c", probe, str(ROOT), str(cache), str(manifest), str(sentinel_directory), str(sentinel)], isolation_evidence)
    finally:
        shutil.rmtree(sentinel_directory)
    for action in (("replay", "binding", "behavior") if args.pilot else ("replay",)):
        command = prefix + ["/input/cargo-bitloom", "external-ip", action,
                            "--manifest", "/input/source.json", "--lock", "/input/source.lock.json",
                            "--cache", "/cache"]
        if action == "replay":
            command += ["--compile"]
        if action == "binding":
            command += ["--out", "/scratch/fifo_v3.binding.json"]
        action_evidence = evidence if action == "replay" else evidence.with_name(f"{evidence.stem}-{action}.json")
        try:
            run(command, action_evidence)
        finally:
            if action_evidence.exists():
                enrich_evidence(action_evidence, manifest, lock, cache, f"network-isolated-{action}")
                record = json.loads(action_evidence.read_text())
                record["transportRoot"] = str(transport)
                record["isolatedEnvironment"] = environment
                action_evidence.write_text(json.dumps(record, indent=2, sort_keys=True) + "\n")
    if args.pilot:
        shutil.copy2(scratch / "fifo_v3.binding.json", evidence.with_name(evidence.stem + "-binding-output.json"))
    if tree_digest(cache) != original_cache_digest or tree_digest(copied_cache) != original_cache_digest:
        raise SystemExit("offline replay mutated the original or read-only copied cache")
    isolation_record = json.loads(isolation_evidence.read_text())
    isolation_record["cacheIntegrity"] = {"originalBefore": original_cache_digest, "originalAfter": tree_digest(cache), "copyAfter": tree_digest(copied_cache)}
    isolation_record["transportCleaned"] = bool(args.evidence)
    isolation_evidence.write_text(json.dumps(isolation_record, indent=2, sort_keys=True) + "\n")
    if args.evidence:
        shutil.rmtree(transport)



def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser()
    sub = result.add_subparsers(dest="action", required=True)
    for name in ("lock", "verify"):
        command = sub.add_parser("fetch" if name == "lock" else name)
        command.set_defaults(action=name, function=fetch)
        command.add_argument("--manifest", required=True)
        command.add_argument("--lock", required=True)
        command.add_argument("--cache", required=True)
        command.add_argument("--evidence")
    command = sub.add_parser("replay")
    command.set_defaults(action="replay", function=replay)
    command.add_argument("--manifest", required=True)
    command.add_argument("--lock", required=True)
    command.add_argument("--cache", required=True)
    command.add_argument("--evidence")
    command.add_argument("--require-commit")
    command.add_argument("--compile", action="store_true")
    command.add_argument("--pilot", action="store_true", help="also execute the FR200 binding and independent wrapper behavior gates")
    return result


def main() -> None:
    args = parser().parse_args()
    args.function(args)


if __name__ == "__main__":
    main()
