use rusqlite::{Connection, Result, Row};
use std::path::PathBuf;
enum DatabaseState {
    Reading,
    Initializing,
}

pub struct Database<'a> {
    location: &'a PathBuf,
    connection: Connection,
    state: DatabaseState,
}

#[derive(Debug)]
pub struct Password {
    pub id: usize,
    pub name: String,
    pub username: String,
    pub password: String,
}

impl std::fmt::Display for DatabaseState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseState::Reading => write!(f, "Reading"),
            DatabaseState::Initializing => write!(f, "Initialized"),
        }
    }
}

impl<'a> Database<'a> {
    pub fn new(location: &'a PathBuf) -> Self {
        Self {
            state: if location.exists() {
                DatabaseState::Reading
            } else {
                DatabaseState::Initializing
            },
            location,
            connection: Connection::open(location).unwrap(),
        }
    }
    pub fn init(&self) -> Result<(), rusqlite::Error> {
        println!("{} database at {:?}", self.state, self.location);
        let query = "
        CREATE TABLE IF NOT EXISTS Passwords (
            ID INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT not null, 
            username TEXT not null, 
            password TEXT not null
        );
        ";
        self.connection.execute(query, ())?;
        Ok(())
    }
    pub fn insert(&self, name: &str, username: &str, password: &str) {
        let query = "INSERT INTO Passwords(name, username, password) VALUES (?1, ?2, ?3);";
        match self.connection.execute(query, (name, username, password)) {
            Err(err) => println!("Unable to insert password: {:?}", err.to_string()),
            Ok(_) => println!("Password insert successful"),
        }
    }
    pub fn clear(&self, name: &str, pattern: bool) {
        let query = if pattern {
            "DELETE FROM Passwords WHERE name LIKE (?1);"
        } else {
            "DELETE FROM Passwords WHERE name = (?1);"
        };

        match self.connection.execute(query, (name,)) {
            Err(err) => println!("Unable to clear database: {:?}", err.to_string()),
            Ok(out) => println!("Affected {:} row(s)", out),
        }
    }

    pub fn list(&self, name: Option<String>, pattern: bool) -> Result<()> {
        let query = if let Some(_) = name {
            if pattern {
                "SELECT id, name, username, password FROM Passwords WHERE name LIKE (?1);"
            } else {
                "SELECT id, name, username, password FROM Passwords WHERE name = (?1);"
            }
        } else {
            "SELECT id, name, username, password FROM Passwords"
        };
        let mut stmt = self.connection.prepare(query)?;

        let map_row_to_password = |row: &Row| {
            Ok(Password {
                id: row.get(0)?,
                name: row.get(1)?,
                username: row.get(2)?,
                password: row.get(3)?,
            })
        };

        let password_iter = if name.is_some() {
            stmt.query_map((name.unwrap(),), map_row_to_password)?
        } else {
            stmt.query_map((), map_row_to_password)?
        };

        for password in password_iter {
            println!("{:?}", password.unwrap());
        }

        Ok(())
    }
}
