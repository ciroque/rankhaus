use anyhow::{Context, Result};
use crate::commands;
use crate::state::AppState;
use crate::Commands;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

const HISTORY_FILE: &str = ".rankhaus_history";

pub fn run() -> Result<()> {
    println!("Rankhaus REPL mode");
    println!("Type 'help' for available commands, 'exit' to quit");
    println!();
    println!("No list loaded. Use 'ranksets list' to see examples or 'ranksets new <name>' to create one.");
    println!();
    
    let mut state = AppState::new();
    let mut rl = DefaultEditor::new()?;
    
    // Load history from previous sessions
    let _ = rl.load_history(HISTORY_FILE);
    
    loop {
        // Read line with history support
        let readline = rl.readline("rankhaus> ");
        
        match readline {
            Ok(line) => {
                let input = line.trim();
                
                // Handle empty input
                if input.is_empty() {
                    continue;
                }
                
                // Add to history
                let _ = rl.add_history_entry(input);
                
                // Handle exit
                if input == "exit" || input == "quit" {
                    if state.has_rankset() {
                        println!("Saving...");
                        if let Err(e) = state.save() {
                            eprintln!("Warning: Failed to save: {}", e);
                        }
                    }
                    println!("Goodbye!");
                    break;
                }
                
                // Handle help
                if input == "help" {
                    print_help();
                    continue;
                }
                
                // Parse and execute command
                match parse_command(input) {
                    Ok(command) => {
                        if let Err(e) = commands::execute_with_state(command, &mut state) {
                            eprintln!("Error: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        eprintln!("Type 'help' for available commands");
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D
                println!("exit");
                if state.has_rankset() {
                    println!("Saving...");
                    if let Err(e) = state.save() {
                        eprintln!("Warning: Failed to save: {}", e);
                    }
                }
                break;
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
                break;
            }
        }
    }
    
    // Save history for next session
    let _ = rl.save_history(HISTORY_FILE);
    
    Ok(())
}

fn parse_command(input: &str) -> Result<Commands> {
    // Use shlex to properly parse shell-like input (handles quotes, escapes, etc.)
    let args = shlex::split(input)
        .ok_or_else(|| anyhow::anyhow!("Failed to parse command line"))?;
    
    if args.is_empty() {
        anyhow::bail!("No command provided");
    }
    
    // Build clap args with program name
    let mut clap_args = vec!["rankhaus".to_string()];
    clap_args.extend(args);
    
    // Parse with clap
    use clap::Parser;
    let cli = crate::Cli::try_parse_from(clap_args)
        .context("Failed to parse command")?;
    
    cli.command.ok_or_else(|| anyhow::anyhow!("No command provided"))
}

fn print_help() {
    println!("Available commands:");
    println!();
    println!("  ranksets list              List available ranksets");
    println!("  ranksets load <file>       Load an existing rankset");
    println!("  ranksets new <name>        Create a new ranking list");
    println!();
    println!("  items list                 List all items");
    println!("  items add [item]           Add item(s) (direct or interactive)");
    println!("  items remove               Remove items");
    println!("  items edit <id> <value>    Edit an item");
    println!();
    println!("  users list                 List all users");
    println!("  users add <username>       Add a new user");
    println!("  users edit <user> <name>   Edit user display name");
    println!("  users select <username>    Select active user");
    println!("  users remove <username>    Remove a user");
    println!();
    println!("  strategies list            List available strategies");
    println!("  strategies select <name>   Select a strategy");
    println!();
    println!("  rank                       Start new ranking session");
    println!("                             (Press 'q' during ranking to suspend)");
    println!();
    println!("  sessions list              List all ranking sessions");
    println!("  sessions show <id>         Show session details");
    println!("  sessions resume <id>       Resume in-progress session");
    println!("  sessions delete <id>       Delete a session");
    println!();
    println!("  help                       Show this help");
    println!("  exit                       Exit REPL (Ctrl+D also works)");
    println!();
}
