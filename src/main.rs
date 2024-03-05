use comfy_table::Table;
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

    let mut table = Table::new();

    table.set_header(vec!["start_time", "name"]);

    let process_row = |pairs: &[(&str, Option<&str>)]| -> bool {
        let mut fact_dt = String::new();
        let mut fact_name = String::new();
        for &(name, value) in pairs.iter() {
            match name {
                "name" => fact_name = String::from(value.unwrap()),
                "start_time" => fact_dt = String::from(value.unwrap()),
                _ => {}
            }
        }
        table.add_row(vec![fact_dt, fact_name]);

        true
    };

    connection.iterate(query, process_row).unwrap();
    println!("{table}");
}
