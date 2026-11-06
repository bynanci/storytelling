//! Pure, portable story contracts. No media or host access.
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidationError {
    pub code: String,
    pub path: String,
}

impl ValidationError {
    pub fn new(code: &str, path: impl Into<String>) -> Self {
        Self { code: code.to_owned(), path: path.into() }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}", self.code, self.path)
    }
}

impl std::error::Error for ValidationError {}
