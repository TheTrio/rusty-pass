use clap::{Parser, Subcommand};

pub mod clear;
pub mod export;
pub mod generate;
pub mod import;
pub mod init;
pub mod insert;
pub mod list;

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
    Insert(insert::InsertArgs),
    Clear(clear::ClearArgs),
    List(list::ListArgs),
    Export(export::ExportArgs),
    Import(import::ImportArgs),
}
