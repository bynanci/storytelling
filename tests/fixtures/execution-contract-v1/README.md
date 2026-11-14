<!-- SPDX-License-Identifier: MIT -->
# Execution V1 synthetic design cases

These independently authored MIT files specify expected future behavior for
[M3-04](../../../docs/validation/m3-04-execution-contract.md). They are design
fixtures, not accepted execution permissions or observed host results.

- `valid-request.json`: proposed closed wire shape; every digest is fabricated
  from a labeled synthetic string, not a real P2 compilation or host receipt.
- `identity-vector.json`: exact canonical payload/byte count/framed SHA-256.
- `applied-result.json` / `receipt-vector.json`: fabricated result/receipt shape
  and framing vector; no host observation is represented by these values.
- `cases.json`: positive/negative cases, required diagnostics/paths, fake call
  counts and scenario setup for future trusted test harnesses. A harness scenario
  is not a request field or caller-provided authority assertion.
- `duplicate-json.json`: deliberately raw duplicate field; preserve its bytes.
- `unknown-version.json`: future version; the parser must probe version before
  complaining about missing domain fields.
- Remaining wire files isolate null/unknown/hash/size/depth/integer defects;
  byte-boundary cases generate inter-token whitespace in memory. EX01–EX37 fix individual
  expected scenarios; subcases explicitly named in notes become focused G3 tests.
- `check_design.py`: design-integrity checks only, with no process/host calls.

Run `python3 tests/fixtures/execution-contract-v1/check_design.py` from the repo
root. Passing checks do not establish that future Rust validation, authority,
single-writer storage or readback has been implemented. The planned G3 tests and
their exact commands are in the linked handoff. No media or private paths belong
in these fixtures. Do not copy the old host proof into the pure engine.
