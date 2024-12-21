/*	libospl - Open Source Photo Library
    an opensource and multiplateform photo library management that can be used
    to store and sort all your photos.
    Copyright (C) 2019-2023 Angelo Frangione & Alan Beveridge

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
use std::path::{PathBuf, Path};
use crate::error::OsplError;

#[derive(Default)]
pub struct Filesystem {
    library_path: PathBuf,
    pictures_path: PathBuf,
    thumbnails_path: PathBuf,
    database_path: PathBuf,
}

// Static Methods
impl Filesystem {
    /// Creates a filesystem object, and returns it.
    pub fn new<P: AsRef<Path>>(
        library: P,
        thumbnails: P,
        pictures: P,
        database: P,
    ) -> Result<Self, OsplError> {
        return Ok(Filesystem {
            library_path: library.as_ref().to_path_buf(),
            thumbnails_path: thumbnails.as_ref().to_path_buf(),
            pictures_path: pictures.as_ref().to_path_buf(),
            database_path: database.as_ref().to_path_buf(),
        });
    }
    //pub fn create_folder<P: AsRef<Path>>(path: P) -> Result<Self, OsplError>
}

// Instance Methods
impl Filesystem {
    /// Returns the path on filesystem to the pictures path in the library
    pub fn get_pictures_path(&self) -> PathBuf {
        self.pictures_path.to_path_buf()
    }

    /// Returns the path on filesystem to the thumbnails path in the library
    pub fn get_thumbnails_path(&self) -> PathBuf {
        self.thumbnails_path.to_path_buf()
    }

    /// Returns the path on filesystem of the database file
    pub fn get_database_path(&self) -> PathBuf {
        self.database_path.clone()
    }
}
