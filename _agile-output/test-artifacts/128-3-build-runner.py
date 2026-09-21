#!/usr/bin/env python3
"""Reproduce Story128.3 gates without overwriting historical build evidence.

Required environment:
  PATH: cargo, rustc, make, sby, yosys, z3, java, sbt, iverilog, vvp, cc, just
  BITLOOM_SBY_SOURCE: clean git checkout of the official pinned SBY source
  RHDL_FIRTOOL_PATH: directory containing product-pin firtool (1.159.0)
Optional BITLOOM_IRQ_RERUN_DIR: new, nonexistent output directory.
Default output: target/fr197-irq-reruns/<UTC timestamp>-<pid>.

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
import re
import shutil
import shlex
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


def sby_identity(env, capture=None):
    source = Path(env["BITLOOM_SBY_SOURCE"]).resolve()
    pins = dict(line.split("=", 1) for line in
                (ROOT / "scripts/ci-sby-pins.env").read_text().splitlines()
                if line and not line.startswith("#"))

    def git(*args):
        command = ["git", "-C", str(source), *args]
        return capture(command) if capture else subprocess.check_output(command, env=env)

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
    # Packages and legacy sourceless .pyc files can precede a checked .py.
    # Reject all unverified sby_* import candidates on every install lookup path.
    for directory in [launcher.parent, launcher.parent / "share/python3", prefix / "share/yosys/python3"]:
        for candidate in directory.glob("sby_*"):
            require(candidate in expected and candidate.is_file(), f"shadow SBY import candidate: {candidate}")
    records = []
    for installed, contents in expected.items():
        require(installed.read_bytes() == contents, f"installed/source mismatch: {installed}")
        records.append({"path": str(installed), "sha256": hashlib.sha256(contents).hexdigest()})
    return dict(verified_utc=utc(), source=str(source), tag_object=pins["SBY_GIT_SHA"],
                commit=PIN_COMMIT, launcher=str(launcher), files=records)


def source_fingerprint(root):
    """Relevant build inputs only; exclude evolving logs, archives and status docs."""
    paths = set()
    for name in ["Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "Justfile", ".github/workflows/ci.yml",
                 "docs/ip/irq-example.rs", "docs/ip/irq-registers.h", "docs/ip/irq-registers.md"]:
        if (root/name).is_file():
            paths.add(root/name)
    suffixes = {".rs", ".toml", ".lock", ".scala", ".v", ".sv", ".fir", ".c", ".h", ".sby", ".sh", ".py", ".env", ".txt"}
    for directory in [root/"crates", root/"scripts", root/".cargo"]:
        for path in directory.rglob("*"):
            # SBY can write its runtime logfile beside in-tree fixtures.
            # It is output evidence, even though its suffix is .txt.
            if path.is_file() and path.name != "logfile.txt" and path.suffix in suffixes and not {"target", "__pycache__", ".git"}.intersection(path.relative_to(root).parts):
                paths.add(path)
    for pattern in ["128-3-build-*.py", "128-3-build-*.c"]:
        paths.update((root/"_agile-output/test-artifacts").glob(pattern))
    return {str(path.relative_to(root)): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(paths)}


def exact_test_passed(output, target):
    require(re.search(r"^test " + re.escape(target) + r" \.\.\. ok$", output, re.MULTILINE) is not None
            and "running 1 test\n" in output
            and "test result: ok. 1 passed; 0 failed; 0 ignored;" in output,
            f"exact test did not execute and pass once: {target}")


def artifact_directories(root):
    return {str(path.resolve()) for name in ["fr197-irq", "fr197-irq-api", "fr197-irq-formal"]
            for path in (root/"target"/name).glob("*") if path.is_dir()}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    choices = ["identity", "atdd", "formal", "backends", "firrtl-regression", "numeric", "semver", "example", "header"]
    parser.add_argument("--only", action="append", choices=choices)
    selected = set(parser.parse_args().only or choices)
    env = os.environ.copy()
    env.pop("PYTHONPATH", None)
    env.update(CARGO_PROFILE_TEST_OPT_LEVEL="1", BITLOOM_REQUIRE_RTL="1", PYTHONDONTWRITEBYTECODE="1")
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S.%fZ")
    out = Path(env.get("BITLOOM_IRQ_RERUN_DIR", str(ROOT / "target/fr197-irq-reruns" / f"{stamp}-{os.getpid()}"))).resolve()
    out.mkdir(parents=True, exist_ok=False)
    env["BITLOOM_CHISEL_WORK"] = str(out / "numeric")
    print(f"Rerun evidence: {out}", flush=True)
    records = []
    fingerprint_complete = False
    invocation = [sys.executable, *sys.argv]

    def save_records():
        (out / "commands.json").write_text(json.dumps(dict(invocation=invocation,
            environment={k: env[k] for k in ["PATH", "RHDL_FIRTOOL_PATH", "BITLOOM_SBY_SOURCE"] if k in env},
            commands=records, workspace=str(ROOT), selected_gates=sorted(selected),
            source_fingerprint_complete=fingerprint_complete), indent=2))

    def gate(label, command, operation, launch=False):
        clock = time.monotonic()
        before_artifacts = artifact_directories(ROOT)
        record = dict(label=label, command=command, start_utc=utc(), status="attempted")
        records.append(record)
        save_records()  # Persist before launching: even exec/OS failures are evidence.
        try:
            result = operation()
            record.update(exit=result, status="pass" if result == 0 else "fail")
        except Exception as error:
            record.update(exit=None, status="launch-error" if launch and isinstance(error, OSError) else "error",
                          error=f"{type(error).__name__}: {error}")
            raise
        finally:
            record.update(end_utc=utc(), seconds=time.monotonic()-clock,
                          artifact_roots=sorted(artifact_directories(ROOT) - before_artifacts))
            save_records()
        if result:
            raise SystemExit(result)

    def run(label, command, exact=None):
        def execute():
            with (out / f"{label}.log").open("w") as log:
                result = subprocess.run(command, cwd=ROOT, env=env, stdout=log, stderr=subprocess.STDOUT).returncode
            records[-1]["process_exit"] = result
            if result == 0 and exact:
                exact_test_passed((out / f"{label}.log").read_text(), exact)
                records[-1]["exact_test"] = exact
            return result
        gate(label, command, execute, launch=True)

    save_records()
    run("source-files", ["git", "ls-files", "-z"])
    initial_fingerprint = source_fingerprint(ROOT)
    (out / "source-fingerprint-start.json").write_text(json.dumps(initial_fingerprint, indent=2))
    (out / "source-sha256.json").write_text(json.dumps(initial_fingerprint, indent=2))

    try:
        identity_serial = 0
        def capture_identity(command):
            nonlocal identity_serial
            label = f"identity-git-{identity_serial}"
            identity_serial += 1
            run(label, command)
            return (out / f"{label}.log").read_bytes()

        if "identity" in selected or "formal" in selected:
            try:
                identity = sby_identity(env, capture_identity)
            except Exception as error:
                (out / "sby-identity-failure.json").write_text(json.dumps(dict(utc=utc(), invocation=invocation, error=str(error)), indent=2))
                raise
            # Run checked sources with no user-site/PYTHONPATH imports, and a new
            # cache prefix: existing __pycache__ (including unchecked-hash .pyc) is ignored.
            runtime = out / "sby-runtime"
            runtime.mkdir()
            cache = runtime / "cache"
            cache.mkdir()
            interpreter = shutil.which("python3", path=env["PATH"])
            require(interpreter, "python3 required for isolated SBY runtime")
            args = [interpreter, "-I", "-B", "-X", f"pycache_prefix={cache}"]
            if sys.flags.optimize:
                args.append("-O")
            support = str(Path(identity["launcher"]).parent.parent / "share/yosys/python3")
            # Keep host dependencies such as click available, but put checked SBY
            # modules before site-packages and never search the script/CWD implicitly.
            bootstrap = "import sys, runpy; sys.path.insert(0, " + repr(support) + "); sys.argv=sys.argv[1:]; runpy.run_path(sys.argv[0], run_name='__main__')"
            args.extend(["-c", bootstrap, identity["launcher"]])
            wrapper = runtime / "sby"
            wrapper.write_text("#!/bin/sh\nexec " + shlex.join(args) + ' "$@"\n')
            wrapper.chmod(0o755)
            env["PATH"] = str(runtime) + os.pathsep + env["PATH"]
            identity["runtime_command"] = args
            identity["cache_policy"] = "new empty prefix; isolated interpreter; verified support first; no bytecode writes"
            (out / "sby-identity.json").write_text(json.dumps(identity, indent=2))
            run("sby-runtime", ["sby", "--version"])
        for label, command in [
            ("atdd", ["cargo", "test", "--locked", "-p", "bitloom", "--test", "fr197_irq", "--test", "fr197_irq_api", "--", "--nocapture"]),
            ("formal", ["cargo", "test", "--locked", "-p", "bitloom", "--test", "fr197_irq_formal", "--", "--ignored", "--nocapture"]),
            ("backends", ["cargo", "test", "--locked", "-p", "bitloom", "--test", "fr197_irq", "p1_irq_firrtl_chisel_same_independent_vectors", "--", "--ignored", "--exact", "--nocapture"]),
            ("firrtl-regression", ["cargo", "test", "--locked", "-p", "bitloom-firrtl"]),
            ("numeric", ["bash", "scripts/chisel-numeric-check.sh"]),
            ("semver", ["just", "semver-check"]),
        ]:
            if label in selected:
                run(label, command, exact="p1_irq_firrtl_chisel_same_independent_vectors" if label == "backends" else None)
                if label == "backends":
                    run("pair-backends", ["cargo", "test", "--locked", "-p", "bitloom", "--test", "fr197_irq_api", "p1_composition_firrtl_chisel_shared_renamed_and_actual_timer", "--", "--ignored", "--exact", "--nocapture"], exact="p1_composition_firrtl_chisel_shared_renamed_and_actual_timer")
        if "example" in selected:
            project = out / "example"
            (project / "src").mkdir(parents=True)
            (project / "Cargo.toml").write_text('[package]\nname="irq-design-example"\nversion="0.0.0"\nedition="2024"\n[workspace]\n[dependencies]\nbitloom-prelude={path=' + json.dumps(str(ROOT / "crates/bitloom-prelude")) + '}\n')
            shutil.copyfile(ROOT / "docs/ip/irq-example.rs", project / "src/main.rs")
            run("example", ["cargo", "run", "--offline", "--manifest-path", str(project / "Cargo.toml"), "--", str(project / "software")])
            def compare_artifacts():
                comparisons = []
                for name in ["irq-registers.h", "irq-registers.md"]:
                    expected = (ROOT / "docs/ip" / name).read_bytes()
                    actual = (project / "software" / name).read_bytes()
                    comparisons.append(dict(name=name, equal=expected == actual,
                        expected_sha256=hashlib.sha256(expected).hexdigest(), actual_sha256=hashlib.sha256(actual).hexdigest()))
                (out / "example-artifacts.json").write_text(json.dumps(comparisons, indent=2))
                return 0 if all(c["equal"] for c in comparisons) else 1
            gate("example-artifacts", ["compare-generated-artifacts", "irq-registers.h", "irq-registers.md"], compare_artifacts)
        if "header" in selected:
            binary = out / "header-check"
            run("header-compile", ["cc", "-std=c11", "-Wall", "-Wextra", "-Werror", "-I", str(ROOT / "docs/ip"), str(ROOT / "_agile-output/test-artifacts/128-3-build-header-check.c"), "-o", str(binary)])
            run("header-execute", [str(binary)])
    finally:
        final_fingerprint = source_fingerprint(ROOT)
        (out / "source-fingerprint-end.json").write_text(json.dumps(final_fingerprint, indent=2))
        fingerprint_complete = bool(initial_fingerprint) and initial_fingerprint == final_fingerprint
        save_records()
        require(fingerprint_complete, "related source changed during run; results cannot certify this source")


if __name__ == "__main__":
    main()
