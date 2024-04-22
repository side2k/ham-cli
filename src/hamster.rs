use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
use sqlite::State;
use std::path::Path;

pub struct HamsterFact {
    pub id: i64,
    pub start_time: DateTime<Local>,
    pub end_time: Option<DateTime<Local>>,
    pub description: String,
    pub activity: String,
    pub category: String,
}

pub struct HamsterData {
    connection: sqlite::Connection,
}

impl HamsterData {
    pub fn open(db_path: Option<String>) -> Result<HamsterData, String> {
        let db_path: String = match db_path {
            Some(db_path) => db_path,
            None => match std::env::var("HOME") {
                Ok(home) => String::from(
                    Path::new(home.as_str())
                        .join(".local/share/hamster/hamster.db")
                        .to_str()
                        .unwrap(),
                ),
                Err(_) => {
                    return Err(String::from(
                        "Hamster database path wasn't supplied, $HOME is not set - I give up",
                    ))
                }
            },
        };
        match sqlite::open(db_path) {
            Ok(connection) => Ok(HamsterData { connection }),
            Err(hamster_error) => Err(format!("couldn't open hamster db: {}", hamster_error)),
        }
    }

    pub fn get_facts(&self, from: NaiveDate, to: NaiveDate) -> Vec<HamsterFact> {
        let mut statement = self
            .connection
            .prepare(
                "
                SELECT
                    facts.id as `fact_id`,
                    activities.name as `activity_name`,
                    categories.name as `category_name`,
                    start_time,
                    end_time,
                    description
                FROM facts
                LEFT JOIN activities
                    ON activities.id=facts.activity_id
                LEFT JOIN categories
                    ON categories.id=activities.category_id
                WHERE
                    start_time >= :from
                    AND start_time < :to
                ORDER BY facts.id;
                ",
            )
            .unwrap();

        statement
            .bind(
                &[
                    (":from", from.to_string().as_str()),
                    (":to", to.to_string().as_str()),
                ][..],
            )
            .unwrap();

        let local_tz = Local::now().timezone();
        let mut data: Vec<HamsterFact> = vec![];

        while let Ok(State::Row) = statement.next() {
            data.push(HamsterFact {
                id: statement.read::<i64, _>("fact_id").unwrap(),
                description: statement.read::<String, _>("description").unwrap(),
                category: statement.read::<String, _>("category_name").unwrap(),
                activity: statement.read::<String, _>("activity_name").unwrap(),

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
                    let end_time_raw = statement.read::<Option<String>, _>("end_time");

                    end_time_raw.unwrap().map(|end_time_str| {
                        NaiveDateTime::parse_from_str(end_time_str.as_str(), "%Y-%m-%d %H:%M:%S")
                            .unwrap()
                            .and_local_timezone(local_tz)
                            .unwrap()
                    })
                },
            });
        }
        data
    }
}
