use crate::constants::{DEFAULT_DATABASE_NAME, DEFAULT_DIRECTORY_NAME};
use std::{fs::create_dir_all, path::PathBuf};

use super::display_error;

pub fn is_valid_path(path: &str) -> Result<PathBuf, String> {
    if path.starts_with('/') {
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

pub fn path_exists(path: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(path);
    if path.exists() {
        Ok(path)
    } else {
        Err(String::from("Path does not exist"))
    }
}

fn get_default_database_path() -> PathBuf {
    let home_dir = home::home_dir();
    if home_dir.is_none() {
        display_error("Unable to retrieve home directory");
    }

    let home_dir = home_dir.unwrap();
    let rust_db = home_dir.join(DEFAULT_DIRECTORY_NAME);
    if !rust_db.is_dir() {
        create_dir_all(rust_db).unwrap_or_else(|e| {
            display_error(&format!("Unable to create directory for database: {}", e))
        });
    }
    home_dir.join(".rustypass").join(DEFAULT_DATABASE_NAME)
}
pub fn get_location(location: Option<PathBuf>) -> PathBuf {
    location.unwrap_or_else(get_default_database_path)
}

pub fn get_config_location() -> PathBuf {
    let home_dir = home::home_dir();
    if home_dir.is_none() {
        display_error("Unable to retrieve home directory");
    }

    home_dir
        .unwrap()
        .join(DEFAULT_DIRECTORY_NAME)
        .join("config.json")
}
