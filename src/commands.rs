use super::is_valid_path;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "A CLI password manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    Init(DatabaseArgs),
    Generate(GenerateArgs),
}

#[derive(Args)]
pub struct DatabaseArgs {
    #[arg(value_parser = is_valid_path)]
    pub location: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct GenerateArgs {
    #[command(subcommand)]
    pub commands: GenerateSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum GenerateSubcommands {
    #[command(
        about = "Use this when you want to generate a password without any restrictions",
        long_about = "This offers no restrictions on the password - there is no minimum character group length(say a minimum of 5 numbers). If you want to ensure that each character group has a minimum length, use the strict subcommand"
    )]
    Simple {
        #[arg(short, long, default_value_t = 20)]
        length: usize,
    },
    #[command(
        about = "This ensures that each character group has a minimum length",
        long_about = "This ensures that each character group has a minimum length"
    )]
    Strict {
        #[arg(short, long, default_value_t = 10)]
        lower: usize,
        #[arg(short, long, default_value_t = 5)]
        upper: usize,
        #[arg(short, long, default_value_t = 5)]
        symbols: usize,
        #[arg(short, long, default_value_t = 5)]
        numbers: usize,
    },
}
