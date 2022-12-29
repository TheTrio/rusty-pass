use home::home_dir;
use sha256::digest;

use crate::{constants::DEFAULT_DIRECTORY_NAME, database::Database};
use std::{fs::File, io::Write, path::PathBuf};
pub mod crypto;
pub mod password;
pub mod path;

pub fn get_database<'a>(location: &'a PathBuf) -> Result<Database, rusqlite::Error> {
    let database = Database::new(location);
    database.init()?;
    Ok(database)
}

pub fn write_password_hash_to_file(master_password: &str) {
    let hash_file = home_dir()
        .unwrap()
        .join(DEFAULT_DIRECTORY_NAME)
        .join("RUSTY_MASTER_HASH");
    let mut file = File::create(hash_file).expect("Unable to create file");

    file.write_all(digest(master_password).as_bytes())
        .expect("Unable to write to file");
}
