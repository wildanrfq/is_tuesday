use chrono::Datelike;

pub fn check() -> bool {
    return matches!(chrono::offset::Local::now().date_naive().weekday(), chrono::Weekday::Tue);
}