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
use crate::Filesystem;
use crate::Library;
use crate::OsplError;
use std::fs::File;
use std::io::{BufWriter, Read};
use std::path::Path;
use std::path::PathBuf;

impl Library {
    /// This function will create a folder at the given path, create the database file, and create the filesystem object.
    pub fn create<P: AsRef<Path>>(
        config: P,
        thumbnails: P,
        pictures: P,
        database: P,
    ) -> Result<Self, OsplError> {
        let config_path_buf = PathBuf::from(config.as_ref());
        let config_path_buf2 = PathBuf::from(config.as_ref());

        // Create filesystem.
        let fs = Filesystem::create(thumbnails, pictures, database)?;

        // Save the FileSystem object into a config json object.
        let config_file = File::create(config_path_buf2)?;
        let writer = BufWriter::new(config_file);
        serde_json::to_writer_pretty(writer, &fs).unwrap();

        // TODO: this is rather wasteful. A database connection was created to create the fs. perhaps use that.
        let db = Database::new(fs.get_database_path()).unwrap();

        // Return it.
        Ok(Library {
            fs: fs,
            config: config_path_buf,
            db: db,
        })
    }

    /// Loads an existing ospl Library from a config file.
    pub fn load<P: AsRef<Path>>(config: P) -> Result<Self, OsplError> {
        let config_path_buf = PathBuf::from(config.as_ref());
        let config_path_buf2 = PathBuf::from(config.as_ref());

        let mut file = File::open(config_path_buf2)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let fs: Filesystem = serde_json::from_str(&contents).unwrap();
        let db = Database::new(fs.get_database_path()).unwrap();
        Ok(Library {
            fs: fs,
            config: config_path_buf,
            db: db,
        })
    }
}
