use chrono::{DateTime, Local, NaiveDateTime};
use sqlite::State;

pub struct FactRecord {
    pub name: String,
    pub start_time: DateTime<Local>,
}

pub fn get_facts(since: DateTime<Local>) -> Vec<FactRecord> {
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

    statement
        .bind((":start_time", since.format("%Y-%m-%d").to_string().as_str()))
        .unwrap();

    let local_tz = Local::now().timezone();
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
    data
}
