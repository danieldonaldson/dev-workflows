//! Test helper utilities for creating domain objects with valid test data.
//!
//! These helpers are only available in test builds and provide convenient
//! ways to create domain objects with sensible defaults.

use chrono::{DateTime, Duration, Utc};

/// Creates a date at UTC midnight for the specified number of days ago.
///
/// This is useful for tests that need dates at UTC midnight (00:00:00Z)
/// to pass the `validate_utc_midnight` validation. Using `Utc::now() - Duration::hours(1)`
/// creates times that aren't at midnight and will fail validation.
///
/// # Arguments
///
/// * `days` - Number of days in the past. Use 0 for today at midnight,
///   1 for yesterday at midnight, etc.
///
/// # Examples
///
/// ```
/// use app::domain::test_helpers::days_ago_at_midnight;
///
/// let yesterday = days_ago_at_midnight(1);
/// assert_eq!(yesterday.time().hour(), 0);
/// assert_eq!(yesterday.time().minute(), 0);
/// assert_eq!(yesterday.time().second(), 0);
/// ```
///
/// # Panics
/// Panics if midnight (00:00:00) is not a valid time, which should never happen.
#[must_use]
pub fn days_ago_at_midnight(days: i64) -> DateTime<Utc> {
    let today = Utc::now().date_naive();
    let target = today - Duration::days(days);
    target
        .and_hms_opt(0, 0, 0)
        .expect("midnight is always valid")
        .and_utc()
}

/// Creates today's date at UTC midnight.
///
/// Equivalent to `days_ago_at_midnight(0)`.
#[must_use]
pub fn today_at_midnight() -> DateTime<Utc> {
    days_ago_at_midnight(0)
}

/// Creates yesterday's date at UTC midnight.
///
/// Equivalent to `days_ago_at_midnight(1)`.
#[must_use]
pub fn yesterday_at_midnight() -> DateTime<Utc> {
    days_ago_at_midnight(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;

    #[test]
    fn test_days_ago_at_midnight_is_at_midnight() {
        let date = days_ago_at_midnight(5);
        assert_eq!(date.hour(), 0);
        assert_eq!(date.minute(), 0);
        assert_eq!(date.second(), 0);
        assert_eq!(date.nanosecond(), 0);
    }

    #[test]
    fn test_days_ago_at_midnight_correct_days() {
        let today = Utc::now().date_naive();
        let five_days_ago = days_ago_at_midnight(5);
        let expected = today - Duration::days(5);
        assert_eq!(five_days_ago.date_naive(), expected);
    }

    #[test]
    fn test_today_at_midnight() {
        let today = Utc::now().date_naive();
        let result = today_at_midnight();
        assert_eq!(result.date_naive(), today);
        assert_eq!(result.hour(), 0);
    }

    #[test]
    fn test_yesterday_at_midnight() {
        let today = Utc::now().date_naive();
        let yesterday = today - Duration::days(1);
        let result = yesterday_at_midnight();
        assert_eq!(result.date_naive(), yesterday);
        assert_eq!(result.hour(), 0);
    }
}
