use chrono::NaiveDate;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to Hamster database file - by default $HOME/.local/share/hamster/hamster.db
    #[arg(long, env = "HAMCLI_DB")]
    pub hamster_db: Option<String>,

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
    /// Synchronize task records to Everhour
    #[command(name = "sync-eh")]
    SyncTasksToEverhour {
        category: Option<String>,
        #[arg(long, env = "EVERHOUR_API_TOKEN")]
        api_token: String,
        #[arg(long)]
        from: Option<NaiveDate>,
        #[arg(long)]
        to: Option<NaiveDate>,
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
}
