use std::env;

use rand::{distributions::Uniform, seq::SliceRandom, Rng};

use crate::constants::{
    DEFAULT_LOWERCASE_LENGTH, DEFAULT_NUMBERS_LENGTH, DEFAULT_SYMBOLS_LENGTH,
    DEFAULT_UPPERCASE_LENGTH, LOWER_CASE, NUMBERS, SYMBOLS, UPPER_CASE,
};

pub enum Password {
    SimplePassword(usize),
    AdvancedPassword {
        lower_case_length: usize,
        upper_case_length: usize,
        numbers_length: usize,
        symbols_length: usize,
    },
}

impl Password {
    pub fn generate(&self) -> String {
        match self {
            Password::SimplePassword(length) => generate_simple_password(*length),
            Password::AdvancedPassword {
                lower_case_length,
                upper_case_length,
                numbers_length,
                symbols_length,
            } => advance_generate_password(
                *lower_case_length,
                *upper_case_length,
                *numbers_length,
                *symbols_length,
            ),
        }
    }
}

fn advance_generate_password(
    lower_case_length: usize,
    upper_case_length: usize,
    numbers_length: usize,
    symbols_length: usize,
) -> String {
    let mut rng = rand::thread_rng();
    let mut password = String::new();
    let between_lower_case = Uniform::from(0..LOWER_CASE.len());
    let between_upper_case = Uniform::from(0..UPPER_CASE.len());
    let between_numbers = Uniform::from(0..NUMBERS.len());
    let between_symbols = Uniform::from(0..SYMBOLS.len());

    for _ in 0..lower_case_length {
        let random_index = rng.sample(between_lower_case);
        password.push(LOWER_CASE.chars().nth(random_index).unwrap());
    }

    for _ in 0..upper_case_length {
        let random_index = rng.sample(between_upper_case);
        password.push(UPPER_CASE.chars().nth(random_index).unwrap());
    }

    for _ in 0..numbers_length {
        let random_index = rng.sample(between_numbers);
        password.push(NUMBERS.chars().nth(random_index).unwrap());
    }

    for _ in 0..symbols_length {
        let random_index = rng.sample(between_symbols);
        password.push(SYMBOLS.chars().nth(random_index).unwrap());
    }
    let mut chars = password.chars().collect::<Vec<char>>();

    chars.shuffle(&mut rng);
    chars.iter().collect()
}

fn generate_simple_password(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let chars = [LOWER_CASE, UPPER_CASE, NUMBERS, SYMBOLS].join("");
    let mut password = String::new();

    for _ in 0..length {
        let random_index = rng.gen_range(0..chars.len());
        password.push(chars.chars().nth(random_index).unwrap());
    }
    password
}

pub fn generate_strict_password() -> String {
    advance_generate_password(
        DEFAULT_LOWERCASE_LENGTH,
        DEFAULT_UPPERCASE_LENGTH,
        DEFAULT_NUMBERS_LENGTH,
        DEFAULT_SYMBOLS_LENGTH,
    )
}

pub fn get_master_password() -> String {
    let master_password = env::var("RUSTY_MASTER_PASSWORD");
    let mut stdin_master_password = String::from("");
    if master_password.is_err() {
        stdin_master_password = rpassword::prompt_password("Your master password: ").unwrap();
    }
    master_password.unwrap_or(stdin_master_password)
}
