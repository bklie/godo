mod cli;
mod commands;
mod config;
mod error;
mod storage;
mod task;

use clap::Parser;
use cli::{Cli, Commands};
use std::process;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init => commands::init(),
        Commands::Add { title } => commands::add(title),
        Commands::List { all } => commands::list(all),
        Commands::Done { id } => commands::done(id),
        Commands::Rm { id } => commands::rm(id),
        Commands::Edit { id, title } => commands::edit(id, title),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
