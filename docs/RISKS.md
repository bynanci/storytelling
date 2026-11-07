# Risk register

| ID | Risk | Mitigation / cancellation condition | Owner / gate |
| --- | --- | --- | --- |
| R01 | Language rewrite replaces product delivery | First prove finite FilmCraft roundtrip; retain TS planner if port cannot converge | Maintainer / M1,M2 |
| R02 | New repo MIT conflicts with later imported source terms | Preserve license, no copied implementation in M0, provenance review before port | Maintainer / G2 |
| R03 | Source/timeline/frame/sample clocks drift | Exact time wrappers, explicit host quantization and readback; unsupported fails closed | Engineer / M1 |
| R04 | Agent or external text authorizes edits | Trusted approval surface and capability separation; no execution in M0 | Security reviewer / M3 |
| R05 | Host timeout causes duplicate/partial edits | New isolated project, durable attempt IDs, unknown -> reconcile, no blind replay | Runtime engineer / M3 |
| R06 | Tests mistaken for real-media or compatibility evidence | Evidence levels and separate manual/host gates | Reviewer / G4,G5 |
| R07 | Many adapters and bindings overwhelm maintenance | One host then a measured second; TS process client before MCP/WASM | Maintainer / M4,M5 |
| R08 | Dependency/toolchain drift or unsupported binary | Lock dependencies, test pinned compiler, review updates before release; no latest-version claim | Maintainer / G4,G5 |
| R09 | Repo policy exists only on paper | Verify actual branch rules/private reporting separately; no auto administrative mutation | Owner / M0-G01 |
| R10 | No measurable advantage over direct FilmCraft + Agent | Fixed-task user evaluation; narrow SDK scope or stop expansion when no benefit | Product owner / M5 |

Each task/PR must cite relevant risk IDs and record new failure evidence. An
unresolved host, legal or hardware gate is not satisfied by a passing mock.
