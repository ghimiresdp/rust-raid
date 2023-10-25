use _lib::input;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
    New { name: Option<String> },
    Drop { name: Option<String> },
    Use { name: Option<String> },
}

fn main() {
    let cli = Cli::parse();
    let selectedDb = "";

    match &cli.command {
        Commands::List => {
            println!("DATABASES\n\n")
        }
        Commands::New { name } => {
            println!("Creating Database '{name:?}'.")
        }
        Commands::Drop { name } => {
            println!("Deleting Database '{name:?}'.")
        }
        Commands::Use { name } => loop {
            let db = name.as_ref().unwrap();
            let command = input(format!("\n[{db}] >>> ").as_str());

            match command.trim() {
                "exit" => {
                    break;
                }
                _ => {
                    println!("Unknown command")
                }
            }
        },
    }
}
