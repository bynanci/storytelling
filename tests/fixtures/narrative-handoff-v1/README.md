<!-- SPDX-License-Identifier: MIT -->
# Narrative handoff V1 fixtures · N-11 / N-R11 / #68

Synthetic umbrella-workshop notice, image description and sound description from
`skills/storytelling/references/workshop-demo.md`. Original assets remain in the
Skill package under MIT; the manifest only declares their paths and digests.
`expected-impact.json` was written before running the new CLI.

| Input | Expected result |
| --- | --- |
| `before-v1.json` | Accept: S1@1→2 changes four date-bearing components; four draft variants need review. Layout and WAV are reusable. |
| `after-v1.json` | Accept: matching variant reviews retain history and supply Sunday copy; variants are current. |
| `descriptions-only-v1.json` | Accept: usable text/brief from supplied descriptions, with original files unavailable. No audio playback or image inspection is claimed. |
| `missing-source-v1.json` | Reject `handoff.dangling_reference`: component names nonexistent source. |
| `stale-revision-v1.json` | Reject `handoff.stale_revision`: descriptor supplies S1@1 while source history is at S1@2. |
| `proposal-leak-v1.json` | Reject `handoff.variant_scope`: poster-only C1 is also explicitly referenced by oral-info. |

“Before” means drafts before their date update, after the source-change event.
The text files show source notice before/after; the JSON ledger retains old and
reviewed draft snapshots. C1 is a fixture author choice, not an observed object.
The original WAV contains synthetic taps and no spoken transcript. Tests for a
transcript representation use a supplied non-speech annotation with missing
original recording, not a claim that the tool transcribed audio.
