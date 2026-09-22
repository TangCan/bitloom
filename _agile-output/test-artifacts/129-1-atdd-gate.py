#!/usr/bin/env python3
"""Story 129.1 AC6: 84 existing-GREEN process integration scenarios.

User AC6 explicitly requires real checks of an existing gate, overriding the
ATDD template's RED/skip default for this portion. No FR198/FR201 product
acceptance is claimed. Cross-story dependencies are fixture setup and need a
separate audit; check_phase24_gate.py does not enforce those edges.
Uses explicit requirements so Python -O cannot erase acceptance checks.
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


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def replace_state(text, prefix, state):
    suffix = r"[^:\n]*" if prefix.endswith("-") else ""
    pattern = rf"^(  {re.escape(prefix)}{suffix}:[ \t]*)[\w-]+([ \t]*(?:#.*)?)$"
    result, count = re.subn(pattern, lambda m: m[1] + state + m[2], text, flags=re.M)
    require(count == 1, f"Expected exactly one status for {prefix!r}, got {count}")
    return result


def main():
    original = SPRINT.read_bytes()
    digest = hashlib.sha256(original).hexdigest()
    text = original.decode("utf-8")
    count = 0
    print(f"Source sprint SHA256: {digest}")
    print("Scope: existing GREEN process gate; seed=N/A; not FR198/FR201 product acceptance.")
    print("Cross-story dependencies are fixture setup, not gate coverage.")
    try:
        with tempfile.TemporaryDirectory(prefix="bitloom-129-1-atdd-") as directory:
            fixture = Path(directory) / "sprint-status.yaml"

            def case(name, content, exit_code, reasons=()):
                nonlocal count
                fixture.write_text(content, encoding="utf-8")
                result = subprocess.run(
                    [sys.executable, *(["-" + "O" * sys.flags.optimize] if sys.flags.optimize else []), str(GATE), str(fixture)],
                    capture_output=True, text=True, check=False, timeout=30,
                )
                # Match the complete CLI contract; unrelated crashes/diagnostics
                # must not masquerade as an expected negative result.
                require(result.returncode == exit_code,
                        f"{name}: exit={result.returncode}; stdout={result.stdout!r}; stderr={result.stderr!r}")
                expected_stdout = SUCCESS + "\n" if exit_code == 0 else ""
                expected_stderr = "".join(f"FAIL: {reason}\n" for reason in reasons)
                require(result.stdout == expected_stdout, f"{name}: unexpected stdout {result.stdout!r}")
                require(result.stderr == expected_stderr, f"{name}: unexpected stderr {result.stderr!r}")
                count += 1
                print(f"PASS [P0] [{count:03d}] {name}: exit={result.returncode}")
                for line in (result.stdout + result.stderr).splitlines():
                    print(f"  {line}")

            case("current sprint", text, 0)
            base = replace_state(text, "epic-129", "in-progress")
            for story in range(1, 4):
                base = replace_state(base, f"129-{story}-", "backlog")
            # Literal story-contract states, not imported production constants.
            for gate_state in (
                "backlog", "ready", "ready-for-dev", "in-progress", "review",
                "blocked", "deferred",
            ):
                for story in (2, 3):
                    for active_state in ("ready", "ready-for-dev", "in-progress", "review", "done"):
                        candidate = replace_state(base, "129-1-", gate_state)
                        candidate = replace_state(candidate, f"129-{story}-", active_state)
                        case(
                            f"129.1 {gate_state}, 129.{story} {active_state}", candidate, 1,
                            (f"129.{story} requires 129.1 NFR14 done",),
                        )
            # Satisfy declared historical prerequisites in fixtures only. The
            # gate does not prove the 129.2 -> 127.4/128.2/128.4/128.5 edges,
            # nor the 129.3 -> 129.2 dependency; those need independent audit.
            future = base
            for epic, total in ((125, 3), (126, 4), (127, 4), (128, 5)):
                future = replace_state(future, f"epic-{epic}", "done")
                for previous in range(1, total + 1):
                    future = replace_state(future, f"{epic}-{previous}-", "done")
            future = replace_state(future, "129-1-", "done")
            for story in (2, 3):
                for active_state in ("ready", "ready-for-dev", "in-progress", "review", "done"):
                    candidate = future
                    if story == 3:
                        candidate = replace_state(candidate, "129-2-", "done")
                    candidate = replace_state(candidate, f"129-{story}-", active_state)
                    case(f"legal future 129.{story} {active_state} with predecessors done", candidate, 0)
            # Isolate M0: no unrelated later active state may add diagnostics.
            candidate = replace_state(base, "epic-125", "in-progress")
            for epic, total in ((126, 4), (127, 4), (128, 5), (129, 3), (130, 3)):
                candidate = replace_state(candidate, f"epic-{epic}", "backlog")
                for story in range(1, total + 1):
                    candidate = replace_state(candidate, f"{epic}-{story}-", "backlog")
            candidate = replace_state(candidate, "epic-129", "in-progress")
            candidate = replace_state(candidate, "129-1-", "ready-for-dev")
            case("M0 not closed", candidate, 1, (
                "Epic 129 cannot advance before M0 closes",
                "129.1 cannot advance before M0 closes",
            ))
            candidate = replace_state(future, "epic-129", "done")
            case("Epic129 premature close", candidate, 1, (
                "Epic 129 done requires its stories done",
            ))
            for story in (2, 3):
                candidate = replace_state(candidate, f"129-{story}-", "done")
            case("legal future Epic129 close with all stories done", candidate, 0)
            require(count == 84, f"Expected 84 scenarios, got {count}")
    finally:
        after = SPRINT.read_bytes()
        require(after == original, "Original sprint bytes changed during test")
        require(hashlib.sha256(after).hexdigest() == digest, "Original sprint hash changed")
        print(f"PASS original sprint byte/hash preservation: {digest}")
    print(f"RESULT: {count} scenarios passed; existing GREEN gate behavior verified.")


if __name__ == "__main__":
    main()
