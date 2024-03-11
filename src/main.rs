use chrono::{Local, NaiveDateTime};
use comfy_table::Table;
use sqlite::State;

fn main() {
    let connection = sqlite::open("../hamster_test.db").unwrap();
    let mut statement = connection
        .prepare(
            "
        SELECT start_time, name, description
        FROM facts
        LEFT JOIN activities
        ON activities.id=facts.activity_id
        ORDER BY facts.id DESC
        LIMIT 5;
    ",
        )
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
