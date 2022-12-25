use crate::utils::path::path_exists;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
#[command(
    about = "Insert a new password into the database. If the -g flag is passed, a password of length 20 will be generated and inserted automatically. Else, you will be prompted to enter a password in your default editor",
    long_about = "Insert a new password into the database. If the -g flag is passed, a password of length 20 will be generated and inserted automatically. Else, you will be prompted to enter a password in your default editor. This reads the master password from the environment variable RUSTY_MASTER_PASSWORD. If it is not set, it will prompt for the same"
)]
pub struct InsertArgs {
    pub name: String,
    pub username: String,

    #[arg(short, long, value_parser = path_exists)]
    pub location: Option<PathBuf>,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Generate and insert a password of length 20 instead of prompting for the same"
    )]
    pub generate: bool,
}
