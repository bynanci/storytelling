use serde_json::{Value, json};
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{self, Read, Write};
use std::process::ExitCode;
use storytelling_engine::{
    MAX_INPUT_BYTES, MAX_REVIEW_CONTEXT_BYTES, MAX_REVIEW_RECORD_BYTES, MAX_STORY_PROPOSAL_BYTES,
    MAX_STORY_REVIEW_BYTES, MAX_VERIFIED_BUFFER_BYTES, NarrativeRevisionReportV1,
    OtioLossChoicesV1, audit_commercial_copy, audit_story_brief, check_narrative_style_profile,
    check_review, check_story_review, export_openfilm_otio, export_story_otio,
    import_openfilm_snapshot, parse_review_context, plan_story, prepare_review,
    prepare_story_review, query_narrative_impact, render_narrative, render_narrative_cue_sheet,
    render_narrative_handoff, track_narrative_revisions, validate_plan_json, verify_supplied_sha256,};

fn emit(value: Value, code: u8) -> ExitCode {
    let Ok(bytes) = serde_json::to_vec(&value) else {
        return ExitCode::from(3);
    };
    emit_bytes(bytes, code)
}

fn emit_bytes(mut bytes: Vec<u8>, code: u8) -> ExitCode {
    bytes.push(b'\n');
    if io::stdout().lock().write_all(&bytes).is_err() {
        return ExitCode::from(3);
    }
    ExitCode::from(code)
}

fn failure(code: &str, exit: u8) -> ExitCode {
    emit(
        json!({"ok": false, "error": {"code": code, "path": "$"}}),
        exit,
    )
}

fn text(message: &str) -> ExitCode {
    if io::stdout().lock().write_all(message.as_bytes()).is_err() {
        return ExitCode::from(3);
    }
    ExitCode::SUCCESS
}

fn render_narrative_text(projection: &storytelling_engine::NarrativeProjectionV1) -> String {
    use std::fmt::Write as _;

    fn quoted(value: &str) -> String {
        serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_owned())
    }
    fn source_tag(source: &storytelling_engine::NarrativeSourceTagV1) -> String {
        format!(
            "[{}@{} | {}]",
            source.unit_id,
            source.revision,
            source.origin.as_str()
        )
    }

    let mut output = String::new();
    let _ = writeln!(
        output,
        "Narrative: {}@{} (schema v{})",
        projection.document_id, projection.document_revision, projection.schema_version
    );
    let _ = writeln!(output, "Purpose: {}", quoted(&projection.purpose));
    let _ = writeln!(output, "Audience: {}\n", quoted(&projection.audience));

    for variant in &projection.variants {
        match variant {
            storytelling_engine::NarrativeProjectedVariantV1::SpokenOutline { id, name, beats } => {
                let _ = writeln!(
                    output,
                    "## Variant {id} — {} [spoken_outline]",
                    quoted(name)
                );
                for (index, beat) in beats.iter().enumerate() {
                    let _ = writeln!(
                        output,
                        "{}. [{}] {} {}",
                        index + 1,
                        beat.id,
                        source_tag(&beat.source),
                        quoted(&beat.text)
                    );
                }
            }
            storytelling_engine::NarrativeProjectedVariantV1::SingleIllustration {
                id,
                name,
                sources,
                brief,
            } => {
                let _ = writeln!(
                    output,
                    "## Variant {id} — {} [single_illustration]",
                    quoted(name)
                );
                let tags = sources
                    .iter()
                    .map(source_tag)
                    .collect::<Vec<_>>()
                    .join(", ");
                let _ = writeln!(output, "Sources: {tags}");
                let _ = writeln!(output, "Brief: {}", quoted(brief));
            }
        }
        output.push('\n');
    }
    output
}

