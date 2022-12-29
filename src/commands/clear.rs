use std::path::PathBuf;

use crate::utils::path::path_exists;
use clap::Args;

#[derive(Args)]
#[command(
    about = "Clears the passwords in the database with the provided name. If the -p flag is passed, it will clear all passwords that match the pattern.",
    long_about = "Clears the passwords in the database with the provided name. If the -p flag is passed, it will list all passwords that match the pattern. This reads the master password from the environment variable RUSTY_MASTER_PASSWORD. If it is not set, it will prompt for the same"
)]
pub struct ClearArgs {
    pub name: String,
    #[arg(short, long, default_value_t = false)]
    pub pattern: bool,

    #[arg(short, long, value_parser = path_exists)]
    pub location: Option<PathBuf>,
}
