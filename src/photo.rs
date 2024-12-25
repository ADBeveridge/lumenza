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

use crate::error::Error;
use crate::error::InternalError;
use crate::library;

pub struct Photo {
    pub filename: PathBuf,
}

impl Photo {
    /// Create a new photo entry in the library
    pub fn new(library: &library::Library, file: &PathBuf) -> Result<Self, Error> {
        let cwd = env::current_dir().unwrap();
        let full_path;

        // Check if the path is relative or absolute.
        if file.to_path_buf().is_absolute() == false {
            full_path = cwd.join(file.to_path_buf());
        } else {
            full_path = file.to_path_buf();
        }
        // Make sure the photo exists.
        if std::fs::exists(full_path.clone()).unwrap() != true {
            return Err(Error::InternalError(InternalError::PathNotExist));
        }

        let photo = Photo {
            filename: full_path,
        };

        // If photo was already in the database, skip insertion.
        let res = library.db.lookup_photo(&photo).unwrap();
        if res == true {
            return Err(Error::InternalError(InternalError::AlreadyExisted));
        }

        library.db.write_photo(&photo).unwrap();

        // Check if the insert was successful.
        let res = library.db.lookup_photo(&photo).unwrap();
        if res == false {
            return Err(Error::DatabaseError(rusqlite::Error::InvalidQuery));
        }

        Ok(photo)
    }
}

// Getters
impl Photo {
    pub fn get_filename(&self) -> PathBuf {
        self.filename.to_path_buf()
    }
}
