use sqlite::Connection;
use std::path::PathBuf;
pub mod commands;
pub mod utils;

enum DatabaseState {
    ReInitializing,
    Initializing,
}

impl std::fmt::Display for DatabaseState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseState::ReInitializing => write!(f, "Reinitialized"),
            DatabaseState::Initializing => write!(f, "Initialized"),
        }
    }
}
pub struct Database<'a> {
    location: &'a PathBuf,
    connection: Connection,
    state: DatabaseState,
}

impl<'a> Database<'a> {
    pub fn new(location: &'a PathBuf) -> Self {
        Self {
            state: if location.exists() {
                DatabaseState::ReInitializing
            } else {
                DatabaseState::Initializing
            },
            location,
            connection: Connection::open(location).unwrap(),
        }
    }
    pub fn init(&self) -> Result<(), sqlite::Error> {
        println!("{} database at {:?}", self.state, self.location);
        let query = "
        CREATE TABLE IF NOT EXISTS Passwords (
            id INTEGER unique, 
            name TEXT not null unique, 
            username TEXT not null, 
            password TEXT not null
        );
        ";
        self.connection.execute(query).unwrap();
        Ok(())
    }
}
