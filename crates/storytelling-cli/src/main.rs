use serde_json::{Value, json};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process::ExitCode;
use storytelling_engine::{MAX_INPUT_BYTES, validate_plan_json};

fn emit(value: Value, code: u8) -> ExitCode {
    let Ok(mut bytes) = serde_json::to_vec(&value) else {
        return ExitCode::from(3);
    };
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
    let mut bytes = Vec::new();
    reader
        .take(MAX_INPUT_BYTES as u64 + 1)
        .read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn main() -> ExitCode {
    let mut args = std::env::args_os().skip(1);
    let (command, input, extra) = (args.next(), args.next(), args.next());
    match command.as_deref() {
        Some(value) if value == OsStr::new("--help") && input.is_none() => {
            return text(
                "Usage: storytelling validate <FILE|->\nRead-only declared-metadata validation; no source hashing or editor execution.\n",
            );
        }
        Some(value) if value == OsStr::new("--version") && input.is_none() => {
            return text(concat!("storytelling ", env!("CARGO_PKG_VERSION"), "\n"));
        }
        Some(value) if value == OsStr::new("validate") && extra.is_none() => {}
        _ => return failure("cli.usage", 2),
    }
    let Some(input) = input else {
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
    match validate_plan_json(&bytes) {
        Ok(report) => emit(json!({"ok": true, "validation": report}), 0),
        Err(error) => emit(json!({"ok": false, "error": error}), 1),
    }
}
