#!/usr/bin/env python3
"""Story130.2 acceptance: empty-cache fetch and isolated offline replay."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
EXPECTED_COMMIT = "1281545696eb3fcba50ec5b4275993476a3c710e"


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(*args: str, env=None):
    return subprocess.run(
        args,
        cwd=ROOT,
        env=env,
        text=True,
        capture_output=True,
        timeout=180,
        check=False,
    )


class ReplayAcceptance(unittest.TestCase):
    def setUp(self):
        self.tmp = Path(tempfile.mkdtemp(prefix="bitloom-fr199-atdd-"))
        self.addCleanup(shutil.rmtree, self.tmp, True)
        self.binary = self.tmp / "cargo-bitloom"
        shutil.copy2(ROOT / "target/debug/cargo-bitloom", self.binary)
        previous_binary = os.environ.get("BITLOOM_BIN")
        os.environ["BITLOOM_BIN"] = str(self.binary)
        self.addCleanup(lambda: os.environ.pop("BITLOOM_BIN", None) if previous_binary is None else os.environ.__setitem__("BITLOOM_BIN", previous_binary))
        self.manifest = (
            ROOT / "ip" / "external" / "pulp-common-cells-fifo-v3.source.json"
        )
        self.lock = (
            ROOT
            / "ip"
            / "external"
            / "pulp-common-cells-fifo-v3.source.lock.json"
        )
        self.runner = ROOT / "scripts" / "phase24-external-ip-replay.py"
        self.assertTrue(self.runner.is_file(), f"missing FR199 runner: {self.runner}")
        self.assertTrue(self.manifest.is_file(), f"missing manifest: {self.manifest}")
        self.assertTrue(self.lock.is_file(), f"missing lock: {self.lock}")
        self.canonical = {path: sha(path) for path in (self.manifest, self.lock)}
        copied_manifest = self.tmp / "source.json"
        shutil.copy2(self.manifest, copied_manifest)
        self.manifest = copied_manifest
        # A fetch is allowed to create this fixture lock; never rewrite the
        # canonical checked-in lock (which binds a particular CLI executable).
        self.lock = self.tmp / "source.lock.json"

    def tearDown(self):
        for path, digest in getattr(self, "canonical", {}).items():
            self.assertEqual(digest, sha(path), f"canonical artifact mutated: {path}")

    def test_p0_empty_cache_fetch_then_movable_network_isolated_replay(self):
        online_cache = self.tmp / "online-cache"
        evidence = self.tmp / "online-evidence.json"
        online = run(
            "python3",
            str(self.runner),
            "fetch",
            "--manifest",
            str(self.manifest),
            "--lock",
            str(self.lock),
            "--cache",
            str(online_cache),
            "--evidence",
            str(evidence),
        )
        self.assertEqual(0, online.returncode, online.stdout + online.stderr)

        copied_cache = self.tmp / "offline-cache"
        shutil.copytree(online_cache, copied_cache)
        offline = run(
            "python3",
            str(self.runner),
            "replay",
            "--manifest",
            str(self.manifest),
            "--lock",
            str(self.lock),
            "--cache",
            str(copied_cache),
            "--require-commit",
            EXPECTED_COMMIT,
            "--compile",
        )
        self.assertEqual(0, offline.returncode, offline.stdout + offline.stderr)
        self.assertIn("network=denied", offline.stdout)
        self.assertIn("hdl_compile=passed", offline.stdout)

    def test_p0_network_and_host_cache_fallback_are_rejected(self):
        empty = self.tmp / "empty-cache"
        empty.mkdir()
        env = {
            "PATH": os.environ.get("PATH", ""),
            "BITLOOM_BIN": str(self.binary),
            "HOME": str(self.tmp / "empty-home"),
            "XDG_CACHE_HOME": str(self.tmp / "empty-xdg"),
        }
        # This negative needs valid tool identity, so create an independent
        # lock first and then replay with a genuinely empty cache.
        fetched = run("python3", str(self.runner), "fetch", "--manifest", str(self.manifest),
                      "--lock", str(self.lock), "--cache", str(self.tmp / "provisioned-cache"))
        self.assertEqual(0, fetched.returncode, fetched.stdout + fetched.stderr)
        out = run(
            "python3",
            str(self.runner),
            "replay",
            "--manifest",
            str(self.manifest),
            "--lock",
            str(self.lock),
            "--cache",
            str(empty),
            "--compile",
            env=env,
        )
        self.assertNotEqual(0, out.returncode, "missing offline cache must not refetch")
        self.assertTrue(
            "cache" in out.stderr.lower() or "offline" in out.stderr.lower(),
            out.stderr,
        )

    def test_p0_path_escape_links_and_undeclared_members_fail_closed(self):
        cache = self.tmp / "cache"
        lock = self.tmp / "source.lock.json"
        fetched = run(
            "python3",
            str(self.runner),
            "fetch",
            "--manifest",
            str(self.manifest),
            "--lock",
            str(lock),
            "--cache",
            str(cache),
        )
        self.assertEqual(0, fetched.returncode, fetched.stdout + fetched.stderr)
        value = json.loads(lock.read_text())
        root = cache / value["sources"][0]["cachePath"]

        extra = root / "undeclared.sv"
        extra.write_text("module undeclared; endmodule\n")
        out = run(
            str(self.binary),
            "external-ip", "verify", "--manifest", str(self.manifest),
            "--lock", str(lock), "--cache", str(cache), "--offline",
        )
        self.assertNotEqual(0, out.returncode)
        self.assertIn("undeclared-file", out.stderr)
        extra.unlink()

        if hasattr(os, "symlink"):
            escape = root / "escape"
            escape.symlink_to("../../../../outside")
            out = run(
                str(self.binary),
                "external-ip", "verify", "--manifest", str(self.manifest),
                "--lock", str(lock), "--cache", str(cache), "--offline",
            )
            self.assertNotEqual(0, out.returncode)
            self.assertIn("symlink", out.stderr)
            escape.unlink()

        original = lock.read_bytes()
        for invalid in ("../escape", "/absolute/escape"):
            mutated = json.loads(original)
            mutated["sources"][0]["cachePath"] = invalid
            lock.write_text(json.dumps(mutated))
            out = run(
                str(self.binary),
                "external-ip", "verify", "--manifest", str(self.manifest),
                "--lock", str(lock), "--cache", str(cache), "--offline",
            )
            self.assertNotEqual(0, out.returncode)
            self.assertIn("path", out.stderr)
        lock.write_bytes(original)


def main() -> int:
    parser = argparse.ArgumentParser()
    # Retained as a compatibility no-op for the original RED activation command.
    parser.add_argument("--run-red", action="store_true")
    _args, _remaining = parser.parse_known_args()
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(ReplayAcceptance)
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    return 0 if result.wasSuccessful() else 1


if __name__ == "__main__":
    raise SystemExit(main())
