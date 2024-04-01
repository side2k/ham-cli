use chrono::{DateTime, Datelike, Days, Local};
use std::time::Duration;

pub fn week_start(dt: DateTime<Local>) -> DateTime<Local> {
    dt.checked_sub_days(Days::new(dt.weekday().num_days_from_monday() as u64))
        .unwrap()
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(dt.timezone())
        .unwrap()
}

pub fn duration_str(duration: Duration) -> String {
    let minutes_total = duration.as_secs() / 60;
    let hours = minutes_total / 60;
    let minutes = minutes_total % 60;

    String::from(format!("{hh}:{mm:02}", hh = hours, mm = minutes))
}
