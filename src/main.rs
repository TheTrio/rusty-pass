use clap::Parser;
use rusty_pass::{get_default_database_path, Cli, GenerateSubcommands, Subcommands};
use rusty_pass::{Database, Password};
use std::path::PathBuf;

fn main() {
    let cli = Cli::parse();
    match cli.commands {
        Subcommands::Init(init) => {
            let location = init
                .location
                .unwrap_or(PathBuf::from(get_default_database_path()));
            let database = Database::new(&location);
            if let Err(err) = database.init() {
                println!("Unable to initialize database: {:?}", err);
            }
        }
        Subcommands::Generate(x) => {
            let password = match x.commands {
                GenerateSubcommands::Strict {
                    lower,
                    upper,
                    symbols,
                    numbers,
                } => Password::AdvancedPassword {
                    lower_case_length: lower,
                    upper_case_length: upper,
                    numbers_length: numbers,
                    symbols_length: symbols,
                },
                GenerateSubcommands::Simple { length } => Password::SimplePassword(length),
            };
            println!("{}", password.generate());
        }
    }
}
