/*	Copyright (C) 2019-2024 Angelo Frangione & Alan Beveridge

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

use std::path::PathBuf;

use crate::error::LumenzaError;

pub struct Filesystem {
    _config_path: PathBuf,
    _pictures_paths: Vec<PathBuf>,
    _thumbnails_path: PathBuf,
}

// Static Methods
impl Filesystem {
    // Initialize filsystem structure for library.
    pub(crate) fn new(
        config: &PathBuf,
        thumbnails: &PathBuf,
        pictures: &Vec<PathBuf>,
    ) -> Result<Self, LumenzaError> {
        // Create empty folders for a new library. Config file will be created by config system, same with database.
        std::fs::create_dir_all(PathBuf::from(thumbnails)).map_err(|_| LumenzaError::IoError())?;
        for path in pictures {
            std::fs::create_dir_all(path).map_err(|_| LumenzaError::IoError())?;
        }

        // Now that the files are there, we can create the whole thing.
        let fs = Self::open(config, thumbnails, pictures)?;

        Ok(fs)
    }

    /// Loads the file paths into the Filesystem object and returns it.
    pub(crate) fn open(
        config_path: &PathBuf,
        thumbnails_path: &PathBuf,
        pictures_paths: &Vec<PathBuf>,
    ) -> Result<Self, LumenzaError> {
        return Ok(Filesystem {
            _config_path: config_path.clone(),
            _thumbnails_path: thumbnails_path.clone(),
            _pictures_paths: pictures_paths.clone(),
        });
    }
}

