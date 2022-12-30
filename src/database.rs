use colored::Colorize;
use rusqlite::{Connection, Result, Row};
use serde::Serialize;
use std::{fmt::Display, path::PathBuf};

use crate::{
    config::Config,
    utils::{crypto::decrypt, display_error},
};
pub enum DatabaseState {
    Reading,
    Initializing,
}

pub struct Database<'a> {
    pub location: &'a PathBuf,
    connection: Connection,
    pub state: DatabaseState,
    pub config: Config,
}

#[derive(Serialize)]
pub struct Password {
    pub id: usize,
    pub name: String,
    pub username: String,
    pub password: String,
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {}\n{} - {}\n{} - {}\n{} - {}",
            "ID".green().bold(),
            self.id,
            "Name".green().bold(),
            self.name,
            "Username".green().bold(),
            self.username,
            "Password".green().bold(),
            self.password
        )
    }
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
    pub fn new(location: &'a PathBuf, config: Config) -> Self {
        let state = if location.exists() {
            DatabaseState::Reading
        } else {
            DatabaseState::Initializing
        };

        Self {
            state,
            location,
            connection: Connection::open(location).unwrap(),
            config,
        }
    }
    pub fn init(&mut self, master_password: &String) -> Result<(), rusqlite::Error> {
        println!(
            "{} database at {:}",
            self.state,
            self.location.to_str().unwrap().blue()
        );
        self.config
            .add_if_not_exists(master_password, self.location);

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

    pub fn list(
        &self,
        name: Option<String>,
        master_password: &String,
        pattern: bool,
    ) -> Result<()> {
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
            let password = decrypt(master_password, row.get(3)?);
            if password.is_err() {
                display_error("Decryption failed. Please check your master password.");
            }
            let password = password.unwrap();
            Ok(Password {
                id: row.get(0)?,
                name: row.get(1)?,
                username: row.get(2)?,
                password,
            })
        };

        println!();

        let password_iter = if name.is_some() {
            stmt.query_map((name.unwrap(),), map_row_to_password)?
        } else {
            stmt.query_map((), map_row_to_password)?
        };

        for password in password_iter {
            println!("{}\n", password.unwrap());
        }

        Ok(())
    }
}
