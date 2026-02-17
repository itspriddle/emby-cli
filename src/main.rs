mod cli;
mod client;
mod commands;
mod config;
mod emby;
mod error;
mod format;

use clap::Parser;
use std::process;

fn main() {
    let cli = cli::Cli::parse();

    if let Err(e) = run(&cli) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run(cli: &cli::Cli) -> error::Result<()> {
    match &cli.command {
        cli::Command::Scan(args) => commands::scan::run(args),
        cli::Command::Playing(args) => commands::playing::run(args),
        cli::Command::Restart => commands::restart::run(),
        cli::Command::System => commands::system::run(),
        cli::Command::Users => commands::users::run(),
        cli::Command::Devices => commands::devices::run(),
        cli::Command::Libraries => commands::libraries::run(),
        cli::Command::FindServer(args) => commands::find_server::run(args),
        cli::Command::Activity(args) => commands::activity::run(args),
        cli::Command::Latest(args) => commands::latest::run(args),
        cli::Command::Search(args) => commands::search::run(args),
        cli::Command::NextUp(args) => commands::next_up::run(args),
        cli::Command::Upcoming(args) => commands::upcoming::run(args),
        cli::Command::Tasks(args) => commands::tasks::run(args),
    }
}
