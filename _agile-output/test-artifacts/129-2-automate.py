#!/usr/bin/env python3
"""Run and audit FR198's two required RTL peripheral recipes."""
from pathlib import Path
import json
import re
import subprocess
import tarfile
from datetime import datetime, timezone

ROOT = Path(__file__).parents[2]
cmd = ["cargo", "test", "-p", "bitloom", "--test", "fr198_peripheral_system", "--", "--nocapture"]
run = subprocess.run(cmd, cwd=ROOT, text=True, capture_output=True)
print(run.stdout)
print(run.stderr)
if run.returncode:
    raise SystemExit(run.returncode)

def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)

require("3 passed; 0 failed; 0 ignored" in run.stdout, "exact Rust test result missing")
matches = dict(re.findall(r"FR198 (axi|direct) PASS artifacts=([^\s]+)", run.stdout))
require(set(matches) == {"axi", "direct"}, "both topology artifact paths are required")
minimum = {"axi": (400, 100), "direct": (350, 95)}
summary = {
    "command": cmd,
    "cargo_exit_code": run.returncode,
    "recorded_at": datetime.now(timezone.utc).isoformat(),
    "topologies": {},
}
for topology, raw_path in matches.items():
    directory = Path(raw_path)
    evidence = json.loads((directory / "evidence.json").read_text())
    min_assertions, min_transactions = minimum[topology]
    require(evidence["topology"] == topology, f"{topology}: evidence label")
    require(evidence["assertions"] >= min_assertions, f"{topology}: assertion count")
    require(evidence["transactions"] >= min_transactions, f"{topology}: transaction count")
    require(evidence["random_transactions"] == 16_000, f"{topology}: frozen random budget")
    require(len(evidence["random_seeds"]) == 16, f"{topology}: frozen seed list")
    require(all(code == 0 for code in evidence["exit_codes"].values()), f"{topology}: nonzero tool exit")
    require(evidence["vcd_bytes"] > 1024, f"{topology}: empty VCD")
    require((directory / f"{topology}.vcd").stat().st_size == evidence["vcd_bytes"], f"{topology}: VCD size mismatch")
    for tool in ["iverilog", "vvp", "yosys"]:
        require(Path(evidence["tools"][tool]["path"]).is_absolute(), f"{topology}: {tool} path not recorded")
    summary["topologies"][topology] = evidence
    print(
        f"{topology}: assertions={evidence['assertions']} "
        f"transactions={evidence['transactions']} "
        f"random_transactions={evidence['random_transactions']} "
        f"vcd={evidence['vcd_bytes']}"
    )

artifact_root = ROOT / "_agile-output/test-artifacts"
(artifact_root / "129-2-latest-results.json").write_text(json.dumps(summary, indent=2) + "\n")
with tarfile.open(artifact_root / "129-2-behavior-evidence.tar.gz", "w:gz") as archive:
    for topology, raw_path in sorted(matches.items()):
        directory = Path(raw_path)
        for name in ["design.v", "tb.sv", f"{topology}.vcd", "evidence.json", "compile.log", "run.log", "yosys.log"]:
            archive.add(directory / name, arcname=f"{topology}/{name}")
print("AUTOMATE PASS: two VVP behavior gates plus strict Yosys structure gates")
