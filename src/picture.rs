use std::path::PathBuf;
use path_absolutize::Absolutize;

use crate::error::LumenzaError;
use crate::library;

pub struct Picture {
    pub id: u32,
    pub filename: PathBuf,
    pub thumbnail: PathBuf,
}

impl Picture {
    /// Create a new picture entry in the library
    pub(crate) fn new(library: &library::Library, file: &PathBuf) -> Result<Self, LumenzaError> {
        let full_path = file.absolutize().unwrap_or_default().into_owned();

        // Can't use default error because std::io::error is not implemented in LumenzaError.
        if !std::fs::metadata(full_path.clone()).is_ok() {
            return Err(LumenzaError::FileNotFound());
        }
        
        let picture = Picture {
            id: 0,
            filename: full_path,
            thumbnail: PathBuf::new(),
        };

        // If picture was already in the database, skip insertion.
        let res = library.db.lookup_picture(&picture)?;
        if res == true {
            return Err(LumenzaError::PictureAlreadyInLibrary());
        }

        library.db.write_picture(&picture)?;

        // Check if the insert was successful.
        let res = library.db.lookup_picture(&picture)?;
        if res == false {
            return Err(LumenzaError::DatabaseError(rusqlite::Error::InvalidQuery));
        }

        Ok(picture)
    }
}

// Getters
impl Picture {
    pub fn get_filename(&self) -> PathBuf {
        self.filename.to_path_buf()
    }
}
