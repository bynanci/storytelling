// SPDX-License-Identifier: Apache-2.0
// Source semantics and attribution: docs/migration/provenance-m1-01.json, LICENSES/third-party/NOTICE.txt.

use std::cmp::Ordering;

use storytelling_core::wire::TimeValue;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) struct Exact {
    pub(super) numerator: u64,
    pub(super) denominator: u64,
}

impl Exact {
    pub(super) const ZERO: Self = Self {
        numerator: 0,
        denominator: 1,
    };

    pub(super) fn parse(raw_number_lexeme: &str) -> Option<Self> {
        let bytes = raw_number_lexeme.as_bytes();
        if bytes.is_empty() || bytes.len() > 64 {
            return None;
        }

        let mut cursor = match bytes[0] {
            b'0' => 1,
            b'1'..=b'9' => {
                let mut end = 1;
                while bytes.get(end).is_some_and(u8::is_ascii_digit) {
                    end += 1;
                }
                end
            }
            _ => return None,
        };
        let integer_end = cursor;
        let mut fraction_start = cursor;
        let mut fraction_end = cursor;
        if bytes.get(cursor) == Some(&b'.') {
            cursor += 1;
            fraction_start = cursor;
            while bytes.get(cursor).is_some_and(u8::is_ascii_digit) {
                cursor += 1;
            }
            fraction_end = cursor;
            if fraction_end == fraction_start {
                return None;
            }
        }

        let mut exponent = 0_i32;
        if matches!(bytes.get(cursor), Some(b'e' | b'E')) {
            cursor += 1;
            let negative_exponent = bytes.get(cursor) == Some(&b'-');
            if matches!(bytes.get(cursor), Some(b'+' | b'-')) {
                cursor += 1;
            }
            let exponent_start = cursor;
            while let Some(digit @ b'0'..=b'9') = bytes.get(cursor) {
                exponent = exponent
                    .checked_mul(10)?
                    .checked_add(i32::from(digit - b'0'))?;
                if exponent > 18 {
                    return None;
                }
                cursor += 1;
            }
            if cursor == exponent_start {
                return None;
            }
            if negative_exponent {
                exponent = -exponent;
            }
        }
        if cursor != bytes.len() {
            return None;
        }

        // Delay decimal zeroes so insignificant trailing digits cannot overflow.
        let mut numerator = 0_u128;
        let mut trailing_zeroes = 0_i32;
        for digit in bytes[..integer_end]
            .iter()
            .chain(bytes[fraction_start..fraction_end].iter())
        {
            if *digit == b'0' {
                if numerator != 0 {
                    trailing_zeroes += 1;
                }
            } else {
                for _ in 0..trailing_zeroes {
                    numerator = numerator.checked_mul(10)?;
                }
                numerator = numerator
                    .checked_mul(10)?
                    .checked_add(u128::from(digit - b'0'))?;
                trailing_zeroes = 0;
            }
        }
        if numerator == 0 {
            return Some(Self::ZERO);
        }

        let fractional_digits = i32::try_from(fraction_end - fraction_start).ok()?;
        let scale = fractional_digits - exponent - trailing_zeroes;
        if scale <= 0 {
            numerator = numerator.checked_mul(10_u128.checked_pow(scale.unsigned_abs())?)?;
            return Self::from_wide_parts(numerator, 1);
        }

        // Cancel powers separately before constructing a potentially large 10^scale.
        let mut twos = u32::try_from(scale).ok()?;
        let mut fives = twos;
        while twos > 0 && numerator % 2 == 0 {
            numerator /= 2;
            twos -= 1;
        }
        while fives > 0 && numerator % 5 == 0 {
            numerator /= 5;
            fives -= 1;
        }
        let denominator = 2_u128
            .checked_pow(twos)?
            .checked_mul(5_u128.checked_pow(fives)?)?;
        Self::from_wide_parts(numerator, denominator)
    }

    pub(super) fn from_parts(numerator: u64, denominator: u64) -> Option<Self> {
        Self::from_wide_parts(u128::from(numerator), u128::from(denominator))
    }

    pub(super) fn checked_add(self, other: Self) -> Option<Self> {
        let common = gcd(u128::from(self.denominator), u128::from(other.denominator));
        let left_scale = u128::from(other.denominator) / common;
        let right_scale = u128::from(self.denominator) / common;
        let numerator = u128::from(self.numerator)
            .checked_mul(left_scale)?
            .checked_add(u128::from(other.numerator).checked_mul(right_scale)?)?;
        let denominator = u128::from(self.denominator).checked_mul(left_scale)?;
        Self::from_wide_parts(numerator, denominator)
    }

    pub(super) fn checked_sub(self, other: Self) -> Option<Self> {
        let common = gcd(u128::from(self.denominator), u128::from(other.denominator));
        let left_scale = u128::from(other.denominator) / common;
        let right_scale = u128::from(self.denominator) / common;
        let numerator = u128::from(self.numerator)
            .checked_mul(left_scale)?
            .checked_sub(u128::from(other.numerator).checked_mul(right_scale)?)?;
        let denominator = u128::from(self.denominator).checked_mul(left_scale)?;
        Self::from_wide_parts(numerator, denominator)
    }

    pub(super) fn wire(self) -> TimeValue {
        TimeValue {
            numerator: self.numerator.to_string(),
            denominator: self.denominator.to_string(),
        }
    }

    fn from_wide_parts(numerator: u128, denominator: u128) -> Option<Self> {
        if denominator == 0 {
            return None;
        }
        let common = gcd(numerator, denominator);
        Some(Self {
            numerator: u64::try_from(numerator / common).ok()?,
            denominator: u64::try_from(denominator / common).ok()?,
        })
    }
}

impl Ord for Exact {
    fn cmp(&self, other: &Self) -> Ordering {
        (u128::from(self.numerator) * u128::from(other.denominator))
            .cmp(&(u128::from(other.numerator) * u128::from(self.denominator)))
    }
}

impl PartialOrd for Exact {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn gcd(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        (left, right) = (right, left % right);
    }
    left
}
