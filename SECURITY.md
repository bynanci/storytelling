# Security policy

No stable release is published. M0 is a read-only local declared-metadata validator,
not a sandbox, source verifier, network service or authorized editor executor.

Do not put private media, paths, credentials or exploit secrets in a public issue.
Use GitHub private vulnerability reporting if the maintainer has enabled it;
otherwise request a private reporting channel without disclosing the exploit.
Private reporting and branch protection are owner settings, not enabled by this file.

## Current boundaries

Input is bounded to 1 MiB, 1024 assets and 4096 shots. Unknown fields, malformed
numbers, duplicate identities and missing references are rejected. The CLI reads an
explicit local file or stdin and only writes its response to stdout. No source URLs,
model calls, database mutation or host commands are accepted. First-party unsafe is
forbidden, but dependency safety and compiler advisories still require review.

M1-03 adds `verify-source` for an explicitly provided file/stdin and expected
SHA-256, with a separate 16 MiB input-byte limit. Core verification owns only
those supplied immutable bytes; its metadata receipt is not deserializable
authority over future filesystem contents. The CLI does not resolve project
URIs, claim path identity/TOCTOU closure or invoke a host. The test-only host
harness generates authorized synthetic media in a new owned run directory;
its actual profile evidence is separate from product approval or execution.

## Before adding execution

Require trusted explicit approval, least privilege, source and host verification,
request fencing, durable receipts, cancellation and unknown-result reconciliation.
Never treat MCP annotations or LLM text as authorization. Isolate host projects and
redact paths/secrets in portable records. No arbitrary shell/plugin evaluation.

Security findings are tracked with severity, affected candidate/version, reproduction,
fix, regression and remaining exposure. A tool returning zero vulnerability-class
findings does not automatically dismiss other security or maintenance advisories.
