use crate::error::LumenzaError;

use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(skip)]
    config_path: String,
    pictures_paths: Vec<String>,
    thumbnails_path: String,
    database_path: String,
}

// Static methods
impl Config {
    pub fn new(
        config_path: &PathBuf,
        pictures_paths: &Vec<PathBuf>,
        thumbnails_path: &PathBuf,
        database_path: &PathBuf,
    ) -> Result<Self, LumenzaError> {
        // Convert all PathBufs to Strings.
        let mut pictures_strings = Vec::new();
        for path in pictures_paths {
            pictures_strings.push(path.as_path().to_str().unwrap().to_string());
        }
        let config_string = String::from(config_path.as_path().to_str().unwrap());
        let thumbnails_string = String::from(thumbnails_path.as_path().to_str().unwrap());
        let database_string = String::from(database_path.as_path().to_str().unwrap());

        let config = Config {
            config_path: config_string,
            pictures_paths: pictures_strings,
            thumbnails_path: thumbnails_string,
            database_path: database_string,
        };
        config.write_config()?;

        Ok(config)
    }
    pub fn open(config_path: &PathBuf) -> Result<Self, LumenzaError> {
        let data = fs::read(config_path).map_err(|_| LumenzaError::IoError())?;
        let text = String::from_utf8(data)?;
        let config: Config = toml::from_str(&text)?;
        Ok(config)
    }
}

// Instance methods
impl Config {
    fn _read_config(&mut self) -> Result<(), LumenzaError> {
        let data = fs::read(&self.config_path).map_err(|_|LumenzaError::IoError())?;
        let text = String::from_utf8(data)?;
        let config: Config = toml::from_str(&text)?;
        *self = config;
        Ok(())
    }

    fn write_config(&self) -> Result<(), LumenzaError> {
        let text = toml::to_string(&self)?;
        std::fs::write(&self.config_path, text).map_err(|_| LumenzaError::IoError())?;
        Ok(())
    }

    pub fn add_folder(&mut self, folder: &PathBuf) -> Result<(), LumenzaError> {
        let folder_string = folder.as_path().to_str().unwrap().to_string();
        self.pictures_paths.push(folder_string);
        self.write_config()?;
        Ok(())
    }

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
