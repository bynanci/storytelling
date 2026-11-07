use serde_json::{Value, json};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process::ExitCode;
use storytelling_engine::{
    MAX_INPUT_BYTES, MAX_VERIFIED_BUFFER_BYTES, OtioLossChoicesV1, audit_story_brief,
    export_openfilm_otio, import_openfilm_snapshot, plan_story, validate_plan_json,
    verify_supplied_sha256,
};

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

fn read_bounded(reader: impl Read) -> io::Result<Vec<u8>> {
    read_with_limit(reader, MAX_INPUT_BYTES)
}

fn read_with_limit(reader: impl Read, limit: usize) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    reader.take(limit as u64 + 1).read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
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
    let command = args.first().map(|value| value.as_os_str());
    let mut otio_choices = OtioLossChoicesV1::default();
    match command {
        Some(value) if value == OsStr::new("--help") && args.len() == 1 => {
            return text(
                "Usage: storytelling validate <FILE|->\n       storytelling import-openfilm <FILE|->\n       storytelling export-otio <FILE|-> [--picture-only] [--metadata-only-locks]\n       storytelling audit-brief <FILE|->\n       storytelling plan-story <FILE|->\n       storytelling verify-source <FILE|-> <SHA256>\nRead-only metadata, interchange and explicit supplied-byte verification; no editor execution.\n",
            );
        }
        Some(value) if value == OsStr::new("--version") && args.len() == 1 => {
            return text(concat!("storytelling ", env!("CARGO_PKG_VERSION"), "\n"));
        }
        Some(value)
            if (value == OsStr::new("validate")
                || value == OsStr::new("import-openfilm")
                || value == OsStr::new("audit-brief")
                || value == OsStr::new("plan-story"))
                && args.len() == 2 => {
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
