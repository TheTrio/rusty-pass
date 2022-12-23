use std::path::PathBuf;

use crate::utils::path::path_exists;
use clap::Args;

#[derive(Args)]
pub struct ClearArgs {
    pub name: String,
    #[arg(short, long, default_value_t = false)]
    pub pattern: bool,

    #[arg(short, long, value_parser = path_exists)]
    pub location: Option<PathBuf>,
}
