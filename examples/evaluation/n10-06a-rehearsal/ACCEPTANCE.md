# Synthetic N10-06A capture acceptance · v1

Frozen before implementation on 2026-11-17, base `06164f19a8b3614f438dd2f97ef810c94e64c3d3`.
Task: #129 A / N-R10-06A, research handoff; no new product contract.

Eight fixed synthetic cases: decimal 2.5–4.5, decimal 0–0.1, CFR [75,135) at 30/1, CFR [0,150) at 30000/1001, decimal comma, end 6 beyond declared duration 5, variable-timing frame input, and two fully declared mappings. Cases derive only from the existing three public A fixtures.

For each case, call the normal native helper by file, stdin and repeated file; retain exit, raw stdout/stderr, exact input snapshot and digest. Valid cases additionally emit V1 and replay the accepted V1 command; invalid cases must also reject emit-v1 without a coverage document. This totals 37 CLI calls. Preserve selected refs, unresolved reason, canonical exact ranges and first error. A complete declaration is still `declarations_only`.

CLI failures and timeouts remain failed machine evidence. Refuse an existing output receipt; never replace previous observations. Hash the explicit binary and fixture bytes before/after replay. A source SHA supplied by a caller is not authenticated build provenance. Capture process elapsed time only; author time and human results remain null, human participants and Host observations remain zero.

Roundtable dialogue is a single-agent authored simulation, not verbatim participant feedback, independent review, G2 acceptance, an author trial or a new model-judged effect study. Do not transform these cases into #121 original inputs or a #122 holdout. A future author receives a separate task sheet before any facilitator key, and must provide actual consent/screening/actions/quotes/timing.
