//! Exact, reduced, nonnegative rational time. No float conversion or rounding.
use crate::ValidationError;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RationalTime {
    numerator: u64,
    denominator: u64,
}

fn decimal(value: &str) -> Result<u64, ValidationError> {
    if value.is_empty()
        || value.len() > 20
        || !value.bytes().all(|byte| byte.is_ascii_digit())
        || (value.len() > 1 && value.starts_with('0'))
    {
        return Err(ValidationError::new("plan.invalid_time", "$"));
    }
    value
        .parse()
        .map_err(|_| ValidationError::new("plan.invalid_time", "$"))
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

impl RationalTime {
    /// Rejects ambiguous encodings rather than silently normalizing them.
    pub fn from_strings(numerator: &str, denominator: &str) -> Result<Self, ValidationError> {
        let numerator = decimal(numerator)?;
        let denominator = decimal(denominator)?;
        if denominator == 0 || gcd(numerator, denominator) != 1 {
            return Err(ValidationError::new("plan.invalid_time", "$"));
        }
        Ok(Self {
            numerator,
            denominator,
        })
    }

    pub fn is_zero(self) -> bool {
        self.numerator == 0
    }
}

impl Ord for RationalTime {
    fn cmp(&self, other: &Self) -> Ordering {
        // A product of two u64 values always fits in u128.
        (u128::from(self.numerator) * u128::from(other.denominator))
            .cmp(&(u128::from(other.numerator) * u128::from(self.denominator)))
    }
}

impl PartialOrd for RationalTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
