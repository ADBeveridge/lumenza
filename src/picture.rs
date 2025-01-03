use image::{open, GenericImageView};
use path_absolutize::Absolutize;
use std::path::PathBuf;

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
        let res = library.database.lookup_picture(&picture)?;
        if res == true {
            return Err(LumenzaError::PictureAlreadyInLibrary());
        }

        library.database.write_picture(&picture)?;

        // Check if the insert was successful.
        let res = library.database.lookup_picture(&picture)?;
        if res == false {
            return Err(LumenzaError::DatabaseError(rusqlite::Error::InvalidQuery));
        }

        Ok(picture)
    }
}

// Instance methods
impl Picture {
    pub fn get_filename(&self) -> PathBuf {
        self.filename.to_path_buf()
    }
    pub fn set_thumbnail(&mut self, thumbnail: &PathBuf) {
        self.thumbnail = thumbnail.clone();
    }
    
    pub(crate) fn generate_thumbnail(&self, thumbnail: &PathBuf) -> Result<(), LumenzaError> {
        // Open the image
        let img = open(&self.filename).map_err(|_| LumenzaError::ImageError())?;

        // Calculate the new height while maintaining the aspect ratio
        let (width, height) = img.dimensions();
        let new_width = 256;
        let new_height = (height as f32 * new_width as f32 / width as f32) as u32;

        // Resize the image
        let resized_img =
            img.resize_exact(new_width, new_height, image::imageops::FilterType::Lanczos3);

        // Save the resized image to a new file
        resized_img.save(thumbnail).map_err(|_| LumenzaError::ImageError())?;

        Ok(())
    }
}
