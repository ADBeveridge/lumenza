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

use crate::photo;
use crate::error::Error;

use rusqlite::Connection;
use std::path::Path;

#[path = "sql.rs"]
mod sql_schema;

/// The database structure manages the connection to the db and every db entry.
pub struct Database {
    pub connection: Connection,
}

// Static Methods
impl Database {
    /// Create the database object and file, and inserts the main structure
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let db = Database {
            connection: Connection::open(path.as_ref()).unwrap(),
        };

        let sql = sql_schema::sql_schema();
        let sql = sql.as_str();

        db.connection.execute_batch(sql).unwrap();
        Ok(db)
    }
}

// Instance Methods
impl Database {
    pub fn write_photo(&self, photo: &photo::Photo) -> Result<(), Error> {
        self.connection.execute(
            "INSERT INTO photos (filename) VALUES (?1)",
            (
                &photo.filename.to_str(),
            ),
        ).unwrap();
        Ok(())
    }
}
