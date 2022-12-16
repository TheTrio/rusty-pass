use crate::utils::path::is_valid_path;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct InitArgs {
    #[arg(value_parser = is_valid_path)]
    pub location: Option<PathBuf>,
}
