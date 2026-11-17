// SPDX-License-Identifier: MIT

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Serialize, Serializer};

    // Catches an encoder that returns empty bytes, erases the failure, or leaks
    // serializer prose instead of the fixed adapter-only diagnostic.
    #[test]
    fn details_encoding_failure_returns_opaque_fixed_diagnostic() {
        struct CannotEncode;
        impl Serialize for CannotEncode {
            fn serialize<S: Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
                Err(serde::ser::Error::custom("private serializer cause"))
            }
        }
        let error = encode_details(&CannotEncode).unwrap_err();
        assert_eq!(error.code(), "sdk.error_details_encoding_failed");
        assert_eq!(error.to_string(), "sdk.error_details_encoding_failed");
    }
}
