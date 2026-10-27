//! Shared validation functions for domain entities.
//!
//! These functions provide reusable validation logic for common fields
//! across different domain entities like `RefuelEntry` and `MaintenanceEvent`.

use crate::domain::errors::DomainError;
use chrono::{DateTime, Utc};

/// Validates that an odometer reading is non-negative.
///
/// # Errors
///
/// Returns [`DomainError::Validation`] if the odometer reading is negative.
pub fn validate_odometer(value: i32) -> Result<(), DomainError> {
    if value < 0 {
        return Err(DomainError::Validation(
            "Odometer reading cannot be negative".into(),
        ));
    }
    Ok(())
}

/// Validates that a date is not in the future.
///
/// # Errors
///
/// Returns [`DomainError::Validation`] if the date is in the future.
pub fn validate_date_not_future(date: DateTime<Utc>, field_name: &str) -> Result<(), DomainError> {
    if date > Utc::now() {
        return Err(DomainError::Validation(format!(
            "{field_name} cannot be in the future"
        )));
    }
    Ok(())
}

/// Validates that a datetime is at UTC midnight (00:00:00.000).
///
/// The frontend should send dates as `YYYY-MM-DDT00:00:00Z` to avoid
/// timezone-related date shifts. This validation ensures the backend
/// receives dates in the expected format.
///
/// # Errors
///
/// Returns [`DomainError::Validation`] if the time component is not midnight.
pub fn validate_utc_midnight(date: DateTime<Utc>, field_name: &str) -> Result<(), DomainError> {
    use chrono::Timelike;

    if date.hour() != 0 || date.minute() != 0 || date.second() != 0 || date.nanosecond() != 0 {
        return Err(DomainError::Validation(format!(
            "{field_name} must be at UTC midnight (00:00:00Z). Received time: {:02}:{:02}:{:02}",
            date.hour(),
            date.minute(),
            date.second()
        )));
    }
    Ok(())
}

/// Validates that a cost in cents is non-negative (if present).
///
/// # Errors
///
/// Returns [`DomainError::Validation`] if the cost is negative.
pub fn validate_cost_cents_non_negative(cost_cents: Option<i64>) -> Result<(), DomainError> {
    if let Some(c) = cost_cents
        && c < 0
    {
        return Err(DomainError::Validation("Cost cannot be negative".into()));
    }
    Ok(())
}

/// Validates that a required cost in cents is positive.
///
/// # Errors
///
/// Returns [`DomainError::Validation`] if the cost is zero or negative.
pub fn validate_cost_cents_positive(cost_cents: i64) -> Result<(), DomainError> {
    if cost_cents <= 0 {
        return Err(DomainError::Validation("Cost must be positive".into()));
    }
    Ok(())
}

/// Validates that a string is not empty/whitespace-only and within max length.
///
/// # Errors
///
/// Returns [`DomainError::Validation`] if the string is empty, whitespace-only, or too long.
pub fn validate_string_length(
    value: &str,
    field_name: &str,
    max_length: usize,
) -> Result<(), DomainError> {
    if value.is_empty() || value.trim().is_empty() {
        return Err(DomainError::Validation(format!(
            "{field_name} cannot be empty"
        )));
    }
    if value.len() > max_length {
        return Err(DomainError::Validation(format!(
            "{field_name} cannot exceed {max_length} characters"
        )));
    }
    Ok(())
}

