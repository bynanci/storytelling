//! Pure, portable story contracts. No filesystem, runtime, or host access.
pub mod approval;
pub mod brief;
pub mod compatibility;
pub mod compiled_plan;
pub mod narrative;
pub mod narrative_cue;
pub mod narrative_handoff;
pub mod narrative_revision;
pub mod review;
pub mod source_evidence;
pub mod time;
pub mod wire;

use serde::Serialize;

/// Stable machine-readable error. Paths name document fields, not local files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidationError {
    pub code: String,
    pub path: String,
}

impl ValidationError {
    pub fn new(code: &str, path: impl Into<String>) -> Self {
        Self {
            code: code.to_owned(),
            path: path.into(),
        }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}", self.code, self.path)
    }
}

impl std::error::Error for ValidationError {}

pub mod planner;
