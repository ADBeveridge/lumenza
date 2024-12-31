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

use crate::error::Error;
use crate::picture;

use rusqlite::Connection;
use std::path::PathBuf;

#[path = "sql.rs"]
mod sql_schema;

/// The database structure manages the connection to the db and every db entry.
pub struct Database {
    pub connection: Connection,
}

// Static Methods
impl Database {
    /// Create the database object and file, and inserts the main structure
    pub fn new(path: &PathBuf) -> Result<Self, Error> {
        let db = Database {
            connection: Connection::open(path).unwrap(),
        };

        let sql = sql_schema::sql_schema();
        let sql = sql.as_str();

        db.connection.execute_batch(sql).unwrap();
        Ok(db)
    }
    pub fn open(path: &PathBuf) -> Result<Self, Error> {
        let db = Database {
            connection: Connection::open(path).unwrap(),
        };
        Ok(db)
    }
}

// Instance Methods
impl Database {
    pub fn write_picture(&self, picture: &picture::Picture) -> Result<bool, Error> {
        self.connection
            .execute(
                "INSERT INTO pictures (filename) VALUES (?1)",
                (&picture.filename.to_str(),),
            )
            .unwrap();
        Ok(true)
    }
    /// Search for a picture in the database
    pub fn lookup_picture(&self, picture: &picture::Picture) -> Result<bool, Error> {
        let mut stmt = self
            .connection
            .prepare("SELECT filename FROM pictures WHERE filename = ?")
            .unwrap();
        let mut rows = stmt.query(&[&picture.get_filename().to_str()]).unwrap();

        // TODO: It returns on the first match, but it should check if there are more than one match.
        while let Some(row) = rows.next().unwrap() {
            // Get the filename from the database.
            let res: String = row.get(0).unwrap();

            // Compare it with our filename.
            if res == picture.get_filename().to_str().unwrap() {
                return Ok(true);
            }
        }
        Ok(false)
    }
    pub fn list_all_pictures(&self) -> Result<Vec<picture::Picture>, Error> {
        let mut pictures: Vec<picture::Picture> = Vec::new();
        let mut stmt = self.connection.prepare("SELECT * FROM pictures").unwrap();
        let mut rows = stmt.query(()).unwrap();
        while let Some(row) = rows.next().unwrap() {
            let filename: String = row.get(1).unwrap();
            let picture = picture::Picture {
                id: row.get(0).unwrap(),
                filename: PathBuf::from(filename),
            };
            pictures.push(picture);
        }
        Ok(pictures)
    }
}
