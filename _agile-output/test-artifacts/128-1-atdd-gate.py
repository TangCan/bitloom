#!/usr/bin/env python3
"""Story 128.1 AC5: 35 existing-GREEN process gate scenarios.

No future peripheral/product acceptance is claimed. Cross-story dependencies
are fixture setup, not coverage provided by check_phase24_gate.py. User AC5
requires actual existing checks, so these tests are active with no fabricated
RED/skip phase. Python standard library only; run from any directory.
"""

import hashlib
from pathlib import Path
import re
import subprocess
import sys
import tempfile


if not __debug__:
    raise SystemExit("FAIL: optimized Python disables acceptance assertions; rerun without -O/PYTHONOPTIMIZE")

ROOT = Path(__file__).resolve().parents[2]
SPRINT = ROOT / "_agile-output/implementation-artifacts/sprint-status.yaml"
GATE = ROOT / "scripts/check_phase24_gate.py"
SUCCESS = (
    "PASS: Phase 24 6 epics / 22 stories; M0 and NFR14 gates; "
    "FR189/Epic122 deferred"
)


def replace_state(text, prefix, state):
    suffix = r"[^:\n]*" if prefix.endswith("-") else ""
    pattern = rf"^(  {re.escape(prefix)}{suffix}:[ \t]*)[\w-]+([ \t]*(?:#.*)?)$"
    result, count = re.subn(pattern, lambda m: m[1] + state + m[2], text, flags=re.M)
    assert count == 1, f"Expected exactly one status for {prefix!r}, got {count}"
    return result


def main():
    original = SPRINT.read_bytes()
    digest = hashlib.sha256(original).hexdigest()
    text = original.decode("utf-8")
    count = 0
    print(f"Source sprint SHA256: {digest}")
    print("Scope: existing GREEN process gate, not FR197 product acceptance.")
    print("Cross-story dependencies are fixture setup, not gate coverage.")
    try:
        with tempfile.TemporaryDirectory(prefix="bitloom-128-1-atdd-") as directory:
            fixture = Path(directory) / "sprint-status.yaml"

            def case(name, content, exit_code, reasons=()):
                nonlocal count
                fixture.write_text(content, encoding="utf-8")
                result = subprocess.run(
                    [sys.executable, str(GATE), str(fixture)],
                    capture_output=True, text=True, check=False, timeout=30,
                )
                # Exact status and complete output prevent unrelated crashes or
                # extra gate failures from masquerading as a passing negative.
                assert result.returncode == exit_code, (
                    name, result.returncode, result.stdout, result.stderr
                )
                expected_stdout = SUCCESS + "\n" if exit_code == 0 else ""
                expected_stderr = "".join(f"FAIL: {reason}\n" for reason in reasons)
                assert result.stdout == expected_stdout, (name, result.stdout)
                assert result.stderr == expected_stderr, (name, result.stderr)
                count += 1
                print(f"PASS [P0] [{count:03d}] {name}: exit={result.returncode}")
                for line in (result.stdout + result.stderr).splitlines():
                    print(f"  {line}")

            case("current sprint", text, 0)
            base = replace_state(text, "epic-128", "in-progress")
            for story in range(1, 6):
                base = replace_state(base, f"128-{story}-", "backlog")
            for state in (
                "backlog", "ready", "ready-for-dev", "in-progress", "review",
                "blocked", "deferred",
            ):
                for story in range(2, 6):
                    candidate = replace_state(base, "128-1-", state)
                    candidate = replace_state(candidate, f"128-{story}-", "ready-for-dev")
                    case(
                        f"128.1 {state}, 128.{story} ready-for-dev", candidate, 1,
                        (f"128.{story} requires 128.1 NFR14 done",),
                    )
            for story in range(2, 6):
                candidate = base
                # M0, M1 and M2 are explicit fixture prerequisites. The gate
                # does not independently validate 127.2/126.2/126.4 edges.
                for epic, total in ((125, 3), (126, 4), (127, 4)):
                    candidate = replace_state(candidate, f"epic-{epic}", "done")
                    for previous in range(1, total + 1):
                        candidate = replace_state(candidate, f"{epic}-{previous}-", "done")
                candidate = replace_state(candidate, "128-1-", "done")
                if story in (4, 5):
                    candidate = replace_state(candidate, "128-3-", "done")
                candidate = replace_state(candidate, f"128-{story}-", "ready-for-dev")
                case(f"128.{story} ready with declared predecessors done", candidate, 0)
            # Isolate M0 failure so the exact diagnostic is stable even after
            # later epics advance; these temporary states never touch sprint.
            candidate = replace_state(base, "epic-125", "in-progress")
            for epic, total in ((126, 4), (127, 4), (128, 5), (129, 3), (130, 3)):
                candidate = replace_state(candidate, f"epic-{epic}", "backlog")
                for story in range(1, total + 1):
                    candidate = replace_state(candidate, f"{epic}-{story}-", "backlog")
            candidate = replace_state(candidate, "epic-128", "in-progress")
            candidate = replace_state(candidate, "128-1-", "ready-for-dev")
            case("M0 not closed", candidate, 1, (
                "Epic 128 cannot advance before M0 closes",
                "128.1 cannot advance before M0 closes",
            ))
            candidate = replace_state(base, "128-1-", "done")
            candidate = replace_state(candidate, "epic-128", "done")
            case("Epic128 premature close", candidate, 1, (
                "Epic 128 done requires its stories done",
            ))
            assert count == 35, f"Expected 35 scenarios, got {count}"
    finally:
        after = SPRINT.read_bytes()
        assert after == original, "Original sprint bytes changed during test"
        assert hashlib.sha256(after).hexdigest() == digest, "Original sprint hash changed"
        print(f"PASS original sprint byte/hash preservation: {digest}")
    print(f"RESULT: {count} scenarios passed; existing GREEN gate behavior verified.")


if __name__ == "__main__":
    main()
