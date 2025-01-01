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

use mime_guess;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::error::Error;
use crate::error::InternalError;
use crate::picture;
use crate::systems::config;
use crate::systems::database;
use crate::systems::filesystem;

pub struct Library {
    pub(crate) _fs: filesystem::Filesystem,
    pub(crate) db: database::Database,
}

// Static methods.
impl Library {
    /// This function will create the files at the given paths.
    pub fn create(
        config_path: &PathBuf,
        thumbnails_path: &PathBuf,
        pictures_paths: &Vec<PathBuf>,
        database_path: &PathBuf,
    ) -> Result<Self, Error> {
        // Initialize config, database, and file systems.
        let fs = filesystem::Filesystem::new(config_path, thumbnails_path, pictures_paths).unwrap();
        let db = database::Database::new(database_path).unwrap();

        let mut pictures_strs = Vec::new();
        for path in pictures_paths {
            pictures_strs.push(path.as_path().to_str().unwrap().to_string());
        }
        config::Config::write_config(
            &config_path.as_path().to_str().unwrap().to_string(),
            &pictures_strs,
            &thumbnails_path.as_path().to_str().unwrap().to_string(),
            &database_path.as_path().to_str().unwrap().to_string(),
        )
        .unwrap();

        Ok(Library { _fs: fs, db: db })
    }

    pub fn open(config_path: &PathBuf) -> Result<Self, Error> {
        let config = config::Config::read_config(config_path).unwrap();

        let thumbnails_path = config.get_thumbnails_path();
        let pictures_paths = config.get_pictures_path();
        let database_path = config.get_database_path();

        let fs =
            filesystem::Filesystem::open(config_path, &thumbnails_path, &pictures_paths).unwrap();
        let db = database::Database::open(&database_path).unwrap();

        Ok(Library { _fs: fs, db: db })
    }
}

// Instance methods. 
impl  Library {
    /// If folder is not in library already, add folder and pictures to library
    pub fn scan_folder(&self, folder: &PathBuf) -> Result<(), Error> {
        let mut image_paths: Vec<PathBuf> = Vec::new();

        let walker = WalkDir::new(folder)
            .into_iter()
            .filter_entry(|e| !is_hidden_folder(e));

        // Walk through all the files inside it, taking only files that are images.
        for entry in walker {
            let entry = entry.expect("Didn't find this file!");
            if entry.file_type().is_file() {
                let path = entry.path().to_path_buf();
                let mime = mime_guess::from_path(&path).first_raw().unwrap_or_default();
                if mime.starts_with("image/") {
                    image_paths.push(path.clone()); // Push the full file path to the images
                }
            }
        }

        // After making sure the picture doesn't exist yet, insert it into the database.
        for image_path in &image_paths {
            let picture = picture::Picture::new(&self, &image_path);

            match picture {
                Ok(_picture) => {
                    continue;
                }
                Err(err) => {
                    if err == Error::InternalError(InternalError::AlreadyExisted) {
                        println!("Skipping over picture: already in library");
                        continue;
                    } else {
                        return Err(err);
                    }
                }
            }
        }
        Ok(())
    }

    /// List all pictures in the library
    pub fn list_all_pictures(&self) -> Result<Vec<picture::Picture>, Error> {
        // For now, only the database is used as a source, as it should be the most up to date. 
        self.db.list_all_pictures()
    }

    /// Manually add a picture to a library from a filename.
    pub fn add_picture(&self, filename: &PathBuf) -> Result<(), Error> {
        picture::Picture::new(self, &filename).unwrap();
        Ok(())
    }
}

// Make sure that we don't search for pictures in hidden folders.
// TODO: Add cross-platform agnostic detection.
fn is_hidden_folder(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
