// SPDX-License-Identifier: Apache-2.0
// Origin/modifications: docs/migration/provenance-m1-02.json; see LICENSES/third-party/NOTICE.txt.
//! Exact bounded content ticks, independent of the declared project frame rate.

use storytelling_core::compiled_plan::{FeatureDisposition, OtioCompileError, OtioFeatureV1};
use storytelling_core::time::RationalTime;
use storytelling_core::wire::{PlanDocumentV1, TimeValue};

pub(crate) const MAX_NATIVE_INTEGER: u64 = 9_007_199_254_740_991;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ContentClock {
    rate: u64,
}

impl ContentClock {
    pub(crate) fn for_plan(plan: &PlanDocumentV1) -> Result<Self, OtioCompileError> {
        let mut rate = 1;
        for (index, asset) in plan.assets.iter().enumerate() {
            rate = include_denominator(
                rate,
                &asset.duration,
                &format!("$.assets[{index}].duration"),
            )?;
        }
        for (index, shot) in plan.shots.iter().enumerate() {
            let path = format!("$.shots[{index}].source_range");
            rate = include_denominator(rate, &shot.source_range.start, &path)?;
            rate = include_denominator(rate, &shot.source_range.end, &path)?;
        }
        Ok(Self { rate })
    }

    pub(crate) fn rate(self) -> u64 {
        self.rate
    }

    pub(crate) fn ticks(self, value: &TimeValue, path: &str) -> Result<u64, OtioCompileError> {
        let (numerator, denominator) = parts(value, path)?;
        if self.rate % denominator != 0 {
            return Err(unrepresentable(path));
        }
        let ticks = u128::from(numerator)
            .checked_mul(u128::from(self.rate / denominator))
            .ok_or_else(|| unrepresentable(path))?;
        bounded(ticks, path)
    }

    pub(crate) fn add_timeline_ticks(
        self,
        total: u64,
        duration: u64,
    ) -> Result<u64, OtioCompileError> {
        let ticks = u128::from(total)
            .checked_add(u128::from(duration))
            .ok_or_else(|| unrepresentable("$.timeline_duration"))?;
        bounded(ticks, "$.timeline_duration")
    }

    pub(crate) fn seconds(self, ticks: u64) -> TimeValue {
        let common = gcd(ticks, self.rate);
        TimeValue {
            numerator: (ticks / common).to_string(),
            denominator: (self.rate / common).to_string(),
        }
    }
}

fn parts(value: &TimeValue, path: &str) -> Result<(u64, u64), OtioCompileError> {
    // Reuse the M0 canonical rational rules, including the unique zero 0/1.
    RationalTime::from_strings(&value.numerator, &value.denominator)
        .map_err(|_| invalid_time(path))?;
    let numerator = value.numerator.parse().map_err(|_| invalid_time(path))?;
    let denominator = value.denominator.parse().map_err(|_| invalid_time(path))?;
    Ok((numerator, denominator))
}

fn include_denominator(rate: u64, value: &TimeValue, path: &str) -> Result<u64, OtioCompileError> {
    let (_, denominator) = parts(value, path)?;
    let common = gcd(rate, denominator);
    let lcm = u128::from(rate / common)
        .checked_mul(u128::from(denominator))
        .ok_or_else(|| unrepresentable("$.content_clock"))?;
    bounded(lcm, "$.content_clock")
}

fn bounded(value: u128, path: &str) -> Result<u64, OtioCompileError> {
    if value > u128::from(MAX_NATIVE_INTEGER) {
        return Err(unrepresentable(path));
    }
    u64::try_from(value).map_err(|_| unrepresentable(path))
}

fn gcd(mut left: u64, mut right: u64) -> u64 {
    while right != 0 {
        (left, right) = (right, left % right);
    }
    left
}

fn invalid_time(path: &str) -> OtioCompileError {
    OtioCompileError {
        code: "otio.invalid_time".to_owned(),
        path: path.to_owned(),
        losses: Vec::new(),
    }
}

fn unrepresentable(path: &str) -> OtioCompileError {
    OtioCompileError {
        code: "otio.unrepresentable_time".to_owned(),
        path: path.to_owned(),
        losses: vec![OtioFeatureV1 {
            feature: "clock.unrepresentable".to_owned(),
            path: path.to_owned(),
            disposition: FeatureDisposition::Unsupported,
            accepted: false,
        }],
    }
}
