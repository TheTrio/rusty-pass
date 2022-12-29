use std::path::PathBuf;

use crate::{config::Config, database::Database};

pub mod crypto;
pub mod password;
pub mod path;

pub fn get_database<'a>(
    location: &'a PathBuf,
    master_password: &'a String,
) -> Result<Database<'a>, rusqlite::Error> {
    let config = Config::new();
    let mut database = Database::new(location, config);
    database.init(master_password)?;
    Ok(database)
}
