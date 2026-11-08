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

enum TimePath {
    Maximum,
    Target,
    Candidate(usize, &'static str),
}

impl TimePath {
    fn render(&self) -> String {
        match self {
            Self::Maximum => "$.max_duration".to_owned(),
            Self::Target => "$.brief.preferences.target_duration".to_owned(),
            Self::Candidate(index, field) => format!("$.candidates[{index}].{field}"),
        }
    }
}

struct CachedTime<'a> {
    value: &'a TimeValue,
    denominator: u64,
    path: TimePath,
}

impl<'a> CachedTime<'a> {
    fn admit(
        value: &'a TimeValue,
        path: TimePath,
        rate: &mut u64,
    ) -> Result<Self, PlanningErrorV1> {
        // The caller already validated canonical positive denominators. Preserve
        // the complete LCM pass before any numerator/scaled-value failure.
        let denominator: u64 = value
            .denominator
            .parse()
            .map_err(|_| error("plan.invalid_time", "$"))?;
        let next = u128::from(*rate / gcd(*rate, denominator)) * u128::from(denominator);
        *rate = u64::try_from(next).map_err(|_| error("planner.unrepresentable_time", "$"))?;
        Ok(Self {
            value,
            denominator,
            path,
        })
    }

    fn scale(&self, rate: u64) -> Result<u64, PlanningErrorV1> {
        scale_cached(self.value, self.denominator, rate, || self.path.render())
    }
}

// The cached denominator is positive and divides the completed common rate.
// Rendering a path is an error-only operation, tested without wall-clock limits.
pub(super) fn scale_cached(
    value: &TimeValue,
    denominator: u64,
    rate: u64,
    path: impl FnOnce() -> String,
) -> Result<u64, PlanningErrorV1> {
    let numerator = match value.numerator.parse::<u64>() {
        Ok(value) => value,
        Err(_) => return Err(error("plan.invalid_time", &path())),
    };
    u64::try_from(u128::from(numerator) * u128::from(rate / denominator))
        .map_err(|_| error("planner.unrepresentable_time", &path()))
}

impl Clock {
    pub fn new(
        candidates: &[PlanningCandidateV1],
        maximum: &TimeValue,
        target: Option<&TimeValue>,
    ) -> Result<Self, PlanningErrorV1> {
        let mut rate = 1;
        let mut values = Vec::with_capacity(3 * candidates.len() + 2);
        values.push(CachedTime::admit(maximum, TimePath::Maximum, &mut rate)?);
        if let Some(value) = target {
            values.push(CachedTime::admit(value, TimePath::Target, &mut rate)?);
        }
        for (index, candidate) in candidates.iter().enumerate() {
            for (value, field) in [
                (&candidate.duration, "duration"),
                (&candidate.source_range.start, "source_range.start"),
                (&candidate.source_range.end, "source_range.end"),
            ] {
                values.push(CachedTime::admit(
                    value,
                    TimePath::Candidate(index, field),
                    &mut rate,
                )?);
            }
        }
        // Scale in the original maximum, target, duration/start/end order.
        // Even unselected candidates still participate in representability checks.
        let maximum = values[0].scale(rate)?;
        let candidate_offset = 1 + usize::from(target.is_some());
        let target = target.map(|_| values[1].scale(rate)).transpose()?;
        let windows = values[candidate_offset..]
            .chunks_exact(3)
            .map(|chunk| {
                chunk[0].scale(rate)?;
                let start = chunk[1].scale(rate)?;
                let end = chunk[2].scale(rate)?;
                Ok(end - start)
            })
            .collect::<Result<Vec<_>, PlanningErrorV1>>()?;
        Ok(Self {
            rate,
            maximum,
            target,
            windows,
        })
    }

    pub fn time(&self, ticks: u64) -> TimeValue {
        let divisor = gcd(ticks, self.rate);
        TimeValue {
            numerator: (ticks / divisor).to_string(),
            denominator: (self.rate / divisor).to_string(),
        }
    }
}
