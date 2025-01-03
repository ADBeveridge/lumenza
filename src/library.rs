use mime_guess;
use path_absolutize::Absolutize;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::error::LumenzaError;
use crate::picture::Picture;
use crate::systems::config;
use crate::systems::database;

pub struct Library {
    pub(crate) config: config::Config,
    pub(crate) database: database::Database,
}

// Static methods.
impl Library {
    /// Create a new library with the given paths. Files and folderes will be 
    /// created if they do not exist. The thumbnail path is just a suggestion, 
    /// as the library allows adding photos/thumbnails that are outside of the 
    /// given paths. You set picture folders later for the sake of easier 
    /// integration with GUI applications.
    pub fn create(
        config_path: &PathBuf,
        thumbnails_path: &PathBuf,
        database_path: &PathBuf,
    ) -> Result<Self, LumenzaError> {
        // Initialize all files and folders.
        let db = database::Database::new(database_path)?;
        let cfg = config::Config::new(config_path, &vec![], thumbnails_path, database_path)?;
        std::fs::create_dir_all(thumbnails_path).map_err(|_| LumenzaError::IoError())?;

        Ok(Library {
            config: cfg,
            database: db,
        })
    }

    /// Open an existing library. The config file must exist, otherwise an error
    /// will be returned. The database file will be created if it does not exist, 
    /// and all pictures in the specified folders will be added to the database.
    pub fn open(config_path: &PathBuf) -> Result<Self, LumenzaError> {
        let cfg = config::Config::open(config_path)?;
        let db = database::Database::open(&cfg.get_database_path())?;

        Ok(Library {
            config: cfg,
            database: db,
        })
    }
}

// Instance methods.
impl Library {
    /// Scan a folder for any images that are not in the library yet. If the
    /// folder is not in the library, it will be added. Pictures that are marked
    /// as independent but are in the given folder will be marked as children
    /// of that folder.
    pub fn process_folder(&mut self, folder: &PathBuf) -> Result<(), LumenzaError> {
        let mut image_paths: Vec<PathBuf> = Vec::new();
        let full_path = folder.absolutize().unwrap_or_default().into_owned();

        let folders = self.config.get_pictures_path();
        if !folders.iter().any(|x| x == &full_path) {
            self.config.add_folder(&full_path)?;
        }

        let walker = WalkDir::new(full_path)
            .into_iter()
            .filter_entry(|e| !is_hidden_folder(e));

        // Walk through all the files inside it, taking only files that are images.
        for entry in walker {
            let entry = entry.map_err(|_| LumenzaError::FileNotFound())?;
            if entry.file_type().is_file() {
                let path = entry.path().to_path_buf();
                let mime = mime_guess::from_path(&path).first_raw().unwrap_or_default();
                // TODO: Add more image types.
                if mime.starts_with("image/") {
                    // We know that this is the full file path being pushed into the vector.
                    image_paths.push(path.clone());
                }
            }
        }

        // After making sure the picture doesn't exist yet, insert it into the database.
        for image_path in &image_paths {
            let picture = self.add_picture(image_path);

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

    /// Generate thumbnails for all pictures in the library. As of right now,
    /// this function is very inefficient. 
    pub fn generate_all_thumbnails(&self) -> Result<(), LumenzaError> {
        let pictures = self.list_all_pictures()?;
        for picture in pictures {
            // Generate thumbnail filename.
            let tfolder = self.config.get_thumbnails_path();
            let tpic = PathBuf::from(picture.filename.file_name().unwrap());
            let tfile = tfolder.join(tpic);
            picture.generate_thumbnail(&tfile)?;
        }
        Ok(())
    }

    /// List all pictures in the library
    pub fn list_all_pictures(&self) -> Result<Vec<Picture>, LumenzaError> {
        // Only the database is used as a source, as it should be the most up to date.
        self.database.list_all_pictures()
    }

    /// This function is a bit of a one-off, as it will not add the folder
    /// the picture is in. It will only add the picture itself. This function is
    /// intended for callers that want to implement lazy loading of pictures. Use
    /// process_folder() when finished with adding all pictures manually.
    pub fn add_picture(&self, filename: &PathBuf) -> Result<Picture, LumenzaError> {
        Picture::new(self, &filename)
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
