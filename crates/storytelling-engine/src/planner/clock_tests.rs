// SPDX-License-Identifier: Apache-2.0
// Downstream exact-clock regressions; provenance-m2-clock-opt.json and LICENSES/third-party/NOTICE.txt.
use super::clock::{Clock, scale_cached};
use super::error;
use std::cell::Cell;
use storytelling_core::planner::PlanningCandidateV1;
use storytelling_core::wire::{SourceRange, TimeValue};

fn time(n: u64, d: u64) -> TimeValue {
    let mut a = n;
    let mut b = d;
    while b != 0 {
        (a, b) = (b, a % b);
    }
    TimeValue {
        numerator: (n / a).to_string(),
        denominator: (d / a).to_string(),
    }
}

fn candidate(i: usize, n: u64, d: u64) -> PlanningCandidateV1 {
    PlanningCandidateV1 {
        asset_id: format!("asset-{i}"),
        content_hash: "a".repeat(64),
        duration: time(n + 3, d),
        source_range: SourceRange {
            start: time(n, d),
            end: time(n + 1, d),
        },
        score: 0,
    }
}

#[test]
fn successful_cached_scaling_never_formats_an_error_path() {
    let calls = Cell::new(0);
    let ticks = scale_cached(&time(5, 4), 4, 12, || {
        calls.set(calls.get() + 1);
        "$.candidates[7].source_range.end".into()
    })
    .unwrap();
    assert_eq!(ticks, 15);
    assert_eq!(
        calls.get(),
        0,
        "success must not allocate/render a diagnostic path"
    );
}

#[test]
fn cached_scaling_formats_the_precise_failure_path_once() {
    for (numerator, code) in [
        ("bad", "plan.invalid_time"),
        ("18446744073709551615", "planner.unrepresentable_time"),
    ] {
        let calls = Cell::new(0);
        let value = TimeValue {
            numerator: numerator.into(),
            denominator: "1".into(),
        };
        let error = scale_cached(&value, 1, 2, || {
            calls.set(calls.get() + 1);
            "$.candidates[7].duration".into()
        })
        .unwrap_err();
        assert_eq!(error.code, code);
        assert_eq!(error.path, "$.candidates[7].duration");
        assert_eq!(calls.get(), 1);
    }
}

fn assert_same_clock(
    candidates: &[PlanningCandidateV1],
    max: &TimeValue,
    target: Option<&TimeValue>,
) {
    let old = legacy::LegacyClock::new(candidates, max, target);
    let new = Clock::new(candidates, max, target);
    match (old, new) {
        (Ok(old), Ok(new)) => {
            assert_eq!(new.rate, old.rate);
            assert_eq!(new.maximum, old.maximum);
            assert_eq!(new.target, old.target);
            assert_eq!(new.windows, old.windows);
            for ticks in [0, 1, new.maximum, u64::MAX] {
                assert_eq!(new.time(ticks).numerator, old.time(ticks).numerator);
                assert_eq!(new.time(ticks).denominator, old.time(ticks).denominator);
            }
        }
        (Err(old), Err(new)) => {
            assert_eq!(new.code, old.code);
            assert_eq!(new.path, old.path);
            assert!(new.conflicts.is_empty());
        }
        _ => panic!("clock result changed"),
    }
}

#[test]
fn fractional_clock_preserves_exact_windows_and_reduced_placements() {
    let mut a = candidate(0, 2, 3);
    a.duration = time(17, 3);
    a.source_range.end = time(11, 3);
    let mut b = candidate(1, 1, 2);
    b.duration = time(7, 2);
    b.source_range.end = time(5, 2);
    let result = Clock::new(&[a, b], &time(11, 2), Some(&time(9, 4))).unwrap();
    assert_eq!(result.rate, 12);
    assert_eq!(result.maximum, 66);
    assert_eq!(result.target, Some(27));
    assert_eq!(result.windows, [36, 24]);
    assert_eq!(
        (result.time(0).numerator, result.time(0).denominator),
        ("0".into(), "1".into())
    );
    assert_eq!(
        (result.time(60).numerator, result.time(60).denominator),
        ("5".into(), "1".into())
    );
}

