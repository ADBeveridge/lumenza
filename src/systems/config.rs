use crate::error::Error;

use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pictures_paths: Vec<String>,
    thumbnails_path: String,
    database_path: String,
}

// Static methods
impl Config {
    pub fn read_config(path: &PathBuf) -> Result<Config, Error> {
        let data = fs::read(path).unwrap();
        let text = String::from_utf8(data).unwrap();
        let config: Config = toml::from_str(&text).unwrap();
        Ok(config)
    }

    pub fn write_config(
        config_path: &String,
        pictures_paths: &Vec<String>,
        thumbnails_path: &String,
        database_path: &String,
    ) -> Result<(), Error> {
        let config = Config {
            pictures_paths: pictures_paths.clone(),
            thumbnails_path: thumbnails_path.clone(),
            database_path: database_path.clone(),
        };
        let text = toml::to_string(&config).unwrap();
        std::fs::write(config_path, text).unwrap();
        Ok(())
    }
}

// Instance methods
impl Config {
    pub fn get_pictures_path(&self) -> Vec<PathBuf> {
        // Convert strings to owned pathbufs.
        let mut vec = Vec::new();
        for path in &self.pictures_paths {
            vec.push(PathBuf::from(path));
        }
        vec
    }
    pub fn get_thumbnails_path(&self) -> PathBuf {
        PathBuf::from(&self.thumbnails_path)
    }
    pub fn get_database_path(&self) -> PathBuf {
        PathBuf::from(&self.database_path)
    }
}
