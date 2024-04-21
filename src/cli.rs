use chrono::NaiveDate;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Outputs information
    Info {},
    /// get facts (last week by default)
    GetFacts {},
    Tasks {
        #[arg(long)]
        from: Option<NaiveDate>,
        #[arg(long)]
        to: Option<NaiveDate>,
        category: Option<String>,
    },
    SyncTasksToEverhour {
        category: Option<String>,
        #[arg(long, env = "EVERHOUR_API_TOKEN")]
        api_token: String,
        #[arg(long)]
        from: NaiveDate,
        #[arg(long)]
        to: NaiveDate,
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
}
