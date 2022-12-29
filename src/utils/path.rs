use crate::constants::{DEFAULT_DATABASE_NAME, DEFAULT_DIRECTORY_NAME};
use std::{fs::create_dir_all, path::PathBuf};

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

pub fn path_exists<'a>(path: &'a str) -> Result<PathBuf, String> {
    let path = PathBuf::from(path);
    if path.exists() {
        Ok(path)
    } else {
        Err(String::from("Path does not exist"))
    }
}

fn get_default_database_path() -> PathBuf {
    let home_dir = home::home_dir().expect("Unable to retrieve home directory");
    let rust_db = home_dir.join(DEFAULT_DIRECTORY_NAME);
    if !rust_db.is_dir() {
        create_dir_all(rust_db).expect("Unable to create directory");
    }
    home_dir.join(".rustypass").join(DEFAULT_DATABASE_NAME)
}
pub fn get_location(location: Option<PathBuf>) -> PathBuf {
    location.unwrap_or(PathBuf::from(get_default_database_path()))
}
