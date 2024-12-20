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

use crate::database::Database;
use crate::element::traits::ElementFilesystem;
use crate::OsplError;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// The Filesystem structure manages every file and directory in the library.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Filesystem {
    pictures_path: PathBuf,
    thumbnails_path: PathBuf,
    database_path: PathBuf,
}

impl Filesystem {
    /// Creates a filesystem object, and returns it.
    pub(crate) fn new<P: AsRef<Path>>(
        thumbnails: P,
        pictures: P,
        database: P,
    ) -> Result<Self, OsplError> {
        // Check if paths exist later.
        return Ok(Filesystem {
            thumbnails_path: thumbnails.as_ref().to_path_buf(),
            pictures_path: pictures.as_ref().to_path_buf(),
            database_path: database.as_ref().to_path_buf(),
        });
    }

    /// Create the filesystem object and creates the main fs structure
    pub(crate) fn create<P: AsRef<Path>>(
        thumbnails: P,
        pictures: P,
        database: P,
    ) -> Result<Self, OsplError> {
        // Create empty stuff for a new library.
        std::fs::create_dir_all(PathBuf::from(thumbnails.as_ref()))?;
        std::fs::create_dir_all(PathBuf::from(pictures.as_ref()))?;
        Database::create(PathBuf::from(database.as_ref()))?;

        // Now that the folders are there, we can create the whole thing.
        let fs = Self::new(thumbnails, pictures, database)?;

        Ok(fs)
    }
}

// Getters
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

// Direct API for Filesystem struct.
impl Filesystem {
    /// Rename the element in the library filesystem
    ///
    /// If fs.rename(object) is called it will call object.rename(Filesystem struct)
    #[allow(dead_code)]
    pub(crate) fn rename(
        &self,
        object: &dyn ElementFilesystem,
        new_name: &str,
    ) -> Result<(), OsplError> {
        object.rename(self, new_name)
    }

    /// Removes the element from the library filesystem
    ///
    /// If fs.remove(object) is called, it will call object.remove_from(Filesystem struct)
    pub(crate) fn remove(&self, object: &dyn ElementFilesystem) -> Result<(), OsplError> {
        object.remove_from(self)
    }
}
