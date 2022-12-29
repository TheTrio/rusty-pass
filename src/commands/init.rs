use crate::utils::path::is_valid_path;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
#[command(
    about = "Initializes the database at the provided location. If none is provided, the database is initialized at $HOME/.rustypass/database.db",
    long_about = "Initializes the database at the provided location. If none is provided, the database is initialized at $HOME/.rustypass/database.db"
)]
pub struct InitArgs {
    #[arg(value_parser = is_valid_path)]
    pub location: Option<PathBuf>,
}
