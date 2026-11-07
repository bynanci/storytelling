# ADR 0003: Preserve the existing repository license

Date: 2026-11-07. Status: bootstrap decision, subject to maintainer review.

The actual initial repository uses MIT, despite the earlier design proposal's
Apache preference. This task does not authorize a license replacement. Keep LICENSE
byte-for-byte unchanged and use MIT in the new Cargo manifests.

M0 introduces new code and synthetic fixtures only. Future OpenFilm extraction must
record source SHA/path/license and retain applicable notices. No blanket claim is
made that all future ported material may be distributed under MIT.

GitHub API publication can assign the connected account as author/committer. Commit
messages and PR must disclose AI implementation; never fabricate a maintainer's
Signed-off-by or cryptographic signature. DCO enforcement is a separate explicit
policy decision, not silently installed by this bootstrap.

See [provenance](../../PROVENANCE.md).