fn render_narrative_command(args: &[OsString]) -> ExitCode {
    if args.len() != 2 || args[1].as_encoded_bytes().starts_with(b"--") {
        return failure("cli.usage", 2);
    }
    let bytes = match read_argument(&args[1], MAX_INPUT_BYTES) {
        Ok(bytes) => bytes,
        Err(_) => return failure("input.io", 3),
    };
    match render_narrative(&bytes) {
        Ok(projection) => text(&render_narrative_text(&projection)),
        Err(error) => emit(json!({"ok": false, "error": error}), 1),
    }
}

fn render_cue_sheet_text(
    projection: &storytelling_engine::NarrativeCueSheetProjectionV1,
) -> String {
    use std::fmt::Write as _;

    fn quoted(value: &str) -> String {
        serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_owned())
    }

    let mut output = String::new();
    let _ = writeln!(
        output,
        "Cue sheet: {}@{} (schema v1)",
        projection.cue_sheet_id, projection.cue_sheet_revision
    );
    let _ = writeln!(
        output,
        "Narrative: {}@{} · variant {}",
        projection.document_id, projection.document_revision, projection.variant_id
    );
    let _ = writeln!(output, "Time unit: {}", projection.time_unit);
    output.push_str("Tracks:\n");
    for track in &projection.tracks {
        let _ = writeln!(
            output,
            "- {} · channel={} · policy={}",
            track.id, track.channel, track.policy
        );
    }
    output.push_str("\nCues:\n");
    for cue in &projection.cues {
        let _ = writeln!(
            output,
            "{}. {} · {} · role={} · track={}",
            cue.order,
            cue.id,
            cue.kind,
            quoted(&cue.role),
            cue.track_id
        );
        let _ = writeln!(output, "   Source: {}", quoted(&cue.source));
        if let Some(spoken_text) = &cue.spoken_text {
            let _ = writeln!(output, "   Voice: {}", quoted(spoken_text));
        }
        let _ = writeln!(output, "   Direction: {}", quoted(&cue.instruction));
        let _ = writeln!(output, "   Time: {}", quoted(&cue.timing));
    }
    let _ = writeln!(
        output,
        "\nValidation: resolved intervals: {}; unresolved intervals: {}",
        projection.resolved_intervals, projection.unresolved_intervals
    );
    output.push_str(
        "Policy: [start, end) ranges; cross-track overlap allowed; overlapping resolved cues on an exclusive track are rejected.\n",
    );
    output.push_str(
        "Scope: declarations only; unresolved intervals are not conflict-checked, and no audio is read or mixed.\n",
    );
    output
}

fn render_cue_sheet_command(args: &[OsString]) -> ExitCode {
    if args.len() != 3
        || args[1..]
            .iter()
            .any(|arg| arg.as_encoded_bytes().starts_with(b"--"))
        || (args[1] == OsStr::new("-") && args[2] == OsStr::new("-"))
    {
        return failure("cli.usage", 2);
    }
    let document = match read_argument(&args[1], MAX_INPUT_BYTES) {
        Ok(bytes) => bytes,
        Err(_) => return failure("input.io", 3),
    };
    let cue_sheet = match read_argument(&args[2], MAX_INPUT_BYTES) {
        Ok(bytes) => bytes,
        Err(_) => return failure("input.io", 3),
    };
    match render_narrative_cue_sheet(&document, &cue_sheet) {
        Ok(projection) => text(&render_cue_sheet_text(&projection)),
        Err(error) => emit(json!({"ok": false, "error": error}), 1),
    }
}

fn source_refs_text(refs: &[storytelling_engine::NarrativeUnitReferenceV1]) -> String {
    refs.iter()
        .map(|reference| format!("{}@{}", reference.unit_id, reference.revision))
        .collect::<Vec<_>>()
        .join(", ")
}

