# Story130.3 upstream FIFO test method

Final required upstream result: **PASS**, from
`130-3-upstream-parallel.json` and its isolation/raw per-case logs. All six cases
completed exactly 100,000 upstream comparisons (600,000 total minimum), each
compile and simulation exited 0, and each simulation reached `$finish` with
`--assert` enabled. `diagnosticOnly` is false and `acceptanceEligible` is true.
The isolated gate took 664.572 seconds; case simulation times were
323.530/331.475/336.806/345.633/423.921/657.425 seconds. Seed1303, source hashes,
full upstream identities, tool/runtime hashes, complete scheduling harnesses and
their hashes, and simulation executable hashes are in that JSON. The consumed
final pilot lock SHA256 is
`49b22842deda1d09bb86baaa5c8c3474f013a52c7d587f624f827705b5141d6e`;
its snapshot is `130-3-upstream-parallel-input.lock.json`. Supplemental version
commands and matching tool hashes are in `130-3-upstream-tool-versions.json`.

The supplemental monolithic run was deliberately stopped **after** the reviewed
parallel full suite passed. Its simulator exited -15 (SIGTERM), outer runner
exited 1, and `passed` remains false. Complete inputs, executed runner, terminal
logs and cancellation reason are archived in `130-3-upstream-monolithic-cancelled/`.
It is incomplete, never PASS. No matching simulator or Z3 child remained after
its namespace exited. Initial compile failure, 900-second timeout and both
reduced-count timing calibrations remain separately labeled; none substitutes
for the final six×100,000 result.

The runner supports the original monolithic `fifo_tb` top and six parallel
instances of its original `fifo_inst_tb` test module. Both modes consume the
unchanged `common_cells` v1.40.0 `test/fifo_tb.sv`,
`src/fifo_v3.sv`, and the locked common_verification `rand_verif_pkg.sv` /
`clk_rst_gen.sv`. Every tracked file in both source trees is compared with the
canonical lock before compiling. There is no rewritten upstream driver or oracle.

`scripts/phase24-external-ip-upstream.py` creates a fresh source copy and runs
with bubblewrap network/PID isolation. Only `/usr`, standard runtime links,
`/proc`, `/dev`, the explicit Verilator installation, copied inputs and writable
scratch are mounted. The host checkout and other caches are absent. A clean
environment provides `/usr/bin:/bin` and uses `z3 --in` for constrained randomization.

The upstream testbench keeps its default 100,000 successful queue comparisons
for each of six variants: depths 1, 8 and 9, with fall-through disabled/enabled.
The seed is 1303. The runner requires all six completion messages and `$finish`,
plus successful compile and simulation exits; timeout is failure. These upstream
variants use DATA_WIDTH=8. They supplement the independent Bitloom composition
test of the frozen 32-bit/depth-8 pilot, and do not broaden its support parameters.

