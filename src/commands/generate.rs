use clap::{Args, Subcommand};

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