#[test]
fn lcm_overflow_precedes_earlier_scaled_value_overflow() {
    let values = [candidate(0, 0, 2), candidate(1, 1, u64::MAX)];
    let e = Clock::new(&values, &time(u64::MAX, 1), None).err().unwrap();
    assert_eq!(e.code, "planner.unrepresentable_time");
    assert_eq!(e.path, "$");
    assert_same_clock(&values, &time(u64::MAX, 1), None);
}

#[test]
fn scaling_failure_priority_remains_maximum_then_target_then_candidate() {
    let mut a = candidate(0, 0, 1);
    a.duration = time(u64::MAX, 1);
    let values = [a, candidate(1, 0, 2)];
    for (max, target, path) in [
        (time(u64::MAX, 1), Some(time(u64::MAX, 1)), "$.max_duration"),
        (
            time(1, 1),
            Some(time(u64::MAX, 1)),
            "$.brief.preferences.target_duration",
        ),
        (time(1, 1), None, "$.candidates[0].duration"),
    ] {
        let e = Clock::new(&values, &max, target.as_ref()).err().unwrap();
        assert_eq!(e.code, "planner.unrepresentable_time");
        assert_eq!(e.path, path);
        assert_same_clock(&values, &max, target.as_ref());
    }
}

#[test]
fn generated_canonical_clock_inputs_match_the_accepted_implementation() {
    let denominators = [1, 2, 3, 5, 7, 11, 97, 4_294_967_291, u64::MAX];
    let mut seed = 20261108u64;
    let mut next = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        seed
    };
    for round in 0..2048 {
        let count = (next() % 8) as usize;
        let candidates: Vec<_> = (0..count)
            .map(|i| {
                candidate(
                    i,
                    next() % 20,
                    denominators[(next() % denominators.len() as u64) as usize],
                )
            })
            .collect();
        let max = time(
            if round % 7 == 0 {
                u64::MAX
            } else {
                next() % 1000 + 1
            },
            denominators[(next() % 7) as usize],
        );
        let target = (round % 3 != 0).then(|| {
            time(
                if round % 11 == 0 {
                    u64::MAX
                } else {
                    next() % 1000 + 1
                },
                denominators[(next() % 7) as usize],
            )
        });
        assert_same_clock(&candidates, &max, target.as_ref());
    }
}

#[test]
fn full_candidate_budget_cache_and_empty_private_clock_keep_exact_results() {
    let candidates: Vec<_> = (0..1024)
        .map(|i| candidate(i, (i % 19) as u64, [1, 2, 3, 7, 13][i % 5]))
        .collect();
    assert_same_clock(&candidates, &time(4096, 1), Some(&time(2048, 1)));
    assert_same_clock(&[], &time(u64::MAX, 1), None);
    assert_same_clock(&[], &time(5, 2), Some(&time(1, 3)));
}

#[test]
#[ignore = "manual informational release benchmark; no timing assertion"]
fn benchmark_clock_cache() {
    use std::{hint::black_box, time::Instant};
    fn median_ns(mut run: impl FnMut(), n: u32) -> u128 {
        for _ in 0..10 {
            run();
        }
        let mut samples = Vec::new();
        for _ in 0..7 {
            let start = Instant::now();
            for _ in 0..n {
                run();
            }
            samples.push(start.elapsed().as_nanos() / u128::from(n));
        }
        samples.sort();
        samples[3]
    }
    for count in [16, 256, 1024] {
        let candidates: Vec<_> = (0..count)
            .map(|i| candidate(i, (i % 19) as u64, [1, 2, 3, 8][i % 4]))
            .collect();
        let max = time(4096, 1);
        let target = time(2048, 1);
        assert_same_clock(&candidates, &max, Some(&target));
        let old = median_ns(
            || {
                black_box(
                    legacy::LegacyClock::new(
                        black_box(&candidates),
                        black_box(&max),
                        Some(black_box(&target)),
                    )
                    .unwrap(),
                );
            },
            250,
        );
        let new = median_ns(
            || {
                black_box(
                    Clock::new(
                        black_box(&candidates),
                        black_box(&max),
                        Some(black_box(&target)),
                    )
                    .unwrap(),
                );
            },
            250,
        );
        println!(
            "clock nodes={count} legacy_ns={old} current_ns={new} speedup={:.2}",
            old as f64 / new as f64
        );
    }
    let candidates: Vec<_> = (0..1024).map(|i| candidate(i, 0, 1)).collect();
    let input=serde_json::to_vec(&serde_json::json!({
        "schema_version":1,"brief":{"schema_version":1,"brief_id":"clock-benchmark","intent":"Synthetic benchmark","assets":candidates.iter().map(|a|serde_json::json!({"id":a.asset_id})).collect::<Vec<_>>(),"manual_selected_asset_ids":[],"hard_constraints":[],"preferences":{"preferred_asset_ids":[]}},
        "candidates":candidates,"max_duration":{"numerator":"1024","denominator":"1"},"max_shots":1024
    })).unwrap();
    let ns = median_ns(
        || {
            black_box(super::plan_story(black_box(&input)).unwrap());
        },
        40,
    );
    println!("full-plan-story nodes=1024 median_ns={ns}");
}

