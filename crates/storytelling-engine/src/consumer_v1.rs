// SPDX-License-Identifier: MIT
//! Versioned, synchronous read-only consumer surface (ADR 0013).
//!
//! Calls delegate original immutable buffers once to accepted backends. Results
//! own their data; independent calls can use caller-controlled threads. Limits
//! and diagnostic order belong to each backend, with no shared pre-parser,
//! cache, implicit I/O or background work. Bound concurrency and memory at the
//! caller. Dropping a result performs no Host cancellation or rollback.
//!
//! V1 here versions the Rust facade, separately from document/profile/compiler
//! versions and the unpublished crate version. Existing root APIs remain
//! available. Unsupported document versions keep their original diagnostics;
//! there is no implicit upgrade. Breaking this surface needs a new facade major,
//! documented migration and compatibility interval before deprecation/removal.
//!
//! Unresolved coverage is an `Ok` domain result with planning blocked. A stale
//! review is `Ok` with `current=false`. Neither a record nor a bound handle grants
//! human approval, source verification, current authority or Host compatibility.
//! The portable [`ReviewRecordV1`] remains untrusted data when supplied again.
//!
//! Execution/approval/recovery is unavailable, including under all features:
//!
//! ```compile_fail
//! use storytelling_engine::consumer_v1::execute;
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::approve;
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::dispatch;
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::retry;
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::recover;
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::write_journal;
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::run_shell;
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v2;
//! ```
//!
//! Prepared reviews cannot be reconstructed from JSON or mutable storage:
//!
//! ```compile_fail
//! use storytelling_engine::consumer_v1::PreparedStoryReviewV1;
//! fn decode<T: serde::de::DeserializeOwned>() {}
//! decode::<PreparedStoryReviewV1>();
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::PreparedStoryReviewV1;
//! let forged = PreparedStoryReviewV1 { review: todo!(), bundle: todo!() };
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::PreparedStoryReviewV1;
//! fn mutate(review: &mut PreparedStoryReviewV1) {
//!     review.compiled_otio()[0] = 0;
//! }
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::PreparedStoryReviewV1;
//! fn mutate(review: &mut PreparedStoryReviewV1) {
//!     review.record().executable = true;
//! }
//! ```
//!
//! SDK errors are opaque Rust values, not new JSON wire contracts:
//!
//! ```compile_fail
//! use storytelling_engine::consumer_v1::SdkErrorV1;
//! let forged = SdkErrorV1 { operation: "video_coverage", source: todo!() };
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::SdkErrorV1;
//! fn decode<T: serde::de::DeserializeOwned>() {}
//! decode::<SdkErrorV1>();
//! ```
//! ```compile_fail
//! use storytelling_engine::consumer_v1::SdkEncodingErrorV1;
//! let forged = SdkEncodingErrorV1 { _private: () };
//! ```

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
