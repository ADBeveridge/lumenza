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

use std::path::Path;

use crate::error::Error;
use crate::systems::database;
use crate::systems::filesystem;

pub struct Library {
    pub fs: filesystem::Filesystem,
    pub db: database::Database,
}

impl Library {
    /// This function will create the files at the given paths.
    pub fn create(
        config: &Path,
        thumbnails: &Path,
        pictures: &Path,
        database: &Path,
    ) -> Result<Self, Error> {
        let fs = filesystem::Filesystem::new(config, thumbnails, pictures).unwrap();
        let db = database::Database::new(database).unwrap();

        // Return it.
        Ok(Library { fs: fs, db: db })
    }

    /// Loads an existing ospl Library from a config file.
    pub fn load<P: AsRef<Path>>(_config: P) -> Result<Self, Error> {
        unimplemented!()
    }

    /// Deletes the library files
    pub fn delete<P: AsRef<Path>>(_config: P) -> Result<Self, Error> {
        unimplemented!()
    }
}
