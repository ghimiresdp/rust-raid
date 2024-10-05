use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::stdin;

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    stdin().read_line(&mut input).expect("");
    input.trim().to_owned()
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    #[command(about = "Opens the vault with the name provided or the default vault ")]
    Open {
        #[arg(short, long, default_value_t = String::from("default"))]
        name: String,
    },
    Destroy,
}

#[derive(Parser, Debug)]
#[command(version, about="Vault CLI", long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    subcommand: Subcommands,
}

fn main() {
    let command = Cli::parse();

    match &command.subcommand {
        Subcommands::Open { name } => {
            println!("opening the vault: {name}");
            let password = input("Enter password:");
            println!("You entered: {password}");
        }
        Subcommands::Destroy => todo!(),
    }
}