/// Validates that an optional string is within max length (if present).
///
/// # Errors
///
/// Returns [`DomainError::Validation`] if the string exceeds max length.
pub fn validate_optional_string_length(
    value: &Option<String>,
    field_name: &str,
    max_length: usize,
) -> Result<(), DomainError> {
    if let Some(s) = value
        && s.len() > max_length
    {
        return Err(DomainError::Validation(format!(
            "{field_name} cannot exceed {max_length} characters"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use proptest::prelude::*;

    #[test]
    fn test_validate_odometer_valid() {
        assert!(validate_odometer(0).is_ok());
        assert!(validate_odometer(100_000).is_ok());
    }

    #[test]
    fn test_validate_odometer_negative() {
        let result = validate_odometer(-1);
        assert!(result.is_err());
        match result {
            Err(DomainError::Validation(msg)) => {
                assert!(msg.contains("Odometer reading cannot be negative"));
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn test_validate_date_not_future_valid() {
        let past = Utc::now() - Duration::hours(1);
        assert!(validate_date_not_future(past, "Event date").is_ok());

        let now = Utc::now();
        assert!(validate_date_not_future(now, "Event date").is_ok());
    }

    #[test]
    fn test_validate_date_not_future_invalid() {
        let future = Utc::now() + Duration::hours(1);
        let result = validate_date_not_future(future, "Event date");
        assert!(result.is_err());
        match result {
            Err(DomainError::Validation(msg)) => {
                assert!(msg.contains("Event date cannot be in the future"));
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn test_validate_utc_midnight_valid() {
        use chrono::NaiveDate;
        // Create a date at exactly midnight UTC
        let midnight = NaiveDate::from_ymd_opt(2024, 1, 15)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc();
        assert!(validate_utc_midnight(midnight, "Refuel date").is_ok());
    }

    #[test]
    fn test_validate_utc_midnight_with_hours() {
        use chrono::NaiveDate;
        let not_midnight = NaiveDate::from_ymd_opt(2024, 1, 15)
            .unwrap()
            .and_hms_opt(14, 0, 0)
            .unwrap()
            .and_utc();
        let result = validate_utc_midnight(not_midnight, "Refuel date");
        assert!(result.is_err());
        match result {
            Err(DomainError::Validation(msg)) => {
                assert!(msg.contains("must be at UTC midnight"));
                assert!(msg.contains("14:00:00"));
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn test_validate_utc_midnight_with_minutes() {
        use chrono::NaiveDate;
        let not_midnight = NaiveDate::from_ymd_opt(2024, 1, 15)
            .unwrap()
            .and_hms_opt(0, 30, 0)
            .unwrap()
            .and_utc();
        let result = validate_utc_midnight(not_midnight, "Event date");
        assert!(result.is_err());
        match result {
            Err(DomainError::Validation(msg)) => {
                assert!(msg.contains("must be at UTC midnight"));
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn test_validate_utc_midnight_with_seconds() {
        use chrono::NaiveDate;
        let not_midnight = NaiveDate::from_ymd_opt(2024, 1, 15)
            .unwrap()
            .and_hms_opt(0, 0, 45)
            .unwrap()
            .and_utc();
        let result = validate_utc_midnight(not_midnight, "Event date");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_utc_midnight_with_nanoseconds() {
        use chrono::NaiveDate;
        let not_midnight = NaiveDate::from_ymd_opt(2024, 1, 15)
            .unwrap()
            .and_hms_nano_opt(0, 0, 0, 123_456_789)
            .unwrap()
            .and_utc();
        let result = validate_utc_midnight(not_midnight, "Event date");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_cost_cents_non_negative_valid() {
        assert!(validate_cost_cents_non_negative(None).is_ok());
        assert!(validate_cost_cents_non_negative(Some(0)).is_ok());
        assert!(validate_cost_cents_non_negative(Some(10000)).is_ok()); // R100.00
    }

    #[test]
    fn test_validate_cost_cents_non_negative_invalid() {
        let result = validate_cost_cents_non_negative(Some(-100));
        assert!(result.is_err());
        match result {
            Err(DomainError::Validation(msg)) => {
                assert!(msg.contains("Cost cannot be negative"));
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn test_validate_cost_cents_positive_valid() {
        assert!(validate_cost_cents_positive(1).is_ok());
        assert!(validate_cost_cents_positive(10000).is_ok()); // R100.00
    }

    #[test]
    fn test_validate_cost_cents_positive_invalid() {
        assert!(validate_cost_cents_positive(0).is_err());
        assert!(validate_cost_cents_positive(-100).is_err());
    }

    #[test]
    fn test_validate_string_length_valid() {
        assert!(validate_string_length("hello", "Name", 100).is_ok());
    }

    #[test]
    fn test_validate_string_length_empty() {
        let result = validate_string_length("", "Name", 100);
        assert!(result.is_err());
        match result {
            Err(DomainError::Validation(msg)) => {
                assert!(msg.contains("Name cannot be empty"));
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn test_validate_string_length_whitespace_only() {
        let result = validate_string_length("   ", "Name", 100);
        assert!(result.is_err());
        match result {
            Err(DomainError::Validation(msg)) => {
                assert!(msg.contains("Name cannot be empty"));
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn test_validate_string_length_too_long() {
        let long_string = "a".repeat(101);
        let result = validate_string_length(&long_string, "Name", 100);
        assert!(result.is_err());
        match result {
            Err(DomainError::Validation(msg)) => {
                assert!(msg.contains("Name cannot exceed 100 characters"));
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn test_validate_optional_string_length_valid() {
        assert!(validate_optional_string_length(&None, "Notes", 100).is_ok());
        assert!(validate_optional_string_length(&Some("hello".into()), "Notes", 100).is_ok());
    }

    #[test]
    fn test_validate_optional_string_length_too_long() {
        let long_string = Some("a".repeat(101));
        let result = validate_optional_string_length(&long_string, "Notes", 100);
        assert!(result.is_err());
    }

    // ==================== Property-Based Tests ====================

    proptest! {
        // Odometer validation
        #[test]
        fn prop_non_negative_odometer_valid(value in 0..=i32::MAX) {
            prop_assert!(validate_odometer(value).is_ok());
        }

        #[test]
        fn prop_negative_odometer_invalid(value in i32::MIN..0i32) {
            prop_assert!(validate_odometer(value).is_err());
        }

        // Cost non-negative validation (cents)
        #[test]
        fn prop_non_negative_cost_cents_valid(cents in 0i64..=i64::MAX) {
            prop_assert!(validate_cost_cents_non_negative(Some(cents)).is_ok());
        }

        #[test]
        fn prop_negative_cost_cents_invalid(cents in 1i64..=1_000_000_000i64) {
            prop_assert!(validate_cost_cents_non_negative(Some(-cents)).is_err());
        }

        #[test]
        fn prop_none_cost_cents_always_valid(_seed in 0..100u32) {
            prop_assert!(validate_cost_cents_non_negative(None).is_ok());
        }

        // Cost positive validation (cents)
        #[test]
        fn prop_positive_cost_cents_valid(cents in 1i64..=i64::MAX) {
            prop_assert!(validate_cost_cents_positive(cents).is_ok());
        }

        #[test]
        fn prop_zero_cost_cents_invalid(_seed in 0..100u32) {
            prop_assert!(validate_cost_cents_positive(0).is_err());
        }

        #[test]
        fn prop_negative_cost_cents_positive_invalid(cents in 1i64..=1_000_000_000i64) {
            prop_assert!(validate_cost_cents_positive(-cents).is_err());
        }

        // String length validation - valid lengths
        #[test]
        fn prop_string_within_max_valid(
            len in 1usize..=100usize,
            max_len in 100usize..=1000usize,
        ) {
            let s: String = "x".repeat(len);
            prop_assert!(validate_string_length(&s, "Field", max_len).is_ok());
        }

        // String length validation - exact boundary
        #[test]
        fn prop_string_at_exact_max_valid(max_len in 1usize..=500usize) {
            let s: String = "x".repeat(max_len);
            prop_assert!(validate_string_length(&s, "Field", max_len).is_ok());
        }

        // String length validation - exceeds max
        #[test]
        fn prop_string_exceeding_max_invalid(
            max_len in 1usize..=500usize,
            extra in 1usize..=100usize,
        ) {
            let s: String = "x".repeat(max_len + extra);
            prop_assert!(validate_string_length(&s, "Field", max_len).is_err());
        }

        // String length validation - whitespace handling
        #[test]
        fn prop_whitespace_only_invalid(spaces in 1usize..=50usize) {
            let s: String = " ".repeat(spaces);
            prop_assert!(validate_string_length(&s, "Field", 100).is_err());
        }

        // String with leading/trailing whitespace but content should be valid
        #[test]
        fn prop_string_with_whitespace_padding_valid(
            content_len in 1usize..=50usize,
            leading in 0usize..=10usize,
            trailing in 0usize..=10usize,
        ) {
            let content: String = "x".repeat(content_len);
            let s = format!("{}{}{}", " ".repeat(leading), content, " ".repeat(trailing));
            // Max length should account for total string length including whitespace
            let max_len = s.len();
            prop_assert!(validate_string_length(&s, "Field", max_len).is_ok());
        }

        // Optional string length validation - None always valid
        #[test]
        fn prop_optional_none_always_valid(max_len in 1usize..=1000usize) {
            prop_assert!(validate_optional_string_length(&None, "Field", max_len).is_ok());
        }

        // Optional string length validation - Some within bounds
        #[test]
        fn prop_optional_within_max_valid(
            len in 1usize..=100usize,
            max_len in 100usize..=1000usize,
        ) {
            let s = Some("x".repeat(len));
            prop_assert!(validate_optional_string_length(&s, "Field", max_len).is_ok());
        }

        // Optional string length validation - Some exceeds max
        #[test]
        fn prop_optional_exceeding_max_invalid(
            max_len in 1usize..=500usize,
            extra in 1usize..=100usize,
        ) {
            let s = Some("x".repeat(max_len + extra));
            prop_assert!(validate_optional_string_length(&s, "Field", max_len).is_err());
        }

        // Optional string length - exact boundary
        #[test]
        fn prop_optional_at_exact_max_valid(max_len in 1usize..=500usize) {
            let s = Some("x".repeat(max_len));
            prop_assert!(validate_optional_string_length(&s, "Field", max_len).is_ok());
        }

        // Date validation - past dates always valid
        #[test]
        fn prop_past_date_valid(hours_ago in 1i64..=8760i64) {
            let past = Utc::now() - Duration::hours(hours_ago);
            prop_assert!(validate_date_not_future(past, "Date").is_ok());
        }

        // Date validation - future dates always invalid
        #[test]
        fn prop_future_date_invalid(hours_ahead in 1i64..=8760i64) {
            let future = Utc::now() + Duration::hours(hours_ahead);
            prop_assert!(validate_date_not_future(future, "Date").is_err());
        }

        // Integer cost cents validation with various values
        #[test]
        fn prop_cost_cents_various_values_valid(cents in 0i64..=1_000_000_000_000i64) {
            prop_assert!(validate_cost_cents_non_negative(Some(cents)).is_ok());
        }

        // UTC midnight validation - midnight is always valid
        #[test]
        fn prop_utc_midnight_valid(
            year in 2000i32..=2100i32,
            month in 1u32..=12u32,
            day in 1u32..=28u32,
        ) {
            use chrono::NaiveDate;
            let date = NaiveDate::from_ymd_opt(year, month, day)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc();
            prop_assert!(validate_utc_midnight(date, "Date").is_ok());
        }

        // UTC midnight validation - non-zero hours always invalid
        #[test]
        fn prop_utc_midnight_nonzero_hour_invalid(
            year in 2000i32..=2100i32,
            month in 1u32..=12u32,
            day in 1u32..=28u32,
            hour in 1u32..=23u32,
        ) {
            use chrono::NaiveDate;
            let date = NaiveDate::from_ymd_opt(year, month, day)
                .unwrap()
                .and_hms_opt(hour, 0, 0)
                .unwrap()
                .and_utc();
            prop_assert!(validate_utc_midnight(date, "Date").is_err());
        }

        // UTC midnight validation - non-zero minutes always invalid
        #[test]
        fn prop_utc_midnight_nonzero_minute_invalid(
            year in 2000i32..=2100i32,
            month in 1u32..=12u32,
            day in 1u32..=28u32,
            minute in 1u32..=59u32,
        ) {
            use chrono::NaiveDate;
            let date = NaiveDate::from_ymd_opt(year, month, day)
                .unwrap()
                .and_hms_opt(0, minute, 0)
                .unwrap()
                .and_utc();
            prop_assert!(validate_utc_midnight(date, "Date").is_err());
        }

        // UTC midnight validation - non-zero seconds always invalid
        #[test]
        fn prop_utc_midnight_nonzero_second_invalid(
            year in 2000i32..=2100i32,
            month in 1u32..=12u32,
            day in 1u32..=28u32,
            second in 1u32..=59u32,
        ) {
            use chrono::NaiveDate;
            let date = NaiveDate::from_ymd_opt(year, month, day)
                .unwrap()
                .and_hms_opt(0, 0, second)
                .unwrap()
                .and_utc();
            prop_assert!(validate_utc_midnight(date, "Date").is_err());
        }

        // UTC midnight validation - non-zero nanoseconds always invalid
        #[test]
        fn prop_utc_midnight_nonzero_nanosecond_invalid(
            year in 2000i32..=2100i32,
            month in 1u32..=12u32,
            day in 1u32..=28u32,
            nanos in 1u32..=999_999_999u32,
        ) {
            use chrono::NaiveDate;
            let date = NaiveDate::from_ymd_opt(year, month, day)
                .unwrap()
                .and_hms_nano_opt(0, 0, 0, nanos)
                .unwrap()
                .and_utc();
            prop_assert!(validate_utc_midnight(date, "Date").is_err());
        }

        // UTC midnight validation - any non-midnight combination is invalid
        #[test]
        fn prop_utc_any_non_midnight_invalid(
            year in 2000i32..=2100i32,
            month in 1u32..=12u32,
            day in 1u32..=28u32,
            hour in 0u32..=23u32,
            minute in 0u32..=59u32,
            second in 0u32..=59u32,
        ) {
            use chrono::NaiveDate;
            // Skip midnight case (0, 0, 0)
            prop_assume!(hour != 0 || minute != 0 || second != 0);

            let date = NaiveDate::from_ymd_opt(year, month, day)
                .unwrap()
                .and_hms_opt(hour, minute, second)
                .unwrap()
                .and_utc();
            prop_assert!(validate_utc_midnight(date, "Date").is_err());
        }

        // UTC midnight validation - positive timezone offset that converts to midnight
        // E.g., 02:00:00+02:00 = 00:00:00Z (midnight UTC)
        #[test]
        fn prop_timezone_offset_positive_to_midnight_valid(
            year in 2000i32..=2100i32,
            month in 1u32..=12u32,
            day in 1u32..=28u32,
            offset_hours in 1i32..=12i32,
        ) {
            use chrono::{FixedOffset, NaiveDate, TimeZone};

            // Create a date with local time = offset hours at midnight UTC
            // E.g., if offset is +2, then 02:00 local = 00:00 UTC
            let offset_secs = offset_hours * 3600;
            let tz = FixedOffset::east_opt(offset_secs).unwrap();

            #[allow(clippy::cast_sign_loss)]
            let local_time = NaiveDate::from_ymd_opt(year, month, day)
                .unwrap()
                .and_hms_opt(offset_hours as u32, 0, 0)
                .unwrap();

            let datetime = tz.from_local_datetime(&local_time).unwrap();
            let utc_datetime = datetime.with_timezone(&Utc);

            prop_assert!(validate_utc_midnight(utc_datetime, "Date").is_ok());
        }

        // UTC midnight validation - negative timezone offset that converts to midnight
        // E.g., 19:00:00-05:00 (on day before) = 00:00:00Z (midnight UTC)
        #[test]
        fn prop_timezone_offset_negative_to_midnight_valid(
            year in 2001i32..=2100i32,  // Start from 2001 to allow day subtraction
            month in 1u32..=12u32,
            day in 2u32..=28u32,  // Start from day 2 to avoid month boundary issues
            offset_hours in 1i32..=12i32,
        ) {
            use chrono::{FixedOffset, NaiveDate, TimeZone};

            // Create a date with local time = (24 - offset) hours on day before
            // E.g., if offset is -5, then 19:00 local on Jan 1 = 00:00 UTC on Jan 2
            let offset_secs = -offset_hours * 3600;
            let tz = FixedOffset::east_opt(offset_secs).unwrap();

            let local_hour = 24 - offset_hours;
            #[allow(clippy::cast_sign_loss)]
            let local_time = NaiveDate::from_ymd_opt(year, month, day - 1)
                .unwrap()
                .and_hms_opt(local_hour as u32, 0, 0)
                .unwrap();

            let datetime = tz.from_local_datetime(&local_time).unwrap();
            let utc_datetime = datetime.with_timezone(&Utc);

            prop_assert!(validate_utc_midnight(utc_datetime, "Date").is_ok());
        }

        // UTC midnight validation - positive offset NOT converting to midnight
        // E.g., 03:00:00+02:00 = 01:00:00Z (NOT midnight)
        #[test]
        fn prop_timezone_offset_positive_not_midnight_invalid(
            year in 2000i32..=2100i32,
            month in 1u32..=12u32,
            day in 1u32..=28u32,
            offset_hours in 1i32..=12i32,
            extra_hours in 1i32..=11i32,  // Add extra hours to offset
        ) {
            use chrono::{FixedOffset, NaiveDate, TimeZone, Timelike};

            let offset_secs = offset_hours * 3600;
            let tz = FixedOffset::east_opt(offset_secs).unwrap();

            // Local time = offset + extra, so UTC will be non-midnight
            let local_hour = (offset_hours + extra_hours) % 24;
            #[allow(clippy::cast_sign_loss)]
            let local_time = NaiveDate::from_ymd_opt(year, month, day)
                .unwrap()
                .and_hms_opt(local_hour as u32, 0, 0)
                .unwrap();

            if let Some(datetime) = tz.from_local_datetime(&local_time).single() {
                let utc_datetime = datetime.with_timezone(&Utc);
                // Only test if UTC time is not midnight
                #[allow(clippy::manual_range_contains)]
                if utc_datetime.hour() != 0 || utc_datetime.minute() != 0 || utc_datetime.second() != 0 {
                    prop_assert!(validate_utc_midnight(utc_datetime, "Date").is_err());
                }
            }
        }

        // UTC midnight validation - all common timezone offsets work correctly
        #[test]
        fn prop_common_timezone_offsets_midnight_valid(
            year in 2000i32..=2100i32,
            month in 1u32..=12u32,
            day in 2u32..=28u32,  // Avoid day 1 for negative offsets
            // Common timezone offsets in minutes: -720 to +840 (-12:00 to +14:00)
            offset_minutes in -720i32..=840i32,
        ) {
            use chrono::{FixedOffset, NaiveDate};

            let tz = FixedOffset::east_opt(offset_minutes * 60).unwrap();

            // Calculate local time that corresponds to UTC midnight
            // If offset is +120 min (+2 hours), local midnight UTC is 02:00 local
            // If offset is -300 min (-5 hours), local midnight UTC is 19:00 local day before
            let utc_midnight = NaiveDate::from_ymd_opt(year, month, day)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc();

            // Convert UTC midnight to local time, then back - should still be midnight UTC
            let local_datetime = utc_midnight.with_timezone(&tz);
            let back_to_utc = local_datetime.with_timezone(&Utc);

            prop_assert!(validate_utc_midnight(back_to_utc, "Date").is_ok());
        }
    }
}
