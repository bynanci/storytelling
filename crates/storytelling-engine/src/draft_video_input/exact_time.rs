// SPDX-License-Identifier: MIT
//! Exact author-input normalization, separate from accepted V1 time validation.

use storytelling_core::time::RationalTime;

const INVALID_DECIMAL: &str = "draft_time.invalid_decimal";
const INVALID_FRAME: &str = "draft_time.invalid_frame";
const INVALID_FPS: &str = "draft_time.invalid_fps";
const OVERFLOW: &str = "draft_time.overflow";

fn canonical_integer(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| byte.is_ascii_digit())
        && (value.len() == 1 || !value.starts_with('0'))
}

fn integer(value: &str, invalid: &'static str) -> Result<u64, &'static str> {
    if !canonical_integer(value) {
        return Err(invalid);
    }
    value.parse::<u64>().map_err(|_| OVERFLOW)
}

fn gcd(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        (left, right) = (right, left % right);
    }
    left
}

fn reduced(numerator: u128, denominator: u128) -> Result<(String, String), &'static str> {
    if denominator == 0 {
        return Err(OVERFLOW);
    }
    let divisor = gcd(numerator, denominator);
    let numerator = u64::try_from(numerator / divisor).map_err(|_| OVERFLOW)?;
    let denominator = u64::try_from(denominator / divisor).map_err(|_| OVERFLOW)?;
    let result = (numerator.to_string(), denominator.to_string());
    RationalTime::from_strings(&result.0, &result.1).map_err(|_| OVERFLOW)?;
    Ok(result)
}

// Digits are numeric values 0..=9, not ASCII. The caller divides only when
// the last digit proves exact divisibility, so no truncation occurs. At most
// 128 digits and 126 factors of each kind bound all scans before integer parse.
fn divide_decimal(digits: &mut Vec<u8>, divisor: u8) -> Result<(), &'static str> {
    let divisor = u128::from(divisor);
    let mut remainder = 0_u128;
    for digit in digits.iter_mut() {
        let dividend = remainder
            .checked_mul(10)
            .and_then(|value| value.checked_add(u128::from(*digit)))
            .ok_or(OVERFLOW)?;
        *digit = u8::try_from(dividend / divisor).map_err(|_| OVERFLOW)?;
        remainder = dividend % divisor;
    }
    if remainder != 0 {
        return Err(OVERFLOW);
    }
    let leading_zeroes = digits.iter().take_while(|digit| **digit == 0).count();
    digits.drain(..leading_zeroes);
    Ok(())
}

pub(super) fn decimal(value: &str) -> Result<(String, String), &'static str> {
    if value.is_empty() || value.len() > 128 {
        return Err(INVALID_DECIMAL);
    }
    let (whole, fraction) = match value.split_once('.') {
        Some((whole, fraction)) => {
            if fraction.is_empty() || !fraction.bytes().all(|byte| byte.is_ascii_digit()) {
                return Err(INVALID_DECIMAL);
            }
            (whole, fraction)
        }
        None => (value, ""),
    };
    if !canonical_integer(whole) {
        return Err(INVALID_DECIMAL);
    }

    // Remove only fractional zeroes. Integer zeroes retain their place value.
    let fraction = fraction.trim_end_matches('0');
    let mut digits: Vec<_> = whole
        .bytes()
        .chain(fraction.bytes())
        .map(|digit| digit - b'0')
        .collect();
    let leading_zeroes = digits.iter().take_while(|digit| **digit == 0).count();
    digits.drain(..leading_zeroes);
    if digits.is_empty() {
        return reduced(0, 1);
    }

    let scale = u32::try_from(fraction.len()).map_err(|_| OVERFLOW)?;
    let mut twos = scale;
    let mut fives = scale;
    while twos > 0 && digits.last().is_some_and(|digit| *digit % 2 == 0) {
        divide_decimal(&mut digits, 2)?;
        twos -= 1;
    }
    while fives > 0 && digits.last().is_some_and(|digit| *digit % 5 == 0) {
        divide_decimal(&mut digits, 5)?;
        fives -= 1;
    }

    // All decimal denominator factors have now been cancelled. An overflow
    // here cannot conceal a smaller representable final u64 fraction.
    let numerator = digits.into_iter().try_fold(0_u128, |value, digit| {
        value
            .checked_mul(10)
            .and_then(|value| value.checked_add(u128::from(digit)))
            .ok_or(OVERFLOW)
    })?;
    let denominator = 2_u128
        .checked_pow(twos)
        .and_then(|value| value.checked_mul(5_u128.checked_pow(fives)?))
        .ok_or(OVERFLOW)?;
    reduced(numerator, denominator)
}

pub(super) fn fps(numerator: &str, denominator: &str) -> Result<(u64, u64), &'static str> {
    let numerator = integer(numerator, INVALID_FPS).map_err(|_| INVALID_FPS)?;
    let denominator = integer(denominator, INVALID_FPS).map_err(|_| INVALID_FPS)?;
    if numerator == 0
        || denominator == 0
        || gcd(u128::from(numerator), u128::from(denominator)) != 1
    {
        return Err(INVALID_FPS);
    }
    Ok((numerator, denominator))
}

