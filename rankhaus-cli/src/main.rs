use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod repl;
mod state;

#[derive(Parser)]
#[command(name = "rankhaus")]
#[command(about = "Interactive stack ranking tool", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage ranksets
    Ranksets {
        #[command(subcommand)]
        command: RanksetsCommands,
    },

    /// Manage items in the current list
    Items {
        #[command(subcommand)]
        command: ItemsCommands,
    },

    /// Manage users
    Users {
        #[command(subcommand)]
        command: UsersCommands,
    },

    /// Manage ranking strategies
    Strategies {
        #[command(subcommand)]
        command: StrategyCommands,
    },

    /// Perform ranking
    Rank,

    /// Manage ranking sessions
    Sessions {
        #[command(subcommand)]
        command: SessionsCommands,
    },
}

#[derive(Subcommand)]
pub enum ItemsCommands {
    /// List all items
    List,

    /// Add an item or enter interactive mode
    Add {
        /// Item to add. If omitted, enters interactive mode.
        item: Option<String>,
    },

    /// Remove items by name
    Remove,

    /// Edit an item's value
    Edit {
        /// Item identifier (ID or value)
        identifier: String,

        /// New value
        new_value: String,
    },
}

#[derive(Subcommand)]
pub enum UsersCommands {
    /// List all users
    List,

    /// Add a new user
    Add {
        /// Username
        username: String,

        #[arg(long)]
        display_name: Option<String>,
    },

    /// Remove a user
    Remove {
        /// Username or user ID
        identifier: String,

        #[arg(long)]
        cascade: bool,
    },

    /// Edit a user's display name
    Edit {
        /// Username or user ID
        identifier: String,

        /// New display name
        new_display_name: String,
    },

    /// Select active user for session
    Select {
        /// Username or user ID
        identifier: String,
    },

    /// Get or set the default user
    Default {
        /// Username or user ID to set as default (omit to show current default)
        identifier: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum StrategyCommands {
    /// List available strategies
    List,

    /// Select a strategy
    Select {
        /// Strategy name
        strategy: String,
    },
}

#[derive(Subcommand)]
pub enum RanksetsCommands {
    /// List available ranksets in ./ranksets/ directory
    List,

    /// Load an existing rankset
    Load {
        /// Path to the rankset file
        file: String,
    },

    /// Create a new ranking list
    New {
        /// Name of the list
        name: String,

        #[arg(long)]
        user: Option<String>,

        #[arg(long)]
        display_name: Option<String>,

        #[arg(long)]
        description: Option<String>,

        #[arg(long)]
        author: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum SessionsCommands {
    /// List all sessions
    List,

    /// Show session details
    Show {
        /// Session ID
        session_id: String,
    },

    /// Delete a session
    Delete {
        /// Session ID
        session_id: String,
    },

    /// Resume an in-progress session
    Resume {
        /// Session ID to resume
        session_id: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        None => {
            // No command provided - enter REPL mode
            repl::run()?;
        }
        Some(command) => {
            // Execute single command
            commands::execute(command)?;
        }
    }

    Ok(())
}
