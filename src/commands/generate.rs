use crate::constants::{
    DEFAULT_LOWERCASE_LENGTH, DEFAULT_NUMBERS_LENGTH, DEFAULT_SYMBOLS_LENGTH,
    DEFAULT_UPPERCASE_LENGTH,
};
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
        long_about = "This ensures that each character group has a minimum length. The default values are 10 for lower case, 5 for upper case, 2 for numbers and 3 for symbols."
    )]
    Strict {
        #[arg(short, long, default_value_t = DEFAULT_LOWERCASE_LENGTH)]
        lower: usize,
        #[arg(short, long, default_value_t = DEFAULT_UPPERCASE_LENGTH)]
        upper: usize,
        #[arg(short, long, default_value_t = DEFAULT_SYMBOLS_LENGTH)]
        symbols: usize,
        #[arg(short, long, default_value_t = DEFAULT_NUMBERS_LENGTH)]
        numbers: usize,
    },
}
