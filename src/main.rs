use std::collections::HashMap;

use chrono::{DateTime, Days, Local, NaiveDate};
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
        cli::Commands::Tasks { from, to } => print_tasks(from, to),
        _ => {
            println!("This command is not implemented yet")
        }
    }
}

fn print_last_week_facts() {
    let hamster_data = hamster::HamsterData::open().unwrap();
    let week_start = utils::week_start(Local::now());
    let week_end = week_start.checked_add_days(Days::new(7)).unwrap();
    let facts = hamster_data.get_facts(week_start, week_end);
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

fn print_tasks(from: Option<NaiveDate>, to: Option<NaiveDate>) {
    let hamster_data = hamster::HamsterData::open().unwrap();
    let now = Local::now();
    let timezone = now.timezone();

    let from = match from {
        Some(from) => from,
        None => now.date_naive(),
    };
    let to = match to {
        Some(to) => to,
        None => now.checked_add_days(Days::new(1)).unwrap().date_naive(),
    };
    let facts = hamster_data.get_facts(
        from.and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(timezone)
            .unwrap(),
        to.and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(timezone)
            .unwrap(),
    );

    let mut tasks = HashMap::new();

    for record in facts {
        let end_time = record.end_time.unwrap_or(Local::now());
        let duration = (end_time - record.start_time).to_std().unwrap();

        let task_id = match record.task() {
            None => String::from("-"),
            Some(task_link) => task_link.task_id,
        };
        let title = match record.task() {
            None => String::from("-"),
            Some(task_link) => task_link.link_title,
        };

        tasks
            .entry(task_id)
            .and_modify(|(_, task_duration)| *task_duration += duration)
            .or_insert((title, duration));
    }

    let mut table = Table::new();
    table.set_header(["Task ID", "name", "duration"]);
    for (task_id, (task_title, duration)) in tasks.into_iter() {
        table.add_row([task_id, task_title, duration.as_hhmm()]);
    }
    println!("{table}");
}
