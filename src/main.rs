fn main() {
    let connection = sqlite::open("../hamster_test.db").unwrap();
    let query = "
        SELECT start_time, name, description
        FROM facts
        LEFT JOIN activities
        ON activities.id=facts.activity_id
        ORDER BY facts.id DESC
        LIMIT 5;
    ";
    connection
        .iterate(query, |pairs| {
            for &(start_date, name) in pairs.iter() {
                println!("{} = {}", start_date, name.unwrap());
            }
            true
        })
        .unwrap();
}