fn render_revision_report(report: &NarrativeRevisionReportV1) -> String {
    use std::fmt::Write as _;

    fn quoted(value: &str) -> String {
        serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_owned())
    }

    let mut output = String::new();
    let _ = writeln!(output, "Narrative revision history: {}", report.document_id);
    let _ = writeln!(output, "\nSource changes:");
    for event in report
        .events
        .iter()
        .filter(|event| event.kind == "source_revision")
    {
        let unit = event.unit_id.as_deref().unwrap_or("unknown");
        let from = event.from_revision.unwrap_or_default();
        let to = event.to_revision.unwrap_or_default();
        let _ = writeln!(output, "- {}: {unit}@{from} -> {unit}@{to}", event.id);
        let impacted = if event.affected_variants.is_empty() {
            "none".to_owned()
        } else {
            event.affected_variants.join(", ")
        };
        let _ = writeln!(output, "  Affected variants: {impacted}");
    }

    let _ = writeln!(output, "\nAuthor choices:");
    for choice in &report.author_decisions {
        let _ = writeln!(
            output,
            "- Choice {}: {} {}@{} {} (variant revision {})",
            choice.id,
            choice.variant_id,
            choice.proposal_ref.unit_id,
            choice.proposal_ref.revision,
            choice.decision.as_str(),
            choice.variant_revision
        );
    }
    let _ = writeln!(output, "\nAssistant suggestions:");
    for suggestion in &report.suggestions {
        let _ = writeln!(
            output,
            "- Suggestion {}: {} {}@{} pending at suggestion time; it records no author decision",
            suggestion.id,
            suggestion.variant_id,
            suggestion.proposal_ref.unit_id,
            suggestion.proposal_ref.revision
        );
    }

    let _ = writeln!(output, "\nSource history:");
    for source in &report.source_history {
        let bases = if source.based_on.is_empty() {
            String::new()
        } else {
            format!("; based on {}", source_refs_text(&source.based_on))
        };
        let _ = writeln!(
            output,
            "- Source {}@{} [{}]{}: {}",
            source.unit_id,
            source.revision,
            source.origin.as_str(),
            bases,
            quoted(&source.text)
        );
    }

    let _ = writeln!(output, "\nVariant history:");
    for variant in &report.variant_history {
        let _ = writeln!(
            output,
            "- Snapshot {}@{} [{}] sources [{}]: {}",
            variant.variant_id,
            variant.revision,
            variant.kind,
            source_refs_text(&variant.source_refs),
            quoted(&variant.content)
        );
    }

    let _ = writeln!(output, "\nCurrent variants:");
    for variant in &report.current_variants {
        let stale = if variant.stale_source_refs.is_empty() {
            String::new()
        } else {
            format!("; stale [{}]", source_refs_text(&variant.stale_source_refs))
        };
        let _ = writeln!(
            output,
            "- Current {}@{}: {}{}; sources [{}]; {}",
            variant.variant_id,
            variant.revision,
            variant.status,
            stale,
            source_refs_text(&variant.source_refs),
            quoted(&variant.content)
        );
    }
    output
}

fn narrative_revision_command(args: &[OsString]) -> ExitCode {
    let querying = args
        .first()
        .is_some_and(|arg| arg == OsStr::new("query-narrative-impact"));
    let expected_args = if querying { 3 } else { 2 };
    if args.len() != expected_args || args[1].as_encoded_bytes().starts_with(b"--") {
        return failure("cli.usage", 2);
    }
    if querying && args[2].as_encoded_bytes().starts_with(b"--") {
        return failure("cli.usage", 2);
    }
    let bytes = match read_argument(&args[1], MAX_INPUT_BYTES) {
        Ok(bytes) => bytes,
        Err(_) => return failure("input.io", 3),
    };
    if querying {
        let Some(change_id) = args[2].to_str() else {
            return failure("cli.usage", 2);
        };
        return match query_narrative_impact(&bytes, change_id) {
            Ok(event) => {
                let unit = event.unit_id.as_deref().unwrap_or("unknown");
                let from = event.from_revision.unwrap_or_default();
                let to = event.to_revision.unwrap_or_default();
                let impacted = if event.affected_variants.is_empty() {
                    "none".to_owned()
                } else {
                    event.affected_variants.join(", ")
                };
                let mut result = String::new();
                use std::fmt::Write as _;
                let _ = writeln!(result, "Change {}: {unit}@{from} -> {unit}@{to}", event.id);
                let _ = writeln!(result, "Affected variants: {impacted}");
                text(&result)
            }
            Err(error) => emit(json!({"ok": false, "error": error}), 1),
        };
    }
    match track_narrative_revisions(&bytes) {
        Ok(report) => text(&render_revision_report(&report)),
        Err(error) => emit(json!({"ok": false, "error": error}), 1),
    }
}

