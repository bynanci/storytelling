//! M0 read-only validation boundary. This initial commit is the RED test scaffold.
use serde::Serialize;
use storytelling_core::ValidationError;

#[derive(Debug, Serialize)]
pub struct ValidationReport {
    pub schema_version: u32,
    pub plan_id: String,
    pub assets: usize,
    pub shots: usize,
    pub scope: &'static str,
}

pub fn validate_plan_json(_input: &[u8]) -> Result<ValidationReport, ValidationError> {
    Err(ValidationError::new("implementation.pending", "$"))
}
