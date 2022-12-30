use crate::utils::path::path_exists;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
#[command(
    about = "Export the passwords in the database to a json file.",
    long_about = "Export the passwords in the database to a json file. This reads the master password from the environment variable RUSTY_MASTER_PASSWORD. If it is not set, it will prompt for the same"
)]
pub struct ExportArgs {
    pub export_file: Option<PathBuf>,
    #[arg(short, long, value_parser = path_exists)]
    pub location: Option<PathBuf>,
}
