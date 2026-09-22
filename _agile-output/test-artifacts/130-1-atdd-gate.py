#!/usr/bin/env python3
"""Story 130.1 AC7: 84 existing-GREEN process integration scenarios.

This verifies the existing Phase 24 state gate on temporary sprint copies. It
does not prove FR199 provenance, FR200 behavior, or NFR14 content quality.
Cross-story dependencies beyond the per-epic .1 gate require a separate audit.
Explicit requirements remain active under optimized Python.
"""

import hashlib
from pathlib import Path
import re
import subprocess
import sys
import tempfile


ROOT = Path(__file__).resolve().parents[2]
SPRINT = ROOT / "_agile-output/implementation-artifacts/sprint-status.yaml"
GATE = ROOT / "scripts/check_phase24_gate.py"
SUCCESS = (
    "PASS: Phase 24 6 epics / 22 stories; M0 and NFR14 gates; "
    "FR189/Epic122 deferred"
)


def require(condition: bool, message: str) -> None:
    if not condition:
        raise RuntimeError(message)


def replace_state(text: str, prefix: str, state: str) -> str:
    suffix = r"[^:\n]*" if prefix.endswith("-") else ""
    pattern = rf"^(  {re.escape(prefix)}{suffix}:[ \t]*)[\w-]+([ \t]*(?:#.*)?)$"
    result, count = re.subn(pattern, lambda match: match[1] + state + match[2], text, flags=re.M)
    require(count == 1, f"Expected exactly one status for {prefix!r}, got {count}")
    return result


def main() -> None:
    original = SPRINT.read_bytes()
    digest = hashlib.sha256(original).hexdigest()
    text = original.decode("utf-8")
    count = 0
    print(f"Source sprint SHA256: {digest}")
    print("Scope: existing GREEN process gate; seed=N/A; not FR199/FR200 acceptance.")
    print("130.3 -> 130.2/126.2 dependencies require a separate semantic audit.")
    try:
        with tempfile.TemporaryDirectory(prefix="bitloom-130-1-atdd-") as directory:
            fixture = Path(directory) / "sprint-status.yaml"

            def case(name: str, content: str, exit_code: int, reasons=()) -> None:
                nonlocal count
                fixture.write_text(content, encoding="utf-8")
                optimized = ["-" + "O" * sys.flags.optimize] if sys.flags.optimize else []
                result = subprocess.run(
                    [sys.executable, *optimized, str(GATE), str(fixture)],
                    capture_output=True,
                    text=True,
                    check=False,
                    timeout=30,
                )
                require(
                    result.returncode == exit_code,
                    f"{name}: exit={result.returncode}; stdout={result.stdout!r}; stderr={result.stderr!r}",
                )
                expected_stdout = SUCCESS + "\n" if exit_code == 0 else ""
                expected_stderr = "".join(f"FAIL: {reason}\n" for reason in reasons)
                require(result.stdout == expected_stdout, f"{name}: unexpected stdout {result.stdout!r}")
                require(result.stderr == expected_stderr, f"{name}: unexpected stderr {result.stderr!r}")
                count += 1
                print(f"PASS [P0] [{count:03d}] {name}: exit={result.returncode}")

            case("current sprint", text, 0)
            base = replace_state(text, "epic-130", "in-progress")
            for story in range(1, 4):
                base = replace_state(base, f"130-{story}-", "backlog")

            for gate_state in (
                "backlog",
                "ready",
                "ready-for-dev",
                "in-progress",
                "review",
                "blocked",
                "deferred",
            ):
                for story in (2, 3):
                    for active_state in ("ready", "ready-for-dev", "in-progress", "review", "done"):
                        candidate = replace_state(base, "130-1-", gate_state)
                        candidate = replace_state(candidate, f"130-{story}-", active_state)
                        case(
                            f"130.1 {gate_state}, 130.{story} {active_state}",
                            candidate,
                            1,
                            (f"130.{story} requires 130.1 NFR14 done",),
                        )

            future = base
            for epic, total in ((125, 3), (126, 4), (127, 4), (128, 5), (129, 3)):
                future = replace_state(future, f"epic-{epic}", "done")
                for previous in range(1, total + 1):
                    future = replace_state(future, f"{epic}-{previous}-", "done")
            future = replace_state(future, "130-1-", "done")
            for story in (2, 3):
                for active_state in ("ready", "ready-for-dev", "in-progress", "review", "done"):
                    candidate = future
                    if story == 3:
                        candidate = replace_state(candidate, "130-2-", "done")
                    candidate = replace_state(candidate, f"130-{story}-", active_state)
                    case(f"legal future 130.{story} {active_state} with predecessors done", candidate, 0)

            candidate = replace_state(base, "epic-125", "in-progress")
            for epic, total in ((126, 4), (127, 4), (128, 5), (129, 3), (130, 3)):
                candidate = replace_state(candidate, f"epic-{epic}", "backlog")
                for story in range(1, total + 1):
                    candidate = replace_state(candidate, f"{epic}-{story}-", "backlog")
            candidate = replace_state(candidate, "epic-130", "in-progress")
            candidate = replace_state(candidate, "130-1-", "ready-for-dev")
            case(
                "M0 not closed",
                candidate,
                1,
                ("Epic 130 cannot advance before M0 closes", "130.1 cannot advance before M0 closes"),
            )

            candidate = replace_state(future, "epic-130", "done")
            case("Epic130 premature close", candidate, 1, ("Epic 130 done requires its stories done",))
            candidate = replace_state(candidate, "130-2-", "done")
            candidate = replace_state(candidate, "130-3-", "done")
            case("legal future Epic130 close with all stories done", candidate, 0)
            require(count == 84, f"Expected 84 scenarios, got {count}")
    finally:
        after = SPRINT.read_bytes()
        require(after == original, "Original sprint bytes changed during test")
        require(hashlib.sha256(after).hexdigest() == digest, "Original sprint hash changed")
        print(f"PASS original sprint byte/hash preservation: {digest}")
    print(f"RESULT: {count} scenarios passed; existing GREEN gate behavior verified.")


if __name__ == "__main__":
    main()
