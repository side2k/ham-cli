use std::collections::HashMap;

use chrono::{DateTime, Days, Local};
use clap::Parser;
use comfy_table::Table;

use crate::{enrichment::HamsterEnrichedData, utils::DurationFormatting};
mod cli;
mod enrichment;
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
    table.set_header(["start time", "end_time", "duration", "name"]);
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

        table.add_row([
            record.start_time.to_rfc3339(),
            end_time_display,
            duration.as_hhmm(),
            record.name,
        ]);
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

    let mut tasks = HashMap::new();

    for record in facts {
        let end_time = record.end_time.unwrap_or(Local::now());
        let duration = (end_time - record.start_time).to_std().unwrap();

        let task_key = record
            .task()
            .map_or(String::from("-"), |task_link| task_link.link_title);

        tasks
            .entry(task_key)
            .and_modify(|task_duration| *task_duration += duration)
            .or_insert(duration);
    }

    let mut table = Table::new();
    table.set_header(["duration", "task"]);
    for (task_title, duration) in tasks.into_iter() {
        table.add_row([task_title, duration.as_hhmm()]);
    }
    println!("{table}");
}
