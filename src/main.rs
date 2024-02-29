fn main() {
    let connection = sqlite::open("../hamster_test.db").unwrap();
    let query = "
        select start_time, name
        from facts
        left join activities
        on activities.id=facts.activity_id
        order by facts.id desc
        limit 5;
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
