<!-- SPDX-License-Identifier: MIT -->
# NarrativeStyleProfileV1

This profile is a separate, versioned file for checkable editorial structure.
It does not add fields to, or change the closed shape of,
[NarrativeDocumentV1](NARRATIVE_DOCUMENT_V1.md). Its wire schema is
[narrative-style-profile-v1.schema.json](../../contracts/narrative-style-profile-v1.schema.json).

The profile binds to one exact **document_id** and **document_revision**. Each
rule names a spoken variant and two beat IDs. The current profile version
supports one rule kind, **must_precede**. A rule passes when the first named beat
appears before the second in that variant's declared beat array. The check reads
no media and does not change the narrative or profile.

Rules from different variants are independent. A cycle within one variant is a
contradictory profile and is rejected with the cycle's rule IDs. An acyclic rule
that disagrees with the document's actual beat order is also rejected. Unknown
variants or beat IDs, non-spoken variants, duplicate keys, unknown fields and
unknown profile versions return stable diagnostics.

Run the local check from the repository root:

    cargo run --release --quiet --locked -p storytelling-cli -- check-narrative-style \
      tests/fixtures/narrative-v1/n08-umbrella-openings-v1.json \
      tests/fixtures/narrative-v1/n08-style-profile-v1.json

The two JSON inputs are each limited to 1 MiB. Exit 0 means the declared rules
match the declared order; exit 1 reports a data/profile diagnostic, 2 is
incorrect command usage, and 3 is an I/O failure. The report scope is only
**declared-beat-order-only**. It does not decide whether a draft is documentary,
poetic, compelling, accurate beyond the input declarations, or suitable for an
audience. Human review remains separate.

Stable profile diagnostics include **narrative.style.unsupported_version**,
**narrative.style.unknown_field**, **narrative.style.duplicate_field**,
**narrative.style.document_mismatch**, **narrative.style.stale_document**,
**narrative.style.dangling_reference**, **narrative.style.invalid_rule_scope**,
**narrative.style.order_conflict**, and
**narrative.style.order_violation**. Error paths identify the safe profile field;
cycle errors also list the conflicting rule IDs in declaration order.