use clap::{Parser, Subcommand};
use std::io::stdin;
mod vault;
use vault::Vault;
/// This function reads input from the standard input (stdin) and returns it as a String.
///
/// # Parameters
///
/// * `prompt` - A string that is printed to the console before reading input. This is used to
///              prompt the user for input.
///
/// # Return
///
/// * A String containing the input read from the console. The leading and trailing whitespace are
///   trimmed from the input.
fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    stdin().read_line(&mut input).expect("");
    input.trim().to_owned()
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    #[command(about = "Creates a new vault with the name provided or the default vault ")]
    Create {
        #[arg(short, long, default_value_t = String::from("default"))]
        name: String,
    },
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

    match command.subcommand {
        Subcommands::Open { name } => {
            println!("opening the vault: {name}");
            let password = input("Enter password:");
            Vault::open(name, password);
        }
        Subcommands::Create { name } => {
            Vault::create(name);
        }
        Subcommands::Destroy => todo!(),
    }
}
