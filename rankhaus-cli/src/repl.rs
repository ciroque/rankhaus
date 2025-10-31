use anyhow::{Context, Result};
use crate::commands;
use crate::state::AppState;
use crate::Commands;
use std::io::{self, Write};

pub fn run() -> Result<()> {
    println!("Rankhaus REPL mode");
    println!("Type 'help' for available commands, 'exit' to quit");
    println!();
    println!("No list loaded. Use 'init <name>' or 'load <file>' to begin.");
    println!();
    
    let mut state = AppState::new();
    let stdin = io::stdin();
    
    loop {
        // Print prompt
        print!("rankhaus> ");
        io::stdout().flush()?;
        
        // Read line
        let mut input = String::new();
        stdin.read_line(&mut input)?;
        let input = input.trim();
        
        // Handle empty input
        if input.is_empty() {
            continue;
        }
        
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
    
    Ok(())
}

fn parse_command(input: &str) -> Result<Commands> {
    // Split input into args
    let args: Vec<&str> = input.split_whitespace().collect();
    
    if args.is_empty() {
        anyhow::bail!("No command provided");
    }
    
    // Build clap args with program name
    let mut clap_args = vec!["rankhaus"];
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
    println!("  init <name>              Initialize a new ranking list");
    println!("  load <file>              Load an existing list");
    println!("  items list               RankSet all items");
    println!("  items add                Add items from stdin");
    println!("  items remove             Remove items");
    println!("  items edit <id> <value>  Edit an item");
    println!("  users list               RankSet all users");
    println!("  users add <username>     Add a new user");
    println!("  users select <username>  Select active user");
    println!("  strategies list          RankSet available strategies");
    println!("  strategies select <name> Select a strategy");
    println!("  rank                     Start ranking");
    println!("  sessions list            RankSet all sessions");
    println!("  help                     Show this help");
    println!("  exit                     Exit REPL");
    println!();
}
