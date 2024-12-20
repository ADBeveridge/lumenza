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

use crate::Database;
use crate::Library;
use crate::OsplError;
use crate::Photo;

use image::imageops::thumbnail;

// Photo related API's for the Library object.
impl Library {
    /// Get a Photo element from an id
    pub fn get_photo_from_id(&self, id: u32) -> Result<Photo, OsplError> {
        let db = Database::new(self.fs.get_database_path())?;
        let mut photo = Photo::default();
        db.load_from_id(&mut photo, id)?;
        Ok(photo)
    }

    /// Get all photos in a Vec<Photo>
    ///
    /// This function gets all photos from the library, and all data related
    /// to the photos inside the Photo struct.
    pub fn list_all_photos(&self) -> Result<Vec<Photo>, OsplError> {
        <Photo as crate::element::traits::ElementListing<Photo>>::list_all(&self.db, &self.fs)
    }

    /// Deletes a photo with given id
    pub fn delete_photo_by_id(&self, id: u32) -> Result<(), OsplError> {
        let db = Database::new(self.fs.get_database_path())?;
        let photo = self.get_photo_from_id(id)?;
        self.fs.remove(&photo)?;
        db.delete(&photo)
    }

    /// Creates a thumbnail, and saves the name of the thumbnail to the Photo object. Does not save to database.
    pub fn create_thumbnail(&self, photo: &mut Photo) -> Result<(), OsplError> {
        // Get our paths together.
        let root_path = String::from(self.fs.get_thumbnails_path().to_str().unwrap());
        let thumbnail_name = uuid::Uuid::new_v4().to_string();
        let full_path = format!("{root_path}/{thumbnail_name}.jpg");

        // Generate the thumbnail.
        let image = image::open(photo.get_filename()).unwrap();
        let new_height: u32 = 325;
        let new_width: u32 = (image.width() * new_height) / image.height();
        let img = thumbnail(&image, new_width, new_height);
        img.save(full_path).unwrap();

        // Now save the name to the photo object.
        photo.thumbnail_name = thumbnail_name;

        Ok(())
    }

    pub fn get_thumbnail(&self, photo: &Photo) -> Result<String, OsplError> {
        let root_path = String::from(self.fs.get_thumbnails_path().to_str().unwrap());
        let thumbnail_name = photo.get_thumbnail_name();
        let full_path = format!("{root_path}/{thumbnail_name}.jpg");

        Ok(full_path)
    }
}
