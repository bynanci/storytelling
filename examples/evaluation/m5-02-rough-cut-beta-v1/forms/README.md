<!-- SPDX-License-Identifier: MIT -->
# Blank facilitator forms

These files contain headers/templates only. Copy them to access-controlled private storage before collecting anything. Do not fill them in this repository.

- `participants.csv`: one row per assigned slot; never add name or contact.
- `allocation.csv`: planned and actual condition/pack order. Keep the random seed private until the round is finished.
- `sessions.csv`: one row per participant-condition; timeout is 2,700 seconds and incomplete.
- `scorecard.csv`: one row per requirement and arm; separately count missed, wrong, extra/collateral edits and rework; use pass, fail or not_observed.
- `process-events.csv`: per-call tool/host timing, separate from author time.
- `ratings.csv`: one row per question and arm. Do not quote without consent.
- `follow-up.csv`: one row per enrolled participant; separately track follow-up invitation/response and actual safe completion of a second project in S during days 14–28. Intent is not actual return.
- `failures.csv`: index to the detailed private `failure-record.md`.
- `deviations.csv`: preserve planned versus actual procedure.
- `pack-freeze.csv`: one row per asset and source card. Use exact rational ranges; use neutral IDs and SHA-256, never local paths.
- `freeze-receipts.csv`: one row per pack with manifest, starter-project, source-card, answer-key and build hashes.
- `summary.csv`: report counts with denominators; unknowns are not zero.

The protocol defines the definitions and decision rules. A CSV value does not establish source truth, participant identity or human approval by itself.

