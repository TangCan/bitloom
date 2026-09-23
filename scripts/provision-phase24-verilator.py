#!/usr/bin/env python3
"""Provision the Story130.3 test-only Verilator tool without system mutation."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shlex
import subprocess

URL = "https://github.com/verilator/verilator.git"
TAG = "v5.052"
COMMIT = "ea338be98e1e838d3518809ce8899f85a009963c"


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--jobs", type=int, default=8)
    args = parser.parse_args()
    root = args.root.resolve()
    deps = root.parent / (root.name + "-deps")
    deps.mkdir(parents=True, exist_ok=True)
    records = []

    def run(command, **kwargs):
        result = subprocess.run(command, text=True, stdout=subprocess.PIPE,
                                stderr=subprocess.STDOUT, timeout=1800, **kwargs)
        records.append({"command": command, "exitCode": result.returncode,
                        "output": result.stdout})
        (deps / "provision-commands.json").write_text(json.dumps(records, indent=2) + "\n")
        if result.returncode:
            raise SystemExit(result.stdout)
        return result.stdout

    uris = run(["apt-get", "--print-uris", "download", "autoconf", "automake", "flex", "bison", "libfl-dev"])
    packages = []
    for row in uris.splitlines():
        url, name, size, expected = shlex.split(row)
        archive = deps / name
        # Preserve apt's exact package version and authenticated-index digest.
        url = url.replace("https://mirrors.ustc.edu.cn/ubuntu", "https://archive.ubuntu.com/ubuntu")
        run(["curl", "-fL", "--max-time", "120", url, "-o", str(archive)])
        algorithm, checksum = expected.split(":", 1)
        if hashlib.new(algorithm.lower(), archive.read_bytes()).hexdigest() != checksum:
            raise SystemExit(f"package checksum mismatch: {name}")
        packages.append({"name": name, "url": url,
                         "sha256": hashlib.sha256(archive.read_bytes()).hexdigest()})
        run(["dpkg-deb", "-x", str(archive), str(deps / "root")])
    data = deps / "root/usr/share/autoconf"
    cfg = data / "autom4te.cfg"
    cfg.write_text(cfg.read_text().replace("/usr/share/autoconf", str(data)).replace(".m4f", ".m4"))
    env = dict(os.environ, PATH=str(deps / "root/usr/bin") + ":" + os.environ["PATH"],
               AUTOM4TE=str(deps / "root/usr/bin/autom4te"), AC_MACRODIR=str(data),
               autom4te_perllibdir=str(data), trailer_m4=str(data / "autoconf/trailer.m4"),
               BISON_PKGDATADIR=str(deps / "root/usr/share/bison"),
               CPLUS_INCLUDE_PATH=str(deps / "root/usr/include"), VERILATOR_ROOT=str(root))
    if not root.exists():
        run(["git", "clone", "--depth", "1", "--branch", TAG, URL, str(root)])
    commit = run(["git", "-C", str(root), "rev-parse", "HEAD"]).strip()
    if commit != COMMIT:
        raise SystemExit(f"Verilator source identity mismatch: {commit}")
    run(["git", "-C", str(root), "diff", "--exit-code", "HEAD"])
    run(["autoconf"], cwd=root, env=env)
    run(["./configure"], cwd=root, env=env)
    run(["make", "-C", "src", f"-j{args.jobs}", "opt"], cwd=root, env=env)
    version = run([str(root / "bin/verilator"), "--version"], env=env).strip()
    record = {"url": URL, "tag": TAG, "commit": commit, "version": version,
              "packages": packages, "verilatorBinSha256": hashlib.sha256((root / "bin/verilator_bin").read_bytes()).hexdigest()}
    (deps / "provenance.json").write_text(json.dumps(record, indent=2) + "\n")
    print(json.dumps(record, indent=2))


if __name__ == "__main__":
    main()