pub(super) fn frame(value: &str, rate: (u64, u64)) -> Result<(String, String), &'static str> {
    let (numerator, denominator) = rate;
    if numerator == 0
        || denominator == 0
        || gcd(u128::from(numerator), u128::from(denominator)) != 1
    {
        return Err(INVALID_FPS);
    }
    let frame = integer(value, INVALID_FRAME)?;
    let numerator_wide = u128::from(frame)
        .checked_mul(u128::from(denominator))
        .ok_or(OVERFLOW)?;
    reduced(numerator_wide, u128::from(numerator))
}

#[cfg(test)]
mod tests {
    use super::{decimal, fps, frame};
    use serde_json::Value;
    use storytelling_core::time::RationalTime;

    fn cases() -> Value {
        serde_json::from_str(include_str!(
            "../../../../tests/fixtures/n10-06a-draft/cases.json"
        ))
        .unwrap()
    }

    fn expected(value: &Value) -> (String, String) {
        (
            value["numerator"].as_str().unwrap().to_owned(),
            value["denominator"].as_str().unwrap().to_owned(),
        )
    }

    #[test]
    fn normalizes_frozen_decimal_examples_and_verifies_canonical_output() {
        for case in cases()["decimal_success"].as_array().unwrap() {
            let result = decimal(case["input"].as_str().unwrap()).unwrap();
            assert_eq!(result, expected(&case["expected"]), "{}", case["id"]);
            assert!(RationalTime::from_strings(&result.0, &result.1).is_ok());
        }
    }

    #[test]
    fn rejects_frozen_decimal_syntax_and_overflow_without_rounding() {
        for case in cases()["decimal_reject"].as_array().unwrap() {
            assert_eq!(
                decimal(case["input"].as_str().unwrap()).unwrap_err(),
                case["code"].as_str().unwrap(),
                "{case}"
            );
        }
        for case in cases()["overflow"].as_array().unwrap() {
            if case["kind"] == "seconds" {
                assert_eq!(
                    decimal(case["input"].as_str().unwrap()).unwrap_err(),
                    case["code"].as_str().unwrap(),
                    "{case}"
                );
            }
        }
    }

    #[test]
    fn normalizes_frozen_frame_boundaries_and_reduces_before_u64_check() {
        for case in cases()["frame_boundary_success"].as_array().unwrap() {
            let rate = fps(
                case["fps"]["numerator"].as_str().unwrap(),
                case["fps"]["denominator"].as_str().unwrap(),
            )
            .unwrap();
            let result = frame(case["frame"].as_str().unwrap(), rate).unwrap();
            assert_eq!(result, expected(&case["expected"]));
            assert!(RationalTime::from_strings(&result.0, &result.1).is_ok());
        }
    }

    #[test]
    fn rejects_frozen_frame_inputs_and_final_overflow() {
        for case in cases()["frame_reject"].as_array().unwrap() {
            let rate = fps(
                case["fps"]["numerator"].as_str().unwrap(),
                case["fps"]["denominator"].as_str().unwrap(),
            )
            .unwrap();
            assert_eq!(
                frame(case["frame"].as_str().unwrap(), rate).unwrap_err(),
                case["code"].as_str().unwrap(),
                "{case}"
            );
        }
    }

    #[test]
    fn rejects_nonpositive_noncanonical_or_nonreduced_fps() {
        for case in cases()["fps_reject"].as_array().unwrap() {
            assert_eq!(
                fps(
                    case["numerator"].as_str().unwrap(),
                    case["denominator"].as_str().unwrap(),
                ),
                Err("draft_time.invalid_fps"),
                "{case}"
            );
        }
        for (numerator, denominator) in [("01", "1"), ("1", "01"), ("NaN", "1")] {
            assert_eq!(fps(numerator, denominator), Err("draft_time.invalid_fps"));
        }
    }

    #[test]
    fn preserves_exact_thirty_fps_half_open_boundaries() {
        let rate = fps("30", "1").unwrap();
        assert_eq!(frame("0", rate).unwrap(), ("0".into(), "1".into()));
        assert_eq!(frame("75", rate).unwrap(), ("5".into(), "2".into()));
        assert_eq!(frame("135", rate).unwrap(), ("9".into(), "2".into()));
    }

    #[test]
    fn accepts_maximum_decimal_byte_bound_and_rejects_the_next_byte() {
        let at_limit = format!("0.{}", "0".repeat(126));
        assert_eq!(at_limit.len(), 128);
        assert_eq!(decimal(&at_limit).unwrap(), ("0".into(), "1".into()));
        assert_eq!(
            decimal(&(at_limit + "0")),
            Err("draft_time.invalid_decimal")
        );
    }

    #[test]
    fn checks_denominator_overflow_and_keeps_decimal_integer_zeroes() {
        assert_eq!(
            decimal("0.00000000000000000001"),
            Err("draft_time.overflow")
        );
        assert_eq!(decimal("10.0").unwrap(), ("10".into(), "1".into()));
        assert_eq!(decimal("100.0500").unwrap(), ("2001".into(), "20".into()));
    }
}