Verilator v5.052 was built locally from official commit
`ea338be98e1e838d3518809ce8899f85a009963c`, using extracted Ubuntu build packages
under `/tmp`. No system packages or product pins changed. Tool binary/package
hashes and build logs are in `130-3-upstream-tool-build/`. The official
[installation guide](https://verilator.org/guide/latest/install.html) documents
building a selected tag in place and using Z3 for constraints.

The first compile failed with `CONTASSINIT` on the `rand_wait` reference-clock
argument at `fifo_tb.sv:97`; original output is preserved in
`130-3-upstream-initial-failure/`. The source declares `logic clk` without an
initializer and continuously assigns it from `clk_i`. A separate Verilator
control file suppresses only this diagnostic for `*/test/fifo_tb.sv`. This is
recorded verbatim in the evidence and does not change RTL, stimulus, comparison
logic, or assertion enablement. The official
[diagnostic control documentation](https://verilator.org/guide/latest/warnings.html)
supports external control files when source must remain unchanged.

The command retains `--binary --timing --assert`. The upstream source itself
excludes one fall-through temporal assertion under `VERILATOR`; that existing
conditional is disclosed, while its queue comparisons and other assertions stay
enabled. This gate is upstream simulation evidence, not an upstream formal proof.

On Ubuntu, provision the test-only pinned tool without changing system packages:

```sh
python3 scripts/provision-phase24-verilator.py --root /tmp/bitloom-verilator-5.052
```

The bootstrap requires `git`, `curl`, `g++`, `make`, `m4`, `apt-get` and
`dpkg-deb`; simulation additionally needs `z3` and `bwrap`. It records exact
APT package URLs/versions and hashes, validates the package-index checksum,
and writes every bootstrap command and output beside the tool tree. The
initial run's preserved package SHA256s and commands are the reproduction
reference; different package-index revisions must produce new provenance.

Run the gate with:

```sh
python3 scripts/phase24-external-ip-upstream.py \
  --lock ip/external/pulp-common-cells-fifo-v3.source.lock.json \
  --cache /tmp/bitloom-130-2-transported-cache \
  --verilator-root /tmp/bitloom-verilator-5.052 \
  --work /tmp/bitloom-130-3-upstream-parallel \
  --timeout 3600 --parallel-cases \
  --evidence _agile-output/test-artifacts/130-3-upstream-parallel.json
```

The JSON is authoritative for pass/failure, six check counts, exact commands,
source/tool hashes, seed, duration and raw output. This method document alone
does not claim that the gate passed.

The initial full-default simulation reached the 900-second timeout; its terminal
exit 124 and complete original input lock are preserved in
`130-3-upstream-900s-timeout/`. It was not reported as passing. A retry uses an
explicit 3600-second bound and the final pilot lock/cache, with the same seed and
unchanged six default check counts. Solver I/O during the first run grew by
thousands of responses per second, establishing active constraint-solving rather
than a single hung query. The retry logs are line-buffered so each upstream
completion can be observed as it occurs.

A separate timing calibration (`130-3-upstream-diagnostic1000.json`) overrides
only the upstream `N_CHECKS` parameter to 1000. It is explicitly
`diagnosticOnly: true`, `acceptanceEligible: false`, and is never accepted as
the full upstream gate. All six calibration variants completed 1000 comparisons
in 29.171 seconds of simulation; this predicts roughly 49 minutes for the
unchanged default 100,000 counts and supports a finite 3600-second deadline.
Production Just/CI invocations use `--parallel-cases` and omit
`--diagnostic-checks`.

## Optional parallel upstream-instance execution

`--parallel-cases` compiles six separate thin scheduling harnesses. Each harness
instantiates the original, unchanged `fifo_inst_tb` (including its random drivers,
queue model and assertions) and original `clk_rst_gen`. It preserves depth
1/8/9 × fall-through off/on, DATA_WIDTH=8, N_CHECKS=100000, TCLK=10ns,
TA=TCLK*1/4, TT=TCLK*3/4 and ten reset clock cycles. Its only procedural code
waits for the upstream `done_o` and finishes. No new oracle or assertion replaces
the upstream test. Each harness text/hash, executable hash, compile/run command,
tool/source identity, seed and upstream completion count is recorded.

Each process starts seed1303 independently. This changes the random sequence
relative to the monolithic top's interleaved RNG calls, and is disclosed rather
than claiming identical stimulus traces. Each process also stops at its own
100,000-comparison completion; in the monolithic top, earlier completed cases
continue running until the final case finishes. The equivalence is the six
configurations, unchanged driver/oracle, and minimum required comparison counts,
not identical traces or extra post-completion traffic. All six upstream assertion checks still
require 100,000 successful comparisons each, with `--assert` enabled and a normal
finish. The original monolithic run remains separate evidence.

The parallel diagnostic calibration `130-3-upstream-parallel-diagnostic1000.json`
is explicitly ineligible for acceptance. It passed six×1000 comparisons in
3.27–6.33 seconds per case; its full-count estimate is roughly 10.5 minutes for
the slowest case. The full-count parallel run uses a finite 1800-second timeout;
its authoritative result is `130-3-upstream-parallel.json`.
