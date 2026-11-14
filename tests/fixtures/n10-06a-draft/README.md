<!-- SPDX-License-Identifier: MIT -->
# N10-06A candidate acceptance fixtures

These are the draft-only G2 acceptance assertions; the [requester decision](../../../docs/validation/n10-06a-g2-decision.md) accepts candidate `f148b6d` for focused A implementation.
[ADR 0012](../../../docs/adr/0012-draft-exact-video-range-input.md) controls the candidate.
Observed implementation receipts are recorded separately in [the validation record](../../../docs/validation/n10-06a-exact-input.md).

The complete synthetic context is copied from
`tests/fixtures/narrative-video-coverage-v1/valid-unresolved.json` at repository
`bynanci/storytelling@52dc6c315f7b3d20693f3c51e052509997ab791b`, MIT.
New helper forms, acceptance vectors and expected exact ranges are authored for
#129. No OpenFilm implementation or third-party test was imported.
No real names, local media paths or media bytes occur in these fixtures.

- `*.draft.json`: opt-in draft helper inputs (enable `draft-video-input`).
- `*.expected-v1.json`: proposed normalization results using the **unchanged**
  accepted V1 fields. These can be checked today with `render-video-coverage`.
  They remain unresolved for S03; the declared catalog duration is 12 seconds.
- `cases.json`: frozen draft-only test vectors for the future pure converter and
  normal CLI tests. Expected results are assertions, not observations.
- `beyond-duration.draft.json`: declares duration 5 seconds and OUT=6;
  after conversion the existing V1 must return `video_coverage.invalid_range`.

The seconds and 30 fps expected V1 documents must be byte-identical because
both express [5/2,9/2). The NTSC example maps [0,150) at 30000/1001 fps to
[0/1,1001/200) seconds. The supplied fps/timing are declarations only.
