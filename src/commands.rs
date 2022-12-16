use clap::{Parser, Subcommand};

pub mod generate;
pub mod init;

#[derive(Parser)]
#[command(about = "A CLI password manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    Init(init::InitArgs),
    Generate(generate::GenerateArgs),
}
