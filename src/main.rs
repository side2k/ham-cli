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
        cli::Commands::Tasks {} => print_tasks(),
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

fn print_tasks() {
    let hamster_data = hamster::HamsterData::open().unwrap();
    let now = Local::now();
    let today = now
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(now.timezone())
        .unwrap();
    let facts = hamster_data.get_facts(today);
    let mut table = Table::new();
    table.set_header(vec!["start time", "end_time", "name"]);
    for record in facts {
        let end_time_str = match record.end_time {
            None => String::from("---"),
            Some(end_time) => end_time.to_rfc3339(),
        };
        table.add_row(vec![
            record.start_time.to_rfc3339(),
            end_time_str,
            record.name,
        ]);
    }
    println!("{table}");
}
