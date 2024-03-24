use chrono::Local;
use clap::Parser;
use comfy_table::Table;
mod cli;
mod hamster;
mod utils;

fn main() {
    let cli_args = cli::Cli::parse();
    match cli_args.command {
        cli::Commands::GetFacts {} => print_last_week_facts(),
        _ => {
            println!("This command is not implemented yet")
        }
    }
}

fn print_last_week_facts() {
    let hamster_data = hamster::HamsterData::open().unwrap();
    let facts = hamster_data.get_facts(utils::week_start(Local::now()));
    let mut table = Table::new();
    table.set_header(vec!["start time", "name"]);
    for record in facts {
        table.add_row(vec![record.start_time.to_rfc3339(), record.name]);
    }
    println!("{table}");
}
