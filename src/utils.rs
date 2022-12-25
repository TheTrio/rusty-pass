use crate::database::Database;
use std::path::PathBuf;
pub mod crypto;
pub mod password;
pub mod path;

pub fn get_database<'a>(location: &'a PathBuf) -> Result<Database, rusqlite::Error> {
    let database = Database::new(location);
    database.init()?;
    Ok(database)
}
