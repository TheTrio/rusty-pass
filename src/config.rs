use std::{fs::File, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::utils::{crypto::get_sha256_hash, display_error, path::get_config_location};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub passwords: Vec<MasterPassword>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MasterPassword {
    pub database_location: PathBuf,
    pub hash: String,
}

impl PartialEq for MasterPassword {
    fn eq(&self, other: &Self) -> bool {
        self.database_location == other.database_location
    }
}

impl Config {
    pub fn write_to_file(&self) -> Result<(), std::io::Error> {
        let config_file = get_config_location();
        let file = File::create(config_file)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }
    pub fn new() -> Self {
        let config_file_path = get_config_location();
        if config_file_path.exists() {
            let file = File::open(config_file_path);
            if let Err(err) = &file {
                display_error(&format!("Unable to read config file - {}", err));
            }
            let file = file.unwrap();

            let rdr = std::io::BufReader::new(file);
            let config: Result<Config, _> = serde_json::from_reader(rdr);
            if config.is_err() {
                display_error(
                    "Unable to read config file. Please delete the config file and try again",
                );
            }
            config.unwrap()
        } else {
            Self { passwords: vec![] }
        }
    }
    pub fn add_if_not_exists(&mut self, master_password_str: &str, location: &PathBuf) {
        let master_password = MasterPassword {
            database_location: location.clone(),
            hash: get_sha256_hash(master_password_str),
        };
        if !self.passwords.contains(&master_password) {
            self.passwords.push(master_password);
        } else if !self.matches_hash(location, master_password_str) {
            display_error("Invalid master password for this database. Please try again.");
        }

        self.write_to_file()
            .expect("Unable to write to config file")
    }

    pub fn matches_hash(&self, location: &PathBuf, entered_master_password: &str) -> bool {
        let master_password = self
            .passwords
            .iter()
            .find(|password| password.database_location == *location);

        if master_password.is_none() {
            display_error("Unable to find entry for this database in the config file. Please use 'init' to create a new database.");
        }
        get_sha256_hash(entered_master_password) == master_password.unwrap().hash
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
