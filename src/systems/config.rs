/*	Copyright (C) 2024 Alan Beveridge

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License along
    with this program; if not, write to the Free Software Foundation, Inc.,
    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
*/

use crate::error::LumenzaError;

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
    pub fn read_config(path: &PathBuf) -> Result<Config, LumenzaError> {
        let data = fs::read(path).map_err(|_|LumenzaError::IoError())?;
        let text = String::from_utf8(data)?;
        let config: Config = toml::from_str(&text)?;
        Ok(config)
    }

    pub fn write_config(
        config_path: &String,
        pictures_paths: &Vec<String>,
        thumbnails_path: &String,
        database_path: &String,
    ) -> Result<(), LumenzaError> {
        let config = Config {
            pictures_paths: pictures_paths.clone(),
            thumbnails_path: thumbnails_path.clone(),
            database_path: database_path.clone(),
        };
        let text = toml::to_string(&config)?;
        std::fs::write(config_path, text).map_err(|_| LumenzaError::IoError())?;
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
