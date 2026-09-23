#!/usr/bin/env python3
"""Run the unchanged locked PULP FIFO upstream testbench with Verilator.

This supplements, and does not replace, Bitloom's independent composition test.
The upstream bench itself excludes its fall-through SVA under VERILATOR.
"""
from __future__ import annotations

import argparse
from concurrent.futures import ThreadPoolExecutor
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
    parser.add_argument("--timeout", type=int, default=3600)
    parser.add_argument("--diagnostic-checks", type=int,
                        help="Timing calibration only; never acceptance evidence")
    parser.add_argument("--parallel-cases", action="store_true",
                        help="Run six unchanged upstream fifo_inst_tb cases in separate processes")
    parser.add_argument("--inside", action="store_true", help=argparse.SUPPRESS)
    args = parser.parse_args()
    if args.diagnostic_checks is not None and args.diagnostic_checks <= 0:
        parser.error("diagnostic checks must be positive")
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
            relative = Path(source["cachePath"])
            if relative.is_absolute() or ".." in relative.parts or not relative.parts:
                raise SystemExit("invalid locked cache path")
            origin = (args.cache / relative).resolve()
            destination = (copied_cache / relative).resolve()
            if not origin.is_relative_to(args.cache.resolve()) or not destination.is_relative_to(copied_cache.resolve()):
                raise SystemExit("locked cache path escapes its root")
            if destination.exists():
                shutil.rmtree(destination)
            shutil.copytree(origin, destination, symlinks=True)
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
        if args.diagnostic_checks is not None:
            command += ["--diagnostic-checks", str(args.diagnostic_checks)]
        if args.parallel_cases:
            command += ["--parallel-cases"]
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
                     "runnerSha256": digest(bundle / "runner.py"),
                     "durationSeconds": time.monotonic() - started,
                     "stdout": result.stdout, "stderr": result.stderr}
        args.evidence.with_name(args.evidence.stem + "-isolation.json").write_text(
            json.dumps(isolation, indent=2) + "\n")
        print(result.stdout, end="")
        print(result.stderr, file=sys.stderr, end="")
        return result.returncode
    record = {"startedUtc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
              "seed": args.seed, "lockSha256": digest(args.lock), "commands": [],
              "diagnosticOnly": args.diagnostic_checks is not None,
              "acceptanceEligible": args.diagnostic_checks is None,
              "executionMode": "parallel-upstream-instances" if args.parallel_cases else "upstream-fifo_tb",
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
        record["commands"].append({"label": label, "command": command, "exitCode": code,
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
        expected = {"i_tb_8", "i_tb_ft_8", "i_tb_1", "i_tb_ft_1", "i_tb_9", "i_tb_ft_9"}
        minimum = args.diagnostic_checks if args.diagnostic_checks is not None else 100000
        def completed(stdout, wanted):
            checks = {name: int(count) for name, count in re.findall(r"\.(i_tb(?:_ft)?_[189])[^\n]*: Checked (\d+) stimuli", stdout)}
            if set(checks) != wanted or any(n < minimum for n in checks.values()) or "$finish" not in stdout:
                raise RuntimeError(f"missing upstream completion/nonzero checks: {checks}")
            return checks

        if args.parallel_cases:
            record["rngBoundary"] = "Each case starts seed independently; not the monolithic interleaved RNG sequence."
            def case(name):
                depth = int(name[-1])
                fallthrough = int("_ft_" in name)
                work = args.work / name
                work.mkdir(exist_ok=True)
                harness = work / "case.sv"
                harness.write_text(f'''// Test-only scheduling harness; upstream driver/oracle unchanged.
module bitloom_upstream_case;
  localparam time TCLK = 10ns;
  localparam time TA = TCLK * 1/4;
  localparam time TT = TCLK * 3/4;
  logic clk, rst_n, done;
  clk_rst_gen #(.ClkPeriod(TCLK), .RstClkCycles(10)) i_clk_rst_gen (
    .clk_o(clk), .rst_no(rst_n));
  fifo_inst_tb #(.FALL_THROUGH(1'b{fallthrough}), .DEPTH({depth}),
    .DATA_WIDTH(8), .N_CHECKS({minimum}), .TA(TA), .TT(TT)) {name} (
    .clk_i(clk), .rst_ni(rst_n), .done_o(done));
  initial begin wait(done); $finish; end
endmodule
''')
                run([str(tools[0]), "--binary", "--timing", "--assert", "-Wno-fatal",
                     "--top-module", "bitloom_upstream_case", "--Mdir", str(work / "obj_dir"),
                     "-j", "2", "+incdir+" + str(cc / "include"), str(control),
                     *map(str, inputs), str(harness)], name + "-compile")
                exe = work / "obj_dir/Vbitloom_upstream_case"
                stdout = run(["/usr/bin/stdbuf", "-oL", str(exe), f"+verilator+seed+{args.seed}"], name + "-simulate")
                return {"name": name, "depth": depth, "fallThrough": fallthrough,
                        "dataWidth": 8, "requiredChecks": minimum, "seed": args.seed,
                        "harness": harness.read_text(), "harnessSha256": digest(harness),
                        "simulationExecutableSha256": digest(exe),
                        "checks": completed(stdout, {name})}
            with ThreadPoolExecutor(max_workers=6) as pool:
                record["cases"] = list(pool.map(case, sorted(expected)))
            checks = {name: count for item in record["cases"] for name, count in item["checks"].items()}
        else:
            run([str(tools[0]), "--binary", "--timing", "--assert", "-Wno-fatal",
                 "--top-module", "fifo_tb", "--Mdir", str(args.work / "obj_dir"),
                 *([f"-GN_CHECKS={args.diagnostic_checks}"] if args.diagnostic_checks is not None else []),
                 "-j", "8", "+incdir+" + str(cc / "include"), str(control), *map(str, inputs)], "compile")
            exe = args.work / "obj_dir/Vfifo_tb"
            record["simulationExecutableSha256"] = digest(exe)
            stdout = run(["/usr/bin/stdbuf", "-oL", str(exe), f"+verilator+seed+{args.seed}"], "simulate")
            checks = completed(stdout, expected)
        if set(checks) != expected or any(n < minimum for n in checks.values()):
            raise RuntimeError(f"missing upstream completion/nonzero checks: {checks}")
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
