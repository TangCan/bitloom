#!/usr/bin/env python3
"""Compile and execute the exact FR196 documentation with prelude-only dependencies."""
import sys

sys.dont_write_bytecode = True
from check_fr194_example import check_example

if __name__ == '__main__':
    check_example('docs/ip/csr.md', 'FR196')
