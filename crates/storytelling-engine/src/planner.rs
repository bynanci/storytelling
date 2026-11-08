// SPDX-License-Identifier: Apache-2.0
// Source-informed fixed-window planner; see provenance-m2-02.json and NOTICE.openfilm.
//! Pure deterministic selection of declared fixed windows; no execution authority.
mod clock;
mod ordering;

#[cfg(test)]
#[path = "planner/clock_tests.rs"]
mod clock_tests;

use ordering::order;

#[cfg(test)]
#[path = "planner/order_tests.rs"]
mod order_tests;

use clock::Clock;
use serde::Deserialize;
use serde_json::value::RawValue;
use std::collections::{BTreeMap, BTreeSet};
use storytelling_core::ValidationError;
use storytelling_core::brief::{AssetBindingV1, BriefAssetV1};
use storytelling_core::planner::{
    PlannedStoryV1, PlanningCandidateV1, PlanningErrorV1, SelectionDecisionV1, SkippedCandidateV1,
    SkippedReasonV1, TargetStatusV1,
};
use storytelling_core::wire::{
    DeclaredAsset, DeclaredShot, PlanDocumentV1, SourceRange, TimeValue,
};

#[derive(Deserialize)]
struct VersionProbe {
    schema_version: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    schema_version: u32,
    brief: Box<RawValue>,
    candidates: Vec<PlanningCandidateV1>,
    max_duration: TimeValue,
    max_shots: u32,
}

#[derive(Deserialize)]
struct Catalog {
    assets: Vec<BriefAssetV1>,
}

fn error(code: &str, path: &str) -> PlanningErrorV1 {
    PlanningErrorV1 {
        code: code.to_owned(),
        path: path.to_owned(),
        conflicts: vec![],
    }
}

fn validation(error: ValidationError) -> PlanningErrorV1 {
    PlanningErrorV1 {
        code: error.code,
        path: error.path,
        conflicts: vec![],
    }
}

fn brief_path(path: &str) -> String {
    format!("$.brief{}", &path[1..])
}

fn copy_time(value: &TimeValue) -> TimeValue {
    TimeValue {
        numerator: value.numerator.clone(),
        denominator: value.denominator.clone(),
    }
}

fn validate_candidates(
    input: &Input,
    catalog: &BTreeSet<String>,
) -> Result<BTreeMap<String, usize>, PlanningErrorV1> {
    if !(1..=1024).contains(&input.max_shots) {
        return Err(error("planner.invalid_max_shots", "$.max_shots"));
    }
    let maximum = crate::time(&input.max_duration, "$.max_duration".into()).map_err(validation)?;
    if maximum.is_zero() {
        return Err(error("plan.invalid_range", "$.max_duration"));
    }
    if input.candidates.len() > 1024 {
        return Err(error("planner.too_many_candidates", "$.candidates"));
    }
    let mut indexes = BTreeMap::new();
    for (index, candidate) in input.candidates.iter().enumerate() {
        let path = format!("$.candidates[{index}]");
        crate::identifier(&candidate.asset_id, format!("{path}.asset_id")).map_err(validation)?;
        if !catalog.contains(&candidate.asset_id) {
            return Err(error(
                "planner.unknown_candidate",
                &format!("{path}.asset_id"),
            ));
        }
        if indexes.insert(candidate.asset_id.clone(), index).is_some() {
            return Err(error(
                "planner.duplicate_candidate",
                &format!("{path}.asset_id"),
            ));
        }
        if candidate.content_hash.len() != 64
            || !candidate
                .content_hash
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(error("plan.invalid_hash", &format!("{path}.content_hash")));
        }
        let duration =
            crate::time(&candidate.duration, format!("{path}.duration")).map_err(validation)?;
        let start = crate::time(
            &candidate.source_range.start,
            format!("{path}.source_range.start"),
        )
        .map_err(validation)?;
        let end = crate::time(
            &candidate.source_range.end,
            format!("{path}.source_range.end"),
        )
        .map_err(validation)?;
        if duration.is_zero() {
            return Err(error("plan.invalid_range", &format!("{path}.duration")));
        }
        if start >= end {
            return Err(error("plan.invalid_range", &format!("{path}.source_range")));
        }
        if end > duration {
            return Err(error(
                "plan.source_out_of_bounds",
                &format!("{path}.source_range.end"),
            ));
        }
    }
    Ok(indexes)
}

