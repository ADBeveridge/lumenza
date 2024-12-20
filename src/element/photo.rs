/*	libospl - Open Source Photo Library
    an opensource and multiplateform photo library management that can be used
    to store and sort all your photos.
    Copyright (C) 2019-2022 Angelo Frangione

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

use crate::element::traits::ElementDatabase;
use crate::element::traits::ElementFilesystem;
use crate::element::traits::ElementListing;
use crate::Database;
use crate::Filesystem;
use crate::{Error, OsplError};

use xxhash_rust::xxh3::xxh3_128;

use std::path::Path;

/// Structure containing a replica of sqlite data
#[derive(Debug)]
#[allow(dead_code)]
pub struct Photo {
    pub id: u32,
    /// Filesystem path to the photo.
    filename: String,
    /// Unique identifier of an image. Mostly used to detect duplicates
    hash: u128,
    /// Width of the image
    width: i32,
    /// Length of the image
    height: i32,
    /// The name of the thumbnail image. Does not contain the filesystem path, that is supplied by the fs module
    pub thumbnail_name: String,
    /// False if the photo is not missing.
    missing: i32,
}

impl Default for Photo {
    fn default() -> Self {
        Photo::new()
    }
}
// Constructors
impl Photo {
    /// Returns an empty Photo element
    pub fn new() -> Self {
        Photo {
            id: 0,
            filename: String::from(""),
            width: 0,
            height: 0,
            hash: 0,
            thumbnail_name: String::from(""),
            missing: 0,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_filename(&self) -> String {
        self.filename.clone()
    }

    pub fn get_thumbnail_name(&self) -> String {
        self.thumbnail_name.clone()
    }
    
    pub fn get_width(&self) -> i32 {
        self.width.clone()
    }
    
    pub fn get_height(&self) -> i32 {
        self.height.clone()
    }
    
    pub fn get_is_missing(&self) -> i32 {
        self.missing.clone()
    }
    
    pub fn set_is_missing(&mut self, is_missing: i32) {
        self.missing = is_missing;
    }

    /// Gets data from an image file and fills self with basic data. No database operations executed.
    pub fn from_file<P: AsRef<Path>>(
        &mut self,
        _db: &Database,
        photo_path: P,
    ) -> Result<(), OsplError> {
        if photo_path.as_ref().is_dir() {
            return Err(OsplError::InternalError(Error::IsADirectory));
        }
        if !is_photo(&photo_path)? {
            return Err(OsplError::InternalError(Error::NotAnImage));
        }
    
        match imagesize::size(&photo_path) {
            Ok(size) => {
                self.width = size.width as i32;
                self.height = size.height as i32; 
            },
            Err(why) => println!("Error getting dimensions: {:?}", why)
        }
        self.filename = photo_path.as_ref().to_str().unwrap().to_string();
        self.hash = xxh3_128(&std::fs::read(&photo_path)?);
        Ok(())
    }
}

impl ElementDatabase for Photo {
    /// Deletes the photo from the database
    fn delete(&self, db: &Database) -> Result<(), OsplError> {
        db.connection
            .execute("DELETE FROM photos WHERE id = ?1", [&self.id])?;
        Ok(())
    }

    /// Ensure the photo is in the database already.
    fn check_existence(&self, db: &Database) -> Result<bool, OsplError> {
        let mut stmt = db.connection.prepare("SELECT filename FROM photos WHERE filename = ?")?;
        let mut rows = stmt.query(&[&self.get_filename().to_string()])?;

        // TODO: It returns on the first match, but it should check if there are more than one match.
        while let Some(row) = rows.next()? {
            // Get the filename from the database.
            let res: String = row.get(0)?;

            // Compare it with our filename.
            if res == self.get_filename().to_string() {
                return Ok(true);
            }
        }

        println!("{} not found in database", self.get_filename().to_string());
        Ok(false)
    }

    fn update(&self, db: &Database) -> Result<(), OsplError> {
        // TODO: Update more stuff.
        db.connection.execute(
            "UPDATE photos SET thumbnail_name = ?1 WHERE id = ?2",
            (&self.thumbnail_name, &self.id),
        )?;

        println!("Updated photo id is {}", self.id);
        Ok(())
    }

    /// Insert a photo into the database, returns the id of it.
    fn insert_into(&self, db: &Database) -> Result<u32, OsplError> {
        db.connection.execute(
            "INSERT INTO photos (filename, hash, thumbnail_name, width, height, missing) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                &self.filename,
                &self.hash.to_ne_bytes(),
                &self.thumbnail_name,
                &self.width,
                &self.height,
                &self.missing,
            ),
        )?;
        println!("Imported file:\n{:#?}", &self);
        Ok(db.connection.last_insert_rowid() as u32)
    }

    fn rename(&self, _db: &Database, _new_name: &str) -> Result<(), OsplError> {
        unimplemented!()
    }

    /// loads the photo object with data from db with its id
    fn load_from_id(&mut self, db: &Database, id: u32) -> Result<(), OsplError> {
        let mut stmt = db
            .connection
            .prepare("SELECT * FROM photos WHERE id = ?1")?;
        let mut rows = stmt.query([&id])?;
        while let Some(row) = rows.next()? {
            self.id = row.get(0)?;
            self.filename = row.get(1)?;
            self.hash = u128::from_ne_bytes(row.get(2)?);
            self.thumbnail_name = row.get(3)?;
            self.width = row.get(4)?;
            self.height = row.get(5)?;
            self.missing = row.get(6)?;
        }
        if self.id == 0 {
            println!("ID was zero.");
            return Err(OsplError::IoError(std::io::ErrorKind::NotFound));
        }
        Ok(())
    }
}

impl ElementFilesystem for Photo {
    /// Remove everything related to a photo from the filesystem.
    fn remove_from(&self, _fs: &Filesystem) -> Result<(), OsplError> {
        unimplemented!()
    }

    fn rename(&self, _fs: &Filesystem, _new_name: &str) -> Result<(), OsplError> {
        unimplemented!()
    }
}

impl ElementListing<Photo> for Photo {
    fn list_all(db: &Database, _fs: &Filesystem) -> Result<Vec<Photo>, OsplError> {
        let mut photos: Vec<Photo> = Vec::new();
        let mut stmt = db.connection.prepare("SELECT * FROM photos")?;
        let mut rows = stmt.query(())?;
        while let Some(row) = rows.next()? {
            let photo = Photo {
                id: row.get(0)?,
                filename: row.get(1)?,
                hash: u128::from_ne_bytes(row.get(2)?),
                thumbnail_name: row.get(3)?,
                width: row.get(4)?,
                height: row.get(5)?,
                missing: row.get(6)?,
            };
            photos.push(photo);
        }
        Ok(photos)
    }
}

/// Checks if the file is an image
fn is_photo<P: AsRef<Path>>(path: P) -> Result<bool, OsplError> {
    match infer::get_from_path(path)? {
        Some(t) => {
            if t.matcher_type() == infer::MatcherType::Image {
                return Ok(true);
            }
        }
        None => {
            return Err(OsplError::InternalError(Error::NotAnImage));
        }
    }
    Ok(false)
}
