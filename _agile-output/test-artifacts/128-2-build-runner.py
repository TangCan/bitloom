#!/usr/bin/env python3
"""Reproduce Story128.2 gates without overwriting historical build evidence.

Required environment:
  PATH: cargo, rustc, make, sby, yosys, z3, java, sbt, iverilog, vvp, cc, just
  BITLOOM_SBY_SOURCE: clean git checkout of the official pinned SBY source
  RHDL_FIRTOOL_PATH: directory containing product-pin firtool (1.159.0)
Optional BITLOOM_TIMER_RERUN_DIR: new, nonexistent output directory.
Default output: target/fr197-timer-reruns/<UTC timestamp>-<pid>.

Use --only identity/example/header for a narrow reproduction check; repeat
--only to select more gates. With no --only, run the original full build set.
SBY identity is independently reconstructed from pinned git blobs and the
upstream install transformations, rather than trusting an earlier JSON file.
"""
from pathlib import Path
import argparse
import datetime
import hashlib
import json
import os
import shutil
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
PIN_COMMIT = "daed0e1544fd96ee7dab843e5a891d92784c6230"


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def sby_identity(env):
    source = Path(env["BITLOOM_SBY_SOURCE"]).resolve()
    pins = dict(line.split("=", 1) for line in
                (ROOT / "scripts/ci-sby-pins.env").read_text().splitlines()
                if line and not line.startswith("#"))

    def git(*args):
        return subprocess.check_output(["git", "-C", str(source), *args], env=env)

    require(git("remote", "get-url", "origin").decode().strip() == pins["SBY_GIT_URL"], "SBY origin mismatch")
    require(git("rev-parse", pins["SBY_GIT_REF"]).decode().strip() == pins["SBY_GIT_SHA"], "SBY tag object mismatch")
    require(git("rev-parse", pins["SBY_GIT_SHA"] + "^{commit}").decode().strip() == PIN_COMMIT, "SBY peeled commit mismatch")
    require(git("rev-parse", "HEAD").decode().strip() == PIN_COMMIT, "SBY HEAD mismatch")
    require(not git("status", "--porcelain", "--untracked-files=no").strip(), "dirty SBY tracked source")
    release = "SBY " + git("describe", "--dirty").decode().strip()
    require(release == "SBY " + pins["SBY_GIT_REF"], "SBY release identity mismatch")
    executable = shutil.which("sby", path=env["PATH"])
    require(executable, "sby must be on PATH")
    launcher = Path(executable).resolve()
    prefix = launcher.parent.parent
    paths = [p for p in git("ls-tree", "-r", "--name-only", "HEAD", "sbysrc").decode().splitlines()
             if Path(p).name.startswith("sby_") and p.endswith(".py")]
    expected = {}
    for path in paths:
        contents = git("show", "HEAD:" + path)
        if Path(path).name == "sby_core.py":
            contents = contents.replace(b"##yosys-program-prefix##", b'""')
        expected[prefix / "share/yosys/python3" / Path(path).name] = contents
    contents = git("show", "HEAD:sbysrc/sby.py")
    contents = contents.replace(b"##yosys-sys-path##", b'sys.path += [os.path.dirname(__file__) + p for p in ["/share/python3", "/../share/yosys/python3"]]')
    contents = contents.replace(b"##yosys-release-version##", f"release_version = '{release}'".encode())
    expected[launcher] = contents
    # Earlier launcher lookup paths must not shadow the verified support files.
    for directory in [launcher.parent, launcher.parent / "share/python3"]:
        require(not list(directory.glob("sby_*.py")), f"shadow SBY modules in {directory}")
    records = []
    for installed, contents in expected.items():
        require(installed.read_bytes() == contents, f"installed/source mismatch: {installed}")
        records.append({"path": str(installed), "sha256": hashlib.sha256(contents).hexdigest()})
    return dict(verified_utc=utc(), source=str(source), tag_object=pins["SBY_GIT_SHA"],
                commit=PIN_COMMIT, files=records)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    choices = ["identity", "atdd", "formal", "backends", "firrtl-regression", "numeric", "semver", "example", "header"]
    parser.add_argument("--only", action="append", choices=choices)
    selected = set(parser.parse_args().only or choices)
    env = os.environ.copy()
    env.pop("PYTHONPATH", None)
    env.update(CARGO_PROFILE_TEST_OPT_LEVEL="1", BITLOOM_REQUIRE_RTL="1", PYTHONDONTWRITEBYTECODE="1")
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S.%fZ")
    out = Path(env.get("BITLOOM_TIMER_RERUN_DIR", str(ROOT / "target/fr197-timer-reruns" / f"{stamp}-{os.getpid()}"))).resolve()
    out.mkdir(parents=True, exist_ok=False)
    print(f"Rerun evidence: {out}", flush=True)
    records = []
    invocation = [sys.executable, *sys.argv]

    def save_records():
        (out / "commands.json").write_text(json.dumps(dict(invocation=invocation,
            environment={k: env[k] for k in ["PATH", "RHDL_FIRTOOL_PATH", "BITLOOM_SBY_SOURCE"] if k in env},
            commands=records), indent=2))

    save_records()  # Keep invocation and tool inputs even when identity fails.

    def run(label, command):
        start, clock = utc(), time.monotonic()
        with (out / f"{label}.log").open("w") as log:
            result = subprocess.run(command, cwd=ROOT, env=env, stdout=log, stderr=subprocess.STDOUT)
        records.append(dict(label=label, command=command, start_utc=start, end_utc=utc(),
                            seconds=time.monotonic()-clock, exit=result.returncode))
        save_records()
        if result.returncode:
            raise SystemExit(result.returncode)

    if "identity" in selected or "formal" in selected:
        try:
            identity = sby_identity(env)
        except Exception as error:
            (out / "sby-identity-failure.json").write_text(json.dumps(dict(utc=utc(), invocation=invocation, error=str(error)), indent=2))
            raise
        (out / "sby-identity.json").write_text(json.dumps(identity, indent=2))
    for label, command in [
        ("atdd", ["cargo", "test", "--locked", "-p", "bitloom", "--test", "fr197_timer", "--test", "fr197_timer_api", "--", "--nocapture"]),
        ("formal", ["cargo", "test", "--locked", "-p", "bitloom", "--test", "fr197_timer_formal", "--", "--ignored", "--nocapture"]),
        ("backends", ["cargo", "test", "--locked", "-p", "bitloom", "--test", "fr197_timer", "p1_timer_firrtl_chisel_same_independent_vectors", "--", "--ignored", "--exact", "--nocapture"]),
        ("firrtl-regression", ["cargo", "test", "--locked", "-p", "bitloom-firrtl"]),
        ("numeric", ["bash", "scripts/chisel-numeric-check.sh"]),
        ("semver", ["just", "semver-check"]),
    ]:
        if label in selected:
            run(label, command)
            if label == "backends":
                run("pair-backends", ["cargo", "test", "--locked", "-p", "bitloom", "--test", "fr197_timer_api", "p1_two_timer_firrtl_chisel_shared_and_renamed_definitions", "--", "--ignored", "--exact", "--nocapture"])
    if "example" in selected:
        project = out / "example"
        (project / "src").mkdir(parents=True)
        (project / "Cargo.toml").write_text('[package]\nname="timer-design-example"\nversion="0.0.0"\nedition="2024"\n[workspace]\n[dependencies]\nbitloom-prelude={path=' + json.dumps(str(ROOT / "crates/bitloom-prelude")) + '}\n')
        shutil.copyfile(ROOT / "docs/ip/timer-example.rs", project / "src/main.rs")
        run("example", ["cargo", "run", "--offline", "--manifest-path", str(project / "Cargo.toml"), "--", str(project / "software")])
        for name in ["timer-registers.h", "timer-registers.md"]:
            require((ROOT / "docs/ip" / name).read_bytes() == (project / "software" / name).read_bytes(), f"generated {name} differs")
    if "header" in selected:
        binary = out / "header-check"
        run("header-compile", ["cc", "-std=c11", "-Wall", "-Wextra", "-Werror", "-I", str(ROOT / "docs/ip"), str(ROOT / "_agile-output/test-artifacts/128-2-build-header-check.c"), "-o", str(binary)])
        run("header-execute", [str(binary)])


if __name__ == "__main__":
    main()
