use clap::Parser;
use colored::Colorize;
use edit::edit;

use rusty_pass::{
    commands::{
        clear, export::ExportArgs, generate::GenerateSubcommands, import::ImportArgs,
        insert::InsertArgs, list::ListArgs, Cli, Subcommands,
    },
    constants::TEMPLATE_EDITOR_INPUT,
    utils::{
        crypto::encrypt,
        display_error, get_database,
        password::{generate_strict_password, get_master_password, Password},
        path::get_location,
    },
};

fn main() {
    let cli = Cli::parse();
    match cli.commands {
        Subcommands::Init(init) => {
            let location = get_location(init.location);
            let master_password = get_master_password();
            get_database(&location, &master_password);
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
            let master_password = get_master_password();
            let location = get_location(location);

            let database = get_database(&location, &master_password);

            if !database.config.matches_hash(&location, &master_password) {
                display_error("Decryption Failed. The Master Password is incorrect");
            }

            let password = if !generate {
                let text = edit(TEMPLATE_EDITOR_INPUT).expect("Unable to read from editor");
                let mut lines = text.split('\n');
                let password = lines
                    .next()
                    .map(String::from)
                    .expect("The password is empty");
                if password.is_empty() {
                    display_error("The password can't be empty");
                }
                password
            } else {
                generate_strict_password()
            };

            database.insert(&name, &username, &encrypt(&master_password, &password))
        }
        Subcommands::Clear(clear::ClearArgs {
            name,
            location,
            pattern: like,
        }) => {
            let location = get_location(location);
            let master_password = get_master_password();
            let database = get_database(&location, &master_password);
            if !database.config.matches_hash(&location, &master_password) {
                display_error("The master password is incorrect. No changes were made");
            }

            database.clear(&name, like);
        }
        Subcommands::List(ListArgs {
            location,
            name,
            pattern,
        }) => {
            let master_password = get_master_password();
            let location = get_location(location);

            let database = get_database(&location, &master_password);
            match database.list_passwords(name, &master_password, pattern) {
                Ok(_) => (),
                Err(err) => println!(
                    "{}: {:}",
                    "Encountered Error".red(),
                    err.to_string().yellow()
                ),
            }
        }
        Subcommands::Export(ExportArgs {
            location,
            export_file,
        }) => {
            let master_password = get_master_password();
            let location = get_location(location);
            let database = get_database(&location, &master_password);

            if let Some(file_path) = export_file {
                database
                    .export_as_json_to_file(&master_password, &file_path)
                    .unwrap();
            } else {
                let json_export = database
                    .export_as_json(&master_password)
                    .expect("Unable to export database");

                println!("{}", json_export);
            }
        }
        Subcommands::Import(ImportArgs {
            import_file,
            location,
        }) => {
            let master_password = get_master_password();
            let location = get_location(location);
            let database = get_database(&location, &master_password);

            match database.import_from_json(&master_password, &import_file) {
                Ok(_) => (),
                Err(err) => println!(
                    "{}: {:}",
                    "Encountered Error".red(),
                    err.to_string().yellow()
                ),
            }
        }
    }
}
