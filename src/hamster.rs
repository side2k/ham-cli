use chrono::{DateTime, Local, NaiveDateTime};
use sqlite::State;

pub struct HamsterFact {
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
    pub fn open() -> Result<HamsterData, String> {
        match sqlite::open("../hamster_test.db") {
            Ok(connection) => Ok(HamsterData { connection }),
            Err(hamster_error) => Err(format!("couldn't open hamster db: {}", hamster_error)),
        }
    }

    pub fn get_facts(&self, from: DateTime<Local>, to: DateTime<Local>) -> Vec<HamsterFact> {
        let mut statement = self
            .connection
            .prepare(
                "
                SELECT
                    start_time, end_time, activities.name as `activity_name`, description,
                    categories.name as `category_name`
                FROM facts
                LEFT JOIN activities
                    ON activities.id=facts.activity_id
                LEFT JOIN categories
                    ON categories.id=activities.category_id
                WHERE
                    start_time >= :start_time
                    AND end_time < :end_time
                ORDER BY facts.id;
                ",
            )
            .unwrap();

        statement
            .bind(
                &[
                    (":start_time", from.format("%Y-%m-%d").to_string().as_str()),
                    (":end_time", to.format("%Y-%m-%d").to_string().as_str()),
                ][..],
            )
            .unwrap();

        let local_tz = Local::now().timezone();
        let mut data: Vec<HamsterFact> = vec![];

        while let Ok(State::Row) = statement.next() {
            data.push(HamsterFact {
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
