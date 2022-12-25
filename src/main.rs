use clap::{error::ErrorKind, Command, Parser};
use edit::edit;
use rusty_pass::{
    commands::{
        clear, generate::GenerateSubcommands, insert::InsertArgs, list::ListArgs, Cli, Subcommands,
    },
    constants::TEMPLATE_EDITOR_INPUT,
    utils::{
        crypto::encrypt,
        get_database,
        password::{generate_strict_password, Password},
        path::get_location,
    },
};
use std::env;

fn main() {
    let cli = Cli::parse();
    match cli.commands {
        Subcommands::Init(init) => {
            let location = get_location(init.location);
            get_database(&location).expect("Unable to initialize/read database");
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
        Subcommands::Insert(InsertArgs {
            username,
            name,
            location,
            generate,
        }) => {
            let master_password = env::var("RUSTY_MASTER_PASSWORD");
            if master_password.is_err() {
                let mut cmd = Command::new("Master Password");
                cmd.error(ErrorKind::InvalidValue, "The master password is not set. Set it using the RUSTY_MASTER_PASSWORD environment variable")
                .exit();
            } else {
                let master_password = master_password.unwrap();
                let location = get_location(location);
                let password = if !generate {
                    let text = edit(TEMPLATE_EDITOR_INPUT).expect("Unable to read from editor");
                    let mut lines = text.split("\n");
                    let password = lines
                        .next()
                        .map(String::from)
                        .expect("The password is empty");
                    if password.is_empty() {
                        let mut cmd = Command::new("Enter password into the editor");
                        cmd.error(ErrorKind::InvalidValue, "The password cannot be empty")
                            .exit();
                    }
                    password
                } else {
                    generate_strict_password()
                };
                let database = get_database(&location).expect("Unable to read database");
                database.insert(
                    &name,
                    &username,
                    &encrypt(String::from(master_password), password),
                )
            }
        }
        Subcommands::Clear(clear::ClearArgs {
            name,
            location,
            pattern: like,
        }) => {
            let location = get_location(location);

            let database = get_database(&location).expect("Unable to read database");
            database.clear(&name, like);
        }
        Subcommands::List(ListArgs {
            location,
            name,
            pattern,
        }) => {
            let master_password = env::var("RUSTY_MASTER_PASSWORD");
            if master_password.is_err() {
                let mut cmd = Command::new("Master Password");
                cmd.error(ErrorKind::InvalidValue, "The master password is not set. Set it using the RUSTY_MASTER_PASSWORD environment variable")
                .exit();
            } else {
                let master_password = master_password.unwrap();
                let location = get_location(location);

                let database = get_database(&location).expect("Unable to read database");
                match database.list(name, master_password, pattern) {
                    Ok(_) => (),
                    Err(err) => println!("Encountered Error: {:?}", err.to_string()),
                }
            }
        }
    }
}