fn check_narrative_style_command(args: &[OsString]) -> ExitCode {
    if args.len() != 3
        || args[1..]
            .iter()
            .any(|arg| arg.as_encoded_bytes().starts_with(b"--"))
        || (args[1] == OsStr::new("-") && args[2] == OsStr::new("-"))
    {
        return failure("cli.usage", 2);
    }
    let document = match read_argument(&args[1], MAX_INPUT_BYTES) {
        Ok(bytes) => bytes,
        Err(_) => return failure("input.io", 3),
    };
    let profile = match read_argument(&args[2], MAX_INPUT_BYTES) {
        Ok(bytes) => bytes,
        Err(_) => return failure("input.io", 3),
    };
    match check_narrative_style_profile(&document, &profile) {
        Ok(style_check) => emit(json!({"ok": true, "style_check": style_check}), 0),
        Err(error) => emit(json!({"ok": false, "error": error}), 1),
    }
}

fn audit_commercial_copy_command(args: &[OsString]) -> ExitCode {
    if args.len() != 3
        || args[1..]
            .iter()
            .any(|arg| arg.as_encoded_bytes().starts_with(b"--"))
        || (args[1] == OsStr::new("-") && args[2] == OsStr::new("-"))
    {
        return failure("cli.usage", 2);
    }
    let copy = match read_argument(&args[1], MAX_INPUT_BYTES) {
        Ok(bytes) => bytes,
        Err(_) => return failure("input.io", 3),
    };
    let ledger = match read_argument(&args[2], MAX_INPUT_BYTES) {
        Ok(bytes) => bytes,
        Err(_) => return failure("input.io", 3),
    };
    match audit_commercial_copy(&copy, &ledger) {
        Ok(report) => emit(json!({"ok": true, "commercial_copy": report}), 0),
        Err(error) => emit(json!({"ok": false, "error": error}), 1),
    }
}

fn read_bounded(reader: impl Read) -> io::Result<Vec<u8>> {
    read_with_limit(reader, MAX_INPUT_BYTES)
}

fn read_with_limit(reader: impl Read, limit: usize) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    reader.take(limit as u64 + 1).read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn read_file_with_limit(path: &OsStr, limit: usize) -> io::Result<Vec<u8>> {
    let file = File::open(path)?;
    if !file.metadata()?.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "not a regular file",
        ));
    }
    read_with_limit(file, limit)
}

fn read_argument(path: &OsStr, limit: usize) -> io::Result<Vec<u8>> {
    if path == OsStr::new("-") {
        read_with_limit(io::stdin().lock(), limit)
    } else {
        read_file_with_limit(path, limit)
    }
}

