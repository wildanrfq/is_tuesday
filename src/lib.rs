use chrono::Datelike;

/// The function to check if the current day is Tuesday.
pub fn check() -> bool {
    return matches!(chrono::offset::Local::now().date_naive().weekday(), chrono::Weekday::Tue);
}