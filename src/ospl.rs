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

pub static VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
pub static VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
pub static VERSION_REVISION: &str = env!("CARGO_PKG_VERSION_PATCH");

mod systems;

pub mod api;
pub mod element;

use database::Database;
pub use element::photo::Photo;
use filesystem::Filesystem;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum OsplError {
    DatabaseError(rusqlite::Error),
    IoError(std::io::ErrorKind),
    InternalError(Error),
}

#[cfg(not(tarpaulin_include))]
impl std::fmt::Display for OsplError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OsplError::DatabaseError(e) => write!(f, "Database error: {}", e),
            OsplError::IoError(e) => write!(f, "IO error: {}", e),
            OsplError::InternalError(e) => write!(f, "Internal error: {:?}", e),
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl From<image::ImageError> for OsplError {
    fn from(error: image::ImageError) -> Self {
        match error {
            image::ImageError::Unsupported(_) => {
                OsplError::IoError(std::io::ErrorKind::Unsupported)
            }
            image::ImageError::IoError(e) => OsplError::IoError(e.kind()),
            image::ImageError::Decoding(_) => OsplError::InternalError(Error::NotAnImage),
            image::ImageError::Limits(_) => OsplError::IoError(std::io::ErrorKind::OutOfMemory),
            _ => OsplError::InternalError(Error::Other),
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl From<Error> for OsplError {
    fn from(err: Error) -> Self {
        OsplError::InternalError(err)
    }
}

#[cfg(not(tarpaulin_include))]
impl From<rusqlite::Error> for OsplError {
    fn from(err: rusqlite::Error) -> Self {
        OsplError::DatabaseError(err)
    }
}

#[cfg(not(tarpaulin_include))]
impl From<std::io::Error> for OsplError {
    fn from(err: std::io::Error) -> Self {
        OsplError::IoError(err.kind())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// other error
    Other = -1000,
    /// The file is not supported by the library
    NotAnImage,
    /// A directory was specified when a non-directory was expected.
    IsADirectory,
    /// No name was specified
    EmptyName,
    /// Path did not exist
    PathNotExist,
}

#[derive(Debug)]
pub struct Library {
    pub fs: Filesystem, // Stores where the images, the thumbnails, the database, etc, are stored.
    #[allow(unused)]
    config: PathBuf, // Contains the locations of the Filesystem values.
    #[allow(unused)]
    pub db: Database,
}

impl Default for Library {
    fn default() -> Self {
        Self {
            db: Database::new("please_delete").unwrap(),
            fs: Filesystem::default(),
            config: PathBuf::default(),
        }
    }
}
