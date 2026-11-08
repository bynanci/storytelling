// SPDX-License-Identifier: Apache-2.0
// Downstream ordering regressions; docs/migration/provenance-m2-order-opt.json.
use super::ordering::order;
use std::collections::BTreeMap;
use storytelling_core::brief::PrecedenceEdgeV1;
use storytelling_core::planner::{PlanningCandidateV1, PlanningErrorV1};
use storytelling_core::wire::{SourceRange, TimeValue};

fn time(n: u64) -> TimeValue {
    TimeValue {
        numerator: n.to_string(),
        denominator: "1".into(),
    }
}

fn candidates(count: usize) -> Vec<PlanningCandidateV1> {
    (0..count)
        .map(|i| PlanningCandidateV1 {
            asset_id: format!("asset-{:04}", count - i),
            content_hash: "a".repeat(64),
            duration: time(1),
            source_range: SourceRange {
                start: time(0),
                end: time(1),
            },
            score: 0,
        })
        .collect()
}

fn edge(candidates: &[PlanningCandidateV1], before: usize, after: usize) -> PrecedenceEdgeV1 {
    PrecedenceEdgeV1 {
        before: candidates[before].asset_id.clone(),
        after: candidates[after].asset_id.clone(),
        origins: vec!["$.hard_constraints[0].asset_ids[1]".into()],
    }
}

// Enumerate whole permutations, independently of any topological traversal.
// Indices are admission positions, so the first valid permutation has the
// existing contract's lexicographically smallest admission-priority sequence.
fn permutations(values: &[usize]) -> Vec<Vec<usize>> {
    if values.is_empty() {
        return vec![vec![]];
    }
    let mut result = Vec::new();
    for (i, &head) in values.iter().enumerate() {
        let rest: Vec<_> = values.iter().copied().filter(|&v| v != values[i]).collect();
        for tail in permutations(&rest) {
            let mut row = vec![head];
            row.extend(tail);
            result.push(row);
        }
    }
    result
}

fn assert_conflict(error: PlanningErrorV1) {
    assert_eq!(error.code, "planner.brief_conflict");
    assert_eq!(error.path, "$.brief");
    assert!(error.conflicts.is_empty());
}

#[test]
fn compares_every_four_vertex_graph_with_brute_force_priority_order() {
    let candidates = candidates(6);
    let admitted = [5, 0, 3, 2]; // Neither catalog order nor ASCII ID order.
    let pairs: Vec<_> = (0..4)
        .flat_map(|a| (0..4).filter(move |&b| a != b).map(move |b| (a, b)))
        .collect();
    let permutations = permutations(&[0, 1, 2, 3]);
    for mask in 0..(1u32 << pairs.len()) {
        let selected: Vec<_> = pairs
            .iter()
            .enumerate()
            .filter_map(|(i, &pair)| ((mask & (1 << i)) != 0).then_some(pair))
            .collect();
        let edges: Vec<_> = selected
            .iter()
            .map(|&(a, b)| edge(&candidates, admitted[a], admitted[b]))
            .collect();
        let expected = permutations.iter().find(|row| {
            selected
                .iter()
                .all(|&(a, b)| row.iter().position(|&v| v == a) < row.iter().position(|&v| v == b))
        });
        let actual = order(&admitted, &candidates, &edges);
        match expected {
            Some(row) => assert_eq!(
                actual.unwrap(),
                row.iter().map(|&i| admitted[i]).collect::<Vec<_>>(),
                "graph {mask}"
            ),
            None => assert_conflict(actual.unwrap_err()),
        }
    }
}

#[test]
fn newly_ready_earlier_admission_precedes_already_queued_vertex() {
    let candidates = candidates(6);
    let admitted = [5, 0, 3, 2];
    let edges = [edge(&candidates, 3, 5)];
    // After 3, newly ready admission 0 must preempt the initially ready 2.
    assert_eq!(order(&admitted, &candidates, &edges).unwrap(), [0, 3, 5, 2]);
}

#[test]
fn duplicate_edges_empty_graphs_and_admission_permutations_preserve_results() {
    let candidates = candidates(4);
    assert_eq!(order(&[], &candidates, &[]).unwrap(), Vec::<usize>::new());
    let edges = [
        edge(&candidates, 2, 0),
        edge(&candidates, 2, 0),
        edge(&candidates, 3, 0),
    ];
    for admitted in permutations(&[0, 1, 2, 3]) {
        let expected = permutations(&[0, 1, 2, 3])
            .into_iter()
            .map(|row| row.iter().map(|&i| admitted[i]).collect::<Vec<_>>())
            .find(|row| {
                row.iter().position(|&i| i == 2) < row.iter().position(|&i| i == 0)
                    && row.iter().position(|&i| i == 3) < row.iter().position(|&i| i == 0)
            })
            .unwrap();
        assert_eq!(order(&admitted, &candidates, &edges).unwrap(), expected);
    }
}

