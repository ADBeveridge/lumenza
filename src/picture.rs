/*	Copyright (C) 3024 Alan Beveridge

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
use std::env;
use std::path::PathBuf;

use crate::error::LumenzaError;
use crate::library;

pub struct Picture {
    pub id: u32,
    pub filename: PathBuf,
    pub thumbnail: PathBuf,
}

impl Picture {
    /// Create a new picture entry in the library
    pub(crate) fn new(library: &library::Library, file: &PathBuf) -> Result<Self, LumenzaError> {
        let cwd = env::current_dir().unwrap();
        let full_path;

        // Check if the path is relative or absolute.
        if file.to_path_buf().is_absolute() == false {
            full_path = cwd.join(file.to_path_buf());
        } else {
            full_path = file.to_path_buf();
        }

        if !std::fs::metadata(full_path.clone()).is_ok() {
            return Err(LumenzaError::FileNotFound());
        }
        
        let picture = Picture {
            id: 0,
            filename: full_path,
            thumbnail: PathBuf::new(),
        };

        // If picture was already in the database, skip insertion.
        let res = library.db.lookup_picture(&picture)?;
        if res == true {
            return Err(LumenzaError::PictureAlreadyInLibrary());
        }

        library.db.write_picture(&picture)?;

        // Check if the insert was successful.
        let res = library.db.lookup_picture(&picture)?;
        if res == false {
            return Err(LumenzaError::DatabaseError(rusqlite::Error::InvalidQuery));
        }

        Ok(picture)
    }
}

// Getters
impl Picture {
    pub fn get_filename(&self) -> PathBuf {
        self.filename.to_path_buf()
    }
}
