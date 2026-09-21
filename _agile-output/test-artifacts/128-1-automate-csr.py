#!/usr/bin/env python3
"""[P0] 128.1-INT-CSR: archive one existing CSR native/actual-RTL probe.

Run explicitly with Python; no new product coverage or FR197 acceptance is claimed.
The existing bank()/directed() builders and handwritten expectations are reused.
Every invocation gets its own durable evidence directory, safe from cargo clean.
"""

import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tarfile
import tempfile
from datetime import datetime, timezone


ROOT = Path(__file__).resolve().parents[2]
TEST = "p0_leaf_owned_rejection_events_and_candidates_native_and_real_rtl"
RTL_ROOT = ROOT / "target/fr196-csr"


def utc() -> str:
    return datetime.now(timezone.utc).isoformat()


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(condition: bool, message: str) -> None:
    if not condition:
        raise RuntimeError(message)


def main() -> int:
    evidence = Path(tempfile.mkdtemp(prefix="128-1-automate-csr-run-", dir=Path(__file__).parent))
    env = dict(os.environ)
    env.update(CARGO_PROFILE_TEST_OPT_LEVEL="1", BITLOOM_REQUIRE_RTL="1", CARGO_TERM_COLOR="never")
    env["PATH"] = "/tmp/bitloom-maintenance-tools/bin:/tmp/bitloom-1263-sby-installed/bin:" + env.get("PATH", "")
    report = {
        "test": TEST, "priority": "P0", "started_utc": utc(), "cwd": str(ROOT),
        "scope": "existing CSR baseline; no new peripheral product coverage",
        "new_runner_integration_cases": 1, "new_product_tests": 0,
        "reused_rust_tests": 1, "commands": [], "status": "FAIL",
        "environment": {key: env[key] for key in ("PATH", "CARGO_PROFILE_TEST_OPT_LEVEL", "BITLOOM_REQUIRE_RTL")},
    }

    def command(label: str, argv: list[str], seconds: int = 60) -> str:
        path = shutil.which(argv[0], path=env["PATH"])
        require(path is not None, f"required tool missing: {argv[0]}")
        log = evidence / f"{label}.log"
        bounded = ["timeout", "--kill-after=5s", f"{seconds}s", *argv]
        record = {"argv": bounded, "tool_path": path, "started_utc": utc(), "log": str(log)}
        report["commands"].append(record)
        with log.open("wb") as output:
            result = subprocess.run(bounded, cwd=ROOT, env=env, stdout=output, stderr=subprocess.STDOUT, check=False)
        record.update(exit_code=result.returncode, finished_utc=utc(), log_sha256=sha(log))
        require(result.returncode == 0, f"{label} failed: exit {result.returncode}; {log}")
        return log.read_text(errors="replace")

    try:
        require(shutil.which("timeout", path=env["PATH"]) is not None, "GNU timeout is required")
        for tool, flag in (("cargo", "--version"), ("rustc", "--version"), ("iverilog", "-V"), ("vvp", "-V"), ("timeout", "--version")):
            command(f"{tool}-version", [tool, flag])
        report["python"] = {"path": sys.executable, "version": sys.version}
        report["source_commit"] = command("git-head", ["git", "rev-parse", "HEAD"]).strip()
        source_files = command("source-files", ["git", "ls-files", "crates", "Cargo.toml", "Cargo.lock", "rust-toolchain.toml"]).splitlines()
        sources_before = {name: sha(ROOT / name) for name in source_files if (ROOT / name).is_file()}
        report["source_sha256"] = sources_before
        report["runner_sha256"] = sha(Path(__file__))
        before = {p.resolve() for p in RTL_ROOT.iterdir()} if RTL_ROOT.exists() else set()
        output = command("cargo-test", ["cargo", "test", "--locked", "-p", "bitloom", "--test", "fr196_csr", TEST, "--", "--exact", "--nocapture", "--test-threads=1"], 1800)
        require(len(re.findall(r"^running 1 test$", output, re.MULTILINE)) == 1, "expected exactly one Rust test")
        require(len(re.findall(r"^test result: ok\. 1 passed; 0 failed; 0 ignored;", output, re.MULTILINE)) == 1, "missing exact non-ignored Rust success summary")
        require(f"test {TEST} ..." in output, "selected test did not run")
        observed = re.findall(r"^artifacts=(.+)$", output, re.MULTILINE)
        require(len(observed) == 1, "expected exactly one logged RTL artifact path")
        artifact = Path(observed[0]).resolve()
        require(artifact.parent == RTL_ROOT.resolve() and re.fullmatch(r"leaf-reject-\d+", artifact.name) is not None, "unexpected RTL artifact path")
        require(artifact not in before, "refusing stale/reused RTL directory")
        report["rtl_artifact_source"] = str(artifact)
        # Archive immediately, before checking contents, preserving failed evidence too.
        archive = evidence / "actual-rtl.tar.gz"
        with tarfile.open(archive, "w:gz") as bundle:
            bundle.add(artifact, arcname=artifact.name)
        report["archive"] = {"path": str(archive), "sha256": sha(archive)}
        required = ("design.v", "tb.sv", "simulation", "commands.log", "iverilog-version.log", "vvp-version.log", "compile.log", "run.log")
        for name in required:
            require((artifact / name).is_file() and (artifact / name).stat().st_size > 0, f"missing/nonempty artifact required: {name}")
        for name in ("iverilog-version.log", "vvp-version.log", "compile.log", "run.log"):
            content = (artifact / name).read_text(errors="replace")
            require(content.rstrip().endswith("status=exit status: 0"), f"actual tool did not succeed: {name}")
        run_log = (artifact / "run.log").read_text()
        require(len(re.findall(r"^FR196 PASS$", run_log, re.MULTILINE)) == 1 and "FATAL:" not in run_log, "missing actual RTL success marker or fatal simulation")
        report["rtl_sha256"] = {name: sha(artifact / name) for name in required}
        require(all((ROOT / name).is_file() and sha(ROOT / name) == digest for name, digest in sources_before.items()), "source changed during probe")
        report["status"] = "PASS"
    except (OSError, RuntimeError, subprocess.SubprocessError) as error:
        report["error"] = str(error)
    finally:
        report["finished_utc"] = utc()
        report["exit_code"] = 0 if report["status"] == "PASS" else 1
        (evidence / "result.json").write_text(json.dumps(report, indent=2) + "\n")
        print(f"{report['status']}: {evidence / 'result.json'}", flush=True)
    return report["exit_code"]


if __name__ == "__main__":
    raise SystemExit(main())