/// Propose a declared M0 plan without probing sources, executing a host or relaxing hard choices.
pub fn plan_story(bytes: &[u8]) -> Result<PlannedStoryV1, PlanningErrorV1> {
    if bytes.len() > crate::MAX_INPUT_BYTES {
        return Err(error("input.too_large", "$"));
    }
    let probe: VersionProbe =
        serde_json::from_slice(bytes).map_err(|_| error("input.invalid_json", "$"))?;
    if probe.schema_version != 1 {
        return Err(error("planner.unsupported_version", "$.schema_version"));
    }
    let input: Input =
        serde_json::from_slice(bytes).map_err(|_| error("input.invalid_json", "$"))?;
    if input.schema_version != 1 {
        return Err(error("planner.unsupported_version", "$.schema_version"));
    }
    let audit = crate::audit_story_brief(input.brief.get().as_bytes()).map_err(|e| {
        let mut result = validation(e);
        result.path = brief_path(&result.path);
        result
    })?;
    if !audit.consistent {
        let conflicts = audit
            .conflicts
            .into_iter()
            .map(|mut conflict| {
                conflict.path = brief_path(&conflict.path);
                conflict
            })
            .collect();
        return Err(PlanningErrorV1 {
            code: "planner.brief_conflict".into(),
            path: "$.brief".into(),
            conflicts,
        });
    }
    // Projection only after the existing strict brief validator has accepted the entire object.
    let catalog: Catalog = serde_json::from_str(input.brief.get())
        .map_err(|_| error("input.invalid_json", "$.brief"))?;
    let catalog = catalog.assets.into_iter().map(|a| a.id).collect();
    let indexes = validate_candidates(&input, &catalog)?;
    for required in &audit.required_assets {
        if !indexes.contains_key(&required.asset_id) {
            return Err(error(
                "planner.required_candidate_missing",
                &brief_path(&required.origins[0]),
            ));
        }
    }
    if audit.required_assets.len() > input.max_shots as usize {
        return Err(error("planner.required_shots_exceeded", "$.max_shots"));
    }
    let clock = Clock::new(
        &input.candidates,
        &input.max_duration,
        audit.preferences.target_duration.as_ref(),
    )?;
    let required: BTreeMap<&str, &AssetBindingV1> = audit
        .required_assets
        .iter()
        .map(|row| (row.asset_id.as_str(), row))
        .collect();
    let excluded: BTreeSet<&str> = audit
        .excluded_assets
        .iter()
        .map(|row| row.asset_id.as_str())
        .collect();
    let preferred: BTreeMap<&str, usize> = audit
        .preferences
        .preferred_asset_ids
        .iter()
        .enumerate()
        .map(|(i, id)| (id.as_str(), i))
        .collect();
    let mut admitted = Vec::new();
    let mut total = 0;
    for row in &audit.required_assets {
        let index = indexes[&row.asset_id];
        if clock.windows[index] > clock.maximum - total {
            return Err(error(
                "planner.required_duration_exceeded",
                "$.max_duration",
            ));
        }
        total += clock.windows[index];
        admitted.push(index);
    }
    let mut reasons = BTreeMap::new();
    let mut optional = Vec::new();
    for (index, candidate) in input.candidates.iter().enumerate() {
        if excluded.contains(candidate.asset_id.as_str()) {
            reasons.insert(index, SkippedReasonV1::Excluded);
        } else if !required.contains_key(candidate.asset_id.as_str()) {
            optional.push(index);
        }
    }
    optional.sort_by(|&a, &b| {
        let a = &input.candidates[a];
        let b = &input.candidates[b];
        preferred
            .get(a.asset_id.as_str())
            .copied()
            .unwrap_or(usize::MAX)
            .cmp(
                &preferred
                    .get(b.asset_id.as_str())
                    .copied()
                    .unwrap_or(usize::MAX),
            )
            .then_with(|| b.score.cmp(&a.score))
            .then_with(|| a.asset_id.cmp(&b.asset_id))
    });
    for index in optional {
        let reason = if clock.target.is_some_and(|target| total >= target) {
            Some(SkippedReasonV1::NotNeeded)
        } else if admitted.len() >= input.max_shots as usize {
            Some(SkippedReasonV1::ShotLimit)
        } else if clock.windows[index] > clock.maximum - total {
            Some(SkippedReasonV1::DurationLimit)
        } else {
            None
        };
        if let Some(reason) = reason {
            reasons.insert(index, reason);
        } else {
            total += clock.windows[index];
            admitted.push(index);
        }
    }
    if admitted.is_empty() {
        return Err(error("planner.no_selection", "$.candidates"));
    }
    let ordered = order(&admitted, &input.candidates, &audit.precedence_edges)?;
    let selected: BTreeSet<usize> = ordered.iter().copied().collect();
    let assets = input
        .candidates
        .iter()
        .enumerate()
        .filter(|(index, _)| selected.contains(index))
        .map(|(_, candidate)| DeclaredAsset {
            id: candidate.asset_id.clone(),
            content_hash: candidate.content_hash.clone(),
            duration: copy_time(&candidate.duration),
        })
        .collect();
    let mut shots = Vec::new();
    let mut selections = Vec::new();
    let mut placement = 0;
    for (ordinal, &index) in ordered.iter().enumerate() {
        let candidate = &input.candidates[index];
        let shot_id = format!("shot-{:06}", ordinal + 1);
        let binding = required.get(candidate.asset_id.as_str());
        let mut paths = if let Some(binding) = binding {
            binding
                .origins
                .iter()
                .map(|path| brief_path(path))
                .collect()
        } else {
            let mut paths = Vec::new();
            if let Some(preference) = preferred.get(candidate.asset_id.as_str()) {
                paths.push(format!(
                    "$.brief.preferences.preferred_asset_ids[{preference}]"
                ));
            }
            paths.push(format!("$.candidates[{index}].score"));
            paths
        };
        paths.push(format!("$.candidates[{index}].source_range"));
        selections.push(SelectionDecisionV1 {
            asset_id: candidate.asset_id.clone(),
            shot_id: shot_id.clone(),
            required: binding.is_some(),
            declared_score: candidate.score,
            timeline_start: clock.time(placement),
            duration: clock.time(clock.windows[index]),
            reason_paths: paths,
        });
        placement += clock.windows[index];
        shots.push(DeclaredShot {
            id: shot_id,
            asset_id: candidate.asset_id.clone(),
            source_range: SourceRange {
                start: copy_time(&candidate.source_range.start),
                end: copy_time(&candidate.source_range.end),
            },
        });
    }
    let plan = PlanDocumentV1 {
        schema_version: 1,
        plan_id: audit.brief_id.clone(),
        assets,
        shots,
    };
    let plan_bytes =
        serde_json::to_vec(&plan).map_err(|_| error("planner.output_too_large", "$"))?;
    if plan_bytes.len() > crate::MAX_INPUT_BYTES {
        return Err(error("planner.output_too_large", "$.plan"));
    }
    crate::validate_plan_json(&plan_bytes).map_err(validation)?;
    let target_status = match clock.target {
        None => TargetStatusV1::NotRequested,
        Some(target) => match total.cmp(&target) {
            std::cmp::Ordering::Less => TargetStatusV1::Below,
            std::cmp::Ordering::Equal => TargetStatusV1::Met,
            std::cmp::Ordering::Greater => TargetStatusV1::Above,
        },
    };
    let skipped_candidates = reasons
        .into_iter()
        .map(|(index, reason)| SkippedCandidateV1 {
            asset_id: input.candidates[index].asset_id.clone(),
            path: format!("$.candidates[{index}]"),
            reason,
        })
        .collect();
    let report = PlannedStoryV1 {
        schema_version: 1,
        profile: "storytelling.fixed-window-planner.v1",
        scope: "declared-fixed-window-plan-only",
        executable: false,
        source_verified: false,
        host_verified: false,
        selection_planned: true,
        duration_analysis: "exact-fixed-windows",
        content_clock_rate: clock.rate.to_string(),
        total_duration: clock.time(total),
        target_status,
        brief_audit: audit,
        plan,
        selections,
        skipped_candidates,
    };
    if serde_json::to_vec(&report)
        .map_err(|_| error("planner.output_too_large", "$"))?
        .len()
        > 4_194_304
    {
        return Err(error("planner.output_too_large", "$"));
    }
    Ok(report)
}
