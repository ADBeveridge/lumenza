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

use std::path::{Path, PathBuf};

use crate::error::Error;

#[derive(Default)]
pub struct Filesystem {
    config_path: PathBuf,
    pictures_path: PathBuf,
    thumbnails_path: PathBuf,
}

// Static Methods
impl Filesystem {
    // Initialize filsystem structure for library.
    pub fn new<P: AsRef<Path>>(
        config: P,
        thumbnails: P,
        pictures: P,
    ) -> Result<Self, Error> {
        // Create empty stuff for a new library.
        std::fs::create_dir_all(PathBuf::from(thumbnails.as_ref())).unwrap();
        std::fs::create_dir_all(PathBuf::from(pictures.as_ref())).unwrap();
        std::fs::File::create(config.as_ref()).unwrap();

        // Now that the files (except the database) are there, we can create the whole thing.
        let fs = Self::open(config, thumbnails, pictures).unwrap();

        Ok(fs)
    }

    /// Loads the file paths into the Filesystem object and returns it.
    pub fn open<P: AsRef<Path>>(config_path: P, thumbnails: P, pictures: P) -> Result<Self, Error> {
        return Ok(Filesystem {
            config_path: config_path.as_ref().to_path_buf(),
            thumbnails_path: thumbnails.as_ref().to_path_buf(),
            pictures_path: pictures.as_ref().to_path_buf(),
        });
    }
}

// Instance Methods
impl Filesystem {
    /// Returns the path on filesystem to the pictures path in the library
    pub fn get_config_path(&self) -> PathBuf {
        self.config_path.to_path_buf()
    }

    /// Returns the path on filesystem to the pictures path in the library
    pub fn get_pictures_path(&self) -> PathBuf {
        self.pictures_path.to_path_buf()
    }

    /// Returns the path on filesystem to the thumbnails path in the library
    pub fn get_thumbnails_path(&self) -> PathBuf {
        self.thumbnails_path.to_path_buf()
    }
}
