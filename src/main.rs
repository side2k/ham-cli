use std::collections::HashMap;

use chrono::{DateTime, Days, Local, NaiveDate};
use clap::Parser;
use comfy_table::Table;
use std::time::Duration;

use crate::{enrichment::HamsterEnrichedData, utils::DurationFormatting};
mod cli;
mod enrichment;
mod hamster;
mod utils;

use everhour_simple_client::client::Client as EverhourClient;
use everhour_simple_client::time_record::TimeRecord;

#[tokio::main]
async fn main() {
    let cli_args = cli::Cli::parse();
    match cli_args.command {
        cli::Commands::GetFacts {} => print_last_week_facts(cli_args.hamster_db),
        cli::Commands::Tasks { from, to, category } => {
            print_tasks(cli_args.hamster_db, from, to, category)
        }
        cli::Commands::SyncTasksToEverhour {
            api_token,
            from,
            to,
            category,
            dry_run,
        } => {
            let today = chrono::Local::now().date_naive();
            let from: NaiveDate = from.unwrap_or(today);
            let to: NaiveDate = to.unwrap_or(from.clone());
            sync_tasks_to_everhour(cli_args.hamster_db, api_token, from, to, category, dry_run)
                .await
        }
        _ => {
            println!("This command is not implemented yet")
        }
    }
}

fn print_last_week_facts(hamster_db: Option<String>) {
    let hamster_data = hamster::HamsterData::open(hamster_db).unwrap();
    let week_start = utils::week_start(Local::now()).date_naive();
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
            record.activity,
        ]);
    }
    println!("{table}");
}

type TasksWithDurations = HashMap<Option<String>, (Option<String>, Duration)>;

fn get_tasks_with_durations(
    hamster_db: Option<String>,
    from: NaiveDate,
    to: NaiveDate,
    category: Option<String>,
) -> TasksWithDurations {
    let hamster_data = hamster::HamsterData::open(hamster_db).unwrap();

    let facts = hamster_data.get_facts(from, to);

    let facts = match category {
        None => facts,
        Some(category) => facts
            .into_iter()
            .filter(|fact| fact.category == category)
            .collect(),
    };

    let mut tasks: TasksWithDurations = HashMap::new();

    for record in facts {
        let end_time = record.end_time.unwrap_or_else(|| Local::now());
        let duration = (end_time - record.start_time).to_std().unwrap();

        let task_id: Option<String>;
        let title: Option<String>;

        if let Some(task_link) = record.task() {
            task_id = Some(task_link.task_id);
            title = Some(task_link.link_title);
        } else {
            task_id = None;
            title = None;
            println!(
                "Error obtaining task id from fact {} ({}@{} - {})",
                record.id, record.activity, record.category, record.description
            )
        }

        tasks
            .entry(task_id)
            .and_modify(|(_, task_duration)| *task_duration += duration)
            .or_insert((title, duration));
    }
    tasks
}

fn print_tasks(
    hamster_db: Option<String>,
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
    category: Option<String>,
) {
    let now = Local::now();

    let from = match from {
        Some(from) => from,
        None => now.date_naive(),
    };
    let to = match to {
        Some(to) => to,
        None => now.checked_add_days(Days::new(1)).unwrap().date_naive(),
    };
    let tasks = get_tasks_with_durations(hamster_db, from, to, category);
    let mut total_duration = Duration::new(0, 0);

    let mut table = Table::new();
    table.set_header(["Task ID", "name", "duration"]);
    for (task_id, (task_title, duration)) in tasks.into_iter() {
        total_duration += duration;
        table.add_row([
            task_id.unwrap_or("-".to_string()),
            task_title.unwrap_or("-".to_string()),
            duration.as_hhmm(),
        ]);
    }
    table.add_row(["", "", total_duration.as_hhmm().as_str()]);
    println!("{table}");
}

async fn sync_tasks_to_everhour(
    hamster_db: Option<String>,
    api_token: String,
    from: NaiveDate,
    to: NaiveDate,
    category: Option<String>,
    dry_run: bool,
) {
    let client = EverhourClient::new(api_token);
    let me = client.get_current_user().await.unwrap();
    let existing_time_records = client
        .get_user_time_records(me.id, Some(from), Some(to))
        .await
        .unwrap();

    // sort existing time records into map by day/id pair
    let mut records_map: HashMap<(NaiveDate, String), TimeRecord> = HashMap::new();

    for time_record in existing_time_records.into_iter() {
        records_map
            .entry((
                time_record.date,
                time_record.task.as_ref().unwrap().id.clone(),
            ))
            .or_insert(time_record);
    }

    let mut day = from;
    while day <= to {
        println!("Processing day {}", day);
        let next_day = day.checked_add_days(Days::new(1)).unwrap();
        let tasks = get_tasks_with_durations(hamster_db.clone(), day, next_day, category.clone());
        let mut total_duration = Duration::new(0, 0);
        for (task_id, (task_title, duration)) in tasks.into_iter() {
            let task_id_eh = match &task_id {
                Some(task_id) => format!("as:{task_id}"),
                None => {
                    if dry_run {
                        "-".to_string()
                    } else {
                        panic!("Missing task id!");
                    }
                }
            };

            total_duration += duration;

            if task_id.is_none() {
                // no task id - not enough data to add anything
                continue;
            }

            let data_msg = format!(
                "{day}: {} seconds ({}) for user {} on task {} ({})",
                duration.as_secs(),
                duration.as_hhmm(),
                me.id,
                task_id_eh,
                task_title.unwrap_or("-".to_string())
            );
            let existing_record = records_map.get(&(day, task_id_eh.clone()));

            match (existing_record, dry_run) {
                (Some(existing_record), true) => {
                    println!(
                        "would sync to record {} - {data_msg}",
                        existing_record.id.unwrap()
                    )
                }
                (None, true) => println!("would add new  record - {data_msg}"),
                (Some(existing_record), false) => {
                    println!(
                        "syncing to record {} - {data_msg}",
                        existing_record.id.unwrap()
                    );
                    client
                        .update_task_time_record(
                            task_id_eh,
                            TimeRecord::for_adding(day, me.id, duration.as_secs() as i64, None),
                        )
                        .await
                        .unwrap();
                }
                (None, false) => {
                    println!("adding - {data_msg}");
                    client
                        .add_task_time_record(
                            task_id_eh,
                            TimeRecord::for_adding(day, me.id, duration.as_secs() as i64, None),
                        )
                        .await
                        .unwrap();
                }
            };
        }

        println!(
            "Total seconds for day: {} ({})",
            total_duration.as_secs(),
            total_duration.as_hhmm()
        );

        day = next_day;
    }
    println!("Everhour user id: {}", me.id);
}
