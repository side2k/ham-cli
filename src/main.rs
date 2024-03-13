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

struct FactRecord {
    name: String,
    start_time: DateTime<Local>,
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

    let local_tz = Local::now().timezone();
    let headers: Vec<String> = statement.column_names().to_vec();
    let mut data: Vec<FactRecord> = vec![];

    while let Ok(State::Row) = statement.next() {
        data.push(FactRecord {
            name: statement.read::<String, _>("name").unwrap(),
            start_time: NaiveDateTime::parse_from_str(
                statement.read::<String, _>("start_time").unwrap().as_str(),
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap()
            .and_local_timezone(local_tz)
            .unwrap(),
        });
    }

    let mut table = Table::new();
    table.set_header(headers);
    for record in data {
        table.add_row(vec![record.start_time.to_rfc3339(), record.name]);
    }
    println!("{table}");
}
