use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "godo")]
#[command(author, version, about = "Git-Oriented DO - CLI-first local TODO management tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize godo (create config and data files)
    Init,

    /// Add a new task
    Add {
        /// Task title
        title: String,
    },

    /// List tasks
    List {
        /// Show all tasks including completed ones
        #[arg(short, long)]
        all: bool,
    },

    /// Mark a task as done
    Done {
        /// Task ID
        id: u32,
    },

    /// Remove a task
    Rm {
        /// Task ID
        id: u32,
    },

    /// Edit a task title
    Edit {
        /// Task ID
        id: u32,
        /// New task title
        title: String,
    },
}
