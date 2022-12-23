use crate::utils::path::path_exists;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct ListArgs {
    pub name: Option<String>,
    #[arg(short, long, default_value_t = false)]
    pub pattern: bool,

    #[arg(short, long, value_parser = path_exists)]
    pub location: Option<PathBuf>,
}
