use chrono::{DateTime, Datelike, Days, Local, NaiveDateTime};
use comfy_table::Table;
use sqlite::State;

fn week_start(dt: DateTime<Local>) -> DateTime<Local> {
    dt.checked_sub_days(Days::new(dt.weekday().num_days_from_monday() as u64))
        .unwrap()
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(dt.timezone())
        .unwrap()
}

fn main() {
    let connection = sqlite::open("../hamster_test.db").unwrap();
    let mut statement = connection
        .prepare(
            "
            SELECT start_time, name, description
            FROM facts
            LEFT JOIN activities
            ON activities.id=facts.activity_id
            WHERE start_time > :start_time
            ORDER BY facts.id;
            ",
        )
        .unwrap();
    let week_start_str = week_start(Local::now()).format("%Y-%m-%d").to_string();

    statement
        .bind((":start_time", week_start_str.as_str()))
        .unwrap();

    let mut table = Table::new();
    let local_tz = Local::now().timezone();

    table.set_header(vec!["start_time", "name"]);

    while let Ok(State::Row) = statement.next() {
        let fact_name = statement.read::<String, _>("name").unwrap();
        let start_time_raw = statement.read::<String, _>("start_time").unwrap();
        let fact_dt = NaiveDateTime::parse_from_str(&start_time_raw, "%Y-%m-%d %H:%M:%S")
            .unwrap()
            .and_local_timezone(local_tz)
            .unwrap()
            .to_rfc3339();
        table.add_row(vec![fact_dt, fact_name]);
    }

    println!("{table}");
}
