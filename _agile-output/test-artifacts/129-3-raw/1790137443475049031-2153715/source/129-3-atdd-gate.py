#!/usr/bin/env python3
from pathlib import Path
import json
import re
import sys
from fr201_evidence import validate

ROOT = Path(__file__).resolve().parents[2]
FAILURES = []


def require(condition: bool, message: str) -> None:
    if not condition:
        FAILURES.append(message)


def text(path: str) -> str:
    target = ROOT / path
    require(target.is_file(), f"missing required artifact: {path}")
    return target.read_text(encoding="utf-8") if target.is_file() else ""


story = text(
    "_agile-output/implementation-artifacts/"
    "129-3-ci-兼容性-贡献模板与核心关闭.md"
)
justfile = text("Justfile")
ci = text(".github/workflows/ci.yml")
runner = text("_agile-output/test-artifacts/129-3-build-runner.py")
contribution = text("docs/ip/contribution-template.md")
support = text("docs/ip/phase24-support-matrix.md")
evidence_path = ROOT / "_agile-output/test-artifacts/129-3-latest-results.json"
require(evidence_path.is_file(), "missing machine-readable 129.3 evidence")
evidence = json.loads(evidence_path.read_text()) if evidence_path.is_file() else {}

require("fr198-fr201-core-check:" in justfile, "missing one-command Justfile gate")
job_start = ci.find("fr198-fr201-core:")
require(job_start >= 0, "missing required CI job")
if job_start >= 0:
    job = ci[job_start:]
    for forbidden in ("continue-on-error: true", "|| true"):
        require(forbidden not in job, f"CI core job swallows failure: {forbidden}")
    require(
        "just fr198-fr201-core-check" in job,
        "CI core job does not invoke the local one-command gate",
    )
require(
    "129-3-isolated-replay.sh" in runner and "skip_isolated" in runner,
    "build runner does not execute isolated replay by default",
)


for field in (
    "owner",
    "behavior contract",
    "independent oracle",
    "two compositions",
    "direct",
    "FIRRTL",
    "Chisel",
    "formal",
    "synthesis",
    "compatibility",
    "source",
    "license",
    "seed",
    "waveform",
    "sha256",
    "exit code",
):
    require(
        field.lower() in contribution.lower(),
        f"contribution template missing field: {field}",
    )

for state in ("catalogued", "locked", "compiled", "behavior-tested", "maintained"):
    require(state in support, f"support matrix missing state: {state}")
require("Epic130" in support and "maintained" in support, "external pilot current support state missing")
try:
    validate(evidence, ROOT, complete=True)
except (ValueError, KeyError, TypeError, OSError) as error:
    require(False, f"independent evidence validation: {error}")

for backend in ("direct", "firrtl", "chisel"):
    row = evidence.get("backends", {}).get(backend, {})
    require(row.get("exit_code") == 0, f"{backend} backend did not exit zero")
    require(row.get("transactions", 0) > 0, f"{backend} has zero transactions")
    require(row.get("assertions", 0) > 0, f"{backend} has zero assertions")
    require(row.get("vcd_bytes", 0) > 0, f"{backend} has empty VCD")

for section in (
    "formal",
    "synthesis",
    "semver",
    "isolated_replay",
    "missing_tool_negative",
):
    require(section in evidence, f"evidence missing section: {section}")


for tool, row in evidence.get("missing_tool_negative", {}).items():
    require(row.get("exit_code") == 4, f"{tool} missing-tool exit code is not 4")

formal = evidence.get("formal", {})
require(formal.get("prove", {}).get("depth") == 4, "formal prove depth changed")
require(
    formal.get("response_reset_bmc", {}).get("depth") == 8,
    "response/reset BMC depth changed",
)
require(formal.get("cover", {}).get("depth") == 2, "formal cover depth changed")
require(
    formal.get("negative_control", {}).get("counterexample_vcd") is True,
    "formal mutation lacks a counterexample VCD",
)

require(evidence.get("compatibility_audit", {}).get("status") == "PASS", "scoped compatibility audit missing")
require("FR201 **核心部分**" in story, "story must preserve core-only claim")

if FAILURES:
    print("ATDD RED: Story129.3 requirements not implemented:")
    for failure in FAILURES:
        print(f"- {failure}")
    sys.exit(1)

print("ATDD GREEN: Story129.3 source/evidence contract satisfied")
