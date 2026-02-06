use clap::{Parser, Subcommand};
use clap::CommandFactory;

mod report;
mod task_start;
mod task_pause;
mod task_stop;
mod task_today;
mod task_clear;
mod task_minute;
mod gui_headline;

#[derive(Parser)]
#[command(
    name = "track",
    version,
    about = "Simple CLI time tracker",
    author,
    help_template = "{about}\n\n{usage-heading} {usage}\n\n{all-args}\n"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generates a report
    #[command(alias = "r")]
    Report,
    /// Starts a task
    #[command(alias = "s")]
    Start {
        #[arg(short, long)]
        task: String,
    },
    /// Pauses a task
    #[command(alias = "p")]
    Pause {
        #[arg(short, long)]
        task: String,
    },
    /// Stops a task
    #[command(alias = "st")]
    Stop {
        #[arg(short, long)]
        task: String,
    },
    /// Starts a timer
    #[command(alias = "m")]
    Min {
        #[arg(short, long, default_value_t = 1)]
        minutes: u32,
    },
    /// Shows today's report
    #[command(alias = "d")]
    Today,
    /// Clears all data
    Clear,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Report) => report::run(),
        Some(Commands::Start { task }) => task_start::run(task),
        Some(Commands::Pause { task }) => task_pause::run(task),
        Some(Commands::Stop { task }) => task_stop::run(task),
        Some(Commands::Min { minutes }) => task_minute::run(*minutes),
        Some(Commands::Today) => task_today::run(),
        Some(Commands::Clear) => task_clear::run(),
        None => {
            Cli::command().print_help().unwrap();
        }
    }
}
