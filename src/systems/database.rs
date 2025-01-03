use crate::error::LumenzaError;
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
    pub fn new(path: &PathBuf) -> Result<Self, LumenzaError> {
        let db = Database {
            connection: Connection::open(path)?,
        };

        let sql = sql_schema::sql_schema();
        let sql = sql.as_str();

        db.connection.execute_batch(sql)?;
        Ok(db)
    }
    pub fn open(path: &PathBuf) -> Result<Self, LumenzaError> {
        let db = Database {
            connection: Connection::open(path)?,
        };
        Ok(db)
    }
}

// Instance Methods
impl Database {
    pub fn write_picture(&self, picture: &picture::Picture) -> Result<bool, LumenzaError> {
        self.connection
            .execute(
                "INSERT INTO pictures (filename) VALUES (?1)",
                (&picture.filename.to_str(),),
            )
            ?;
        Ok(true)
    }
    /// Search for a picture in the database
    pub fn lookup_picture(&self, picture: &picture::Picture) -> Result<bool, LumenzaError> {
        let mut stmt = self
            .connection
            .prepare("SELECT filename FROM pictures WHERE filename = ?")
            ?;
        let mut rows = stmt.query(&[&picture.get_filename().to_str()])?;

        // TODO: It returns on the first match, but it should check if there are more than one match.
        while let Some(row) = rows.next()? {
            // Get the filename from the database.
            let res: String = row.get(0)?;

            // Compare it with our filename.
            if res == picture.get_filename().to_str().unwrap() {
                return Ok(true);
            }
        }
        Ok(false)
    }
    pub fn list_all_pictures(&self) -> Result<Vec<picture::Picture>, LumenzaError> {
        let mut pictures: Vec<picture::Picture> = Vec::new();
        let mut stmt = self.connection.prepare("SELECT * FROM pictures")?;
        let mut rows = stmt.query(())?;
        while let Some(row) = rows.next()? {

            let id = row.get(0)?;
            let filename: String = row.get(1)?;
            let thumbnail: Option<String> = row.get(2).ok();

        
            let picture = picture::Picture {
                id: id,
                filename: PathBuf::from(filename),
                thumbnail: thumbnail.map(PathBuf::from).unwrap_or_default(),
            };
            pictures.push(picture);
        }
        Ok(pictures)
    }
}
