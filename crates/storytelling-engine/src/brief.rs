// SPDX-License-Identifier: Apache-2.0
// Source-informed behavior: docs/migration/provenance-m2-01.json; LICENSES/third-party/NOTICE.txt.
//! Pure bounded audit of declared brief constraints; no selection or feasibility proof.
mod cycle;

use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use storytelling_core::ValidationError;
use storytelling_core::brief::{
    AssetBindingV1, BriefAuditV1, BriefConflictKindV1, BriefConflictV1, BriefWarningKindV1,
    BriefWarningV1, HardConstraintV1, PrecedenceEdgeV1, StoryBriefDocumentV1,
};
use storytelling_core::time::RationalTime;

const MAX_LIST: usize = 1024;
const MAX_CONSTRAINTS: usize = 256;
const MAX_REFERENCES: usize = 8192;
const MAX_DIAGNOSTICS: usize = 256;
const MAX_OUTPUT_BYTES: usize = 4_194_304;

#[derive(Deserialize)]
struct VersionProbe {
    schema_version: u32,
}

/// Audit only declared reference, membership and ordering consistency.
/// A clean result is not proof that a playable composition or target duration exists.
pub fn audit_story_brief(bytes: &[u8]) -> Result<BriefAuditV1, ValidationError> {
    if bytes.len() > crate::MAX_INPUT_BYTES {
        return Err(error("input.too_large", "$"));
    }
    let version: VersionProbe =
        serde_json::from_slice(bytes).map_err(|_| error("input.invalid_json", "$"))?;
    if version.schema_version != 1 {
        return Err(error("brief.unsupported_version", "$.schema_version"));
    }
    let brief: StoryBriefDocumentV1 =
        serde_json::from_slice(bytes).map_err(|_| error("input.invalid_json", "$"))?;
    let catalog = validate(&brief)?;
    let mut required = Bindings::default();
    let mut excluded = Bindings::default();
    let mut edges = Edges::default();
    let mut conflicts = Vec::new();
    let mut warnings = Vec::new();

    for (index, id) in brief.manual_selected_asset_ids.iter().enumerate() {
        let path = format!("$.manual_selected_asset_ids[{index}]");
        check_reference(id, &path, &catalog, &mut conflicts)?;
        required.add(id, path);
    }
    edges.add_order(
        &brief.manual_selected_asset_ids,
        "$.manual_selected_asset_ids",
    );
    for (index, constraint) in brief.hard_constraints.iter().enumerate() {
        let path = format!("$.hard_constraints[{index}].asset_ids");
        for (offset, id) in constraint_ids(constraint).iter().enumerate() {
            let origin = format!("{path}[{offset}]");
            check_reference(id, &origin, &catalog, &mut conflicts)?;
            if matches!(constraint, HardConstraintV1::MustExclude { .. }) {
                excluded.add(id, origin);
            } else {
                required.add(id, origin);
            }
        }
        if matches!(constraint, HardConstraintV1::AssetOrder { .. }) {
            edges.add_order(constraint_ids(constraint), &path);
        }
    }
    for binding in &required.rows {
        if excluded.indexes.contains_key(&binding.asset_id) {
            push_conflict(
                &mut conflicts,
                BriefConflictV1 {
                    kind: BriefConflictKindV1::RequiredExcluded,
                    path: binding.origins[0].clone(),
                    asset_ids: vec![binding.asset_id.clone()],
                },
            )?;
        }
    }
    if let Some(conflict) = cycle::first_order_cycle(&edges.rows) {
        push_conflict(&mut conflicts, conflict)?;
    }
    for (index, id) in brief.preferences.preferred_asset_ids.iter().enumerate() {
        let kind = if !catalog.contains(id) {
            Some(BriefWarningKindV1::UnknownPreferredAsset)
        } else if excluded.indexes.contains_key(id) {
            Some(BriefWarningKindV1::ExcludedPreference)
        } else {
            None
        };
        if let Some(kind) = kind {
            if conflicts.len() + warnings.len() == MAX_DIAGNOSTICS {
                return Err(error("brief.too_many_diagnostics", "$"));
            }
            warnings.push(BriefWarningV1 {
                kind,
                path: format!("$.preferences.preferred_asset_ids[{index}]"),
                asset_ids: vec![id.clone()],
            });
        }
    }
    let result = BriefAuditV1 {
        schema_version: 1,
        brief_id: brief.brief_id,
        intent: brief.intent,
        scope: "declared-constraint-audit-only".to_owned(),
        executable: false,
        selection_planned: false,
        duration_analysis: "not-performed".to_owned(),
        consistent: conflicts.is_empty(),
        manual_selected_asset_ids: brief.manual_selected_asset_ids,
        required_assets: required.rows,
        excluded_assets: excluded.rows,
        precedence_edges: edges.rows,
        preferences: brief.preferences,
        conflicts,
        warnings,
    };
    let output = serde_json::to_vec(&result).map_err(|_| error("brief.output_too_large", "$"))?;
    if output.len() > MAX_OUTPUT_BYTES {
        return Err(error("brief.output_too_large", "$"));
    }
    Ok(result)
}

