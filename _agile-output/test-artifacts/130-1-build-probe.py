#!/usr/bin/env python3
"""Story130.1 tool and empty-black-box mechanism probe.

This intentionally proves environment and file-binding mechanics only. It does
not acquire an external core or claim FR199/FR200 behavior.
"""

from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import time


ROOT = Path(__file__).resolve().parents[2]
BLACKBOX = ROOT / "crates/bitloom-prelude/src/ip/blackbox.rs"
VENDOR = "module vendor_ext_ip(input clk, input rst, input [7:0] data_in, output [7:0] data_out);\nendmodule\n"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise RuntimeError(message)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, required=True)
    args = parser.parse_args()
    output = args.output_dir.resolve()
    output.mkdir(parents=True, exist_ok=False)
    report: dict[str, object] = {
        "scope": "Story130.1 environment and empty black-box binding mechanism; not FR199/FR200",
        "started_utc": datetime.now(timezone.utc).isoformat(),
        "python": sys.version,
        "optimize": sys.flags.optimize,
        "script_sha256": sha256(Path(__file__)),
        "commands": [],
        "success": False,
    }

    def run(
        label: str,
        argv: list[str],
        cwd: Path = output,
        expected: int | tuple[int, ...] = 0,
    ) -> subprocess.CompletedProcess[str]:
        started = time.monotonic()
        entry: dict[str, object] = {
            "label": label,
            "argv": argv,
            "cwd": str(cwd),
            "started_utc": datetime.now(timezone.utc).isoformat(),
        }
        report["commands"].append(entry)  # type: ignore[union-attr]
        log = output / f"{label}.log"
        try:
            result = subprocess.run(
                argv,
                cwd=cwd,
                text=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                timeout=60,
                check=False,
            )
        except subprocess.TimeoutExpired as exc:
            raw = exc.stdout or ""
            captured = raw.decode(errors="replace") if isinstance(raw, bytes) else raw
            log.write_text(captured, encoding="utf-8")
            entry.update(
                timeout=True,
                duration_seconds=time.monotonic() - started,
                log=log.name,
                log_sha256=sha256(log),
            )
            raise RuntimeError(f"{label}: timed out after 60 seconds") from exc
        log.write_text(result.stdout, encoding="utf-8")
        entry.update(
            returncode=result.returncode,
            duration_seconds=time.monotonic() - started,
            log=log.name,
            log_sha256=sha256(log),
        )
        accepted = (expected,) if isinstance(expected, int) else expected
        require(result.returncode in accepted, f"{label}: exit {result.returncode}, expected one of {accepted}")
        return result

    try:
        source = BLACKBOX.read_text(encoding="utf-8")
        require(source.count(VENDOR.replace("\n", "\\n")) == 1, "vendor stub literal drifted from probe")
        report["source"] = {
            "path": str(BLACKBOX.relative_to(ROOT)),
            "sha256": sha256(BLACKBOX),
            "vendor_literal_sha256": hashlib.sha256(VENDOR.encode()).hexdigest(),
        }

        tools: dict[str, dict[str, str]] = {}
        for name, flag in (
            ("git", "--version"),
            ("sha256sum", "--version"),
            ("bwrap", "--version"),
            ("unshare", "--version"),
            ("iverilog", "-V"),
            ("vvp", "-V"),
            ("yosys", "-V"),
        ):
            resolved = shutil.which(name)
            require(resolved is not None, f"required probe tool missing: {name}")
            path = Path(resolved).resolve()
            tools[name] = {"path": str(path), "sha256": sha256(path)}
            run(f"version-{name}", [str(path), flag])
        report["tools"] = tools

        hashed = run("sha256-blackbox", [tools["sha256sum"]["path"], str(BLACKBOX)], ROOT).stdout.split()[0]
        require(hashed == sha256(BLACKBOX), "sha256sum disagreed with Python source digest")

        unshare = run("network-denial-unshare", [tools["unshare"]["path"], "-n", "true"], expected=(0, 1))
        report["unshare_net_supported"] = unshare.returncode == 0

        head = run("git-head", [tools["git"]["path"], "rev-parse", "HEAD^{commit}"], ROOT).stdout.strip()
        require(len(head) == 40, "git did not resolve a full commit")
        run("git-recursive-closure", [tools["git"]["path"], "submodule", "status", "--recursive"], ROOT)
        archive = output / "head.tar"
        archive_started = time.monotonic()
        archive_entry: dict[str, object] = {
            "label": "git-archive",
            "argv": [tools["git"]["path"], "archive", "--format=tar", "HEAD"],
            "cwd": str(ROOT),
            "started_utc": datetime.now(timezone.utc).isoformat(),
        }
        report["commands"].append(archive_entry)  # type: ignore[union-attr]
        archive_log = output / "git-archive.log"
        with archive.open("wb") as stream:
            try:
                archived = subprocess.run(
                    archive_entry["argv"],
                    cwd=ROOT,
                    stdout=stream,
                    stderr=subprocess.PIPE,
                    timeout=60,
                    check=False,
                )
            except subprocess.TimeoutExpired as exc:
                archive_log.write_bytes(exc.stderr or b"")
                archive_entry.update(
                    timeout=True,
                    duration_seconds=time.monotonic() - archive_started,
                    log=archive_log.name,
                    log_sha256=sha256(archive_log),
                )
                raise RuntimeError("git-archive: timed out after 60 seconds") from exc
        archive_log.write_bytes(archived.stderr)
        archive_entry.update(
            returncode=archived.returncode,
            duration_seconds=time.monotonic() - archive_started,
            log=archive_log.name,
            log_sha256=sha256(archive_log),
        )
        require(archived.returncode == 0 and archive.stat().st_size > 0, "git archive closure probe failed")
        report["repository_identity"] = {"commit": head, "archive_sha256": sha256(archive)}
        archive.unlink()

        isolated = run(
            "network-denial-bwrap",
            [
                tools["bwrap"]["path"],
                "--unshare-net",
                "--ro-bind",
                "/",
                "/",
                "--dev",
                "/dev",
                "--proc",
                "/proc",
                sys.executable,
                "-c",
                "import socket,sys; n=socket.if_nameindex(); print(n); sys.exit(0 if n == [(1, 'lo')] else 9)",
            ],
        )
        require("[(1, 'lo')]" in isolated.stdout, "network namespace did not contain loopback only")

        run(
            "rust-blackbox-contract",
            [
                "cargo",
                "test",
                "--locked",
                "--offline",
                "-p",
                "bitloom",
                "--test",
                "fr82_fifo_uart_baseline",
                "fr82_blackbox_boundary_documented",
                "--",
                "--exact",
                "--nocapture",
            ],
            ROOT,
        )

        vendor = output / "vendor_ext_ip.v"
        testbench = output / "tb.sv"
        vendor.write_text(VENDOR, encoding="utf-8")
        testbench.write_text(
            "module tb; reg clk=0, rst=0; reg [7:0] data_in=0; wire [7:0] data_out;\n"
            "vendor_ext_ip dut(.clk(clk),.rst(rst),.data_in(data_in),.data_out(data_out));\n"
            "initial begin $display(\"BLACKBOX BINDING MECHANISM PASS behavior=absent\"); #1 $finish; end\n"
            "endmodule\n",
            encoding="utf-8",
        )
        report["rtl_sources"] = {vendor.name: sha256(vendor), testbench.name: sha256(testbench)}
        run("rtl-compile", [tools["iverilog"]["path"], "-g2012", "-s", "tb", "-o", "simulation", vendor.name, testbench.name])
        simulated = run("rtl-simulate", [tools["vvp"]["path"], "simulation"])
        require("BLACKBOX BINDING MECHANISM PASS behavior=absent" in simulated.stdout, "mechanism marker missing")
        run(
            "rtl-synthesis",
            [
                tools["yosys"]["path"],
                "-p",
                "read_verilog vendor_ext_ip.v; hierarchy -check -top vendor_ext_ip; proc; check -assert; stat",
            ],
        )
        (output / "simulation").unlink()

        report["limitations"] = [
            "The vendor module has no behavioral assignment; output behavior is intentionally not accepted.",
            "Compilation, simulation launch, and synthesis prove external-file binding mechanics only.",
            "No upstream source, tag, license, recursive dependency, or offline replay was acquired.",
            "This result must not promote the Epic130 support-matrix row.",
        ]
        report["success"] = True
        print("PASS: Story130.1 tools, network denial, and empty-black-box binding mechanism")
    except Exception as exc:
        report["error"] = f"{type(exc).__name__}: {exc}"
        print(report["error"], file=sys.stderr)
    finally:
        report["finished_utc"] = datetime.now(timezone.utc).isoformat()
        (output / "results.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return 0 if report["success"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
