use std::{fs::File, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::utils::{crypto::get_sha256_hash, path::get_config_location};

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
            let file = File::open(config_file_path).expect("Unable to read config file");
            let rdr = std::io::BufReader::new(file);
            let config: Config = serde_json::from_reader(rdr).expect("Unable to parse config file");
            config
        } else {
            Self { passwords: vec![] }
        }
    }
    pub fn add_if_not_exists(&mut self, master_password: &String, location: &PathBuf) {
        let master_password = MasterPassword {
            database_location: location.clone(),
            hash: get_sha256_hash(master_password),
        };
        if !self.passwords.contains(&master_password) {
            self.passwords.push(master_password);
        }
        self.write_to_file()
            .expect("Unable to write to config file")
    }

    pub fn matches_hash(&self, location: &PathBuf, entered_master_password: &String) -> bool {
        let master_password = (&self.passwords)
            .into_iter()
            .filter(|password| password.database_location == *location)
            .next()
            .expect("Unable to find entry for this database in the config file. Please use 'init' to create a new database.");

        return get_sha256_hash(&entered_master_password) == master_password.hash;
    }
}
