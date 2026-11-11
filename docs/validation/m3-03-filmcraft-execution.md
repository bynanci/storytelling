# M3-03 isolated FilmCraft proof record

Status: bounded test-only host proof passed locally; M3-03 production execution
remains open. This record does not grant approval or authorize user-project edits.

| Item | Evidence |
| --- | --- |
| Base / branch | Base main SHA  d259196c0a356b547e86e4648e270af7bd2453b3; branch codex/m3-03-filmcraft-execution-20261111 |
| Host | Official FilmCraft CLI v0.18.0; binary SHA-256 064a1f55cde5f6e1ac04bcd3a5b72e5d7366905f3d7333793e78dda0c109d33f |
| Run root | /workspace/scratch/m3-03-filmcraft-20261111-10; unique and absent before run |
| Source / review | Synthetic source SHA-256 ecced2fea2cd8e5398a973fbb20f800c0aad00e0eff18ccf4ae02727b7120a21; review binding bf559675138cb55e9cc91feafc809d12b87e3eafa22ea4c6b7d2ba1ad332950f; OTIO artifact 9d3404c3a85d74516417751ffb01640b2971e6342de879e9a260ebbf63c30309 |
| Review freshness | Current = true; approval_granted = false; executable = false; source and review inputs unchanged |
| Host readback | One online native clip; 25 frames at 25/1 fps; source range 1/25 through 26/25 seconds; original empty project hash unchanged |
| Lost acknowledgement | Host child exited successfully; caller persisted outcome_unknown; native readback resolved applied_observed; a late response was rejected |
| Side-effect count | One import attempt; zero retries; zero Undo |
| Machine evidence | [evidence.json](../../tests/oracles/m3-03-filmcraft-v1/evidence.json), SHA-256 669202f428c3b0a3a751cd1dcc29608f974915921453bc7346368c5ba913dbee |
| Oracle sources | Runner SHA-256 5ef76ffc681931b557632f2a8cb51e10765640e5219687fc6e241e93e16bbf09; Rust test SHA-256 37170ec6a38a6be52d0f1e1ed1c58cc5f6fa5bbccddf4a983b4bdf42a69b829b |

## Test-first findings

The integration assertions caught two real boundaries during the isolated proof:
integer-second cuts caused FilmCraft to create a 1 fps sequence, and checking
the inspect-project summary did not inspect the native project file. The fixture
was changed to use exact 25 fps source ticks and the readback now checks both host
inspection output and the project bytes. Final isolated run 10 passed. Earlier
roots were never reused; any already-imported test project remained in its own
run root and was not re-sent or undone.

## Local verification

- cargo test --offline -p storytelling-journal --test journal: 11 passed.
- cargo test --offline --locked --workspace and the release-mode workspace run: passed; the host integration test is ignored by default and ran separately through the oracle.
- cargo test --locked -p storytelling-journal --test filmcraft_execution --no-run: compiled the manual host test.
- python3 tests/oracles/m3-03-filmcraft-v1/run.py ...: passed on the pinned Linux host; complete command paths and requirements are in the oracle README.
- cargo fmt --all -- --check, cargo clippy --workspace --all-targets --locked -- -D warnings, and rustdoc with warnings denied: passed.
- python3 scripts/check_repo.py and git diff --check: passed.
- Cross-platform journal regressions are part of the Linux, Windows and macOS workspace jobs. The actual pinned FilmCraft execution is Linux-only and manual.

## Remaining gates

The proof withholds a response after the host exits; it does not exercise a
FilmCraft process that hangs mid-operation, establish production process-group
quiescence, or prove a NotApplied retry safe. No retry is issued in the proof.
The ignored integration test adds no product executor command. Production
approval/authorization, recoverable host process supervision, ambiguous/partial
readback UI, user-project protection, and host coverage outside this Linux
profile remain open. PR #28 independent G2 review and explicit candidate
acceptance are pending and are not substitutes for these gates.
