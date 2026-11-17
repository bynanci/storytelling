<!-- SPDX-License-Identifier: MIT -->
# Consumer SDK V1 integration fixture

`proposal-response.json` is the exact 1,573-byte stdout, including its final LF, from the unchanged baseline CLI at `0b39c00354f343d7a61821a029777d88a8373254`:

```sh
storytelling plan-story tests/fixtures/story-review-v1/planner-request.json
```

It is frozen once for both CLI and SDK preparation. Do not independently serialize or regenerate it on either side of the comparison. [Baseline receipts](../../../docs/validation/m4-01-readonly-sdk/baseline.json) record the locally built executable identity and exact output hashes; the baseline bundle and OTIO stdout are in that same directory. CLI output adds one LF to the backend payload; tests compare that one framing byte explicitly.

This is independently written first-party MIT integration data/glue, not a new OpenFilm port. Existing source-informed composition/context/test fixtures are referenced in place under their existing provenance and licenses. No upstream code, private media, actual approval, verified media identity or Host observation is imported here.
