use std::path::PathBuf;

use crate::database::Database;

use self::path::get_default_database_path;

pub mod password;
pub mod path;

pub fn get_database<'a>(location: &'a PathBuf) -> Result<Database, rusqlite::Error> {
    let database = Database::new(location);
    database.init()?;
    Ok(database)
}

pub fn get_location(location: Option<PathBuf>) -> PathBuf {
    location.unwrap_or(PathBuf::from(get_default_database_path()))
}
