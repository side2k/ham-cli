use chrono::{DateTime, Datelike, Days, Local};

pub fn week_start(dt: DateTime<Local>) -> DateTime<Local> {
    dt.checked_sub_days(Days::new(dt.weekday().num_days_from_monday() as u64))
        .unwrap()
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(dt.timezone())
        .unwrap()
}
