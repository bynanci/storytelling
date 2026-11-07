// SPDX-License-Identifier: Apache-2.0
// Source-informed exact finite planner timing; see provenance-m2-02.json.
use super::error;
use storytelling_core::planner::{PlanningCandidateV1, PlanningErrorV1};
use storytelling_core::wire::TimeValue;

pub(super) struct Clock {
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

impl Clock {
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
            rate = u64::try_from(next).map_err(|_| error("planner.unrepresentable_time", "$"))?;
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
