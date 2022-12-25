use crate::utils::path::path_exists;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
#[command(
    about = "Lists the passwords in the database. If a name is passed, it will list the password with that name. If the -p flag is passed, it will list all passwords that match the pattern.",
    long_about = "Lists the passwords in the database. If a name is passed, it will list the password with that name. If the -p flag is passed, it will list all passwords that match the pattern. This reads the master password from the environment variable RUSTY_MASTER_PASSWORD. If it is not set, it will prompt for the same"
)]
pub struct ListArgs {
    pub name: Option<String>,
    #[arg(short, long, default_value_t = false)]
    pub pattern: bool,

    #[arg(short, long, value_parser = path_exists)]
    pub location: Option<PathBuf>,
}
