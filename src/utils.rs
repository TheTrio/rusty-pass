use rand::seq::SliceRandom;
use rand::{distributions::Uniform, Rng};
use std::{fs::create_dir_all, path::PathBuf};

const DEFAULT_DATABASE_NAME: &str = "database.db";
const LOWER_CASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPER_CASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()_+-";

pub enum Password {
    SimplePassword(usize),
    AdvancedPassword {
        lower_case_length: usize,
        upper_case_length: usize,
        numbers_length: usize,
        symbols_length: usize,
    },
}

pub fn is_valid_path<'a>(path: &'a str) -> Result<PathBuf, String> {
    if path.starts_with("/") {
        let path = PathBuf::from(path);
        if path.exists() {
            Err(String::from("Path already exists"))
        } else {
            Ok(path)
        }
    } else {
        Err(String::from("Must be an absolute path"))
    }
}

pub fn get_default_database_path() -> PathBuf {
    let home_dir = home::home_dir().expect("Unable to retrieve home directory");
    let rust_db = home_dir.join("rustdb");
    if !rust_db.is_dir() {
        create_dir_all(rust_db).expect("Unable to create directory");
    }
    home_dir.join("rustdb").join(DEFAULT_DATABASE_NAME)
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
