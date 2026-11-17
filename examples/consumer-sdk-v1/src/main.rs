// SPDX-License-Identifier: MIT
//! Independent example: caller-owned I/O, facade-only domain imports.
use std::io::Write;
use storytelling_engine::consumer_v1::{self, NarrativeVideoCoverageResultV1};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args_os()
        .nth(1)
        .ok_or("usage: storytelling-consumer-v1-example COVERAGE.json")?;
    let bytes = std::fs::read(path)?;
    let result: NarrativeVideoCoverageResultV1 = consumer_v1::video_coverage(&bytes)?;
    let mut stdout = std::io::stdout().lock();
    serde_json::to_writer(&mut stdout, &result)?;
    writeln!(stdout)?;
    Ok(())
}
