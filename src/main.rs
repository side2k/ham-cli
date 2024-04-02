use chrono::{DateTime, Days, Local};
use clap::Parser;
use comfy_table::Table;

use crate::utils::DurationFormatting;
mod cli;
mod hamster;
mod utils;

fn main() {
    let cli_args = cli::Cli::parse();
    match cli_args.command {
        cli::Commands::GetFacts {} => print_last_week_facts(),
        cli::Commands::Tasks { days } => print_tasks(days),
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

fn print_tasks(days: u32) {
    let hamster_data = hamster::HamsterData::open().unwrap();
    let now = Local::now();
    let today = now
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(now.timezone())
        .unwrap();
    let from = today.checked_sub_days(Days::new(days as u64)).unwrap();
    let facts = hamster_data.get_facts(from);
    let mut table = Table::new();
    table.set_header(vec!["start time", "end_time", "duration", "name"]);
    for record in facts {
        let end_time: DateTime<Local>;

        let end_time_display = match record.end_time {
            None => {
                end_time = Local::now();
                String::from("---")
            }
            Some(end_time_db) => {
                end_time = end_time_db;
                end_time_db.to_rfc3339()
            }
        };

        let duration = (end_time - record.start_time).to_std().unwrap();

        table.add_row(vec![
            record.start_time.to_rfc3339(),
            end_time_display,
            duration.as_hhmm(),
            record.name,
        ]);
    }
    println!("{table}");
}
