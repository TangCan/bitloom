#!/usr/bin/env python3
"""Compile and run the exact prelude-only four-window CSR documentation example."""
import sys

sys.dont_write_bytecode = True
from check_fr194_example import check_example

if __name__ == '__main__':
    check_example('docs/ip/csr-decoder.md', 'FR196-DECODER')
