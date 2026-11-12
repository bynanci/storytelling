// SPDX-License-Identifier: MIT
//! Declarative, read-only mapping from selected narrative units to video ranges.
use serde::{Deserialize, Serialize};

use crate::narrative::{NarrativeDocumentV1, NarrativeUnitReferenceV1};
use crate::wire::{SourceRange, TimeValue};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NarrativeVideoCoverageV1 {
    pub schema_version: u32,
    pub coverage_id: String,
    pub narrative_document: NarrativeDocumentV1,
    pub variant_id: String,
    pub selected_unit_refs: Vec<NarrativeUnitReferenceV1>,
    pub asset_catalog: VideoAssetCatalogV1,
    pub mappings: Vec<VideoCoverageMappingV1>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VideoAssetCatalogV1 {
    pub catalog_id: String,
    pub revision: u32,
    pub assets: Vec<VideoAssetV1>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VideoAssetV1 {
    pub asset_id: String,
    pub revision: u32,
    pub duration: TimeValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VideoAssetReferenceV1 {
    pub asset_id: String,
    pub revision: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum VideoCoverageMappingV1 {
    Mapped {
        unit_ref: NarrativeUnitReferenceV1,
        asset_ref: VideoAssetReferenceV1,
        source_range: SourceRange,
    },
    Unresolved {
        unit_ref: NarrativeUnitReferenceV1,
        reason: String,
    },
}

impl VideoCoverageMappingV1 {
    pub fn unit_ref(&self) -> &NarrativeUnitReferenceV1 {
        match self {
            Self::Mapped { unit_ref, .. } | Self::Unresolved { unit_ref, .. } => unit_ref,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NarrativeVideoCoverageReferenceV1 {
    pub document_id: String,
    pub document_revision: u32,
    pub variant_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VideoAssetCatalogReferenceV1 {
    pub catalog_id: String,
    pub revision: u32,
}

/// Output mapping fields are ordered to keep the JSON wire result byte-stable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VideoCoverageMappingResultV1 {
    pub unit_ref: NarrativeUnitReferenceV1,
    pub status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_ref: Option<VideoAssetReferenceV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_range: Option<SourceRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VideoCoverageDiagnosticV1 {
    pub code: String,
    pub path: String,
}

/// Read-only projection of the selected mapping declarations and planning gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NarrativeVideoCoverageResultV1 {
    pub schema_version: u32,
    pub profile: &'static str,
    pub coverage_id: String,
    pub narrative_ref: NarrativeVideoCoverageReferenceV1,
    pub asset_catalog_ref: VideoAssetCatalogReferenceV1,
    pub selected_unit_refs: Vec<NarrativeUnitReferenceV1>,
    pub mappings: Vec<VideoCoverageMappingResultV1>,
    pub ready_for_planning: bool,
    pub diagnostics: Vec<VideoCoverageDiagnosticV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NarrativeVideoCoverageErrorV1 {
    pub code: String,
    pub path: Option<String>,
}

impl NarrativeVideoCoverageErrorV1 {
    pub fn new(code: &str, path: impl Into<String>) -> Self {
        Self {
            code: code.to_owned(),
            path: Some(path.into()),
        }
    }

    pub fn with_optional_path(code: &str, path: Option<String>) -> Self {
        Self {
            code: code.to_owned(),
            path,
        }
    }
}
