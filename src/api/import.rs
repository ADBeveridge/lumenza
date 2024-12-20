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

use crate::Library;
use crate::OsplError;
use crate::Photo;
use mime_guess::from_path;
use std::path::PathBuf;
use walkdir::WalkDir;

// Related to tasks like scanning for new photos, or importing new photos.
impl Library {
    // This function will scan the folder for photos not in the database, and import them into the database.
    pub fn scan_new_photos(&mut self) -> Result<bool, OsplError> {
        let folder_path = self.fs.get_pictures_path();
        let mut image_paths: Vec<PathBuf> = Vec::new();

        let walker = WalkDir::new(folder_path)
            .into_iter()
            .filter_entry(|e| !is_hidden_folder(e));

        // Walk through all the files inside it, taking only files that are images.
        for entry in walker {
            let entry = entry.expect("Didn't find this file!");
            if entry.file_type().is_file() {
                let path = entry.path().to_path_buf();
                let mime = from_path(&path).first_raw().unwrap_or_default();
                if mime.starts_with("image/") {
                    image_paths.push(path.clone()); // Push the full file path to the images
                }
            }
        }

        // Now image_paths contains PathBufs to all image files in and below the specified folder
        for image_path in &image_paths {
            let mut photo = Photo::default(); 
            photo.from_file(&self.db, &image_path)?;
            
            let res = self.db.check_existence(&photo)?;
            if res == true {
                // The photo is already in the database, so we can skip it.
                continue;
            }           
            //self.create_thumbnail(&mut photo)?; //TODO: Not here. 

            self.db.insert(&photo)?;
            
            let res = self.db.check_existence(&photo)?;
            if res == false {
                println!("Error: photo not inserted into database");
            }
        }
        Ok(true)
    }
    
    /// Check if all photos exist on the filesystem. Marks the db entries that are not on the fs as missing. 
    pub fn check_photos_existence(&mut self) -> Result<(), OsplError> {
        let photos: Vec<Photo> = self.list_all_photos().unwrap();
        for mut photo in photos {
            let filename = photo.get_filename();
            let exists = std::path::Path::new(&filename).try_exists()?;
            if exists == false {
                println!("Could not find file {}", filename);
                photo.set_is_missing(1);
            }
            
        }
        Ok(())
    }
}

fn is_hidden_folder(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
