# M3-03 isolated FilmCraft execution slice

Status: a test-only synthetic execution/readback proof is implemented. The
production executor, trusted approval gate and multi-platform host matrix remain
open; this is not acceptance of the whole M3-03 milestone. See the
[requirements](../REQUIREMENTS.md#m3-03-isolated-filmcraft-execution),
[work index](m3-03-filmcraft-work-index.json) and
[validation record](../validation/m3-03-filmcraft-execution.md).

## Boundary and sequence

The bounded profile is one video cut, one new FilmCraft v0.18.0 Linux project,
one explicitly selected OTIO artifact and one writer. The harness uses synthetic
media and a fresh, absolute run root. It refuses an existing root and opens no
existing FilmCraft project or OpenFilm Catalog.

For each host mutation, the integration harness persists an OperationIntentV1,
claims it, and commits begin_attempt before starting FilmCraft. Project
creation and importing the cut are separate journal operations. Each identity
stores the project ID, current review-binding digest and exact OTIO artifact
digest. The operation journal contains no file paths or media.

    fresh proposal + snapshot + context
            -> recompile and check current bundle
            -> persist create-project intent -> begin attempt -> create empty project
            -> inspect empty project -> record applied/readback
            -> persist import intent -> begin attempt -> import OTIO once
            -> suppress the successful host reply
            -> persist outcome_unknown
            -> inspect native project and sequence
            -> record applied_observed

The lost-reply injection waits for the FilmCraft process to exit successfully,
then holds the wrapper response. The caller sees no acknowledgement, persists
outcome_unknown, and reads the resulting project before recording
applied_observed. A late response is rejected by the journal and a terminal
operation cannot begin another attempt. The harness sends neither a second
import nor Undo.

An Applied readback is accepted only after the test checks the native project
has an active sequence, one video clip, the exact 25 fps timebase, 25 timeline
frames, source ticks 1/25 through 26/25 seconds, and online media pointing to
the synthetic source under the same run root. The exact selected planner proposal
is checked against the generated OTIO bytes before the host call.

## What this does and does not prove

The freshness check means the supplied proposal, snapshot, context, bundle and
artifact agree. It does not prove human approval. The observed result has
approval_granted=false, executable=false and source_verified=false.
The PR #28 independent G2 review and explicit candidate acceptance remain
separate, pending gates.

This proof exercises a test-only adapter in an ignored Rust integration test. It
does not add an editor-writing command or production executor to the CLI. It
does not simulate FilmCraft hanging during the host operation; the injected
failure is a successful host exit with a lost acknowledgement. It does not test
Windows/macOS FilmCraft, trusted signer authorization, in-flight host requests,
safe NotApplied retry, audio/effects, user footage, or exactly-once behavior.
The existing journal regressions for ambiguous readback, stale owners, restart
recovery and late responses run in the Linux/Windows/macOS CI jobs.

Production M3-03 must still establish host-process quiescence or a host
idempotency key before retrying after NotApplied. Ambiguous or partial readback
must remain blocked. M4 stays behind the accepted executor/readback gate and the
one-writer policy.

## Reproduction

Follow the exact command and safety preconditions in the
[oracle README](../../tests/oracles/m3-03-filmcraft-v1/README.md). The run
produces a SQLite database, readback JSON, logs and a machine-readable evidence
record under the new run root. Preserve that root if a proof fails; never resend
its import or reuse the root.
