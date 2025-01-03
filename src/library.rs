use mime_guess;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::error::LumenzaError;
use crate::picture;
use crate::systems::config;
use crate::systems::database;
use crate::systems::filesystem;

pub struct Library {
    pub(crate) _fs: filesystem::Filesystem,
    pub(crate) db: database::Database,
}

// Static methods.
impl Library {
    /// This function will create the files at the given paths.
    pub fn create(
        config_path: &PathBuf,
        thumbnails_path: &PathBuf,
        pictures_paths: &Vec<PathBuf>,
        database_path: &PathBuf,
    ) -> Result<Self, LumenzaError> {
        // Initialize config, database, and file systems.
        let fs = filesystem::Filesystem::new(config_path, thumbnails_path, pictures_paths)?;
        let db = database::Database::new(database_path)?;

        let mut pictures_strs = Vec::new();
        for path in pictures_paths {
            pictures_strs.push(path.as_path().to_str().unwrap().to_string());
        }
        config::Config::write_config(
            &config_path.as_path().to_str().unwrap().to_string(),
            &pictures_strs,
            &thumbnails_path.as_path().to_str().unwrap().to_string(),
            &database_path.as_path().to_str().unwrap().to_string(),
        )?;

        Ok(Library { _fs: fs, db: db })
    }

    pub fn open(config_path: &PathBuf) -> Result<Self, LumenzaError> {
        let config = config::Config::read_config(config_path)?;

        let thumbnails_path = config.get_thumbnails_path();
        let pictures_paths = config.get_pictures_path();
        let database_path = config.get_database_path();

        let fs = filesystem::Filesystem::open(config_path, &thumbnails_path, &pictures_paths)?;
        let db = database::Database::open(&database_path)?;

        Ok(Library { _fs: fs, db: db })
    }
}

// Instance methods.
impl Library {
    /// Scan a folder for any images that are not in the library yet. If the
    /// folder is not in the library, it will be added.
    pub fn scan_folder(&self, folder: &PathBuf) -> Result<(), LumenzaError> {
        let mut image_paths: Vec<PathBuf> = Vec::new();

        let walker = WalkDir::new(folder)
            .into_iter()
            .filter_entry(|e| !is_hidden_folder(e));

        // Walk through all the files inside it, taking only files that are images.
        for entry in walker {
            let entry = entry.expect("Didn't find this file!");
            if entry.file_type().is_file() {
                let path = entry.path().to_path_buf();
                let mime = mime_guess::from_path(&path).first_raw().unwrap_or_default();
                if mime.starts_with("image/") {
                    image_paths.push(path.clone()); // Push the full file path to the images
                }
            }
        }

        // After making sure the picture doesn't exist yet, insert it into the database.
        for image_path in &image_paths {
            let picture = picture::Picture::new(&self, &image_path);

            match picture {
                Ok(_picture) => {
                    continue;
                }
                Err(err) => {
                    // Not a fatal error in this case, as we can just skip over the picture.
                    if err == LumenzaError::PictureAlreadyInLibrary() {
                        println!("Skipping over picture: already in library");
                        continue;
                    } else {
                        return Err(err);
                    }
                }
            }
        }
        Ok(())
    }

    /// List all pictures in the library
    pub fn list_all_pictures(&self) -> Result<Vec<picture::Picture>, LumenzaError> {
        // Only the database is used as a source, as it should be the most up to date.
        self.db.list_all_pictures()
    }

    /// This function is a bit of a one-off, as it will not add the folder
    /// the picture is in. It will only add the picture itself.
    pub fn add_picture(&self, filename: &PathBuf) -> Result<(), LumenzaError> {
        picture::Picture::new(self, &filename)?;
        Ok(())
    }
}

// Make sure that we don't search for pictures in hidden folders.
// TODO: Add cross-platform agnostic detection.
fn is_hidden_folder(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