#[test]
fn rejects_unselected_endpoints_self_edges_and_cycles_without_partial_order() {
    let candidates = candidates(4);
    for edges in [
        vec![edge(&candidates, 3, 0)],
        vec![edge(&candidates, 0, 3)],
        vec![edge(&candidates, 1, 1)],
        vec![edge(&candidates, 0, 1), edge(&candidates, 1, 0)],
    ] {
        assert_conflict(order(&[0, 1, 2], &candidates, &edges).unwrap_err());
    }
}

#[test]
fn maximum_candidate_graphs_keep_every_vertex_and_precedence() {
    let candidates = candidates(1024);
    let admitted: Vec<_> = (0..1024).rev().collect();
    assert_eq!(order(&admitted, &candidates, &[]).unwrap(), admitted);
    let chain: Vec<_> = admitted
        .windows(2)
        .map(|pair| edge(&candidates, pair[1], pair[0]))
        .collect();
    assert_eq!(
        order(&admitted, &candidates, &chain).unwrap(),
        admitted.iter().rev().copied().collect::<Vec<_>>()
    );
    let dense: Vec<_> = (0..64)
        .flat_map(|a| (64..128).map(move |b| (a, b)))
        .map(|(a, b)| edge(&candidates, admitted[a], admitted[b]))
        .collect();
    assert_eq!(order(&admitted, &candidates, &dense).unwrap(), admitted);
}

// Original Rust scheduling algorithm retained only as a benchmark baseline.
// Correctness expectations above use exhaustive whole-permutation enumeration.
fn legacy_order(
    admitted: &[usize],
    candidates: &[PlanningCandidateV1],
    edges: &[PrecedenceEdgeV1],
) -> Vec<usize> {
    let positions: BTreeMap<&str, usize> = admitted
        .iter()
        .enumerate()
        .map(|(i, &n)| (candidates[n].asset_id.as_str(), i))
        .collect();
    let mut degrees = vec![0; admitted.len()];
    let mut outgoing = vec![Vec::new(); admitted.len()];
    for edge in edges {
        let before = positions[edge.before.as_str()];
        let after = positions[edge.after.as_str()];
        outgoing[before].push(after);
        degrees[after] += 1;
    }
    let mut emitted = vec![false; admitted.len()];
    let mut ordered = Vec::new();
    while ordered.len() < admitted.len() {
        let next = (0..admitted.len())
            .find(|&i| !emitted[i] && degrees[i] == 0)
            .unwrap();
        emitted[next] = true;
        ordered.push(admitted[next]);
        for &after in &outgoing[next] {
            degrees[after] -= 1;
        }
    }
    ordered
}

#[test]
#[ignore = "manual informational release benchmark; no timing threshold"]
fn benchmark_ordering() {
    use std::{hint::black_box, time::Instant};
    fn median_ns(mut run: impl FnMut(), iterations: u32) -> u128 {
        for _ in 0..10 {
            run();
        }
        let mut samples = Vec::new();
        for _ in 0..7 {
            let start = Instant::now();
            for _ in 0..iterations {
                run();
            }
            samples.push(start.elapsed().as_nanos() / u128::from(iterations));
        }
        samples.sort();
        samples[3]
    }
    for count in [16, 256, 1024] {
        let candidates = candidates(count);
        let admitted: Vec<_> = (0..count).collect();
        for pattern in ["independent", "reverse-chain", "layered"] {
            let edges = match pattern {
                "reverse-chain" => (1..count)
                    .map(|i| edge(&candidates, i, i - 1))
                    .collect::<Vec<_>>(),
                "layered" => (0..count / 2)
                    .map(|i| edge(&candidates, i, i + count / 2))
                    .collect(),
                _ => vec![],
            };
            assert_eq!(
                order(&admitted, &candidates, &edges).unwrap(),
                legacy_order(&admitted, &candidates, &edges)
            );
            let scan_ns = median_ns(
                || {
                    black_box(legacy_order(
                        black_box(&admitted),
                        black_box(&candidates),
                        black_box(&edges),
                    ));
                },
                250,
            );
            let current_ns = median_ns(
                || {
                    black_box(
                        order(
                            black_box(&admitted),
                            black_box(&candidates),
                            black_box(&edges),
                        )
                        .unwrap(),
                    );
                },
                250,
            );
            println!(
                "{pattern} nodes={count} scan_ns={scan_ns} current_ns={current_ns} speedup={:.2}",
                scan_ns as f64 / current_ns as f64
            );
        }
    }
    let candidates = candidates(1024);
    let input = serde_json::to_vec(&serde_json::json!({
        "schema_version": 1,
        "brief": { "schema_version": 1, "brief_id": "ordering-benchmark", "intent": "Synthetic benchmark", "assets": candidates.iter().map(|c| serde_json::json!({"id": c.asset_id})).collect::<Vec<_>>(), "manual_selected_asset_ids": [], "hard_constraints": [], "preferences": {"preferred_asset_ids": []} },
        "candidates": candidates, "max_duration": {"numerator": "1024", "denominator": "1"}, "max_shots": 1024
    })).unwrap();
    let e2e_ns = median_ns(
        || {
            black_box(super::plan_story(black_box(&input)).unwrap());
        },
        40,
    );
    println!("full-plan-story nodes=1024 median_ns={e2e_ns}");
}
