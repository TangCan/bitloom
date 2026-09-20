#!/usr/bin/env python3
"""Compile and execute the documented FR194 design with only a prelude dependency.

Run after workspace dependencies are fetched (CI runs this after cargo test).
Uses the local toolchain and offline registry cache; never publishes anything.
"""
from pathlib import Path
import json
import os
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def main():
    document = (ROOT / 'docs/ip/module-composition.md').read_text()
    blocks = document.split('```rust\n')
    if len(blocks) != 2 or '```' not in blocks[1]:
        raise RuntimeError('expected exactly one complete rust design example')
    source = blocks[1].split('```', 1)[0]
    with tempfile.TemporaryDirectory(prefix='bitloom-fr194-example-') as temp:
        project = Path(temp)
        (project / 'src').mkdir()
        (project / 'src/main.rs').write_text(source)
        dependency = json.dumps(str(ROOT / 'crates/bitloom-prelude'))
        (project / 'Cargo.toml').write_text(
            '[package]\nname="fr194-design-example"\nversion="0.0.0"\nedition="2024"\n'
            f'\n[dependencies]\nbitloom-prelude={{path={dependency}}}\n'
        )
        environment = os.environ.copy()
        environment['CARGO_TARGET_DIR'] = str(ROOT / 'target/fr194-doc-example')
        command = ['cargo', 'run', '--offline', '--manifest-path', str(project / 'Cargo.toml')]
        print('FR194 prelude-only example:', ' '.join(command), flush=True)
        subprocess.run(command, env=environment, check=True, timeout=180)
    print('PASS: FR194 documented prelude-only design compiled and executed')


if __name__ == '__main__':
    main()
