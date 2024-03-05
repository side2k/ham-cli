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

    fn process_row(pairs: &[(&str, Option<&str>)]) -> bool {
        for &(name, value) in pairs.iter() {
            println!("{} = {}", name, value.unwrap());
        }
        true
    }

    connection.iterate(query, process_row).unwrap();
}
