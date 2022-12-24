use clap::{error::ErrorKind, Command, Parser};
use edit::edit;
use rusty_pass::{
    commands::{
        clear, generate::GenerateSubcommands, insert::InsertArgs, list::ListArgs, Cli, Subcommands,
    },
    utils::{get_database, password::Password, path::get_location},
};

const TEMPLATE_EDITOR_INPUT: &str = "

# The first line is treated as the password
# Anything after the first line is ignored 
";
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
        }) => {
            let location = get_location(location);

            let text = edit(TEMPLATE_EDITOR_INPUT).expect("Unable to read from editor");
            let password = text.split("\n").next().expect("The password is empty");
            if password.is_empty() {
                let mut cmd = Command::new("Enter password into the editor");
                cmd.error(ErrorKind::InvalidValue, "The password cannot be empty")
                    .exit();
            }
            let database = get_database(&location).expect("Unable to read database");
            database.insert(&name, &username, &password)
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
            let location = get_location(location);

            let database = get_database(&location).expect("Unable to read database");
            match database.list(name, pattern) {
                Ok(_) => (),
                Err(err) => println!("Encountered Error: {:?}", err.to_string()),
            }
        }
    }
}
