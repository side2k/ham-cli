use chrono::{DateTime, Local, NaiveDateTime};
use sqlite::State;

pub struct FactRecord {
    pub name: String,
    pub start_time: DateTime<Local>,
    pub end_time: Option<DateTime<Local>>,
}

pub struct HamsterData {
    connection: sqlite::Connection,
}

impl HamsterData {
    pub fn open() -> Result<HamsterData, String> {
        match sqlite::open("../hamster_test.db") {
            Ok(connection) => Ok(HamsterData { connection }),
            Err(hamster_error) => Err(format!("couldn't open hamster db: {}", hamster_error)),
        }
    }

    pub fn get_facts(&self, since: DateTime<Local>) -> Vec<FactRecord> {
        let mut statement = self
            .connection
            .prepare(
                "
                SELECT start_time, end_time, name, description
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

                start_time: {
                    NaiveDateTime::parse_from_str(
                        statement.read::<String, _>("start_time").unwrap().as_str(),
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .unwrap()
                    .and_local_timezone(local_tz)
                    .unwrap()
                },

                end_time: {
                    let end_time_raw = statement.read::<Option<String>, _>("end_time").unwrap();

                    match end_time_raw {
                        Some(end_time_str) => Some(
                            NaiveDateTime::parse_from_str(
                                end_time_str.as_str(),
                                "%Y-%m-%d %H:%M:%S",
                            )
                            .unwrap()
                            .and_local_timezone(local_tz)
                            .unwrap(),
                        ),
                        None => None,
                    }
                },
            });
        }
        data
    }
}
