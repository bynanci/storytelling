// SPDX-License-Identifier: Apache-2.0
// Source-informed ordering policy; provenance-m2-order-opt.json and NOTICE.openfilm.
//! Stable topological ordering by candidate admission position.

use super::error;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};
use storytelling_core::planner::{PlanningCandidateV1, PlanningErrorV1};

pub(super) fn order(
    admitted: &[usize],
    candidates: &[PlanningCandidateV1],
    edges: &[storytelling_core::brief::PrecedenceEdgeV1],
) -> Result<Vec<usize>, PlanningErrorV1> {
    let positions: BTreeMap<&str, usize> = admitted
        .iter()
        .enumerate()
        .map(|(position, &index)| (candidates[index].asset_id.as_str(), position))
        .collect();
    let mut degrees = vec![0usize; admitted.len()];
    let mut outgoing = vec![Vec::new(); admitted.len()];
    for edge in edges {
        let before = positions
            .get(edge.before.as_str())
            .ok_or_else(|| error("planner.brief_conflict", "$.brief"))?;
        let after = positions
            .get(edge.after.as_str())
            .ok_or_else(|| error("planner.brief_conflict", "$.brief"))?;
        outgoing[*before].push(*after);
        degrees[*after] += 1;
    }
    // Compare admission positions, never catalog positions or asset names.
    // A newly unblocked earlier admission must preempt later queued vertices.
    let mut ready: BinaryHeap<_> = degrees
        .iter()
        .enumerate()
        .filter_map(|(position, &degree)| (degree == 0).then_some(Reverse(position)))
        .collect();
    let mut ordered = Vec::with_capacity(admitted.len());
    while let Some(Reverse(next)) = ready.pop() {
        ordered.push(admitted[next]);
        for &after in &outgoing[next] {
            degrees[after] -= 1;
            if degrees[after] == 0 {
                ready.push(Reverse(after));
            }
        }
    }
    if ordered.len() != admitted.len() {
        return Err(error("planner.brief_conflict", "$.brief"));
    }
    Ok(ordered)
}
