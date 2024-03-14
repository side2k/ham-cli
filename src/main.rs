use chrono::Local;
use comfy_table::Table;
mod hamster;
mod utils;

fn main() {
    let facts = hamster::get_facts(utils::week_start(Local::now()));
    let mut table = Table::new();
    table.set_header(vec!["start time", "name"]);
    for record in facts {
        table.add_row(vec![record.start_time.to_rfc3339(), record.name]);
    }
    println!("{table}");
}