fn story_review_command(args: &[OsString]) -> ExitCode {
    let checking = args[0] == OsStr::new("check-story-review");
    let proposal_index = if checking { 2 } else { 1 };
    let expected = if checking { 5 } else { 4 };
    if args.len() != expected
        || args[1..].iter().enumerate().any(|(index, arg)| {
            arg.as_encoded_bytes().starts_with(b"--")
                || (arg == OsStr::new("-") && index + 1 != proposal_index)
        })
    {
        return failure("cli.usage", 2);
    }
    let mut inputs = Vec::new();
    for (index, arg) in args.iter().enumerate().skip(1) {
        let (limit, code) = if checking && index == 1 {
            (MAX_STORY_REVIEW_BYTES, "story_review.bundle_too_large")
        } else if index == proposal_index {
            (MAX_STORY_PROPOSAL_BYTES, "story_review.proposal_too_large")
        } else if index == proposal_index + 1 {
            (MAX_INPUT_BYTES, "input.too_large")
        } else {
            (MAX_REVIEW_CONTEXT_BYTES, "review.context_too_large")
        };
        let bytes = match read_argument(arg, limit) {
            Ok(bytes) => bytes,
            Err(_) => return failure("input.io", 3),
        };
        if bytes.len() > limit {
            return failure(code, 1);
        }
        inputs.push(bytes);
    }
    if checking {
        return match check_story_review(&inputs[0], &inputs[1], &inputs[2], &inputs[3]) {
            Ok(freshness) => {
                let exit = if freshness.current { 0 } else { 1 };
                emit(json!({"ok":true, "freshness":freshness}), exit)
            }
            Err(error) => emit(json!({"ok":false,"error":error}), 1),
        };
    }
    if args[0] == OsStr::new("export-story-otio") {
        return match export_story_otio(&inputs[0], &inputs[1], &inputs[2]) {
            Ok(otio) => emit_bytes(otio, 0),
            Err(error) => emit(json!({"ok":false,"error":error}), 1),
        };
    }
    match prepare_story_review(&inputs[0], &inputs[1], &inputs[2]) {
        Ok(bound) => emit_bytes(bound.bundle_json().to_vec(), 0),
        Err(error) => emit(json!({"ok":false,"error":error}), 1),
    }
}

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args
        .first()
        .is_some_and(|arg| arg == OsStr::new("audit-copy-claims"))
    {
        return audit_commercial_copy_command(&args);
    }
    if args
        .first()
        .is_some_and(|arg| arg == OsStr::new("render-narrative"))
    {
        return render_narrative_command(&args);
    }
    if args
        .first()
        .is_some_and(|arg| arg == OsStr::new("render-cue-sheet"))
    {
        return render_cue_sheet_command(&args);
    }
    if args.first().is_some_and(|arg| {
        arg == OsStr::new("track-narrative-revisions")
            || arg == OsStr::new("query-narrative-impact")
    }) {
        return narrative_revision_command(&args);
    }
    if args
        .first()
        .is_some_and(|arg| arg == OsStr::new("check-narrative-style"))
    {
        return check_narrative_style_command(&args);
    }
    if args.first().is_some_and(|arg| {
        [
            "prepare-story-review",
            "export-story-otio",
            "check-story-review",
        ]
        .iter()
        .any(|command| arg == OsStr::new(command))
    }) {
        return story_review_command(&args);
    }
    if args
        .first()
        .is_some_and(|v| v == OsStr::new("verify-source"))
    {
        if args.len() != 3 {
            return failure("cli.usage", 2);
        }
        let Some(expected) = args[2].to_str() else {
            return failure("cli.usage", 2);
        };
        let bytes = if args[1] == OsStr::new("-") {
            read_with_limit(io::stdin().lock(), MAX_VERIFIED_BUFFER_BYTES)
        } else {
            match File::open(&args[1]) {
                Ok(file) => match file.metadata() {
                    Ok(m) if m.is_file() => read_with_limit(file, MAX_VERIFIED_BUFFER_BYTES),
                    _ => return failure("input.io", 3),
                },
                Err(_) => return failure("input.io", 3),
            }
        };
        let Ok(bytes) = bytes else {
            return failure("input.io", 3);
        };
        return match verify_supplied_sha256(&bytes, expected) {
            Ok(verification) => emit(json!({"ok": true, "verification": verification}), 0),
            Err(error) => emit(json!({"ok": false, "error": error}), 1),
        };
    }
    if args
        .first()
        .is_some_and(|v| v == OsStr::new("prepare-review"))
    {
        if args.len() != 3
            || args[2] == OsStr::new("-")
            || args[2].as_encoded_bytes().starts_with(b"--")
            || args[1].as_encoded_bytes().starts_with(b"--")
        {
            return failure("cli.usage", 2);
        }
        let composition = match read_argument(&args[1], MAX_INPUT_BYTES) {
            Ok(bytes) => bytes,
            Err(_) => return failure("input.io", 3),
        };
        if composition.len() > MAX_INPUT_BYTES {
            return failure("input.too_large", 1);
        }
        let context_bytes = match read_file_with_limit(&args[2], MAX_REVIEW_CONTEXT_BYTES) {
            Ok(bytes) => bytes,
            Err(_) => return failure("input.io", 3),
        };
        if context_bytes.len() > MAX_REVIEW_CONTEXT_BYTES {
            return failure("review.context_too_large", 1);
        }
        let context = match parse_review_context(&context_bytes) {
            Ok(context) => context,
            Err(error) => return emit(json!({"ok": false, "error": error}), 1),
        };
        return match prepare_review(&composition, &context) {
            Ok(review) => emit(json!({"ok": true, "review": review.record()}), 0),
            Err(error) => emit(json!({"ok": false, "error": error}), 1),
        };
    }
    if args
        .first()
        .is_some_and(|v| v == OsStr::new("check-review"))
    {
        if args.len() != 4
            || args[1] == OsStr::new("-")
            || args[3] == OsStr::new("-")
            || args[1].as_encoded_bytes().starts_with(b"--")
            || args[3].as_encoded_bytes().starts_with(b"--")
            || args[2].as_encoded_bytes().starts_with(b"--")
        {
            return failure("cli.usage", 2);
        }
        let record = match read_file_with_limit(&args[1], MAX_REVIEW_RECORD_BYTES) {
            Ok(bytes) => bytes,
            Err(_) => return failure("input.io", 3),
        };
        if record.len() > MAX_REVIEW_RECORD_BYTES {
            return failure("review.record_too_large", 1);
        }
        let composition = match read_argument(&args[2], MAX_INPUT_BYTES) {
            Ok(bytes) => bytes,
            Err(_) => return failure("input.io", 3),
        };
        if composition.len() > MAX_INPUT_BYTES {
            return failure("input.too_large", 1);
        }
        let context = match read_file_with_limit(&args[3], MAX_REVIEW_CONTEXT_BYTES) {
            Ok(bytes) => bytes,
            Err(_) => return failure("input.io", 3),
        };
        if context.len() > MAX_REVIEW_CONTEXT_BYTES {
            return failure("review.context_too_large", 1);
        }
        return match check_review(&record, &composition, &context) {
            Ok(freshness) => emit(json!({"ok": true, "freshness": freshness}), 0),
            Err(error) => emit(json!({"ok": false, "error": error}), 1),
        };
    }
    let command = args.first().map(|value| value.as_os_str());
    let mut otio_choices = OtioLossChoicesV1::default();
    match command {
        Some(value) if value == OsStr::new("--help") && args.len() == 1 => {
            return text(
"Usage: storytelling validate <FILE|->\n       storytelling import-openfilm <FILE|->\n       storytelling export-otio <FILE|-> [--picture-only] [--metadata-only-locks]\n       storytelling audit-brief <FILE|->\n       storytelling plan-story <FILE|->\n       storytelling render-narrative <FILE|->\n       storytelling render-cue-sheet <DOCUMENT_FILE|-> <CUE_SHEET_FILE|->\n       storytelling render-handoff <FILE|->\n       storytelling track-narrative-revisions <FILE|->\n       storytelling query-narrative-impact <FILE|-> <CHANGE_ID>\n       storytelling check-narrative-style <DOCUMENT_FILE|-> <PROFILE_FILE|->\n       storytelling audit-copy-claims <COPY_FILE|-> <EVIDENCE_LEDGER_FILE|->\n       storytelling prepare-story-review <PROPOSAL|-> <SNAPSHOT_FILE> <CONTEXT_FILE>\n       storytelling export-story-otio <PROPOSAL|-> <SNAPSHOT_FILE> <CONTEXT_FILE>\n       storytelling check-story-review <REVIEW_BUNDLE> <PROPOSAL|-> <SNAPSHOT_FILE> <CONTEXT_FILE>\n       storytelling prepare-review <COMPOSITION|-> <CONTEXT_FILE>\n       storytelling check-review <REVIEW_RECORD> <COMPOSITION|-> <CONTEXT_FILE>\n       storytelling verify-source <FILE|-> <SHA256>\nRead-only metadata, interchange and declared review identity; no editor execution.\n",            );
        }
        Some(value) if value == OsStr::new("--version") && args.len() == 1 => {
            return text(concat!("storytelling ", env!("CARGO_PKG_VERSION"), "\n"));
        }
        Some(value)
            if (value == OsStr::new("validate")
                || value == OsStr::new("import-openfilm")
                || value == OsStr::new("audit-brief")
                || value == OsStr::new("plan-story"))
                && args.len() == 2 =>
        {
            if value == OsStr::new("plan-story") && args[1].as_encoded_bytes().starts_with(b"--") {
                return failure("cli.usage", 2);
            }
        }
        Some(value) if value == OsStr::new("export-otio") && (2..=4).contains(&args.len()) => {
            if args[1].as_encoded_bytes().starts_with(b"--") {
                return failure("cli.usage", 2);
            }
            for flag in &args[2..] {
                if flag == OsStr::new("--picture-only") && !otio_choices.picture_only {
                    otio_choices.picture_only = true;
                } else if flag == OsStr::new("--metadata-only-locks")
                    && !otio_choices.metadata_only_locks
                {
                    otio_choices.metadata_only_locks = true;
                } else {
                    return failure("cli.usage", 2);
                }
            }
        }
        _ => return failure("cli.usage", 2),
    }
    let Some(input) = args.get(1) else {
        return failure("cli.usage", 2);
    };
    let bytes = if input == OsStr::new("-") {
        read_bounded(io::stdin().lock())
    } else {
        match File::open(input) {
            Ok(file) => match file.metadata() {
                Ok(metadata) if metadata.is_file() => read_bounded(file),
                _ => return failure("input.io", 3),
            },
            Err(_) => return failure("input.io", 3),
        }
    };
    let Ok(bytes) = bytes else {
        return failure("input.io", 3);
    };
    if command == Some(OsStr::new("plan-story")) {
        return match plan_story(&bytes) {
            Ok(planning) => emit(json!({"ok": true, "planning": planning}), 0),
            Err(error) => emit(json!({"ok": false, "error": error}), 1),
        };
    }
    if command == Some(OsStr::new("audit-brief")) {
        return match audit_story_brief(&bytes) {
            Ok(audit) => {
                let consistent = audit.consistent;
                emit(
                    json!({"ok": consistent, "audit": audit}),
                    if consistent { 0 } else { 1 },
                )
            }
            Err(error) => emit(json!({"ok": false, "error": error}), 1),
        };
    }
    if command == Some(OsStr::new("export-otio")) {
        return match export_openfilm_otio(&bytes, &Default::default(), &otio_choices) {
            Ok(compiled) => emit(compiled.document, 0),
            Err(error) => emit(json!({"ok": false, "error": error}), 1),
        };
    }
    if command == Some(OsStr::new("import-openfilm")) {
        // Profile is fixed inside the engine; no caller input grants capabilities.
        // Type inference keeps the CLI's existing engine-only dependency boundary.
        return match import_openfilm_snapshot(&bytes, &Default::default()) {
            Ok(report) => {
                let Ok(serialized) = serde_json::to_vec(&report) else {
                    return failure("input.io", 3);
                };
                // Keep raw source number lexemes; json! would round them through Value.
                let mut response = b"{\"ok\":true,\"import\":".to_vec();
                response.extend(serialized);
                response.push(b'}');
                emit_bytes(response, 0)
            }
            Err(error) => emit(json!({"ok": false, "error": error}), 1),
        };
    }
    match validate_plan_json(&bytes) {
        Ok(report) => emit(json!({"ok": true, "validation": report}), 0),
        Err(error) => emit(json!({"ok": false, "error": error}), 1),
    }
}

