use crate::utils::path::path_exists;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
#[command(
    about = "Imports the passwords into the specified database from a json file.",
    long_about = "Imports the passwords into the specified database from a json file. This reads the master password from the environment variable RUSTY_MASTER_PASSWORD. If it is not set, it will prompt for the same"
)]
pub struct ImportArgs {
    #[arg(short, long, value_parser = path_exists)]
    pub import_file: PathBuf,
    #[arg(short, long, value_parser = path_exists)]
    pub location: Option<PathBuf>,
}