fn validate(brief: &StoryBriefDocumentV1) -> Result<BTreeSet<String>, ValidationError> {
    if brief.assets.is_empty() {
        return Err(error("brief.empty_collection", "$.assets"));
    }
    bound(brief.assets.len(), MAX_LIST, "$.assets")?;
    bound(
        brief.manual_selected_asset_ids.len(),
        MAX_LIST,
        "$.manual_selected_asset_ids",
    )?;
    bound(
        brief.preferences.preferred_asset_ids.len(),
        MAX_LIST,
        "$.preferences.preferred_asset_ids",
    )?;
    bound(
        brief.hard_constraints.len(),
        MAX_CONSTRAINTS,
        "$.hard_constraints",
    )?;
    let mut references =
        brief.manual_selected_asset_ids.len() + brief.preferences.preferred_asset_ids.len();
    for (index, constraint) in brief.hard_constraints.iter().enumerate() {
        let path = format!("$.hard_constraints[{index}].asset_ids");
        let ids = constraint_ids(constraint);
        let minimum = if matches!(constraint, HardConstraintV1::AssetOrder { .. }) {
            2
        } else {
            1
        };
        if ids.len() < minimum {
            return Err(error("brief.empty_collection", &path));
        }
        bound(ids.len(), MAX_LIST, &path)?;
        references += ids.len();
    }
    bound(references, MAX_REFERENCES, "$")?;
    bound(brief.intent.len(), 4096, "$.intent")?;
    identifier(&brief.brief_id, "$.brief_id")?;
    let mut catalog = BTreeSet::new();
    for (index, asset) in brief.assets.iter().enumerate() {
        let path = format!("$.assets[{index}].id");
        identifier(&asset.id, &path)?;
        if !catalog.insert(asset.id.clone()) {
            return Err(error("brief.duplicate_id", &path));
        }
    }
    validate_ids(
        &brief.manual_selected_asset_ids,
        "$.manual_selected_asset_ids",
    )?;
    validate_ids(
        &brief.preferences.preferred_asset_ids,
        "$.preferences.preferred_asset_ids",
    )?;
    for (index, constraint) in brief.hard_constraints.iter().enumerate() {
        validate_ids(
            constraint_ids(constraint),
            &format!("$.hard_constraints[{index}].asset_ids"),
        )?;
    }
    if let Some(target) = &brief.preferences.target_duration {
        let parsed = RationalTime::from_strings(&target.numerator, &target.denominator)
            .map_err(|_| error("brief.invalid_time", "$.preferences.target_duration"))?;
        if parsed.is_zero() {
            return Err(error("brief.invalid_time", "$.preferences.target_duration"));
        }
    }
    Ok(catalog)
}

fn constraint_ids(constraint: &HardConstraintV1) -> &[String] {
    match constraint {
        HardConstraintV1::MustInclude { asset_ids }
        | HardConstraintV1::MustExclude { asset_ids }
        | HardConstraintV1::AssetOrder { asset_ids } => asset_ids,
    }
}

fn validate_ids(ids: &[String], path: &str) -> Result<(), ValidationError> {
    let mut seen = BTreeSet::new();
    for (index, id) in ids.iter().enumerate() {
        let path = format!("{path}[{index}]");
        identifier(id, &path)?;
        if !seen.insert(id) {
            return Err(error("brief.duplicate_id", &path));
        }
    }
    Ok(())
}

fn identifier(value: &str, path: &str) -> Result<(), ValidationError> {
    crate::identifier(value, path.to_owned()).map_err(|_| error("brief.invalid_id", path))
}

fn bound(length: usize, maximum: usize, path: &str) -> Result<(), ValidationError> {
    if length > maximum {
        Err(error("brief.budget_exceeded", path))
    } else {
        Ok(())
    }
}

fn error(code: &str, path: &str) -> ValidationError {
    ValidationError::new(code, path)
}

fn check_reference(
    id: &str,
    path: &str,
    catalog: &BTreeSet<String>,
    conflicts: &mut Vec<BriefConflictV1>,
) -> Result<(), ValidationError> {
    if !catalog.contains(id) {
        push_conflict(
            conflicts,
            BriefConflictV1 {
                kind: BriefConflictKindV1::UnknownAsset,
                path: path.to_owned(),
                asset_ids: vec![id.to_owned()],
            },
        )?;
    }
    Ok(())
}

fn push_conflict(
    conflicts: &mut Vec<BriefConflictV1>,
    conflict: BriefConflictV1,
) -> Result<(), ValidationError> {
    if conflicts.len() == MAX_DIAGNOSTICS {
        return Err(error("brief.too_many_diagnostics", "$"));
    }
    conflicts.push(conflict);
    Ok(())
}

#[derive(Default)]
struct Bindings {
    rows: Vec<AssetBindingV1>,
    indexes: BTreeMap<String, usize>,
}

impl Bindings {
    fn add(&mut self, id: &str, origin: String) {
        if let Some(index) = self.indexes.get(id) {
            self.rows[*index].origins.push(origin);
        } else {
            self.indexes.insert(id.to_owned(), self.rows.len());
            self.rows.push(AssetBindingV1 {
                asset_id: id.to_owned(),
                origins: vec![origin],
            });
        }
    }
}

#[derive(Default)]
struct Edges {
    rows: Vec<PrecedenceEdgeV1>,
    indexes: BTreeMap<(String, String), usize>,
}

impl Edges {
    fn add_order(&mut self, ids: &[String], path: &str) {
        for (index, pair) in ids.windows(2).enumerate() {
            let key = (pair[0].clone(), pair[1].clone());
            let origin = format!("{path}[{}]", index + 1);
            if let Some(index) = self.indexes.get(&key) {
                self.rows[*index].origins.push(origin);
            } else {
                self.indexes.insert(key, self.rows.len());
                self.rows.push(PrecedenceEdgeV1 {
                    before: pair[0].clone(),
                    after: pair[1].clone(),
                    origins: vec![origin],
                });
            }
        }
    }
}