// Original Rust factory retained test-only for compatibility and measurements.
mod legacy {
    // SPDX-License-Identifier: Apache-2.0
    // Source-informed exact finite planner timing; see provenance-m2-02.json.
    use super::error;
    use storytelling_core::planner::{PlanningCandidateV1, PlanningErrorV1};
    use storytelling_core::wire::TimeValue;

    pub(super) struct LegacyClock {
        pub rate: u64,
        pub maximum: u64,
        pub target: Option<u64>,
        pub windows: Vec<u64>,
    }

    fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }

    impl LegacyClock {
        pub fn new(
            candidates: &[PlanningCandidateV1],
            maximum: &TimeValue,
            target: Option<&TimeValue>,
        ) -> Result<Self, PlanningErrorV1> {
            let mut values = vec![(maximum, "$.max_duration".to_owned())];
            if let Some(value) = target {
                values.push((value, "$.brief.preferences.target_duration".to_owned()));
            }
            for (index, candidate) in candidates.iter().enumerate() {
                let path = format!("$.candidates[{index}]");
                values.push((&candidate.duration, format!("{path}.duration")));
                values.push((
                    &candidate.source_range.start,
                    format!("{path}.source_range.start"),
                ));
                values.push((
                    &candidate.source_range.end,
                    format!("{path}.source_range.end"),
                ));
            }
            let mut rate = 1;
            for (value, _) in &values {
                // All syntax/domain checks precede clock construction.
                let denominator: u64 = value
                    .denominator
                    .parse()
                    .map_err(|_| error("plan.invalid_time", "$"))?;
                let next = u128::from(rate / gcd(rate, denominator)) * u128::from(denominator);
                rate =
                    u64::try_from(next).map_err(|_| error("planner.unrepresentable_time", "$"))?;
            }
            for (value, path) in &values {
                Self::scale(value, rate, path)?;
            }
            let windows = candidates
                .iter()
                .enumerate()
                .map(|(index, candidate)| {
                    let start = Self::scale(
                        &candidate.source_range.start,
                        rate,
                        &format!("$.candidates[{index}].source_range.start"),
                    )?;
                    let end = Self::scale(
                        &candidate.source_range.end,
                        rate,
                        &format!("$.candidates[{index}].source_range.end"),
                    )?;
                    Ok(end - start)
                })
                .collect::<Result<Vec<_>, PlanningErrorV1>>()?;
            Ok(Self {
                rate,
                maximum: Self::scale(maximum, rate, "$.max_duration")?,
                target: target
                    .map(|value| Self::scale(value, rate, "$.brief.preferences.target_duration"))
                    .transpose()?,
                windows,
            })
        }

        fn scale(value: &TimeValue, rate: u64, path: &str) -> Result<u64, PlanningErrorV1> {
            let numerator: u64 = value
                .numerator
                .parse()
                .map_err(|_| error("plan.invalid_time", path))?;
            let denominator: u64 = value
                .denominator
                .parse()
                .map_err(|_| error("plan.invalid_time", path))?;
            u64::try_from(u128::from(numerator) * u128::from(rate / denominator))
                .map_err(|_| error("planner.unrepresentable_time", path))
        }

        pub fn time(&self, ticks: u64) -> TimeValue {
            let divisor = gcd(ticks, self.rate);
            TimeValue {
                numerator: (ticks / divisor).to_string(),
                denominator: (self.rate / divisor).to_string(),
            }
        }
    }
}
