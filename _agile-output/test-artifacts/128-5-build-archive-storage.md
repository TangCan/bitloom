# UART consolidated evidence storage

The immutable archive was verified before storage splitting:3196 members and archiveSHA256 `14360ff18ea57e3c1719733d81459f58efc7985064e7590e69ee951bc2bc1cc0`. Its original member manifest remains `128-5-build-raw-consolidated.manifest.json`. The selected run is `target/fr197-uart-reruns/20260921T133203.691419Z-1329506`, all39 commands passed with570 source fingerprints identical at start/end/current. Root independently checked original bytes in `128-5-root-raw-audit.json`.

The187582296-byte tar exceeds100MiB. No existing archive split convention was found by searching tracked and untracked test-artifact names and repository scripts/docs, so it is stored in three deterministic numbered parts, each at most64MiB. `128-5-build-raw-consolidated.chunks.json` records exact ordered names, lengths and SHA256. This is only storage transformation: runner/archive/test sources and the certified source fingerprint remain unchanged. The original full tar is preserved locally at `/tmp/128-5-build-raw-consolidated.tar.gz` for the root auditor and is not staged in the repository.

From the repository root, reconstruct without overwriting an existing destination:

```sh
(set -C; cat \
  _agile-output/test-artifacts/128-5-build-raw-consolidated.tar.gz.part000 \
  _agile-output/test-artifacts/128-5-build-raw-consolidated.tar.gz.part001 \
  _agile-output/test-artifacts/128-5-build-raw-consolidated.tar.gz.part002 \
  > /tmp/uart-build-reassembled.tar.gz)
sha256sum /tmp/uart-build-reassembled.tar.gz
```

The output must be `14360ff18ea57e3c1719733d81459f58efc7985064e7590e69ee951bc2bc1cc0` before reading the tar. An actual independent `cat part000 part001 part002 | sha256sum` returned that digest after splitting, in addition to per-part reread and streaming digest verification. Use the original manifest to check every extracted member. Do not replace old evidence or use historical PASS unions to certify another run.
