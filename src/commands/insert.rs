use crate::utils::path::path_exists;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct InsertArgs {
    pub name: String,
    pub username: String,
    pub password: String,

    #[arg(short, long, value_parser = path_exists)]
    pub location: Option<PathBuf>,
}
