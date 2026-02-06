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
#[command(name = "track", version, about = "Time tracker CLI", author)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Vygeneruje report
    #[command(alias = "r")]
    Report,
    /// Spustí úkol
    #[command(alias = "s")]
    Start {
        #[arg(short, long)]
        task: String,
    },
    /// Pozastaví úkol
    #[command(alias = "p")]
    Pause {
        #[arg(short, long)]
        task: String,
    },
    /// Ukončí úkol
    #[command(alias = "st")]
    Stop {
        #[arg(short, long)]
        task: String,
    },
    /// Spustí minutku
    #[command(alias = "m")]
    Min {
        #[arg(short, long, default_value_t = 1)]
        minutes: u32,
    },
    /// Zobrazí report za dnešní den
    #[command(alias = "d")]
    Today,
    /// Smaže všechna data
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
