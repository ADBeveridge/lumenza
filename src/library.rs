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

use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use mime_guess;

use crate::error::Error;
use crate::error::InternalError;
use crate::systems::database;
use crate::systems::filesystem;
use crate::photo;

pub struct Library {
    pub fs: filesystem::Filesystem,
    pub db: database::Database,
}

impl Library {
    /// This function will create the files at the given paths.
    pub fn create(
        config: &Path,
        thumbnails: &Path,
        pictures: &Vec<PathBuf>,
        database: &Path,
    ) -> Result<Self, Error> {
        let fs = filesystem::Filesystem::new(config, thumbnails, pictures).unwrap();
        let db = database::Database::new(database).unwrap();

        // Return it.
        Ok(Library { fs: fs, db: db })
    }

    /// If folder is not in library already, add folder and photos to library
    pub fn scan_folder(&self, folder: &Path) -> Result<(), Error> {
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

        // After making sure the photo doesn't exist yet, insert it into the database.
        for image_path in &image_paths {
            let photo = photo::Photo::new(&self, &image_path);

            match photo {
                Ok(_photo) => {
                    continue;
                }
                Err(err) => {
                    if err == Error::InternalError(InternalError::AlreadyExisted) {
                        println!("Skipping over photo: already in library");
                        continue;
                    } else {
                        return Err(err);
                    }
                }
            }
        }
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
