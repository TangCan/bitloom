#!/usr/bin/env python3
"""Run the unchanged locked PULP FIFO upstream testbench with Verilator.

This supplements, and does not replace, Bitloom's independent composition test.
The upstream bench itself excludes its fall-through SVA under VERILATOR.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import time


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lock", type=Path, required=True)
    parser.add_argument("--cache", type=Path, required=True)
    parser.add_argument("--verilator-root", type=Path, required=True)
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--evidence", type=Path, required=True)
    parser.add_argument("--seed", type=int, default=1303)
    parser.add_argument("--timeout", type=int, default=900)
    parser.add_argument("--inside", action="store_true", help=argparse.SUPPRESS)
    args = parser.parse_args()
    args.work = args.work.resolve()
    args.verilator_root = args.verilator_root.resolve()
    args.work.mkdir(parents=True, exist_ok=True)
    args.evidence.parent.mkdir(parents=True, exist_ok=True)
    if not args.inside:
        # Materialize a fresh source copy; the namespace cannot read the host
        # checkout or its other caches. Tool/runtime mounts are explicit.
        bundle = args.work / "input"
        scratch = args.work / "scratch"
        bundle.mkdir(exist_ok=True)
        scratch.mkdir(exist_ok=True)
        shutil.copy2(__file__, bundle / "runner.py")
        shutil.copy2(args.lock, bundle / "source.lock.json")
        locked = json.loads(args.lock.read_text())
        copied_cache = args.work / "copied-cache"
        for source in locked["sources"]:
            destination = copied_cache / source["cachePath"]
            if destination.exists():
                shutil.rmtree(destination)
            shutil.copytree(args.cache / source["cachePath"], destination, symlinks=True)
        command = ["bwrap", "--unshare-net", "--unshare-pid", "--die-with-parent",
                   "--ro-bind", "/usr", "/usr"]
        for runtime in ("/bin", "/lib", "/lib64", "/sbin"):
            if Path(runtime).is_symlink():
                command += ["--symlink", os.readlink(runtime), runtime]
            elif Path(runtime).exists():
                command += ["--ro-bind", runtime, runtime]
        command += ["--proc", "/proc", "--dev", "/dev", "--tmpfs", "/tmp",
                    "--ro-bind", str(bundle), "/input",
                    "--ro-bind", str(copied_cache), "/cache",
                    "--ro-bind", str(args.verilator_root), "/verilator",
                    "--bind", str(scratch), "/scratch", "--chdir", "/scratch",
                    "--clearenv", "--setenv", "PATH", "/usr/bin:/bin",
                    "--setenv", "HOME", "/scratch", "--setenv", "TMPDIR", "/tmp",
                    "/usr/bin/python3", "/input/runner.py", "--inside",
                    "--lock", "/input/source.lock.json", "--cache", "/cache",
                    "--verilator-root", "/verilator", "--work", "/scratch/build",
                    "--evidence", "/scratch/" + args.evidence.name,
                    "--seed", str(args.seed), "--timeout", str(args.timeout)]
        started_utc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
        started = time.monotonic()
        try:
            result = subprocess.run(command, text=True, capture_output=True, check=False,
                                    timeout=args.timeout * 3 + 60)
        except subprocess.TimeoutExpired as exc:
            result = subprocess.CompletedProcess(command, 124,
                exc.stdout.decode(errors="replace") if isinstance(exc.stdout, bytes) else (exc.stdout or ""),
                exc.stderr.decode(errors="replace") if isinstance(exc.stderr, bytes) else (exc.stderr or ""))
        for path in scratch.glob(args.evidence.stem + "*"):
            if path.is_file():
                shutil.copy2(path, args.evidence.parent / path.name)
        isolation = {"command": command, "exitCode": result.returncode,
                     "startedUtc": started_utc,
                     "durationSeconds": time.monotonic() - started,
                     "stdout": result.stdout, "stderr": result.stderr}
        args.evidence.with_name(args.evidence.stem + "-isolation.json").write_text(
            json.dumps(isolation, indent=2) + "\n")
        print(result.stdout, end="")
        print(result.stderr, file=sys.stderr, end="")
        return result.returncode
    record = {"startedUtc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
              "seed": args.seed, "lockSha256": digest(args.lock), "commands": [],
              "passed": False, "upstreamBenchUnmodified": True,
              "limitations": ["Upstream VERILATOR conditional disables fall-through SVA; queue assertions remain enabled.",
                              "Upstream bench uses DATA_WIDTH=8; pilot DATA_WIDTH=32 is covered separately."]}
    env = dict(os.environ, VERILATOR_ROOT=str(args.verilator_root), VERILATOR_SOLVER="z3 --in")

    def run(command: list[str], label: str) -> str:
        started = time.monotonic()
        stdout_path = args.evidence.parent / f"{args.evidence.stem}-{label}.stdout.log"
        stderr_path = args.evidence.parent / f"{args.evidence.stem}-{label}.stderr.log"
        with stdout_path.open("w") as out, stderr_path.open("w") as err:
            try:
                result = subprocess.run(command, cwd=args.work, env=env, text=True,
                                        stdout=out, stderr=err, timeout=args.timeout, check=False)
                code = result.returncode
            except subprocess.TimeoutExpired:
                code = 124
        stdout = stdout_path.read_text(errors="replace")
        stderr = stderr_path.read_text(errors="replace")
        record["commands"].append({"command": command, "exitCode": code,
                                   "durationSeconds": time.monotonic() - started,
                                   "stdout": stdout, "stderr": stderr})
        if code:
            raise RuntimeError(f"{label} failed with exit {code}")
        return stdout

    try:
        lock = json.loads(args.lock.read_text())
        sources = {s["name"]: s for s in lock["sources"]}
        roots = {}
        for name in ("common_cells", "common_verification"):
            source = sources[name]
            root = (args.cache / source["cachePath"]).resolve()
            if not root.is_relative_to(args.cache.resolve()):
                raise RuntimeError("cache escape")
            for file in source["files"]:
                path = (root / file["path"]).resolve()
                if not path.is_relative_to(root) or digest(path) != file["sha256"]:
                    raise RuntimeError(f"locked source drift: {path}")
            roots[name] = root
        cc, cv = roots["common_cells"], roots["common_verification"]
        inputs = [cv / "src/rand_verif_pkg.sv", cv / "src/clk_rst_gen.sv",
                  cc / "src/fifo_v3.sv", cc / "test/fifo_tb.sv"]
        record["sources"] = [{"name": s["name"], "commit": s["commit"], "treeDigest": s["treeDigest"]}
                             for s in sources.values()]
        record["inputs"] = [{"path": str(p), "sha256": digest(p)} for p in inputs]
        tools = [args.verilator_root / "bin/verilator", args.verilator_root / "bin/verilator_bin"]
        for name in ("g++", "make", "z3", "stdbuf"):
            path = shutil.which(name)
            if path is None:
                raise RuntimeError(f"missing required tool: {name}")
            tools.append(Path(path).resolve())
        record["tools"] = [{"path": str(p), "sha256": digest(p)} for p in tools]
        record["verilatorRuntime"] = [
            {"path": str(p.relative_to(args.verilator_root)), "sha256": digest(p)}
            for folder in ("bin", "include")
            for p in sorted((args.verilator_root / folder).rglob("*")) if p.is_file()
        ]
        record["verilatorVersion"] = run([str(tools[0]), "--version"], "version").strip()
        # The upstream ref-clock argument triggers CONTASSINIT even though
        # clk has no initializer. Limit the diagnostic control to that bench.
        control = args.work / "upstream.vlt"
        control.write_text('`verilator_config\nlint_off -rule CONTASSINIT -file "*/test/fifo_tb.sv"\n')
        record["diagnosticControl"] = control.read_text()
        run([str(tools[0]), "--binary", "--timing", "--assert", "-Wno-fatal",
             "--top-module", "fifo_tb", "--Mdir", str(args.work / "obj_dir"),
             "-j", "8", "+incdir+" + str(cc / "include"), str(control), *map(str, inputs)], "compile")
        exe = args.work / "obj_dir/Vfifo_tb"
        record["simulationExecutableSha256"] = digest(exe)
        stdout = run(["/usr/bin/stdbuf", "-oL", str(exe), f"+verilator+seed+{args.seed}"], "simulate")
        expected = {"i_tb_8", "i_tb_ft_8", "i_tb_1", "i_tb_ft_1", "i_tb_9", "i_tb_ft_9"}
        checks = {name: int(count) for name, count in re.findall(r"\.(i_tb(?:_ft)?_[189])[^\n]*: Checked (\d+) stimuli", stdout)}
        if set(checks) != expected or any(n < 100000 for n in checks.values()):
            raise RuntimeError(f"missing upstream completion/nonzero checks: {checks}")
        if "$finish" not in stdout:
            raise RuntimeError("upstream testbench did not finish")
        record["checks"] = checks
        record["passed"] = True
    except (OSError, ValueError, KeyError, RuntimeError) as exc:
        record["error"] = str(exc)
    finally:
        args.evidence.write_text(json.dumps(record, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"passed": record["passed"], "evidence": str(args.evidence), "error": record.get("error")}))
    return 0 if record["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
